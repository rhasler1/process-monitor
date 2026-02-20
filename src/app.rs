// Ratatui
use ratatui::prelude::{Frame,Layout,Direction,Constraint,Alignment,Style,Span,Color};
use ratatui::widgets::Paragraph;
// Internal application
use crate::adapters::crossterm::input::Key;
use crate::domain::process::model::{ProcessSnapShot};
//use crate::components::{Component,DrawableComponent};
use crate::components::process_table::component::ProcessTableComponent;
use crate::events::EventState;

pub struct App {
    process_snapshot: ProcessSnapShot, // DomainModel
    process_table:    ProcessTableComponent     // Component
}

impl App {
    /// Must call app.init() after creating an `App` with App::default()
    pub fn default() -> Self {
        Self {
            process_snapshot: ProcessSnapShot::default(),
            process_table:    ProcessTableComponent::default(),
        }
    }
    
    pub fn model_update(&mut self, process_snapshot: ProcessSnapShot) {
        self.process_snapshot = process_snapshot;
        self.process_table.handle_model_update(&self.process_snapshot);
    }

    pub fn key_event(&self, key: Key) -> EventState {
        EventState::NotConsumed
    }

    pub fn draw(&mut self, frame: &mut Frame) -> anyhow::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
            ])
            .split(frame.size());
        self.process_table.handle_draw(
            frame,
            chunks[0],
            true,
            &self.process_snapshot)?;
        Ok(())
    }
}

