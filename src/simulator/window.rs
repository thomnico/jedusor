//! Simulator window implementation using minifb
//!
//! Creates a window matching reMarkable 2 display (1404x1872) and converts
//! mouse events to simulated Wacom digitizer events.

use anyhow::Result;
use log::{debug, info};
use minifb::{Window, WindowOptions, Key, MouseMode, MouseButton};

#[cfg(feature = "simulator")]
use image::{RgbImage, Rgb};

use crate::input::wacom::{WacomEvent, Tool, WACOM_MAX_X, WACOM_MAX_Y};
use crate::stroke::{Point, Stroke};

/// reMarkable 2 display dimensions
pub const DISPLAY_WIDTH: usize = 1404;
pub const DISPLAY_HEIGHT: usize = 1872;

/// Background color (white)
const COLOR_WHITE: u32 = 0xFFFFFF;
/// Foreground color (black)
const COLOR_BLACK: u32 = 0x000000;

/// Simulator window state
pub struct SimulatorWindow {
    /// minifb window
    window: Window,
    /// Framebuffer (DISPLAY_WIDTH x DISPLAY_HEIGHT pixels)
    buffer: Vec<u32>,
    /// Current mouse state
    mouse_down: bool,
    /// Last mouse position (for tracking)
    last_mouse_pos: Option<(f32, f32)>,
}

impl SimulatorWindow {
    /// Create a new simulator window
    pub fn new() -> Result<Self> {
        info!("Creating simulator window ({}x{})", DISPLAY_WIDTH, DISPLAY_HEIGHT);

        let mut window = Window::new(
            "Jedusor Simulator - reMarkable 2",
            DISPLAY_WIDTH,
            DISPLAY_HEIGHT,
            WindowOptions {
                resize: false,
                scale: minifb::Scale::X1,
                ..WindowOptions::default()
            },
        )?;

        // Limit update rate to ~60 FPS
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        let buffer = vec![COLOR_WHITE; DISPLAY_WIDTH * DISPLAY_HEIGHT];

        Ok(Self {
            window,
            buffer,
            mouse_down: false,
            last_mouse_pos: None,
        })
    }

    /// Check if window is still open
    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    /// Check if screenshot key was pressed
    pub fn screenshot_requested(&self) -> bool {
        self.window.is_key_pressed(Key::S, minifb::KeyRepeat::No)
    }

    /// Save screenshot to file
    pub fn save_screenshot(&self) -> Result<String> {
        // Generate filename with timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let filename = format!("jedusor-screenshot-{}.png", timestamp);

        // Convert framebuffer (u32 RGB) to image RGB8
        let mut img = RgbImage::new(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32);

        for (idx, pixel) in self.buffer.iter().enumerate() {
            let x = (idx % DISPLAY_WIDTH) as u32;
            let y = (idx / DISPLAY_WIDTH) as u32;

            // Extract RGB from u32 (0x00RRGGBB format)
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;

            img.put_pixel(x, y, Rgb([r, g, b]));
        }

        // Save to file
        img.save(&filename)?;
        info!("Screenshot saved: {}", filename);

        Ok(filename)
    }

