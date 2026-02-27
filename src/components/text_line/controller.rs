use crate::adapters::crossterm::input::Key;
use crate::components::text_line::model::{MoveDirection, TextLineAction};

#[derive(Default)]
pub struct TextLineController;
impl TextLineController {
    pub fn handle_key_event(&self, key: Key) -> Option<TextLineAction> {
        match key {
            Key::Left       => Some(TextLineAction::MoveCursor(MoveDirection::Left)),
            Key::Right      => Some(TextLineAction::MoveCursor(MoveDirection::Right)),
            Key::Char(c)    => Some(TextLineAction::InsertCharacter(c)),
            Key::Backspace  => Some(TextLineAction::RemoveCharacter),
            _               => None
        }
    }
}

