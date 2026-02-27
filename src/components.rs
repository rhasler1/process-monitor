// In this module all `Components` are `UIComponents` comprised of a `view`-`controller` pair.
// `Components` DO NOT have to depend on a `Domain model` and can depend on static data (e.g.,
// help.rs).
// `Components` that DO depend on a `Domain model` DO NOT store references to the `Domain model` they depend on.
// Synchronization between a `DomainModel` and a `Component` is the responsibility of the `App`.
// When the `App` receives a new `DomainModel` or a mutation occurs it is must call the handle_model_update() handler
// on all Components that depend on that `DomainModel`. As such all `Components` must implement the trait on_model_update().

// Make component modules visible ouside parent
pub mod process_table;
pub mod text_line;
pub mod utils;

// Internal project imports
use crate::adapters::crossterm::input::Key;
use crate::events::EventState;
//use crate::domain::DomainModel;
// Ratatui for drawing
use ratatui::prelude::*;
// Anyhow for return values
use anyhow::Result;

// All components dependant on a `DomainModel` must implement this trait
//pub trait DomainModelComponent {
    // Each component picks a DomainModel type
//    type Model: DomainModel;
//    fn on_model_update(&mut self, model: &Self::Model);
//}

/// All components that can be drawn to the screen must implement this trait
pub trait ViewableComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> Result<()>;
}

/// All components that can take user input must implement this trait
pub trait ControllableComponent {
    fn key_event(&mut self, key: Key) -> Result<EventState>;
}
