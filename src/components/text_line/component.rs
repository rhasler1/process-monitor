// Unlike `ProcessTableComponent` the `TextLineComponent` is independent of a `DomainModel`. The `TextLineComponent`
// contains a mutable model. I believe this Component matches Martin Fowler's description of MVC.

use anyhow::Result;
use crate::components::text_line::model::TextLineModel;
use crate::components::text_line::controller::TextLineController;
use crate::components::text_line::view::TextLineView;
use crate::adapters::crossterm::input::Key;
use crate::events::EventState;

pub struct TextLineComponent {
    model:      TextLineModel,
    controller: TextLineController,
    view:       TextLineView
}

impl Default for TextLineComponent {
    fn default() -> Self {
        Self {
            model:      TextLineModel::default(),
            controller: TextLineController,
            view:       TextLineView::default()
        }
    }
}

use ratatui::prelude::{Frame, Rect};
impl TextLineComponent {
    pub fn handle_key_event(&mut self, key: Key) -> EventState {
        let action = self.controller.handle_key_event(key);
        if let Some(a) = action { // The key is recognized by the controller
            // The ret val indicates if the action could be processed by the `Model`
            // This value can be used to display some information to the screen
            let _ = self.model.handle_event(a);
            EventState::Consumed
        } else { // The key is not recognized by the controller
            EventState::NotConsumed
        }
    }

    // Ratatui
    pub fn handle_draw(&mut self,
        frame: &mut Frame,
        area:  Rect,
        focus: bool) -> Result<()> {
        self.view.handle_draw(frame, area, focus, &self.model)
    }
}
