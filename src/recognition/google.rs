//! Google Input Tools handwriting recognition implementation
//!
//! Converts stroke data to text using the Google Input Tools API.
//! API endpoint: https://inputtools.google.com/request

use anyhow::{Result, Context};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use crate::stroke::Stroke;

/// Google Input Tools API client
pub struct GoogleRecognizer {
    client: reqwest::Client,
    api_url: String,
    language: String,
}

/// Request format for Google Input Tools API
#[derive(Debug, Serialize)]
struct RecognitionRequest {
    #[serde(rename = "app_version")]
    app_version: String,
    #[serde(rename = "api_level")]
    api_level: String,
    device: String,
    #[serde(rename = "input_type")]
    input_type: i32,
    options: String,
    requests: Vec<InkRequest>,
}

/// Individual ink recognition request
#[derive(Debug, Serialize)]
struct InkRequest {
    language: String,
    #[serde(rename = "writing_guide")]
    writing_guide: WritingGuide,
    ink: Vec<Vec<Vec<i32>>>,
}

/// Writing area dimensions
#[derive(Debug, Serialize)]
struct WritingGuide {
    width: i32,
    height: i32,
}

/// API response structure
/// Format: ["SUCCESS", [["request_id", ["candidate1", "candidate2", ...], [], {...}]]]
#[derive(Debug, Deserialize)]
struct RecognitionResponse(
    String,  // "SUCCESS" or error status
    Vec<ResponseResult>,
);

/// Result tuple: [request_id, candidates, empty_array, metadata]
#[derive(Debug, Deserialize)]
struct ResponseResult(
    String,           // request_id
    Vec<String>,      // candidate strings
    Vec<serde_json::Value>,  // empty array
    serde_json::Value,       // metadata object
);

