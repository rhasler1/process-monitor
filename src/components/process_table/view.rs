use anyhow::Result;
use crate::components::process_table::column::{ColumnID, MemUnitOptions, CPUUnitOptions};
use crate::components::process_table::table::{TableFocus, TableModel};
use crate::components::utils::scroll::Scroll;
use ratatui::layout::Alignment;
use ratatui::prelude::{Frame, Rect, Line, Layout, Direction, Constraint};
use ratatui::widgets::{Cell, Row, Table, Paragraph, Block, Borders};
use ratatui::style::{Style, Color};

#[derive(Default)]
pub struct TableView {
    table_scroll:  Scroll
}

impl TableView {
    pub fn draw(
        &mut self,
        frame: &mut Frame,
        area:  Rect,
        _focus: bool,
        table: &TableModel) -> Result<()>
    {
        // Split up area into chunks
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),  // Header & rows
                Constraint::Length(3) // Filter
            ]).split(area);

        // Subtract header height of 1 from self.height to get the count of rows that can be viewed
        let row_count = chunks[0].height.saturating_sub(1);

        // Mapping columns to ratatui `Cell's` and collecting into a ratatui `Row`
        // Styling is also done here 
        let header: Row = table.cols_iter()
            .map(|(col, selection_flag)| Cell::from(col.header()).style(if selection_flag && matches!(table.focus(), TableFocus::Columns) {
                Style::default().fg(Color::White).bg(Color::Blue)
            } else {
                Style::default().fg(Color::Black)
            })).collect::<Row>().style(Style::default().bg(Color::Green));

        // Setting the starting index to begin row iteration, see utils/scroll.rs for Scroll impl
        let start = if let Some(selection) = table.row_selection() {
            self.table_scroll.calc_start(row_count.into(), selection)
        } else {
            0
        };

        let rows = table.rows_iter()
            .skip(start)
            .take(row_count.into())
            .map(|(process_item, row_selection_flag)| {
                // [4/20/26] Cell creation can probably be optimized
                let mut cells: Vec<Cell> = Vec::new();
                table.cols_iter().for_each(|(col, _col_selection_flag)| {
                    let cell = match &col.id {
                        ColumnID::PID  => Cell::from(format!("{}",process_item.pid)),
                        ColumnID::Name => Cell::from(format!("{}",process_item.name)),
                        ColumnID::CPU(unit)  => {
                            match unit {
                                CPUUnitOptions::Avg => Cell::from(format!("{:.1}",process_item.avg_cpu_usage)),
                                CPUUnitOptions::Tot => Cell::from(format!("{:.1}",process_item.total_cpu_usage)),
                            }
                        }
                        ColumnID::Mem(unit)  => {
                            match unit {
                                MemUnitOptions::B  => Cell::from(format!("{}", process_item.mem_usage_as_b())),
                                MemUnitOptions::KB => Cell::from(format!("{}", process_item.mem_usage_as_kb())),
                                MemUnitOptions::MB => Cell::from(format!("{}", process_item.mem_usage_as_mb())),
                                MemUnitOptions::GB => Cell::from(format!("{}", process_item.mem_usage_as_gb())),
                            }
                        }
                    };
                    cells.push(cell);
                });

                let style = if row_selection_flag && matches!(table.focus(), TableFocus::Rows) {
                    Style::default().fg(Color::White).bg(Color::Blue)
                } else if matches!(table.focus(), TableFocus::Rows) {
                    Style::default().fg(Color::White)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                Row::new(cells).style(style)
            }).collect::<Vec<_>>();

        let col_width = 100 / table.cols_count();
        let col_widths: Vec<Constraint> = table.cols_iter().map(|_| (Constraint::Percentage(col_width as u16))).collect();
        let table_render = Table::new(rows, col_widths).header(header);

        // Filter
        let filter_style = if matches!(table.focus(), TableFocus::Filter) {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let filter_render = Paragraph::new(table.filter_str())
            .alignment(Alignment::Center)
            .style(filter_style)
            .block(Block::default().borders(Borders::ALL).title(" Filter ").title_alignment(Alignment::Center));

        frame.render_widget(table_render, chunks[0]);
        frame.render_widget(filter_render, chunks[1]);
        Ok(())
    }
}
