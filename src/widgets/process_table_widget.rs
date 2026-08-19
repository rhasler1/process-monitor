use process_table::{Column, ProcessTable, MemoryUnitOptions};
use crate::components::process_table::{ProcessTableViews, ViewsOrientation};

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
                    Constraint::Fill(1)
                ]).split(view_chunks[view_index]);

                // Get visual row selection, if none, then table is empty, return.
                let Some(visual_row_selection) = view
                    .table_state()
                    .row_selection()
                    .selection() else {
                        // Bug here, header won't get drawn.
                        return
                };

                view
                    .mut_table_state()
                    .mut_row_scroll()
                    .calc_start(
                        row_chunks[1].height as usize,
                        visual_row_selection
                    );
            }
        }

        for (view_index, view) in state.views().enumerate() {

            let row_chunks = Layout::vertical(vec![
                Constraint::Length(1), // Header
                Constraint::Fill(1)
            ]).split(view_chunks[view_index]);

            // Get visual row selection, if none, then table is empty, return.
            let Some(visual_row_selection) = view
                .table_state()
                .row_selection()
                .selection() else {
                    // Bug here, header won't get drawn.
                    return
            };

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
                .take(row_chunks[1].height as usize)
                .map(|(visible_index, process_table_row)| {
                    let mut cells: Vec<Cell> = Vec::new();


                    for (column_index, column_config) in view.table_state().columns().columns().enumerate() {
                        let mut cell = match column_config.column() {
                            Column::Pid => {
                                Cell::from(format!("{:?}", process_table_row.process().pid().as_u32()))
                            }
                            Column::CpuTotal => {
                                Cell::from(format!("{:?}", process_table_row.process().cpu_total().as_f32()))
                            }
                            Column::CpuAverage => {
                                Cell::from(format!("{:?}", process_table_row.process().cpu_average().as_f32()))
                            }
                            Column::Memory(unit) => {
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
                            Column::Name => {
                                Cell::from(process_table_row.process().name().as_str())
                            }
                            Column::MeanCpuUsageOverLastMinute => {
                                Cell::from(format!("{:?}", process_table_row.statistics().mean_cpu_usage_last_minute()))
                            }
                            Column::MeanCpuUsageAsTotalOverLastMinute => {
                                Cell::from(format!("{:?}", process_table_row.statistics().mean_cpu_usage_as_total_last_minute()))
                            }
                        };

                        let style = if let Some(column_selection) = column_selection {
                            if column_selection == column_index {
                                Style::default().bg(Color::Blue)
                            } else {
                                Style::default()
                            }
                        } else {
                            Style::default()
                        };

                        cells.push(cell.style(style))
                    }

                    let style = if visible_index == visual_row_selection {
                        Style::default().bg(Color::Cyan)
                    } else {
                        Style::default()
                    };

                    Row::new(cells).style(style)
                }).collect::<Vec<_>>();

            // For now just a percent
            let col_count = view.table_state().columns().columns().count();
            
            let col_widths = if col_count > 0 {100 / col_count} else {0};

            let col_widths: Vec<Constraint> = view
                .table_state()
                .columns()
                .columns()
                .map(|_| Constraint::Percentage(col_widths as u16))
                .collect();
            
            let header = view
                .table_state()
                .columns()
                .columns()
                .enumerate()
                .map(|(col_idx, col)| {
                    let style = if let Some(column_selection) = column_selection {
                        if column_selection == col_idx {
                            Style::default().bg(Color::Blue)
                        } else {
                            Style::default()
                        }
                    } else {
                        Style::default()
                    };
                    
                    Cell::new(col.column().as_str()).style(style)
                }).collect::<Row>().style(Style::default().bg(Color::Green));

            let table_widget = Table::new(rows, col_widths).header(header);

            table_widget.render(view_chunks[view_index], buf, &mut TableState::default());
        }
    }
}


