// Log
use log::{debug, info};
// Ratatui for rendering and widget creation
use ratatui::prelude::{Frame, Layout, Direction, Constraint};
use ratatui::widgets::StatefulWidget;
// Internal application
use crate::adapters::crossterm::input::Key;
use crate::config::app_config::Config;
use crate::domain::process::model::{ProcessSnapShot};
use crate::components::process_table::ProcessTableComponent;
use crate::components::process_term::component::Component as ProcTermComponent;
use crate::events::EventState;
use crate::widgets::ProcessTableWidget;

use crate::components::{Draw, Event};

use anyhow::Result;

#[derive(Default)]
pub enum Focus {
    #[default]
    Table,
    Termination
}

pub struct App {
    process_snapshot: ProcessSnapShot, // DomainModel
    process_table:    ProcessTableComponent,  // Component
    process_term:     ProcTermComponent,
    focus:            Focus
}

impl App {
    pub fn new(config: &Config) -> Result<Self> {
        let process_snapshot = ProcessSnapShot::default();
        let process_table    = ProcessTableComponent::new(&process_snapshot)?;
        let process_term     = ProcTermComponent::default();
        let focus            = Focus::default();

        Ok(Self {
            process_snapshot,
            process_table,
            process_term,
            focus
        })
    }
    
    pub fn model_update(&mut self, process_snapshot: ProcessSnapShot) {
        self.process_snapshot = process_snapshot;
        self.process_table.new_snapshot(&self.process_snapshot);
    }

    pub fn key_event(&mut self, key: Key) -> EventState {
        debug!("`App`: key_event()");
        match self.focus {
            Focus::Table => {
                let mut event_state = self.process_table.event(key);
                if event_state.is_return_pid() {
                    // safe to unwrap here
                    let pid = event_state.pid();
                    info!("`App`: key_event(): focus = Table: event_state.pid() = {pid:?}");
                    self.process_term.set(pid);
                    self.focus = Focus::Termination;
                    event_state = EventState::Consumed;
                }
                event_state
            }
            Focus::Termination => {
                let event_state = self.process_term.event(key);
                if event_state.is_consumed() || event_state.is_return_pid() {
                    self.process_term.set(None);
                    self.focus = Focus::Table;
                }
                event_state
            }
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) -> anyhow::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
            ])
            .split(frame.size());

        // Get table and views
        let (table, views) = self.process_table.table_and_views();

        let widget = ProcessTableWidget::new(table);
        frame.render_stateful_widget(widget, chunks[0], views);

        // TODO: Create widget from table and views
        Ok(())
    }
    
    /*pub fn draw(&mut self, frame: &mut Frame) -> anyhow::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
            ])
            .split(frame.size());

        match self.focus {
            Focus::Table => {
                self.process_table.draw(
                    frame,
                    chunks[0],
                    true)?;
            }
            Focus::Termination => {
                self.process_term.draw(
                    frame, 
                    chunks[0], 
                    true)?;
            }
        }

        Ok(())
    }*/
}

