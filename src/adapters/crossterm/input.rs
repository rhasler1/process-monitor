// Crossterm event (adapt)=> application input
use crossterm::event::{
    KeyCode, KeyEvent, KeyModifiers
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
    
    Ctrlc,
    Ctrlx,
    Ctrls,
    Ctrlh,
    Ctrlv,
    Ctrld,
    Ctrlp,
    Ctrln,

    Unknown
}

impl From<KeyEvent> for Key {
    fn from(event: KeyEvent) -> Self {
        match (event.code, event.modifiers) {
            (KeyCode::Char('c'),    KeyModifiers::CONTROL)  => Key::Ctrlc,
            (KeyCode::Char('x'),    KeyModifiers::CONTROL)  => Key::Ctrlx,
            (KeyCode::Char('s'),    KeyModifiers::CONTROL)  => Key::Ctrls,
            (KeyCode::Char('h'),    KeyModifiers::CONTROL)  => Key::Ctrlh,
            (KeyCode::Char('v'),    KeyModifiers::CONTROL)  => Key::Ctrlv,
            (KeyCode::Char('d'),    KeyModifiers::CONTROL)  => Key::Ctrld,
            (KeyCode::Char('p'),    KeyModifiers::CONTROL)  => Key::Ctrlp,
            (KeyCode::Char('n'),    KeyModifiers::CONTROL)  => Key::Ctrln,

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

