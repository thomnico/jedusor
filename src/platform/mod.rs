//! Platform abstraction layer
//!
//! Provides unified interface for device and simulator backends.
//! All business logic should be platform-agnostic using these traits.

use anyhow::Result;
use crate::input::WacomEvent;

#[cfg(feature = "device")]
pub mod device;

#[cfg(feature = "simulator")]
pub mod simulator;

/// Color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
}

/// Display abstraction - platform-specific rendering
pub trait Display {
    /// Clear the entire display
    fn clear(&mut self);

    /// Draw a line between two points with given width
    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, width: i32, color: Color);

    /// Draw text at position
    fn draw_text(&mut self, text: &str, x: usize, y: usize, size: f32);

    /// Refresh the display (show pending changes)
    fn refresh(&mut self);

    /// Refresh a specific region (optimization)
    fn refresh_region(&mut self, x: i32, y: i32, width: u32, height: u32);
}

/// Input abstraction - platform-specific event polling
pub trait InputSource {
    /// Poll for next input event (non-blocking)
    fn poll_event(&mut self) -> Option<WacomEvent>;

    /// Check if platform is still running
    fn is_running(&self) -> bool;
}

/// Platform abstraction - combines display + input
pub trait Platform: Sized {
    type Display: Display;
    type Input: InputSource;

    /// Create new platform instance
    fn new() -> Result<Self>;

    /// Get mutable reference to display
    fn display(&mut self) -> &mut Self::Display;

    /// Get mutable reference to input source
    fn input(&mut self) -> &mut Self::Input;
}
