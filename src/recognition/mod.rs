//! Handwriting recognition module
//!
//! Converts strokes to text using Google Input Tools API

use anyhow::Result;

/// Trait for handwriting recognition services
pub trait Recognizer {
    /// Recognize strokes and convert to text
    fn recognize(&self, strokes: &[crate::stroke::Stroke]) -> Result<String>;
}

// TODO: Add google.rs for Google Input Tools integration
// TODO: Add local recognition option
