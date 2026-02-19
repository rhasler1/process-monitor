// Ratatui
use ratatui::prelude::{Frame,Layout,Direction,Constraint,Alignment,Style,Span,Color};
use ratatui::widgets::Paragraph;
// Internal application
use crate::adapters::crossterm::input::{KeyInput,MouseInputKind,MouseInput};
use crate::core::process::model::{ProcessSnapShot};
use crate::components::{Component,DrawableComponent};
use crate::components::process_table::component::ProcessTable;
use crate::events::EventState;

pub struct App {
    process_snapshot: ProcessSnapShot,
    process_table:    ProcessTable
}

impl App {
    pub fn default() -> Self {
        Self {
            process_snapshot: ProcessSnapShot::default(),
            process_table:    ProcessTable::default(),
        }
    }

    pub fn update(&mut self, process_snapshot: ProcessSnapShot) {
        self.process_snapshot = process_snapshot;
        self.process_table.update(&self.process_snapshot);
    }

    pub fn key_event(&self, key: KeyInput) -> EventState {
        EventState::NotConsumed
    }

    pub fn draw(&mut self, frame: &mut Frame) -> anyhow::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
            ])
            .split(frame.size());
        self.process_table.draw(&self.process_snapshot, frame, chunks[0], true)?;
        Ok(())
    }
}

