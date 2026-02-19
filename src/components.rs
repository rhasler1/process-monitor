pub mod process_table;

use anyhow::Result;
use ratatui::prelude::*;
use crate::events::EventState;
use crate::adapters::crossterm::input::{KeyInput,MouseInput};

pub trait DrawableComponent {
    fn draw(&mut self, f: &mut Frame, area: Rect, focused: bool) -> Result<()>;
}

pub trait Component {
    fn key_event(&mut self, key: KeyInput) -> Result<EventState>;
    fn mouse_event(&mut self, mouse: MouseInput) -> Result<EventState>;
}
