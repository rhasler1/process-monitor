use crate::components::process_table::state::{
    ProcessTableSort,MoveDirection
};
use crate::adapters::crossterm::input::Key;

// TODO 2/19/2026 is `action` the correct term here?
/// Supported actions
pub enum ProcessTableAction {
    Sort(ProcessTableSort),
    Move(MoveDirection)
}

/// Process key input and return a
#[derive(Default)]
pub struct ProcessTableController;
impl ProcessTableController {
    pub fn handle_key_event(&self, key: Key) -> Option<ProcessTableAction> {
        match key {
            Key::Up   => Some(ProcessTableAction::Move(MoveDirection::Up)),
            Key::Down => Some(ProcessTableAction::Move(MoveDirection::Down)),
            _         => None 
        }
    }
}

