use ratatui::prelude::{Frame, Rect, Layout, Direction, Constraint};
use ratatui::widgets::{Cell, Row, Table};
use ratatui::style::{Style, Color};
use anyhow::Result;
use crate::components::process_table::column::ColumnID;
use crate::components::process_table::table::TableModel;
use crate::components::utils::scroll::Scroll;

#[derive(Default)]
pub struct TableView {
    scroll: Scroll
}

impl TableView {
    pub fn draw(
        &mut self,
        frame: &mut Frame,
        area:  Rect,
        _focus: bool,
        table: &TableModel) -> Result<()>
    {
        // Ratatui
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1)
            ]).split(area);

        // As percentage; Does not support columns of different widths
        let col_width = 100 / table.cols_count();
        let col_widths: Vec<_> = table.cols_iter().map(|_| (Constraint::Percentage(col_width as u16))).collect();

        // Subtract header height of 1 from self.height to get the count of rows that can be viewed
        let row_count = chunks[0].height.saturating_sub(1);

        // Ratatui
        let header: Row = table.cols_iter()
            .map(|(col, selection_flag)| Cell::from(col.header()).style(if selection_flag {
                Style::default().bg(Color::Blue)} else {Style::default().fg(Color::White)})).collect::<Row>();

        let start = if let Some(selection) = table.row_selection() {
            self.scroll.calc_start(row_count.into(), selection)
        } else {
            0
        };

        // If Columns are "injected" into Rows this can probably be flattened
        let rows = table.rows_iter()
            .skip(start)
            .take(row_count.into())
            .map(|(process_item, row_selection_flag)| {
                // cells can probably be optimized
                let mut cells: Vec<Cell> = Vec::new();
                table.cols_iter().for_each(|(col, _col_selection_flag)| {
                    let cell = match col.id {
                        ColumnID::PID  => Cell::from(format!("{}",process_item.pid)),
                        ColumnID::Name => Cell::from(format!("{}",process_item.name)),
                        ColumnID::CPU  => Cell::from(format!("{}",process_item.cpu_usage)),
                        ColumnID::Mem  => Cell::from(format!("{}",process_item.mem_usage))
                    };
                    cells.push(cell);
                });

                let style = if row_selection_flag {
                    Style::default().bg(Color::Blue)
                } else {
                    Style::default().fg(Color::White)
                };
                Row::new(cells).style(style)
            }).collect::<Vec<_>>();

        let table = Table::new(rows, col_widths).header(header);
        frame.render_widget(table, chunks[0]);
        Ok(())
    }
}
