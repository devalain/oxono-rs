#[cfg(not(feature = "tui"))]
mod console;

#[cfg(feature = "tui")]
mod tui;

#[cfg(not(feature = "tui"))]
pub use console::Controller;

#[cfg(feature = "tui")]
pub use tui::{Controller, UIState};
