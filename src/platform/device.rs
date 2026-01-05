//! Device platform implementation for reMarkable
//!
//! Wraps libremarkable's framebuffer and input system.

use anyhow::Result;
use crate::platform::{Display, Color};
use crate::stroke::Stroke;

#[cfg(feature = "device")]
use libremarkable::framebuffer::{
    FramebufferDraw, FramebufferRefresh, FramebufferBase,
    common::{color, mxcfb_rect},
    PartialRefreshMode, display_temp, waveform_mode, dither_mode,
};

/// Device display adapter wrapping libremarkable framebuffer
///
/// This is created inside the libremarkable event loop callback
/// with a reference to the framebuffer.
#[cfg(feature = "device")]
pub struct DeviceDisplay<'a, T>
where
    T: FramebufferDraw + FramebufferRefresh + FramebufferBase,
{
    framebuffer: &'a mut T,
}

#[cfg(feature = "device")]
impl<'a, T> DeviceDisplay<'a, T>
where
    T: FramebufferDraw + FramebufferRefresh + FramebufferBase,
{
    pub fn new(framebuffer: &'a mut T) -> Self {
        Self { framebuffer }
    }
}

#[cfg(feature = "device")]
impl<'a, T> Display for DeviceDisplay<'a, T>
where
    T: FramebufferDraw + FramebufferRefresh + FramebufferBase,
{
    fn clear(&mut self) {
        self.framebuffer.clear();
    }

    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, width: i32, color: Color) {
        let device_color = if color == Color::BLACK {
            color::BLACK
        } else {
            color::WHITE
        };

        self.framebuffer.draw_line(
            (x1, y1).into(),
            (x2, y2).into(),
            width,
            device_color,
        );
    }

    fn draw_text(&mut self, text: &str, x: usize, y: usize, size: f32) {
        // Use the existing text renderer
        let renderer = crate::render::TextRenderer::new();
        renderer.draw_text(self.framebuffer, text, x, y, size);
    }

    fn refresh(&mut self) {
        // Full screen refresh
        self.framebuffer.full_refresh(
            waveform_mode::WAVEFORM_MODE_GC16,
            display_temp::TEMP_USE_REMARKABLE_DRAW,
            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
            0,
            false,
        );
    }

    fn refresh_region(&mut self, x: i32, y: i32, width: u32, height: u32) {
        let region = mxcfb_rect {
            top: y.max(0) as u32,
            left: x.max(0) as u32,
            width,
            height,
        };

        self.framebuffer.partial_refresh(
            &region,
            PartialRefreshMode::Async,
            waveform_mode::WAVEFORM_MODE_GC16,
            display_temp::TEMP_USE_REMARKABLE_DRAW,
            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
            0,
            false,
        );
    }
}
