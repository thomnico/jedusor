//! Stroke rendering for e-ink display

use crate::stroke::Stroke;

#[cfg(feature = "device")]
use libremarkable::framebuffer::{
    FramebufferDraw, FramebufferRefresh, common::color,
};

/// Renders strokes to the framebuffer
pub struct StrokeRenderer;

impl StrokeRenderer {
    pub fn new() -> Self {
        Self
    }

    #[cfg(feature = "device")]
    /// Draw a stroke on the framebuffer
    pub fn draw_stroke<T>(&self, framebuffer: &mut T, stroke: &Stroke, color: color)
    where
        T: FramebufferDraw + FramebufferRefresh,
    {
        if stroke.points.len() < 2 {
            return;
        }

        // Draw lines connecting consecutive points
        for window in stroke.points.windows(2) {
            let p1 = &window[0];
            let p2 = &window[1];

            framebuffer.draw_line(
                p1.x as usize,
                p1.y as usize,
                p2.x as usize,
                p2.y as usize,
                3, // Line width
                color,
            );
        }
    }

    #[cfg(not(feature = "device"))]
    /// Placeholder for non-device builds
    pub fn draw_stroke(&self, _stroke: &Stroke) {
        // No-op on non-device platforms
    }
}
