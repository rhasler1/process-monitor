// In this module all `Components` are `UIComponents` comprised of a `view`-`controller` pair.
// `Components` DO NOT have to depend on a `Domain model` and can depend on static data (e.g.,
// help.rs).
// `Components` that DO depend on a `Domain model` DO NOT store references to the `Domain model` they depend on.
// Synchronization between a `DomainModel` and a `Component` is the responsibility of the `App`.
// When the `App` receives a new `DomainModel` or a mutation occurs it is must call the handle_model_update() handler
// on all Components that depend on that `DomainModel`. As such all `Components` must implement the trait on_model_update().

// Make component modules visible
pub mod process_table;
pub mod process_term;
pub mod text_line;
pub mod utils;

use crate::adapters::crossterm::input::Key;
use crate::events::EventState;
use ratatui::prelude::*;
use anyhow::Result;

// Draw component w/ ratatui
pub trait Draw {
    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> Result<()>;
}

// Process Key
pub trait Event {
    fn event(&mut self, key: Key) -> EventState;
}
