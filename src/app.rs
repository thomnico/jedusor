//! Main application state and event loop
//!
//! Manages mode switching (Journal/Document/Research), state, and the event loop.

use anyhow::Result;
use log::{info, debug, warn};

#[cfg(feature = "device")]
use crate::input::{WacomHandler, WacomEvent, Tool, GestureDetector, Gesture};
#[cfg(feature = "device")]
use crate::render::{StrokeRenderer, TextRenderer};
#[cfg(feature = "device")]
use crate::stroke::Stroke;
#[cfg(feature = "device")]
use crate::recognition::{GoogleRecognizer, Recognizer};

/// Interaction modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Blank page conversation (magical diary experience)
    Journal,
    /// AI assistant overlaid on PDFs
    #[allow(dead_code)]
    Document,
    /// Multi-document context (future)
    #[allow(dead_code)]
    Research,
}

impl Default for Mode {
    fn default() -> Self {
        // Start in Journal mode for MVP
        Mode::Journal
    }
}

/// Main application state
pub struct App {
    mode: Mode,
}

impl App {
    /// Create a new application instance
    pub fn new() -> Result<Self> {
        info!("Initializing application in Journal mode");

        Ok(Self {
            mode: Mode::default(),
        })
    }

    /// Run the main event loop
    pub fn run(&mut self) -> Result<()> {
        info!("Starting event loop (mode: {:?})", self.mode);

        #[cfg(feature = "device")]
        {
            // Set up cleanup handler to restart xochitl on exit
            let result = self.run_device_loop();

            // Always restart xochitl on exit
            info!("Restarting xochitl...");
            std::process::Command::new("systemctl")
                .arg("start")
                .arg("xochitl")
                .status()
                .ok();

            result?;
        }

        #[cfg(feature = "simulator")]
        {
            self.run_simulator_loop()?;
        }

        #[cfg(not(any(feature = "device", feature = "simulator")))]
        {
            info!("Running in development mode (no device or simulator)");
            info!("Use 'cargo run --no-default-features --features simulator' for simulator");
            info!("Use 'cross build --target armv7-unknown-linux-gnueabihf' for device builds");
        }

        Ok(())
    }

