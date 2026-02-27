// Import to create theme adapter
use std::collections::HashMap;
// Import ratatui
use ratatui::style::{Style, Color};

/// Screen elements here 
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum StyleToken {
    // Token 
    Header, Row, Cell,             // Table
    Select, Focus, NotFocus,       // State
    Info, Warning, Error, Success, // Status
    Line                           // TextLine
}

/// A theme mapping from `StyleToken` to ratatui style
pub struct RatatuiTheme {
    map: HashMap<StyleToken, ratatui::style::Style>,
} 

impl RatatuiTheme {
    /// Mapping function
    pub fn style(&self, token: StyleToken) -> ratatui::style::Style {
        *self.map.get(&token).unwrap()
    }
}

impl Default for RatatuiTheme {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(StyleToken::Header,   Style::default().fg(Color::Green));
        map.insert(StyleToken::Row,      Style::default().fg(Color::White));
        map.insert(StyleToken::Cell,     Style::default().fg(Color::White));
        map.insert(StyleToken::Select,   Style::default().fg(Color::Black).bg(Color::Cyan));
        map.insert(StyleToken::Focus,    Style::default().fg(Color::Blue));
        map.insert(StyleToken::NotFocus, Style::default().fg(Color::DarkGray));
        map.insert(StyleToken::Info,     Style::default().fg(Color::Yellow));
        map.insert(StyleToken::Warning,  Style::default().fg(Color::LightRed));
        map.insert(StyleToken::Error,    Style::default().fg(Color::Red));
        map.insert(StyleToken::Success,  Style::default().fg(Color::LightGreen));
        map.insert(StyleToken::Line,     Style::default().fg(Color::White));
        Self {
            map
        }
    }
}

