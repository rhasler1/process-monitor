use anyhow::Result;
use crate::components::process_table::table::{TableModel, TableEvent};
use crate::components::process_table::controller::TableController;
use crate::domain::process::model::ProcessSnapShot;
use crate::events::EventState;
use crate::adapters::crossterm::input::Key;
use crate::components::process_table::view::TableView;
use ratatui::prelude::{Frame, Rect};

pub enum Focus {
    TableComponent,
    TerminationComponent
}

// TODO [5/1/26] include a termination model here, maybe
// a controller, view, and focus
pub struct TableComponent {
    model:      TableModel,
    controller: TableController,
    view:       TableView
}

impl TableComponent {
    pub fn key_event(&mut self, key: Key) -> EventState {
        if let Some(event) = self.controller.key_event(key, &self.model) {
            self.model.table_event(event)
        } else {
            EventState::NotConsumed
        }
    }

    // Special key event for terminating 
    /*pub fn key_event_term(&self, key: Key) -> Option<u32> {
        // Designating `T` as termination key
        if matches!(key, Key::Char('T')) {
            self.model.table_event_term()
        } else {
            None
        }
    }*/

    pub fn new_snapshot(&mut self, snapshot: &ProcessSnapShot) {
        self.model.new_snapshot(snapshot);
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect, focus: bool) -> Result<()> {
        self.view.draw(frame, area, focus, &self.model)?;
        Ok(())
    }
}

impl From<&ProcessSnapShot> for TableComponent {
    fn from(snapshot: &ProcessSnapShot) -> Self {
        Self {
            model:      TableModel::from(snapshot),
            controller: TableController::default(),
            view:       TableView::default()
        }
    }
}
