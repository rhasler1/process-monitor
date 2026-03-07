use crate::components::process_table::model::{Action, ProcessOrder, Direction};
use crate::adapters::crossterm::input::Key;

pub enum Action {
    Sort(ProcessOrder),
    Move(Direction)
}

#[derive(Default)]
pub struct ProcessTableController;
impl ProcessTableController {
    pub fn handle_key_event(&self, key: Key) -> Option<Action> {
        match key {
            Key::Up   => Some(Action::Move(Direction::Up)),
            Key::Down => Some(Action::Move(Direction::Down)),
            _         => None 
        }
    }
}

