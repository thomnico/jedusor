//! Shared event handler - platform-agnostic business logic
//!
//! ALL gesture detection, recognition, and response logic is here.
//! This code is shared 100% between device and simulator.

use anyhow::Result;
use log::{info, debug, warn};
use crate::input::{WacomHandler, WacomEvent, GestureDetector, Gesture};
use crate::stroke::Stroke;
use crate::recognition::GoogleRecognizer;
use crate::platform::{Display, Color};

/// Shared event handler containing all business logic
pub struct EventHandler {
    wacom_handler: WacomHandler,
    gesture_detector: GestureDetector,
    recognizer: GoogleRecognizer,
    runtime: tokio::runtime::Runtime,
    all_strokes: Vec<Stroke>,
    pending_stroke_points: Vec<crate::stroke::Point>,
}

impl EventHandler {
    /// Create a new event handler
    pub fn new() -> Result<Self> {
        let wacom_handler = WacomHandler::new();
        let gesture_detector = GestureDetector::new();
        let recognizer = GoogleRecognizer::new()?;
        let runtime = tokio::runtime::Runtime::new()?;

        info!("Google Input Tools recognizer initialized");

        Ok(Self {
            wacom_handler,
            gesture_detector,
            recognizer,
            runtime,
            all_strokes: Vec::new(),
            pending_stroke_points: Vec::new(),
        })
    }

    /// Handle a single Wacom event - SHARED LOGIC FOR ALL PLATFORMS
    pub fn handle_event<D: Display>(
        &mut self,
        event: WacomEvent,
        display: &mut D,
    ) -> Result<()> {
        // Track points for real-time rendering
        match &event {
            WacomEvent::ToolDown { x, y, pressure, .. } => {
                debug!("ToolDown at ({}, {}) pressure={}", x, y, pressure);
                self.pending_stroke_points.clear();
                self.pending_stroke_points.push(crate::stroke::Point {
                    x: *x,
                    y: *y,
                    pressure: *pressure,
                    timestamp_ms: 0,
                });
            }
            WacomEvent::ToolMove { x, y, pressure } => {
                self.pending_stroke_points.push(crate::stroke::Point {
                    x: *x,
                    y: *y,
                    pressure: *pressure,
                    timestamp_ms: 0,
                });

                // Draw line from last point to current point (real-time rendering)
                if self.pending_stroke_points.len() >= 2 {
                    let len = self.pending_stroke_points.len();
                    let p1 = &self.pending_stroke_points[len - 2];
                    let p2 = &self.pending_stroke_points[len - 1];

                    display.draw_line(p1.x, p1.y, p2.x, p2.y, 3, Color::BLACK);
                }
            }
            _ => {}
        }

        // Process event through wacom handler
        match self.wacom_handler.handle_event(event)? {
            Some(stroke) => {
                // Stroke completed
                info!("✨ Stroke completed with {} points", stroke.points.len());
                self.pending_stroke_points.clear();

                // Check for gestures
                match self.gesture_detector.detect(&stroke) {
                    Gesture::Circle { center_x, center_y, .. } => {
                        self.handle_circle_gesture(center_x, center_y, display)?;
                    }
                    Gesture::Underline { .. } => {
                        info!("Underline gesture detected");
                    }
                    Gesture::Lasso { .. } => {
                        info!("Lasso gesture detected");
                    }
                    Gesture::None => {
                        debug!("Regular stroke (no gesture)");
                    }
                }

                // Add stroke to collection
                self.all_strokes.push(stroke);
            }
            None => {
                // Stroke in progress - already drawn in real-time above
            }
        }

        Ok(())
    }

    /// Handle circle gesture - triggers handwriting recognition
    fn handle_circle_gesture<D: Display>(
        &mut self,
        center_x: i32,
        center_y: i32,
        display: &mut D,
    ) -> Result<()> {
        info!("Circle gesture detected at ({}, {})", center_x, center_y);

        // Trigger recognition on all accumulated strokes (excluding the circle)
        if !self.all_strokes.is_empty() {
            info!("Recognizing {} accumulated strokes", self.all_strokes.len());

            match self.runtime.block_on(self.recognizer.recognize(&self.all_strokes)) {
                Ok(result) => {
                    info!("Recognized: '{}' (confidence: {:.2})", result.text, result.confidence);

                    // Display recognized text
                    let response_x = (center_x / 15).max(100) as usize;
                    let response_y = ((center_y / 15) + 50).max(300) as usize;

                    display.draw_text(
                        &format!("You wrote: {}", result.text),
                        response_x,
                        response_y,
                        45.0,
                    );
                    display.draw_text(
                        &format!("Confidence: {:.0}%", result.confidence * 100.0),
                        response_x,
                        response_y + 60,
                        35.0,
                    );

                    display.refresh_region(
                        response_x as i32,
                        response_y as i32,
                        800,
                        200,
                    );

                    // Clear strokes after recognition
                    self.all_strokes.clear();
                }
                Err(e) => {
                    warn!("Recognition failed: {}", e);
                    let response_x = (center_x / 15).max(100) as usize;
                    let response_y = ((center_y / 15) + 50).max(300) as usize;
                    display.draw_text(
                        "Recognition failed (network error?)",
                        response_x,
                        response_y,
                        40.0,
                    );
                    display.refresh_region(
                        response_x as i32,
                        response_y as i32,
                        800,
                        100,
                    );
                }
            }
        } else {
            info!("Circle detected but no strokes to recognize");
            display.draw_text(
                "Draw some text, then circle to recognize",
                100,
                300,
                35.0,
            );
            display.refresh_region(100, 300, 800, 50);
        }

        Ok(())
    }
}