impl GoogleRecognizer {
    /// Create a new Google Input Tools recognizer
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            api_url: "https://inputtools.google.com/request?ime=handwriting&app=mobilesearch&cs=1&oe=UTF-8".to_string(),
            language: "en".to_string(),
        })
    }

    /// Set recognition language (default: "en")
    pub fn with_language(mut self, language: &str) -> Self {
        self.language = language.to_string();
        self
    }

    /// Convert strokes to Google Input Tools ink format
    fn strokes_to_ink(&self, strokes: &[Stroke]) -> Vec<Vec<Vec<i32>>> {
        strokes
            .iter()
            .map(|stroke| {
                // Each stroke is [x_coords, y_coords, timestamps]
                let x_coords: Vec<i32> = stroke.points.iter().map(|p| p.x).collect();
                let y_coords: Vec<i32> = stroke.points.iter().map(|p| p.y).collect();
                let timestamps: Vec<i32> = stroke
                    .points
                    .iter()
                    .map(|p| p.timestamp_ms as i32)
                    .collect();

                vec![x_coords, y_coords, timestamps]
            })
            .collect()
    }

    /// Recognize handwriting strokes
    pub async fn recognize(&self, strokes: &[Stroke]) -> Result<RecognitionResult> {
        if strokes.is_empty() {
            return Ok(RecognitionResult {
                text: String::new(),
                confidence: 0.0,
                alternatives: vec![],
            });
        }

        debug!("Recognizing {} strokes", strokes.len());

        // Convert strokes to ink format
        let ink = self.strokes_to_ink(strokes);

        // Build request
        let request = RecognitionRequest {
            app_version: "0.1.0".to_string(),
            api_level: "537.36".to_string(),
            device: "jedusor-remarkable".to_string(),
            input_type: 0, // 0 = handwriting
            options: "enable_pre_space".to_string(),
            requests: vec![InkRequest {
                language: self.language.clone(),
                writing_guide: WritingGuide {
                    width: 1404,  // reMarkable 2 width
                    height: 1872, // reMarkable 2 height
                },
                ink,
            }],
        };

        // Send request with retry logic
        let response = self.send_with_retry(&request, 3).await?;

        // Parse response
        self.parse_response(response)
    }

    /// Send request with retry logic
    async fn send_with_retry(
        &self,
        request: &RecognitionRequest,
        max_retries: u32,
    ) -> Result<RecognitionResponse> {
        let mut last_error = None;

        for attempt in 0..max_retries {
            if attempt > 0 {
                let delay = std::time::Duration::from_millis(100 * 2_u64.pow(attempt));
                debug!("Retry attempt {} after {:?}", attempt + 1, delay);
                tokio::time::sleep(delay).await;
            }

            match self.send_request(request).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    warn!("Recognition request failed (attempt {}): {}", attempt + 1, e);
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow::anyhow!("Recognition failed after retries")))
    }

    /// Send single recognition request
    async fn send_request(&self, request: &RecognitionRequest) -> Result<RecognitionResponse> {
        let start = std::time::Instant::now();

        let response = self
            .client
            .post(&self.api_url)
            .json(request)
            .send()
            .await
            .context("Failed to send recognition request")?;

        let status = response.status();
        if !status.is_success() {
            anyhow::bail!("API returned error status: {}", status);
        }

        let response_text = response
            .text()
            .await
            .context("Failed to read response body")?;

        debug!("API response (raw): {}", response_text);

        let parsed: RecognitionResponse = serde_json::from_str(&response_text)
            .with_context(|| format!("Failed to parse API response. Raw response: {}", response_text))?;

        let duration = start.elapsed();
        info!("Recognition completed in {:?}", duration);

        Ok(parsed)
    }

    /// Parse API response and extract best candidate
    fn parse_response(&self, response: RecognitionResponse) -> Result<RecognitionResult> {
        // Check status
        if response.0 != "SUCCESS" {
            anyhow::bail!("API returned error status: {}", response.0);
        }

        // Check if we have results
        if response.1.is_empty() {
            return Ok(RecognitionResult {
                text: String::new(),
                confidence: 0.0,
                alternatives: vec![],
            });
        }

        let result = &response.1[0];
        let candidates = &result.1;  // Vec<String> of candidates

        if candidates.is_empty() {
            return Ok(RecognitionResult {
                text: String::new(),
                confidence: 0.0,
                alternatives: vec![],
            });
        }

        // Extract top candidate
        let text = candidates[0].clone();
        let confidence = 1.0;  // Google doesn't provide confidence scores in this format

        // Extract alternatives (up to 4)
        let alternatives: Vec<String> = candidates
            .iter()
            .skip(1)
            .take(4)
            .cloned()
            .collect();

        info!("Recognized text: '{}' (confidence: {:.2})", text, confidence);
        debug!("Alternatives: {:?}", alternatives);

        Ok(RecognitionResult {
            text,
            confidence,
            alternatives,
        })
    }
}

/// Recognition result with confidence and alternatives
#[derive(Debug, Clone)]
pub struct RecognitionResult {
    /// Recognized text (top candidate)
    pub text: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Alternative candidates
    pub alternatives: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stroke::Point;

    fn create_test_stroke() -> Stroke {
        let mut stroke = Stroke::new();
        // Simple horizontal line (might be recognized as "-" or similar)
        for i in 0..10 {
            stroke.add_point(Point {
                x: 100 + i * 10,
                y: 100,
                pressure: 2048,
                timestamp_ms: i as u64 * 10,
            });
        }
        stroke
    }

    #[test]
    fn test_stroke_to_ink_conversion() {
        let recognizer = GoogleRecognizer::new().unwrap();
        let stroke = create_test_stroke();
        let ink = recognizer.strokes_to_ink(&[stroke]);

        assert_eq!(ink.len(), 1); // One stroke
        assert_eq!(ink[0].len(), 3); // x, y, timestamps
        assert_eq!(ink[0][0].len(), 10); // 10 points in x coords
        assert_eq!(ink[0][1].len(), 10); // 10 points in y coords
        assert_eq!(ink[0][2].len(), 10); // 10 timestamps
    }

    #[tokio::test]
    #[ignore] // Requires network access
    async fn test_google_recognition_api() {
        let recognizer = GoogleRecognizer::new().unwrap();
        let stroke = create_test_stroke();

        let result = recognizer.recognize(&[stroke]).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        println!("Recognized: {:?}", result);
        // Note: Recognition accuracy depends on stroke data
    }
}
