//! Wacom digitizer input handling
//!
//! Captures stylus events from the reMarkable tablet's Wacom digitizer.
//! Coordinates: X (0-20967), Y (0-15725)
//! Pressure: 0-4095
//! Tilt: -9000 to 9000 (X and Y)

use crate::stroke::{Point, Stroke};
use anyhow::Result;
use log::{debug, trace};

/// Wacom digitizer dimensions (reMarkable 2)
pub const WACOM_MAX_X: i32 = 20967;
pub const WACOM_MAX_Y: i32 = 15725;
pub const WACOM_MAX_PRESSURE: u16 = 4095;

/// Tool types detected by the Wacom digitizer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    /// Pen tip (normal writing)
    Pen,
    /// Eraser end of the pen
    Eraser,
}

/// Wacom event types
#[derive(Debug, Clone)]
pub enum WacomEvent {
    /// Pen/eraser touched the screen
    ToolDown {
        tool: Tool,
        x: i32,
        y: i32,
        pressure: u16,
    },
    /// Pen/eraser moved while touching
    ToolMove {
        x: i32,
        y: i32,
        pressure: u16,
    },
    /// Pen/eraser lifted from screen
    ToolUp,
    /// Pen is hovering (not touching)
    Hover {
        x: i32,
        y: i32,
    },
}

/// Handles incoming Wacom events and builds strokes
pub struct WacomHandler {
    /// Current stroke being collected
    current_stroke: Option<Stroke>,
    /// Current tool in use
    current_tool: Option<Tool>,
    /// Timestamp of session start for relative timing
    session_start_ms: u64,
    /// Whether a tool is currently down
    is_down: bool,
}

impl WacomHandler {
    /// Create a new Wacom event handler
    pub fn new() -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Self {
            current_stroke: None,
            current_tool: None,
            session_start_ms: now,
            is_down: false,
        }
    }

    /// Process a Wacom event and return completed stroke if any
    pub fn handle_event(&mut self, event: WacomEvent) -> Result<Option<Stroke>> {
        match event {
            WacomEvent::ToolDown { tool, x, y, pressure } => {
                self.handle_tool_down(tool, x, y, pressure)
            }
            WacomEvent::ToolMove { x, y, pressure } => {
                self.handle_tool_move(x, y, pressure)
            }
            WacomEvent::ToolUp => self.handle_tool_up(),
            WacomEvent::Hover { x: _, y: _ } => {
                // Ignore hover events for now
                Ok(None)
            }
        }
    }

    /// Handle tool down (start of stroke)
    fn handle_tool_down(&mut self, tool: Tool, x: i32, y: i32, pressure: u16) -> Result<Option<Stroke>> {
        debug!("Tool down: {:?} at ({}, {}) pressure={}", tool, x, y, pressure);

        self.is_down = true;
        self.current_tool = Some(tool);

        let mut stroke = Stroke::new();
        stroke.add_point(Point {
            x,
            y,
            pressure,
            timestamp_ms: self.get_timestamp(),
        });

        self.current_stroke = Some(stroke);

        Ok(None)
    }

    /// Handle tool move (continuation of stroke)
    fn handle_tool_move(&mut self, x: i32, y: i32, pressure: u16) -> Result<Option<Stroke>> {
        if !self.is_down {
            trace!("Tool move while not down, ignoring");
            return Ok(None);
        }

        let timestamp = self.get_timestamp();

        if let Some(ref mut stroke) = self.current_stroke {
            stroke.add_point(Point {
                x,
                y,
                pressure,
                timestamp_ms: timestamp,
            });
            trace!("Added point to stroke: ({}, {}) pressure={}", x, y, pressure);
        }

        Ok(None)
    }

    /// Handle tool up (end of stroke)
    fn handle_tool_up(&mut self) -> Result<Option<Stroke>> {
        debug!("Tool up");

        self.is_down = false;
        self.current_tool = None;

        // Return the completed stroke
        let completed_stroke = self.current_stroke.take();

        if let Some(ref stroke) = completed_stroke {
            debug!("Completed stroke with {} points", stroke.points.len());
        }

        Ok(completed_stroke)
    }

    /// Get current timestamp relative to session start
    fn get_timestamp(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        now - self.session_start_ms
    }

    /// Check if currently drawing
    pub fn is_drawing(&self) -> bool {
        self.is_down
    }

    /// Get current tool
    pub fn current_tool(&self) -> Option<Tool> {
        self.current_tool
    }

    /// Get reference to current stroke for real-time rendering
    pub fn current_stroke(&self) -> Option<&Stroke> {
        self.current_stroke.as_ref()
    }
}

