use std::option;

use crossterm::style::style;
use process_table::{ColumnOptions, ProcessTable, MemoryUnitOptions};
use crate::components::config_manager::ConfigManagerMenu;

use ratatui::{
    layout::{Constraint::Fill, Layout}, prelude::{Buffer, Constraint, Rect}, style::{Color, Style}, text::{Line, Span}, widgets::{Block, Borders, List, ListItem, ListState, Paragraph, StatefulWidget, TableState, Widget},
};
use ratatui::prelude::{Direction, Alignment};
use ratatui::widgets::Cell;
use ratatui::widgets::Row;
use ratatui::widgets::Table;

pub struct ConfigMenuWidget<'a> {
    menu: &'a ConfigManagerMenu,
}

impl <'a> ConfigMenuWidget<'a> {
    pub fn new(menu: &'a ConfigManagerMenu) -> Self {
        Self {
            menu
        }
    }
}

impl Widget for ConfigMenuWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized
    {
        let options = self.menu.menu_options();

        let selection = self.menu.menu_selection();

        let menu_width = {
            let longest_option = options
                .iter()
                .map(|option| option.to_string().len())
                .max()
                .unwrap_or(0);
            
            // +6:
            //      +2, for selection indicator,
            //      +2, for borders
            //      +2, for padding between borders
            longest_option as u16 + 6.min(area.width)
        };

        // +2 for borders
        let menu_height = (options.len() as u16 + 2).min(area.height);

        let [_, menu_vert, _] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(menu_height),
                Constraint::Fill(1)
            ])
            .areas(area);

        let [_, menu_horizontal, _] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(menu_width),
                Constraint::Fill(1)
            ])
            .areas(menu_vert);

        let items: Vec<ListItem> = options
            .iter()
            .map(|option| {
                ListItem::new(format!("{}", option))
            })
            .collect();

        let mut state = ListState::default();
        state.select(Some(selection));

        StatefulWidget::render(List::new(items)
            .block(
                Block::default()
                .borders(Borders::all())
                .title(" Config Menu ")
                .title_alignment(Alignment::Center)
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::LightCyan)
            )
            .highlight_symbol("> "),
            menu_horizontal,
            buf,
            &mut state,
        )
    }
}
