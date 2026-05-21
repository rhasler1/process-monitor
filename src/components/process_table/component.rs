use anyhow::Result;
use ratatui::prelude::{Frame, Rect};
use crate::components::{Draw, Event};
use crate::events::EventState;
use crate::adapters::crossterm::input::Key;
use crate::domain::process::model::ProcessSnapShot;
use crate::components::process_table::{
    table::TableModel,
    view::TableView,
    controller::TableController
};
use crate::config::app_config::Config;

pub struct TableComponent {
    model:      TableModel,
    controller: TableController,
    view:       TableView
}

impl TableComponent {
    pub fn new(snapshot: &ProcessSnapShot, config: &Config) -> Self {
        Self {
            model:      TableModel::new(snapshot, config),
            controller: TableController,
            view:       TableView::default()
        }
    }

    pub fn new_snapshot(&mut self, snapshot: &ProcessSnapShot) {
        self.model.new_snapshot(snapshot);
    }
}

impl Event for TableComponent {
    fn event(&mut self, key: Key) -> EventState {
        if let Some(event) = self.controller.key_event(key, &self.model) {
            self.model.event(event)
        } else {
            EventState::NotConsumed
        }
    }
}

impl Draw for TableComponent {
    fn draw(&mut self, frame: &mut Frame, area: Rect, focus: bool) -> Result<()> {
        self.view.draw(frame, area, focus, &self.model)?;
        Ok(())
    }
}

impl From<&ProcessSnapShot> for TableComponent {
    fn from(snapshot: &ProcessSnapShot) -> Self {
        Self {
            model:      TableModel::from(snapshot),
            controller: TableController,
            view:       TableView::default()
        }
    }
}