    /// Update window and get next Wacom event
    pub fn poll_event(&mut self) -> Option<WacomEvent> {
        // Update window (processes events)
        self.window
            .update_with_buffer(&self.buffer, DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .ok()?;

        // Get mouse position
        let mouse_pos = self.window.get_mouse_pos(MouseMode::Clamp)?;

        // Check mouse button state
        let mouse_currently_down = self.window.get_mouse_down(MouseButton::Left);

        // Generate events based on state changes
        match (self.mouse_down, mouse_currently_down) {
            (false, true) => {
                // Mouse just pressed
                debug!("Mouse down at ({}, {})", mouse_pos.0, mouse_pos.1);
                self.mouse_down = true;
                self.last_mouse_pos = Some(mouse_pos);

                Some(WacomEvent::ToolDown {
                    tool: Tool::Pen,
                    x: self.mouse_x_to_wacom(mouse_pos.0),
                    y: self.mouse_y_to_wacom(mouse_pos.1),
                    pressure: 2048, // Fixed mid-range pressure
                })
            }
            (true, true) => {
                // Mouse is down and moved
                if let Some(last_pos) = self.last_mouse_pos {
                    // Only emit event if position changed
                    if (last_pos.0 - mouse_pos.0).abs() > 0.5
                        || (last_pos.1 - mouse_pos.1).abs() > 0.5 {
                        self.last_mouse_pos = Some(mouse_pos);

                        Some(WacomEvent::ToolMove {
                            x: self.mouse_x_to_wacom(mouse_pos.0),
                            y: self.mouse_y_to_wacom(mouse_pos.1),
                            pressure: 2048,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            (true, false) => {
                // Mouse just released
                debug!("Mouse up");
                self.mouse_down = false;
                self.last_mouse_pos = None;

                Some(WacomEvent::ToolUp)
            }
            (false, false) => {
                // Mouse is up and stationary
                None
            }
        }
    }

    /// Convert mouse X coordinate to Wacom X coordinate
    fn mouse_x_to_wacom(&self, mouse_x: f32) -> i32 {
        // Map [0, DISPLAY_WIDTH] → [0, WACOM_MAX_X]
        ((mouse_x as f64 / DISPLAY_WIDTH as f64) * WACOM_MAX_X as f64) as i32
    }

    /// Convert mouse Y coordinate to Wacom Y coordinate
    fn mouse_y_to_wacom(&self, mouse_y: f32) -> i32 {
        // Map [0, DISPLAY_HEIGHT] → [0, WACOM_MAX_Y]
        ((mouse_y as f64 / DISPLAY_HEIGHT as f64) * WACOM_MAX_Y as f64) as i32
    }

    /// Convert Wacom X coordinate to display X coordinate
    fn wacom_x_to_display(&self, wacom_x: i32) -> usize {
        ((wacom_x as f64 / WACOM_MAX_X as f64) * DISPLAY_WIDTH as f64) as usize
    }

    /// Convert Wacom Y coordinate to display Y coordinate
    fn wacom_y_to_display(&self, wacom_y: i32) -> usize {
        ((wacom_y as f64 / WACOM_MAX_Y as f64) * DISPLAY_HEIGHT as f64) as usize
    }

    /// Draw a stroke to the framebuffer
    pub fn draw_stroke(&mut self, stroke: &Stroke, color: u32) {
        for point in &stroke.points {
            self.draw_point(point, color);
        }
    }

    /// Draw a single point (small circle for visibility)
    fn draw_point(&mut self, point: &Point, color: u32) {
        let x = self.wacom_x_to_display(point.x);
        let y = self.wacom_y_to_display(point.y);

        // Draw a small 3x3 circle for visibility
        for dy in -1..=1 {
            for dx in -1..=1 {
                let px = (x as i32 + dx) as usize;
                let py = (y as i32 + dy) as usize;

                if px < DISPLAY_WIDTH && py < DISPLAY_HEIGHT {
                    let idx = py * DISPLAY_WIDTH + px;
                    self.buffer[idx] = color;
                }
            }
        }
    }

    /// Clear the display
    pub fn clear(&mut self) {
        self.buffer.fill(COLOR_WHITE);
    }

    /// Draw text at position (basic implementation)
    pub fn draw_text(&mut self, text: &str, x: usize, y: usize) {
        // For now, just draw a placeholder rectangle where text would go
        // TODO: Use a bitmap font or text rendering library
        let text_width = text.len() * 8;  // Approximate 8px per char
        let text_height = 16;

        debug!("Drawing text '{}' at ({}, {})", text, x, y);

        // Draw a simple rectangle to indicate text area
        for py in y..y + text_height {
            for px in x..x + text_width {
                if px < DISPLAY_WIDTH && py < DISPLAY_HEIGHT {
                    let idx = py * DISPLAY_WIDTH + px;
                    self.buffer[idx] = 0xCCCCCC; // Light gray
                }
            }
        }
    }

    /// Get black color constant
    pub fn color_black() -> u32 {
        COLOR_BLACK
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test coordinate conversion logic without creating a window
    #[test]
    fn test_coordinate_conversion() {
        // Test X coordinate mapping
        let mouse_x_to_wacom = |x: f32| -> i32 {
            ((x as f64 / DISPLAY_WIDTH as f64) * WACOM_MAX_X as f64) as i32
        };

        let wacom_x_to_display = |wacom_x: i32| -> usize {
            ((wacom_x as f64 / WACOM_MAX_X as f64) * DISPLAY_WIDTH as f64) as usize
        };

        assert_eq!(mouse_x_to_wacom(0.0), 0);
        assert_eq!(mouse_x_to_wacom(DISPLAY_WIDTH as f32), WACOM_MAX_X);

        // Test Y coordinate mapping
        let mouse_y_to_wacom = |y: f32| -> i32 {
            ((y as f64 / DISPLAY_HEIGHT as f64) * WACOM_MAX_Y as f64) as i32
        };

        assert_eq!(mouse_y_to_wacom(0.0), 0);
        assert_eq!(mouse_y_to_wacom(DISPLAY_HEIGHT as f32), WACOM_MAX_Y);

        // Test round-trip conversion
        let wacom_x = 10000;
        let display_x = wacom_x_to_display(wacom_x);
        let back_to_wacom = mouse_x_to_wacom(display_x as f32);

        // Allow small rounding error
        assert!((wacom_x - back_to_wacom).abs() < 100);
    }
}