    #[cfg(feature = "device")]
    fn run_device_loop(&mut self) -> Result<()> {
        use libremarkable::appctx::ApplicationContext;
        use libremarkable::input::{InputEvent, gpio, multitouch};
        use libremarkable::framebuffer::common::*;
        use libremarkable::framebuffer::{FramebufferRefresh, PartialRefreshMode};

        info!("Initializing reMarkable device");

        let mut app = ApplicationContext::default();
        let mut wacom_handler = WacomHandler::new();
        let gesture_detector = GestureDetector::new();
        let stroke_renderer = StrokeRenderer::new();
        let text_renderer = TextRenderer::new();

        // Create recognizer and runtime for async API calls
        let recognizer = GoogleRecognizer::new()?;
        let runtime = tokio::runtime::Runtime::new()?;
        info!("Google Input Tools recognizer initialized");

        let mut all_strokes: Vec<Stroke> = Vec::new();

        info!("Clearing screen");
        app.clear(true);

        // Draw welcome instructions
        let fb = app.get_framebuffer_ref();
        text_renderer.draw_text(fb, "Jedusor - Journal Mode", 50, 50, 50.0);
        text_renderer.draw_text(fb, "Draw with stylus - strokes appear in real-time", 50, 120, 35.0);
        text_renderer.draw_text(fb, "Draw a circle to trigger handwriting recognition", 50, 170, 35.0);
        text_renderer.draw_text(fb, "Press middle button to exit", 50, 220, 35.0);

        // Full screen refresh for instructions
        fb.full_refresh(
            waveform_mode::WAVEFORM_MODE_GC16,
            display_temp::TEMP_USE_REMARKABLE_DRAW,
            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
            0,
            false,
        );

        info!("Starting event loop - ready for input!");

        app.start_event_loop(true, true, true, |ctx, event| {
            match event {
                InputEvent::WacomEvent { event } => {
                    // Convert libremarkable event to our format
                    match event {
                        libremarkable::input::WacomEvent::Draw { position, pressure, .. } => {
                            debug!("📝 Wacom Draw event: pos=({:.1}, {:.1}), pressure={}",
                                   position.x, position.y, pressure);

                            let wacom_event = if !wacom_handler.is_drawing() {
                                debug!("✏️  Starting new stroke");
                                WacomEvent::ToolDown {
                                    tool: Tool::Pen,
                                    x: position.x as i32,
                                    y: position.y as i32,
                                    pressure,
                                }
                            } else {
                                WacomEvent::ToolMove {
                                    x: position.x as i32,
                                    y: position.y as i32,
                                    pressure,
                                }
                            };

                            if let Ok(completed_stroke) = wacom_handler.handle_event(wacom_event) {
                                // Draw current stroke in real-time
                                if let Some(stroke) = wacom_handler.current_stroke() {
                                    debug!("🎨 Drawing stroke with {} points", stroke.points.len());
                                    let fb = ctx.get_framebuffer_ref();

                                    // Draw stroke in real-time
                                    stroke_renderer.draw_stroke(
                                        fb,
                                        stroke,
                                        color::BLACK,
                                    );
                                    debug!("✅ Stroke drawn to framebuffer");

                                    // Partial refresh for stroke area
                                    if let Some((x_min, y_min, x_max, y_max)) = stroke.bounding_box() {
                                        let region = mxcfb_rect {
                                            top: y_min.max(0) as u32,
                                            left: x_min.max(0) as u32,
                                            width: ((x_max - x_min).max(1) + 20) as u32,
                                            height: ((y_max - y_min).max(1) + 20) as u32,
                                        };
                                        debug!("🔄 Refreshing region: x={}-{}, y={}-{}",
                                               x_min, x_max, y_min, y_max);
                                        fb.partial_refresh(
                                            &region,
                                            PartialRefreshMode::Async,
                                            waveform_mode::WAVEFORM_MODE_DU,
                                            display_temp::TEMP_USE_REMARKABLE_DRAW,
                                            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
                                            0,
                                            false,
                                        );
                                        debug!("✅ Refresh triggered");
                                    } else {
                                        debug!("⚠️  No bounding box for stroke");
                                    }
                                }

                                if let Some(stroke) = completed_stroke {
                                    info!("✨ Stroke completed with {} points", stroke.points.len());

                                    // Check for gestures
                                    match gesture_detector.detect(&stroke) {
                                        Gesture::Circle { center_x, center_y, .. } => {
                                            info!("Circle gesture detected at ({}, {})", center_x, center_y);

                                            let fb = ctx.get_framebuffer_ref();

                                            // Show placeholder AI response
                                            text_renderer.draw_text(
                                                fb,
                                                "AI: This is a placeholder response.",
                                                center_x.max(100) as usize,
                                                (center_y + 50).max(200) as usize,
                                                40.0,
                                            );
                                            text_renderer.draw_text(
                                                fb,
                                                "Circle gesture recognized!",
                                                center_x.max(100) as usize,
                                                (center_y + 100).max(250) as usize,
                                                30.0,
                                            );

                                            let region = mxcfb_rect {
                                                top: (center_y + 50).max(0) as u32,
                                                left: center_x.max(0) as u32,
                                                width: 800,
                                                height: 200,
                                            };
                                            fb.partial_refresh(
                                                &region,
                                                PartialRefreshMode::Async,
                                                waveform_mode::WAVEFORM_MODE_GC16,
                                                display_temp::TEMP_USE_REMARKABLE_DRAW,
                                                dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
                                                0,
                                                false,
                                            );
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

                                    all_strokes.push(stroke);
                                }
                            }
                        }
                        libremarkable::input::WacomEvent::InstrumentChange { .. } => {
                            // Tool up event
                            if let Ok(Some(stroke)) = wacom_handler.handle_event(WacomEvent::ToolUp) {
                                info!("✨ Stroke completed with {} points", stroke.points.len());

                                // Check for gestures
                                match gesture_detector.detect(&stroke) {
                                    Gesture::Circle { center_x, center_y, .. } => {
                                        info!("Circle gesture detected at ({}, {})", center_x, center_y);

                                        let fb = ctx.get_framebuffer_ref();

                                        // Trigger recognition on all accumulated strokes (excluding the circle)
                                        if !all_strokes.is_empty() {
                                            info!("Recognizing {} strokes", all_strokes.len());

                                            match runtime.block_on(recognizer.recognize(&all_strokes)) {
                                                Ok(result) => {
                                                    info!("Recognized: '{}' (confidence: {:.2})", result.text, result.confidence);

                                                    // Display recognized text
                                                    let response_x = 100_usize;
                                                    let response_y = (center_y / 15).max(300) as usize;

                                                    text_renderer.draw_text(
                                                        fb,
                                                        &format!("You wrote: {}", result.text),
                                                        response_x,
                                                        response_y,
                                                        45.0,
                                                    );
                                                    text_renderer.draw_text(
                                                        fb,
                                                        &format!("Confidence: {:.0}%", result.confidence * 100.0),
                                                        response_x,
                                                        response_y + 60,
                                                        35.0,
                                                    );

                                                    // Clear strokes after recognition
                                                    all_strokes.clear();
                                                }
                                                Err(e) => {
                                                    warn!("Recognition failed: {}", e);
                                                    text_renderer.draw_text(
                                                        fb,
                                                        "Recognition failed (network error?)",
                                                        100,
                                                        300,
                                                        40.0,
                                                    );
                                                }
                                            }
                                        } else {
                                            info!("Circle detected but no strokes to recognize");
                                            text_renderer.draw_text(
                                                fb,
                                                "Draw some text, then circle to recognize",
                                                100,
                                                300,
                                                35.0,
                                            );
                                        }

                                        let region = mxcfb_rect {
                                            top: 250,
                                            left: 50,
                                            width: 1300,
                                            height: 300,
                                        };
                                        fb.partial_refresh(
                                            &region,
                                            PartialRefreshMode::Async,
                                            waveform_mode::WAVEFORM_MODE_GC16,
                                            display_temp::TEMP_USE_REMARKABLE_DRAW,
                                            dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
                                            0,
                                            false,
                                        );
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

                                all_strokes.push(stroke);
                            }
                        }
                        _ => {}
                    }
                }
                InputEvent::MultitouchEvent { event } => {
                    match event {
                        libremarkable::input::MultitouchEvent::Press { finger } => {
                            debug!("Touch detected: finger {}", finger.tracking_id);
                            // Touch events are ignored - use power button or swipe to exit
                        }
                        _ => {}
                    }
                }
                InputEvent::GPIO { event } => {
                    match event {
                        libremarkable::input::GPIOEvent::Press { button } => {
                            info!("Button press: {:?}", button);
                            if button == libremarkable::input::PhysicalButton::POWER
                                || button == libremarkable::input::PhysicalButton::MIDDLE
                            {
                                let button_name = if button == libremarkable::input::PhysicalButton::POWER {
                                    "Power"
                                } else {
                                    "Middle"
                                };
                                info!("{} button pressed - exiting and restarting xochitl", button_name);

                                // Show exit animation
                                let fb = ctx.get_framebuffer_ref();

                                // Clear screen
                                fb.clear();

                                // Show exit message
                                text_renderer.draw_text(
                                    fb,
                                    "Exiting Jedusor...",
                                    500,
                                    850,
                                    60.0,
                                );
                                text_renderer.draw_text(
                                    fb,
                                    "Restarting reMarkable UI",
                                    450,
                                    950,
                                    45.0,
                                );

                                // Full refresh to show message
                                fb.full_refresh(
                                    waveform_mode::WAVEFORM_MODE_GC16,
                                    display_temp::TEMP_USE_REMARKABLE_DRAW,
                                    dither_mode::EPDC_FLAG_USE_DITHERING_PASSTHROUGH,
                                    0,
                                    false,
                                );

                                // Brief delay so user sees the message
                                std::thread::sleep(std::time::Duration::from_millis(800));

                                // Restart xochitl
                                std::process::Command::new("systemctl")
                                    .arg("start")
                                    .arg("xochitl")
                                    .status()
                                    .ok();

                                std::process::exit(0);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });

        Ok(())
    }

    #[cfg(feature = "simulator")]
    fn run_simulator_loop(&mut self) -> Result<()> {
        use crate::input::{WacomHandler, WacomEvent, GestureDetector, Gesture};
        use crate::recognition::{GoogleRecognizer, Recognizer};
        use crate::simulator::SimulatorWindow;
        use crate::stroke::Stroke;

        info!("Initializing simulator window");

        let mut window = SimulatorWindow::new()?;
        let mut wacom_handler = WacomHandler::new();
        let gesture_detector = GestureDetector::new();
        let recognizer = GoogleRecognizer::new()?;

        info!("Google Input Tools recognizer initialized");

        let mut all_strokes: Vec<Stroke> = Vec::new();
        let mut pending_points: Vec<crate::stroke::Point> = Vec::new();
        let mut recognized_text: Vec<String> = Vec::new();

        // Create tokio runtime for async recognition calls
        let runtime = tokio::runtime::Runtime::new()?;

        info!("Clearing screen");
        window.clear();

        // Draw welcome message
        window.draw_text("Jedusor - Journal Mode (Simulator)", 50, 50);
        window.draw_text("Click and drag to draw strokes", 50, 100);
        window.draw_text("Text will be recognized automatically", 50, 150);
        window.draw_text("Press S to save screenshot", 50, 200);
        window.draw_text("Press ESC to exit", 50, 250);

        info!("Starting simulator event loop - ready for input!");

        while window.is_open() {
            // Check for screenshot request
            if window.screenshot_requested() {
                match window.save_screenshot() {
                    Ok(filename) => info!("Screenshot saved: {}", filename),
                    Err(e) => debug!("Failed to save screenshot: {}", e),
                }
            }

            // Poll for events
            if let Some(event) = window.poll_event() {
                // Clone event for drawing (since handler consumes it)
                let event_for_drawing = match &event {
                    WacomEvent::ToolDown { tool, x, y, pressure } => {
                        debug!("ToolDown: {:?} at ({}, {}) pressure={}", tool, x, y, pressure);
                        Some(crate::stroke::Point {
                            x: *x,
                            y: *y,
                            pressure: *pressure,
                            timestamp_ms: 0,
                        })
                    }
                    WacomEvent::ToolMove { x, y, pressure } => {
                        Some(crate::stroke::Point {
                            x: *x,
                            y: *y,
                            pressure: *pressure,
                            timestamp_ms: 0,
                        })
                    }
                    _ => None,
                };

                // Add point to pending for real-time drawing
                if let Some(point) = event_for_drawing {
                    pending_points.push(point);

                    // Create temp stroke for drawing
                    let mut temp_stroke = Stroke::new();
                    for p in &pending_points {
                        temp_stroke.add_point(*p);
                    }
                    window.draw_stroke(&temp_stroke, SimulatorWindow::color_black());
                }

                // Process event through handler
                match wacom_handler.handle_event(event) {
                    Ok(Some(stroke)) => {
                        // Stroke completed
                        debug!("Stroke completed with {} points", stroke.points.len());
                        pending_points.clear();

                        // Check for gestures first
                        let is_gesture = match gesture_detector.detect(&stroke) {
                            Gesture::Circle { center_x, center_y, .. } => {
                                info!("Circle gesture detected at ({}, {})", center_x, center_y);

                                // Trigger recognition on all strokes
                                if !all_strokes.is_empty() {
                                    info!("Recognizing {} accumulated strokes", all_strokes.len());
                                    match runtime.block_on(recognizer.recognize(&all_strokes)) {
                                        Ok(result) => {
                                            info!("Recognized: '{}' (confidence: {:.2})", result.text, result.confidence);

                                            // Display recognized text
                                            let response_x = (center_x / 15).max(100) as usize;
                                            let response_y = ((center_y / 15) + 50).max(300) as usize;

                                            window.draw_text(&format!("You wrote: {}", result.text), response_x, response_y);
                                            window.draw_text(&format!("Confidence: {:.0}%", result.confidence * 100.0), response_x, response_y + 30);

                                            recognized_text.push(result.text);

                                            // Clear strokes after recognition
                                            all_strokes.clear();
                                        }
                                        Err(e) => {
                                            warn!("Recognition failed: {}", e);
                                            let response_x = (center_x / 15).max(100) as usize;
                                            let response_y = ((center_y / 15) + 50).max(300) as usize;
                                            window.draw_text("Recognition failed (network error?)", response_x, response_y);
                                        }
                                    }
                                } else {
                                    info!("Circle detected but no strokes to recognize");
                                }
                                true
                            }
                            Gesture::Underline { .. } => {
                                info!("Underline gesture detected");
                                true
                            }
                            Gesture::Lasso { .. } => {
                                info!("Lasso gesture detected");
                                true
                            }
                            Gesture::None => false,
                        };

                        // Add stroke to collection (gestures are also stored)
                        if !is_gesture {
                            debug!("Regular stroke added to collection");
                        }
                        all_strokes.push(stroke);
                    }
                    Ok(None) => {
                        // Stroke in progress or no stroke
                    }
                    Err(e) => {
                        debug!("Error handling wacom event: {}", e);
                    }
                }
            }

            // Small sleep to prevent busy-waiting
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        info!("Simulator window closed");
        Ok(())
    }
}
