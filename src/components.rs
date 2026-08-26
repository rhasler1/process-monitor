
// Make component modules visible
pub mod process_table;
pub mod process_term;
pub mod config_manager;

use crate::adapters::crossterm::input::Key;

use anyhow::Result;

// Process Key
pub trait Event {
    type EventState;

    fn event(&mut self, key: Key) -> Result<Self::EventState>;
}
