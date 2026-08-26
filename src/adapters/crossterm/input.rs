// Crossterm event (adapt)=> application input
use crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers, ModifierKeyCode
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
    Alto,
    Alts,
    Altd,
    Alth,
    Altv,
    Ctrlc,
    Ctrls,
    AltLeft,
    AltRight,
    Unknown
}

impl From<KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        match (event.code, event.modifiers) {
            (KeyCode::Char('c'),    KeyModifiers::CONTROL)  => Key::Ctrlc,
            (KeyCode::Char('s'),    KeyModifiers::CONTROL)  => Key::Ctrls,
            (KeyCode::Char('s'),    KeyModifiers::ALT)      => Key::Alts,
            (KeyCode::Char('d'),    KeyModifiers::ALT)      => Key::Altd,
            (KeyCode::Char('h'),    KeyModifiers::ALT)      => Key::Alth,
            (KeyCode::Char('v'),    KeyModifiers::ALT)      => Key::Altv,
            (KeyCode::Char('o'),    KeyModifiers::ALT)      => Key::Alto,
            (KeyCode::Left,         KeyModifiers::ALT)      => Key::AltLeft,
            (KeyCode::Right,        KeyModifiers::ALT)      => Key::AltRight,


            (KeyCode::Enter,        _)  => Key::Enter,
            (KeyCode::Esc,          _)  => Key::Esc,
            (KeyCode::Char(char),   _)  => Key::Char(char),
            (KeyCode::Backspace,    _)  => Key::Backspace,
            (KeyCode::Delete,       _)  => Key::Delete,
            (KeyCode::Up,           _)  => Key::Up,
            (KeyCode::Down,         _)  => Key::Down,
            (KeyCode::Left,         _)  => Key::Left,
            (KeyCode::Right,        _)  => Key::Right,
            (KeyCode::Tab,          _)  => Key::Tab,
            (KeyCode::PageUp,       _)  => Key::PageUp,
            (KeyCode::PageDown,     _)  => Key::PageDown,

            _                           => Key::Unknown
        }
    }
}

