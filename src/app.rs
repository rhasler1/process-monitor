// Ratatui
use ratatui::prelude::{Frame,Layout,Direction,Constraint};
// Internal application
use crate::adapters::crossterm::input::Key;
use crate::domain::process::model::{ProcessSnapShot};
//use crate::components::{Component,DrawableComponent};
// TODO [3/3/26] Add ProcessTableComponent back after rewrite
//use crate::components::process_table::component::ProcessTableComponent;
use crate::components::text_line::component::TextLineComponent;
use crate::events::EventState;


pub struct App {
    process_snapshot: ProcessSnapShot, // DomainModel
    //process_table:    ProcessTableComponent,     // Component
    text_line:        TextLineComponent
}

impl App {
    /// Must call app.init() after creating an `App` with App::default()
    pub fn default() -> Self {
        Self {
            process_snapshot: ProcessSnapShot::default(),
            //process_table:    ProcessTableComponent::default(),
            text_line:        TextLineComponent::default()
        }
    }
    
    pub fn model_update(&mut self, process_snapshot: ProcessSnapShot) {
        self.process_snapshot = process_snapshot;
        //self.process_table.handle_model_update(&self.process_snapshot);
    }

    pub fn key_event(&mut self, key: Key) -> EventState {
        let _ = self.text_line.handle_key_event(key);
        EventState::NotConsumed
    }

    pub fn draw(&mut self, frame: &mut Frame) -> anyhow::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(10),
                Constraint::Length(1)
            ])
            .split(frame.size());
        let hchunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(10),
                Constraint::Min(1)
            ]).split(chunks[1]);

        //self.process_table.handle_draw(
        //    frame,
        //    chunks[0],
        //    true,
        //    &self.process_snapshot)?;
        self.text_line.handle_draw(
            frame,
            hchunks[0],
            true
            )?;
        Ok(())
    }
}

