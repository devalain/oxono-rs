#[cfg(not(feature = "tui"))]
mod console;

#[cfg(feature = "tui")]
mod tui;

#[cfg(not(feature = "tui"))]
pub use console::*;

#[cfg(feature = "tui")]
pub use tui::draw;
