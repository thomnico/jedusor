//! Handwriting recognition module
//!
//! Converts strokes to text using Google Input Tools API

pub mod google;

pub use google::{GoogleRecognizer, RecognitionResult};

use anyhow::Result;

/// Trait for handwriting recognition services
#[async_trait::async_trait]
pub trait Recognizer {
    /// Recognize strokes and convert to text
    async fn recognize(&self, strokes: &[crate::stroke::Stroke]) -> Result<RecognitionResult>;
}

/// Implement Recognizer trait for GoogleRecognizer
#[async_trait::async_trait]
impl Recognizer for GoogleRecognizer {
    async fn recognize(&self, strokes: &[crate::stroke::Stroke]) -> Result<RecognitionResult> {
        self.recognize(strokes).await
    }
}
