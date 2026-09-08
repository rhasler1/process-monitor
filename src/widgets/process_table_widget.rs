use process_table::{ColumnOptions, ProcessTable, MemoryUnitOptions};
use crate::components::process_table::{ProcessTableViews, ViewsOrientation};

use ratatui::{
    layout::Layout,
    prelude::{Buffer, Constraint, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Cell, Row, Table, StatefulWidget, Widget},
};

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
        let view_count = state.count_views();

        let constraints = (0..view_count)
            .map(|_| Constraint::Ratio(1, view_count as u32))
            .collect::<Vec<_>>();

        let view_layout = match state.views_orientation() {
            ViewsOrientation::SplitVertical => Layout::vertical(constraints.clone()),
            ViewsOrientation::SplitHorizontal => Layout::horizontal(constraints.clone())
        };

        let view_chunks = view_layout.split(area);

        // Update view scrolls before looping:
        {
            for (view_index, view) in state.mut_views().enumerate() {
                let row_chunks = Layout::vertical(vec![
                    Constraint::Length(1), // Header
                    Constraint::Length(1), // Border top
                    Constraint::Fill(1),
                    Constraint::Length(4), // Filter
                    Constraint::Length(1), // Border bottom
                ]).split(view_chunks[view_index]);

                // Get visual_row_selection, if None, set var to 0.
                let visual_row_selection = view
                    .table_state()
                    .row_selection()
                    .selection()
                    .unwrap_or_default();

                // Update scroll state
                view
                    .mut_table_state()
                    .mut_row_scroll()
                    .calc_start(
                        row_chunks[2].height as usize,
                        visual_row_selection
                    );
            }
        }

        for (view_index, view) in state.views().enumerate() {

            let row_chunks = Layout::vertical(vec![
                Constraint::Length(1), // Border
                Constraint::Length(1), // Header
                Constraint::Fill(1),
                Constraint::Length(4), // Filter
                Constraint::Length(1), // Border
            ]).split(view_chunks[view_index]);

            // Get visual row selection, if none, then table is empty, return.
            let visual_row_selection = view
                .table_state()
                .row_selection()
                .selection()
                .unwrap_or_default();

            // Create visible rows iterator with sort & filter applied
            let row_sort = view
                .table_state()
                .row_sort();

            let filter_ast = view
                .table_state()
                .filter_ast();
            
            let visible_table_rows = self.process_table.visible_rows(
                row_sort,
                filter_ast,
            );

            let visual_row_offset = view
                .table_state()
                .row_scroll()
                .start();

            let column_selection = view
                .table_state()
                .columns()
                .selection();

            let rows = visible_table_rows
                .enumerate()
                .skip(visual_row_offset)
                .take(row_chunks[2].height as usize)
                .map(|(visible_index, process_table_row)| {
                    let mut cells: Vec<Cell> = Vec::new();

                    for (column_index, column_config) in view.table_state().columns().columns().enumerate() {
                        let cell = match column_config {
                            ColumnOptions::Pid => {
                                Cell::from(format!("{:?}", process_table_row.process().pid().as_u32()))
                            }
                            ColumnOptions::CpuTotal => {
                                Cell::from(format!("{:?}", process_table_row.process().cpu_total().as_f32()))
                            }
                            ColumnOptions::CpuAverage => {
                                Cell::from(format!("{:?}", process_table_row.process().cpu_average().as_f32()))
                            }
                            ColumnOptions::Memory(unit) => {
                                match unit {
                                    MemoryUnitOptions::B => {
                                        Cell::from(format!("{:?}", process_table_row.process().mem().as_bytes()))
                                    }
                                    MemoryUnitOptions::KB => {
                                        Cell::from(format!("{:?}", process_table_row.process().mem().as_kb()))
                                    }
                                    MemoryUnitOptions::MB => {
                                        Cell::from(format!("{:?}", process_table_row.process().mem().as_mb()))
                                    }
                                    MemoryUnitOptions::GB => {
                                        Cell::from(format!("{:?}", process_table_row.process().mem().as_gb()))
                                    }
                                }
                            }
                            ColumnOptions::Name => {
                                Cell::from(process_table_row.process().name().as_str())
                            }
                            ColumnOptions::MeanCpuUsageOverLastMinute => {
                                Cell::from(format!("{:?}", process_table_row.statistics().mean_cpu_usage_last_minute()))
                            }
                            ColumnOptions::MeanCpuUsageAsTotalOverLastMinute => {
                                Cell::from(format!("{:?}", process_table_row.statistics().mean_cpu_usage_as_total_last_minute()))
                            }
                        };

                        let style = match column_selection {
                            Some(selection) if selection == column_index => {
                                Style::default()
                                    .fg(Color::Black)
                                    .bg(Color::Cyan)
                            }
                            _ => Style::default()
                        };

                        cells.push(cell.style(style))
                    }

                    let style = if visible_index == visual_row_selection {
                        Style::default().fg(Color::Black).bg(Color::Cyan)
                    } else {
                        Style::default()
                    };

                    Row::new(cells).style(style)
                }).collect::<Vec<_>>();

            let columns: Vec<_> = view
                .table_state()
                .columns()
                .columns()
                .collect();

            let col_width = if columns.is_empty() {
                0
            } else {
                100 / columns.len()
            };

            let col_widths = vec![Constraint::Percentage(col_width as u16); columns.len()];

            let header = columns
                .iter()
                .enumerate()
                .map(|(col_idx, col)| {
                    let style = match column_selection {
                        Some(selection) if selection == col_idx => {
                            Style::default()
                                .fg(Color::Black)
                                .bg(Color::Cyan)
                        }
                        _ => Style::default()
                    };

                    Cell::new(col.as_str()).style(style)
                })
                .collect::<Row>()
                .style(Style::default().fg(Color::Black).bg(Color::LightBlue));

            let border_style = if view_index == state.views_selection() {
                Style::default().fg(Color::LightBlue)
            } else {
                Style::default()
            };

            let table_widget = Table::new(rows, col_widths)
                .header(header)
                .block(
                    Block::default()
                    .borders(Borders::all())
                    .title_top(format!(" view {} ", view_index))
                    .border_style(border_style)
                );

            // Render table
            Widget::render(table_widget, view_chunks[view_index], buf);
            
            let mut lines = vec![];
            let filte_err_span = if let Some(err_msg) = view.filter_err_msg() {
                err_msg
            } else {
                ""
            };

            lines.push(Line::from(vec![Span::from(filte_err_span).style(Style::default().fg(Color::Red))]));
            let filter_str = "> ".to_owned() + view.table_state().filter_string().as_str();
            lines.push(Line::from(vec![Span::from(filter_str)]));
            
            let filter_chunks = Layout::horizontal(vec![
                Constraint::Ratio(1, 64),
                Constraint::Ratio(62, 64),
                Constraint::Ratio(1, 64)
            ]).split(row_chunks[3]);

            let filter_widget = Paragraph::new(lines)
                .block(
                    Block::default()
                    .borders(Borders::all())
                    .title(" Filter ")
                    .style(Style::default().fg(Color::LightBlue))
                );
            
            // Render filter
            filter_widget.render(filter_chunks[1], buf);
        }
    }
}

