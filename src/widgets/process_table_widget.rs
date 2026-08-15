use process_table::{ProcessTable, ProcessEntry as ProcessTableRow, Column};
use crate::components::process_table::ProcessTableViews;

use ratatui::{
    layout::{Constraint::Fill, Layout}, prelude::{Buffer, Constraint, Rect}, style::{Color, Style}, text::Span, widgets::{StatefulWidget, TableState},
};

use ratatui::widgets::Cell;
use ratatui::widgets::Row;
use ratatui::widgets::Table;

pub struct ProcessTableWidget<'a> {
    process_table: &'a ProcessTable,
}

impl <'a> ProcessTableWidget<'a> {
    pub fn new(process_table: &'a ProcessTable) -> Self {
        Self {
            process_table
        }
    }
}

impl StatefulWidget for ProcessTableWidget<'_> {
    type State = ProcessTableViews;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {

        let row_chunks = Layout::vertical(
            vec![Constraint::Length(1); area.height as usize]).split(area);

        // Get visual row selection, if none, then table is empty, return.
        let Some(visual_row_selection) = state.active_view().visual_row_selection().selection() else {
            return
        };

        // Calculate row scroll offset
        let visual_row_offset = 
            state.mut_active_view().mut_row_scroll().calc_start(row_chunks.len(), visual_row_selection);

        // Create visible rows iterator with sort & filter applied
        let visible_table_rows = self.process_table.visible_rows(
            state.active_view().table_state().row_sort(),
            state.active_view().table_state().filter_ast()
        );

        let rows = visible_table_rows
            .enumerate()
            .skip(visual_row_offset)
            .take(row_chunks.len())
            .map(|(visible_index, process_table_row)| {
                if visible_index == visual_row_selection {
                    Row::new(
                        vec![
                            Cell::from(process_table_row.name().as_str())
                        ]
                    ).style(Style::default().bg(Color::Cyan))
                } else {
                    Row::new(
                        vec![
                            Cell::from(process_table_row.name().as_str())
                        ]
                    ).style(Style::default())
                }
            }).collect::<Vec<_>>();

        let col_widths: Vec<Constraint> = vec![Constraint::Percentage(100)];
        let table_widget = Table::new(rows, col_widths);

        table_widget.render(area, buf, &mut TableState::default());
    }
}
