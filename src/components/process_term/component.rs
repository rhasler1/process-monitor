use anyhow::{Ok, Result};

use ratatui::layout::Alignment;
use ratatui::prelude::{Frame, Rect, Layout, Direction, Constraint, Line};
use ratatui::widgets::{Block, Paragraph, Wrap};
use ratatui::style::{Style, Stylize};

use crate::adapters::crossterm::input::Key;
use crate::events::EventState;

pub enum Event {
    Confirm,
    Deny
}

#[derive(Default)]
pub struct Model {
    pid: Option<u32>
}

impl Model {
    pub fn event(&self, event: Event) -> EventState {
        match event {
            Event::Confirm => {
                if let Some(pid) = self.pid {
                    EventState::ReturnPID(pid)
                } else {
                    EventState::Consumed
                }
            },
            Event::Deny => EventState::Consumed
        }
    }

    pub fn set(&mut self, pid: Option<u32>) {
        self.pid = pid;
    }

    pub fn get(&self) -> Option<u32> {
        self.pid
    }
}

#[derive(Default)]
pub struct Controller;
impl Controller {
    pub fn key_event(&self, key: Key) -> Option<Event> {
        match key {
            Key::Char('y') => Some(Event::Confirm),
            Key::Char('n') => Some(Event::Deny),
            _ => None
        }
    }
}

#[derive(Default)]
pub struct View;
impl View {
    pub fn draw(
        &self,
        frame: &mut Frame,
        area: Rect,
        _focus: bool,
        model: &Model) -> Result<()> {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Fill(1)
                ]).split(area);

            let pid = model.get();
            let msg1 = format!("Selected PID to terminate: {pid:?}");
            let msg2 = format!("Press 'y' to confirm or 'n' to deny");
            let line1 = Line::from(msg1);
            let line2 = Line::from(msg2);
            let vec = vec![line1,line2];
            let paragraph = Paragraph::new(vec)
                .block(Block::bordered().title("Termination Screen"))
                .style(Style::new().white().on_black())
                .alignment(Alignment::Center)
                .wrap(Wrap {trim: true});

            frame.render_widget(paragraph, chunks[0]);

            Ok(())
        }
}

#[derive(Default)]
pub struct Component {
    model:      Model,
    view:       View,
    controller: Controller,
}

impl Component {
    pub fn event(&self, key: Key) -> EventState {
        if let Some(event) = self.controller.key_event(key) {
            self.model.event(event)
        } else {
            EventState::NotConsumed
        }
    }

    pub fn set(&mut self, pid: Option<u32>) {
        self.model.set(pid);
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect, focus: bool) -> Result<()> {
        self.view.draw(frame, area, focus, &self.model)?;
        Ok(())
    }
}
