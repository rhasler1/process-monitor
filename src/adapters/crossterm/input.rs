// Crossterm event (adapt)=> application input
use crossterm::event::{
    KeyEvent,
    KeyCode,
    MouseEvent,
    MouseEventKind,
    MouseButton
};

#[derive(PartialEq, Eq)]
pub enum KeyInput {
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

impl From<KeyEvent> for KeyInput {
    fn from(event: KeyEvent) -> Self {
        match event.code {
            KeyCode::Enter      => KeyInput::Enter,
            KeyCode::Esc        => KeyInput::Esc,
            KeyCode::Char(char) => KeyInput::Char(char),
            KeyCode::Backspace  => KeyInput::Backspace,
            KeyCode::Up         => KeyInput::Up,
            KeyCode::Down       => KeyInput::Down,
            KeyCode::Left       => KeyInput::Left,
            KeyCode::Right      => KeyInput::Right,
            KeyCode::Tab        => KeyInput::Tab,
            _                   => KeyInput::Unknown
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum MouseInputKind {
    LeftClick,
    ScrollUp,
    ScrollDown,
    Unknown
}

pub struct MouseInput {
    pub kind: MouseInputKind,
    pub column: u16,
    pub row: u16
}

impl From<MouseEvent> for MouseInput {
    fn from(event: MouseEvent) -> Self {
        let kind = match event.kind {
            MouseEventKind::Down(MouseButton::Left) => MouseInputKind::LeftClick,
            MouseEventKind::ScrollUp   => MouseInputKind::ScrollUp,
            MouseEventKind::ScrollDown => MouseInputKind::ScrollDown,
            _ => MouseInputKind::Unknown
        };

        Self {
            kind,
            column: event.column,
            row: event.row
        }
    }
}