#[cfg(feature = "device")]
mod device {
    use super::*;

    /// Convert libremarkable Wacom event to our WacomEvent
    pub fn convert_wacom_event(event: &libremarkable::input::WacomEvent) -> Option<WacomEvent> {
        match event {
            libremarkable::input::WacomEvent::Draw { position, pressure, .. } => {
                Some(WacomEvent::ToolMove {
                    x: position.x as i32,
                    y: position.y as i32,
                    pressure: *pressure,
                })
            }
            libremarkable::input::WacomEvent::InstrumentChange { pen, .. } => {
                // Tool change without position - will be followed by Draw event
                let tool = match pen {
                    libremarkable::input::WacomPen::ToolPen => Tool::Pen,
                    libremarkable::input::WacomPen::ToolRubber => Tool::Eraser,
                    _ => return None,
                };
                debug!("Tool changed to {:?}", tool);
                None
            }
            libremarkable::input::WacomEvent::Hover { position, .. } => {
                Some(WacomEvent::Hover {
                    x: position.x as i32,
                    y: position.y as i32,
                })
            }
            _ => None,
        }
    }
}

#[cfg(feature = "device")]
pub use device::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wacom_handler_basic_stroke() {
        let mut handler = WacomHandler::new();

        // Start stroke
        let result = handler.handle_event(WacomEvent::ToolDown {
            tool: Tool::Pen,
            x: 100,
            y: 200,
            pressure: 1000,
        });
        assert!(result.unwrap().is_none());
        assert!(handler.is_drawing());

        // Add points
        handler.handle_event(WacomEvent::ToolMove {
            x: 110,
            y: 210,
            pressure: 1200,
        }).unwrap();

        handler.handle_event(WacomEvent::ToolMove {
            x: 120,
            y: 220,
            pressure: 1400,
        }).unwrap();

        // End stroke
        let result = handler.handle_event(WacomEvent::ToolUp);
        let stroke = result.unwrap();
        assert!(stroke.is_some());

        let stroke = stroke.unwrap();
        assert_eq!(stroke.points.len(), 3);
        assert_eq!(stroke.points[0].x, 100);
        assert_eq!(stroke.points[0].y, 200);
        assert_eq!(stroke.points[2].x, 120);
        assert_eq!(stroke.points[2].y, 220);
    }

    #[test]
    fn test_wacom_handler_tool_detection() {
        let mut handler = WacomHandler::new();

        handler.handle_event(WacomEvent::ToolDown {
            tool: Tool::Eraser,
            x: 100,
            y: 100,
            pressure: 1000,
        }).unwrap();

        assert_eq!(handler.current_tool(), Some(Tool::Eraser));
        assert!(handler.is_drawing());

        handler.handle_event(WacomEvent::ToolUp).unwrap();
        assert_eq!(handler.current_tool(), None);
        assert!(!handler.is_drawing());
    }

    #[test]
    fn test_wacom_handler_ignore_move_when_up() {
        let mut handler = WacomHandler::new();

        // Move without tool down should be ignored
        let result = handler.handle_event(WacomEvent::ToolMove {
            x: 100,
            y: 100,
            pressure: 1000,
        });

        assert!(result.unwrap().is_none());
        assert!(!handler.is_drawing());
    }
}
