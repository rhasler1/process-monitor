pub mod process_component;

pub trait DrawableComponent {
    fn draw(&mut self,
        frame:   &mut ratatui::prelude::Frame, 
        area:    ratatui::prelude::Rect, 
        focused: bool) -> anyhow::Result<()>;
}

// Internal application imports 
use crate::adapters::crossterm::input::{KeyInput,MouseInput};
use crate::app::EventState;
pub trait Component {
    fn key_event(&mut self, key: KeyInput) -> EventState;
    fn mouse_event(&mut self, mouse: MouseInput) -> EventState;
}
