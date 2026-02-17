// Ratatui
use ratatui::prelude::{Frame,Layout,Direction,Constraint,Alignment,Style,Span,Color};
use ratatui::widgets::Paragraph;
// Internal application adapters
//use crate::adapters::sysinfo::sysinfo_datasource::SysinfoDataSource;
use crate::adapters::crossterm::input::{KeyInput,MouseInputKind,MouseInput};
// Internal application components
//use crate::components::process_component::process::ProcessComponent;
// Internal application component traits
//use crate::components::{Component,DrawableComponent};
// Internal application common types
//use crate::app::EventState; // TODO 2/16/2026 - Move EventState to events/?

/// This enumerator describes all of the components that can be in focus
enum AppFocus {
    ProcessComponent
}

pub struct App {
    //data_source:       SysinfoDataSource,
    focus:             AppFocus,
    //process_component: ProcessComponent
}

impl App {
    pub fn init() -> Self {
        //let mut data_source = SysinfoSource::default();
        //data_source.refresh_all();
        //let process_component: ProcessComponent = ProcessComponent::new(&data_source);
        let focus: AppFocus = AppFocus::ProcessComponent;
        Self {
            //data_source,
            focus,
            //process_component
        }
    }

    pub fn key_event(&mut self, key: KeyInput) -> EventState {
        match self.focus {
            AppFocus::ProcessComponent => {
                //if self.process_component.key_event(key).is_consumed() {
                    //return EventState::Consumed
                //}
            }
        }
        if self.move_focus(key).is_consumed() {
            return EventState::Consumed
        }
        EventState::NotConsumed
    }

    fn move_focus(&mut self, key: KeyInput) -> EventState {
        if key == KeyInput::Tab { /*TODO*/ }
        return EventState::NotConsumed
    }

    pub fn mouse_event(&mut self, mouse: MouseInput) -> EventState {
        // TODO 2/15/2026
        EventState::NotConsumed
    }

    pub fn refresh_event(&mut self) {
        //self.data_source.refresh_all();
        //self.process_component.refresh_event(&self.data_source)
    }

    pub fn draw(&mut self, frame: &mut Frame) -> anyhow::Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),
            ])
            .split(frame.size());
        //self.process_component.draw(frame, chunks[0], true);

        Ok(())
    }
}

// TODO 2/16/2026 - Move to events/?
#[derive(PartialEq)]
pub enum EventState {
    Consumed,
    NotConsumed
}

impl EventState {
    pub fn is_consumed(&self) -> bool {
        *self == Self::Consumed
    }
}
