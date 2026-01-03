//! macOS/desktop simulator for local development
//!
//! Provides a GUI-based simulator using minifb that enables testing without
//! a physical reMarkable device. Mouse input is converted to simulated Wacom
//! events, and the display is rendered in a window matching the reMarkable 2
//! resolution (1404x1872).

#[cfg(feature = "simulator")]
pub mod window;

#[cfg(feature = "simulator")]
pub use window::SimulatorWindow;

// Re-export simulator types when simulator feature is enabled
#[cfg(feature = "simulator")]
pub use crate::input::wacom::{WacomEvent, Tool};
