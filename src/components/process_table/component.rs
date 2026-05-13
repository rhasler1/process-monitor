use anyhow::Result;
use crate::{components::process_table::table::TableModel, config::config::Config};
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

pub struct TableComponent {
    model:      TableModel,
    controller: TableController,
    view:       TableView
}

impl TableComponent {
    pub fn new(snapshot: &ProcessSnapShot, config: &Config) -> Self {
        Self {
            model:      TableModel::new(snapshot, config),
            controller: TableController::default(),
            view:       TableView::default()
        }
    }

    pub fn key_event(&mut self, key: Key) -> EventState {
        if let Some(event) = self.controller.key_event(key, &self.model) {
            self.model.event(event)
        } else {
            EventState::NotConsumed
        }
    }

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
