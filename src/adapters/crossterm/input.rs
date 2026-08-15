// Crossterm event (adapt)=> application input
use crossterm::event::{
    KeyEvent,
    KeyCode,
    ModifierKeyCode,
    //KeyModifiers
};

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum Key {
    Enter,
    Esc,
    Char(char),
    Backspace,
    Delete,
    Up,
    Down,
    Left,
    Right,
    Tab,
    PageUp,
    PageDown,
    Unknown
}

impl From<KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        match event.code {
            KeyCode::Enter      => Key::Enter,
            KeyCode::Esc        => Key::Esc,
            KeyCode::Char(char) => Key::Char(char),
            KeyCode::Backspace  => Key::Backspace,
            KeyCode::Delete     => Key::Delete,
            KeyCode::Up         => Key::Up,
            KeyCode::Down       => Key::Down,
            KeyCode::Left       => Key::Left,
            KeyCode::Right      => Key::Right,
            KeyCode::Tab        => Key::Tab,
            KeyCode::PageUp     => Key::PageUp,
            KeyCode::PageDown   => Key::PageDown,

            _                   => Key::Unknown
        }
    }
}

