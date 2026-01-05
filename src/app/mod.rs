//! Main application state and event loop
//!
//! Manages mode switching (Journal/Document/Research), state, and the event loop.

mod event_handler;

use anyhow::Result;
use log::{info, debug, warn};
use event_handler::EventHandler;

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
        use libremarkable::input::InputEvent;
        use libremarkable::framebuffer::common::*;
        use libremarkable::framebuffer::{FramebufferRefresh, PartialRefreshMode};
        use crate::platform::device::DeviceDisplay;
        use crate::platform::Display;

        info!("Initializing reMarkable device");

        let mut app = ApplicationContext::default();
        let text_renderer = TextRenderer::new();

        // Create shared event handler with all business logic
        let mut handler = EventHandler::new()?;

        // Track whether we're currently drawing (for event conversion)
        let mut is_drawing = false;

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
                    // Convert libremarkable event to our WacomEvent format
                    let our_event = match event {
                        libremarkable::input::WacomEvent::Draw { position, pressure, .. } => {
                            let wacom_event = if !is_drawing {
                                is_drawing = true;
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
                            Some(wacom_event)
                        }
                        libremarkable::input::WacomEvent::InstrumentChange { .. } => {
                            // Pen up
                            is_drawing = false;
                            Some(WacomEvent::ToolUp)
                        }
                        _ => None,
                    };

                    // Handle event with shared EventHandler
                    if let Some(event) = our_event {
                        let fb = ctx.get_framebuffer_ref();
                        let mut display = DeviceDisplay::new(fb);

                        // ALL business logic is in the shared EventHandler!
                        if let Err(e) = handler.handle_event(event, &mut display) {
                            warn!("Error handling event: {}", e);
                        }
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
        use crate::platform::simulator::SimulatorPlatform;
        use crate::platform::{Platform, Display, InputSource};

        info!("Initializing simulator window");

        let mut platform = SimulatorPlatform::new()?;
        let mut handler = EventHandler::new()?;

        info!("Clearing screen");
        platform.display().clear();

        info!("Starting simulator event loop - ready for input!");

        while platform.input().is_running() {
            // Check for screenshot request (platform-specific feature)
            // TODO: Abstract screenshot into platform trait if needed

            // Poll for events and handle with shared EventHandler
            if let Some(event) = platform.input().poll_event() {
                handler.handle_event(event, platform.display())?;
            }

            // Small sleep to prevent busy-waiting
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        info!("Simulator window closed");
        Ok(())
    }
}
