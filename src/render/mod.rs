//! Rendering module
//!
//! E-ink optimized display with text layout and animation

pub mod strokes;

pub use strokes::StrokeRenderer;

#[cfg(feature = "device")]
use libremarkable::framebuffer::{FramebufferDraw, common::mxcfb_rect};

/// Text rendering configuration
pub struct TextRenderer;

impl TextRenderer {
    pub fn new() -> Self {
        Self
    }

    #[cfg(feature = "device")]
    /// Render text to the framebuffer
    pub fn draw_text<T>(&self, framebuffer: &mut T, text: &str, x: usize, y: usize, size: f32)
    where
        T: FramebufferDraw,
    {
        use libremarkable::framebuffer::common::color;
        framebuffer.draw_text(
            (x as f32, y as f32).into(),  // Point2<f32>
            text,
            size,
            color::BLACK,
            false,  // dryrun
        );
    }

    #[cfg(not(feature = "device"))]
    /// Placeholder for non-device builds
    pub fn draw_text(&self, _text: &str, _x: usize, _y: usize, _size: f32) {
        // No-op on non-device platforms
    }
}

// TODO: Add animation.rs for streaming reveal
// TODO: Add zones.rs for response areas (margin/footer)
