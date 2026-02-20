// Crossterm event (adapt)=> application input
use crossterm::event::{
    KeyEvent,
    KeyCode
};

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum Key {
    Enter,
    Esc,
    Char(char),
    Backspace,
    Up,
    Down,
    Left,
    Right,
    Tab,
    Unknown
}

impl From<KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        match event.code {
            KeyCode::Enter      => Key::Enter,
            KeyCode::Esc        => Key::Esc,
            KeyCode::Char(char) => Key::Char(char),
            KeyCode::Backspace  => Key::Backspace,
            KeyCode::Up         => Key::Up,
            KeyCode::Down       => Key::Down,
            KeyCode::Left       => Key::Left,
            KeyCode::Right      => Key::Right,
            KeyCode::Tab        => Key::Tab,
            _                   => Key::Unknown
        }
    }
}

