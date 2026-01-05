//! Simulator platform implementation
//!
//! Uses minifb window for display and mouse input for pen simulation.

use anyhow::Result;
use std::rc::Rc;
use std::cell::RefCell;
use crate::input::WacomEvent;
use crate::platform::{Display, InputSource, Platform, Color};
use crate::simulator::SimulatorWindow;
use crate::stroke::Stroke;

/// Simulator display implementation
pub struct SimulatorDisplay {
    window: Rc<RefCell<SimulatorWindow>>,
}

impl SimulatorDisplay {
    fn new(window: Rc<RefCell<SimulatorWindow>>) -> Self {
        Self { window }
    }
}

impl Display for SimulatorDisplay {
    fn clear(&mut self) {
        self.window.borrow_mut().clear();
    }

    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, _width: i32, color: Color) {
        // Convert Color to u32
        let color_u32 = ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32);

        // Create a temp stroke with two points
        let mut stroke = Stroke::new();
        stroke.add_point(crate::stroke::Point {
            x: x1,
            y: y1,
            pressure: 2048,
            timestamp_ms: 0,
        });
        stroke.add_point(crate::stroke::Point {
            x: x2,
            y: y2,
            pressure: 2048,
            timestamp_ms: 0,
        });

        self.window.borrow_mut().draw_stroke(&stroke, color_u32);
    }

    fn draw_text(&mut self, text: &str, x: usize, y: usize, _size: f32) {
        self.window.borrow_mut().draw_text(text, x, y);
    }

    fn refresh(&mut self) {
        // minifb handles refresh automatically in poll_event
    }

    fn refresh_region(&mut self, _x: i32, _y: i32, _width: u32, _height: u32) {
        // minifb doesn't support partial refresh, full refresh happens in poll_event
    }
}

/// Simulator input implementation
pub struct SimulatorInput {
    window: Rc<RefCell<SimulatorWindow>>,
}

impl SimulatorInput {
    fn new(window: Rc<RefCell<SimulatorWindow>>) -> Self {
        Self { window }
    }
}

impl InputSource for SimulatorInput {
    fn poll_event(&mut self) -> Option<WacomEvent> {
        self.window.borrow_mut().poll_event()
    }

    fn is_running(&self) -> bool {
        self.window.borrow().is_open()
    }
}

/// Simulator platform
pub struct SimulatorPlatform {
    display: SimulatorDisplay,
    input: SimulatorInput,
}

impl Platform for SimulatorPlatform {
    type Display = SimulatorDisplay;
    type Input = SimulatorInput;

    fn new() -> Result<Self> {
        let window = Rc::new(RefCell::new(SimulatorWindow::new()?));

        // Draw welcome message
        {
            let mut w = window.borrow_mut();
            w.clear();
            w.draw_text("Jedusor - Journal Mode (Simulator)", 50, 50);
            w.draw_text("Click and drag to draw strokes", 50, 100);
            w.draw_text("Draw a circle around your text to recognize", 50, 150);
            w.draw_text("Press S to save screenshot", 50, 200);
            w.draw_text("Press ESC to exit", 50, 250);
        }

        let display = SimulatorDisplay::new(Rc::clone(&window));
        let input = SimulatorInput::new(window);

        Ok(Self { display, input })
    }

    fn display(&mut self) -> &mut Self::Display {
        &mut self.display
    }

    fn input(&mut self) -> &mut Self::Input {
        &mut self.input
    }
}
