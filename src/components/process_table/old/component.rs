use crate::components::process_table::state::ProcessTableState;
use crate::components::process_table::controller::{ProcessTableAction,ProcessTableController};
use crate::components::process_table::view::ProcessTableView;
use crate::events::EventState;
use crate::adapters::crossterm::input::Key;
use crate::domain::process::model::ProcessSnapShot;

/// Idea: model update => Reconstruct ProcessTableComponent
/// Some state might want to be preserved
pub struct ProcessTableComponent {
    state:      ProcessTableState,
    controller: ProcessTableController,
    view:       ProcessTableView
}

impl Default for ProcessTableComponent {
    fn default() -> Self {
        let state      = ProcessTableState::default();
        let view       = ProcessTableView::default();
        let controller = ProcessTableController::default();
        Self { state, view, controller }
    }
}

// import ratatui
use ratatui::prelude::{Frame,Rect};
impl ProcessTableComponent {
    pub fn handle_model_update(&mut self, process_snapshot: &ProcessSnapShot) {
        self.state.handle_model_update(&process_snapshot);
    }

    pub fn handle_key_event(&mut self, key: Key, process_snapshot: &ProcessSnapShot) -> EventState {
        let action: Option<ProcessTableAction> = self.controller.handle_key_event(key);
        if let Some(a) = action {
            self.state.handle_action(a, &process_snapshot);
            EventState::Consumed
        } else {
            EventState::NotConsumed
        }
    }

    pub fn handle_draw(&mut self,
        frame: &mut Frame,
        area: Rect,
        focus: bool,
        process_snapshot: &ProcessSnapShot) -> anyhow::Result<()> {
        self.view.handle_draw(frame, area, focus, &process_snapshot, &self.state)
    }
}

