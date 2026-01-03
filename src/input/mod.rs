//! Input handling module
//!
//! Wacom stylus strokes and gesture detection (circle, underline, lasso)

pub mod wacom;
pub mod gesture;

pub use wacom::{WacomHandler, WacomEvent, Tool};
pub use gesture::{GestureDetector, Gesture};
