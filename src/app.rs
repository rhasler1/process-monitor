// Log
use log::{debug, info};
// Ratatui for rendering and widget creation
use ratatui::prelude::{Frame, Layout, Direction, Constraint};
// Internal application
use crate::adapters::crossterm::input::Key;
use crate::domain::process::model::{ProcessSnapShot};
use crate::components::process_table::component::TableComponent;
use crate::components::process_term::component::Component as ProcTermComponent;
use crate::events::EventState;

#[derive(Default)]
pub enum Focus {
    #[default]
    Table,
    Termination
}

pub struct App {
    process_snapshot: ProcessSnapShot, // DomainModel
    process_table:    TableComponent,  // Component
    process_term:     ProcTermComponent,
    focus:            Focus
}

impl App {
    pub fn default() -> Self {
        let process_snapshot = ProcessSnapShot::default();
        let process_table    = TableComponent::from(&process_snapshot);
        let process_term     = ProcTermComponent::default();
        let focus            = Focus::default();

        Self {
            process_snapshot,
            process_table,
            process_term,
            focus
        }
    }
    
    pub fn model_update(&mut self, process_snapshot: ProcessSnapShot) {
        self.process_snapshot = process_snapshot;
        self.process_table.new_snapshot(&self.process_snapshot);
    }

    pub fn key_event(&mut self, key: Key) -> EventState {
        debug!("`App`: key_event()");
        let return_val = match self.focus {
            Focus::Table => {
                let mut event_state = self.process_table.key_event(key);
                if event_state.is_return_payload() {
                    // safe to unwrap here
                    let pid = event_state.payload().unwrap();
                    info!("`App`: key_event(): event_state.payload() = {pid}");
                    self.process_term.set(pid);
                    //TODO [5/5]
                    //self.focus = Focus::Termination;
                    
                    // payload is processed and set event_state to consumed
                    event_state = EventState::Consumed;
                }
                event_state
            }
            Focus::Termination => {
                // TODO [5/5]
                EventState::NotConsumed
            }
        };
        return_val
    }


    
    pub fn draw(&mut self, frame: &mut Frame) -> anyhow::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
            ])
            .split(frame.size());

        self.process_table.draw(
            frame,
            chunks[0],
            true)?;

        Ok(())
    }
}

