use crate::adapters::crossterm::input::Key;
use crate::components::text_line::model::{MoveDirection, TextLineEvent};

#[derive(Default)]
pub struct TextLineController;
impl TextLineController {
    pub fn handle_key_event(&self, key: Key) -> Option<TextLineEvent> {
        match key {
            Key::Left       => Some(TextLineEvent::MoveCursor(MoveDirection::Left)),
            Key::Right      => Some(TextLineEvent::MoveCursor(MoveDirection::Right)),
            Key::Char(c)    => Some(TextLineEvent::InsertCharacter(c)),
            Key::Backspace  => Some(TextLineEvent::RemoveCharacter),
            _               => None
        }
    }
}

