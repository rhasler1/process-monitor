mod view;
use std::time::Duration;

pub use view::{ProcessTableViewFocus, ProcessTableView, ProcessTableViews, ViewsOrientation};

use process_table::{
    ColumnConfig, Column, Process, ProcessTable, RowSort, MemoryUnitOptions
};

use crate::components::Event;
use crate::adapters::crossterm::input::Key;
use crate::domain::process::model::ProcessSnapShot;

use anyhow::Result;

pub struct ProcessTableComponent {
    table: ProcessTable,
    views: ProcessTableViews,
}

impl From<&ProcessSnapShot> for Vec<Process> {
    fn from(snapshot: &ProcessSnapShot) -> Vec<Process> {
        snapshot.iter().map(|s| {
            Process::new(
                s.pid(),
                s.total_cpu_usage(),
                s.avg_cpu_usage(),
                s.mem_usage(),
                s.name().to_string_lossy().to_string())
        }).collect()
    }
}

impl ProcessTableComponent {
    pub fn new(snapshot: &ProcessSnapShot) -> Result<Self> {
        // TODO: time_interval should be provided by config...
        let table = ProcessTable::new(snapshot.into(), Duration::from_secs(2))?;

        let mut views = ProcessTableViews::default();

        // Update row selection
        let visible_rows_upper_bound = table.count_visible_rows(
            views.active_view().table_state().row_sort(),
            views.active_view().table_state().filter_ast()
        );

        views
            .mut_active_view()
            .mut_table_state()
            .mut_row_selection()
            .update_selection(visible_rows_upper_bound);

        Ok(Self {
            table,
            views: ProcessTableViews::default()
        })
    }

    pub fn new_snapshot(&mut self, snapshot: &ProcessSnapShot) -> Result<()> {
        // Update table rows
        self.table.update(snapshot.into())?;

        // Update all views selections
        for view in self.views.mut_views() {
            // Calculate new row selection upper bound for each view
            let visible_rows_upper_bound = self.table.count_visible_rows(
                view.table_state().row_sort(),
                view.table_state().filter_ast()
            );

            // Update views row selection
            view
                .mut_table_state()
                .mut_row_selection()
                .update_selection(visible_rows_upper_bound);
        }

        Ok(())
    }

    pub fn table_and_views(&mut self) -> (&ProcessTable, &mut ProcessTableViews) {
        (&self.table, &mut self.views)
    }
}

pub enum ProcessTableEventState {
    Consumed,
    NotConsumed,
    TerminatePid(u32)
}

impl Event for ProcessTableComponent {
    type EventState = ProcessTableEventState;

    fn event(&mut self, key: Key) -> Result<ProcessTableEventState> {
        let focus = self.views.active_view().view_focus().clone();
        
        match (key, focus) {
            /* Events that operate on views */
            
            (Key::Alts, _) => {
                self.views
                    .create_new_view_from_active();
            }

            (Key::Altd, _) => {
                self.views
                    .remove_active_view();
            }

            (Key::Alth, _) => {
                self.views
                    .mut_views_orientation()
                    .set_to_split_horizontal();
            }

            (Key::Altv, _) => {
                self.views
                    .mut_views_orientation()
                    .set_to_split_vertical();
            }

            (Key::AltLeft, _) => {
                self.views
                    .dec_selection();
            }

            (Key::AltRight, _) => {
                self.views
                    .inc_selection();
            }

            /* Events that operate on columns */

            (Key::Left, ProcessTableViewFocus::Columns) => {
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_columns()
                    .dec_selection();
            }

            (Key::Right, ProcessTableViewFocus::Columns) => {
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_columns()
                    .inc_selection();
            }

            (Key::Delete, ProcessTableViewFocus::Columns) => {
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_columns()
                    .remove_column();
            }

            (Key::Char('p'), ProcessTableViewFocus::Columns) => {
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_columns()
                    .insert_column(
                        ColumnConfig::new(
                            Column::Pid,
                        )
                    );
            }

            (Key::Char('c'), ProcessTableViewFocus::Columns) => {
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_columns()
                    .insert_column(
                        ColumnConfig::new(
                            Column::CpuAverage,
                        )
                    );
            }

            (Key::Char('C'), ProcessTableViewFocus::Columns) => {
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_columns()
                    .insert_column(
                        ColumnConfig::new(
                            Column::MeanCpuUsageOverLastMinute,
                        )
                    );
            }

            (Key::Char('t'), ProcessTableViewFocus::Columns) => {
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_columns()
                    .insert_column(
                        ColumnConfig::new(
                            Column::CpuTotal,
                        )
                    );
            }

            // Can maybe make stats delta t rotate
            (Key::Char('T'), ProcessTableViewFocus::Columns) => {
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_columns()
                    .insert_column(
                        ColumnConfig::new(
                            Column::MeanCpuUsageAsTotalOverLastMinute,
                        )
                    );
            }

            (Key::Char('m'), ProcessTableViewFocus::Columns) => {
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_columns()
                    .insert_column(
                        ColumnConfig::new(
                            Column::Memory(MemoryUnitOptions::B),
                        )
                    );
            }

            (Key::Char('n'), ProcessTableViewFocus::Columns) => {
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_columns()
                    .insert_column(
                        ColumnConfig::new(
                            Column::Name,
                        )
                    );
            }

            (Key::Char('u'), ProcessTableViewFocus::Columns) => {
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_columns()
                    .rotate_unit();
            }

            (Key::Tab, ProcessTableViewFocus::Columns) => {
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_columns()
                    .deselect();

                self.views
                    .mut_active_view()
                    .mut_view_focus()
                    .set_to_rows();
            }

            /* Events that operate on rows */

            (Key::Char('/'), ProcessTableViewFocus::Rows) =>
                self.views
                .mut_active_view()
                .mut_view_focus()
                .set_to_filter(),

            (Key::Tab, ProcessTableViewFocus::Rows) => {
                self.views
                    .mut_active_view()
                    .mut_view_focus()
                    .set_to_columns();
                
                // Init selection       
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_columns()
                    .inc_selection();
            }

            (Key::Char('p'), ProcessTableViewFocus::Rows) => {
                if matches!(
                    self.views.active_view().table_state().row_sort(),
                    RowSort::PidDec) {
                    self.views
                        .mut_active_view()
                        .mut_table_state()
                        .row_sort_by(RowSort::PidInc);
                } else {
                    self.views
                        .mut_active_view()
                        .mut_table_state()
                        .row_sort_by(RowSort::PidDec);
                } 
            }

            (Key::Char('c'), ProcessTableViewFocus::Rows) => {
                if matches!(
                    self.views.active_view().table_state().row_sort(),
                    RowSort::CpuDec) {
                    self.views
                        .mut_active_view()
                        .mut_table_state()
                        .row_sort_by(RowSort::CpuInc);
                } else {
                    self.views
                        .mut_active_view()
                        .mut_table_state()
                        .row_sort_by(RowSort::CpuDec);
                }
            }

            (Key::Char('m'), ProcessTableViewFocus::Rows) => {
                if matches!(
                    self.views.active_view().table_state().row_sort(),
                    RowSort::MemDec) {
                    self.views
                        .mut_active_view()
                        .mut_table_state()
                        .row_sort_by(RowSort::MemInc);
                } else {
                    self.views
                        .mut_active_view()
                        .mut_table_state()
                        .row_sort_by(RowSort::MemDec);
                }
            }
            
            (Key::Char('n'), ProcessTableViewFocus::Rows) => {
                if matches!(
                    self.views.active_view().table_state().row_sort(),
                    RowSort::NameDec) {
                    self.views
                        .mut_active_view()
                        .mut_table_state()
                        .row_sort_by(RowSort::NameInc);
                } else {
                    self.views
                        .mut_active_view()
                        .mut_table_state()
                        .row_sort_by(RowSort::NameDec);
                }
            }

            /* Events that change the active view's visual row selection */
            (Key::Up, ProcessTableViewFocus::Rows) => {
                let visible_rows_upper_bound = self.table
                    .count_visible_rows(
                        self.views.active_view().table_state().row_sort(),
                        self.views.active_view().table_state().filter_ast()
                    );

                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_row_selection()
                    .dec_selection(visible_rows_upper_bound);
            }

            (Key::Down, ProcessTableViewFocus::Rows) => {
                let visible_rows_upper_bound = self.table
                    .count_visible_rows(
                        self.views.active_view().table_state().row_sort(),
                        self.views.active_view().table_state().filter_ast()
                    );
                
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_row_selection()
                    .inc_selection(visible_rows_upper_bound);
            }

            (Key::Char('T'), ProcessTableViewFocus::Rows) => {
                let table_state = self.views.active_view().table_state();
                
                if let Some(sel) = table_state.row_selection().selection() &&
                    let Some(row) = self.table.visible_row(
                        table_state.row_sort(),
                        table_state.filter_ast(),
                        sel) 
                {
                    return Ok(ProcessTableEventState::TerminatePid(row.process().pid().as_u32()))
                }
            }

            /* Events that operate on filter */

            (Key::Char(c), ProcessTableViewFocus::Filter) => {
                // Update filer_string & ast
                match self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_filter_string()
                    .insert_ascii_ch(c) {
                        Err(e) => self.views
                            .mut_active_view()
                            .set_filter_err_msg(&e.to_string()),
                        
                        Ok(_) => self.views
                            .mut_active_view()
                            .set_filter_err_msg_to_none(),
                    }

                match self.views
                    .mut_active_view()
                    .mut_table_state()
                    .update_filter_ast() {
                        Err(e) => self.views
                            .mut_active_view()
                            .set_filter_err_msg(&e.to_string()),
                        
                        Ok(_) => self.views
                            .mut_active_view()
                            .set_filter_err_msg_to_none(),
                    }
        
                // Calculate new row selection upper bound for active view
                let visible_rows_upper_bound = self.table
                    .count_visible_rows(
                        self.views.active_view().table_state().row_sort(),
                        self.views.active_view().table_state().filter_ast()
                    );

                // Update the active view's row selection
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_row_selection()
                    .update_selection(visible_rows_upper_bound);
            }

            // Remove char from filter
            (Key::Backspace, ProcessTableViewFocus::Filter) => {
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_filter_string()
                    .remove_ch();
                
                match self.views
                    .mut_active_view()
                    .mut_table_state()
                    .update_filter_ast() {
                        Err(e) => self.views
                            .mut_active_view()
                            .set_filter_err_msg(&e.to_string()),
                        
                        Ok(_) => self.views
                            .mut_active_view()
                            .set_filter_err_msg_to_none(),
                    }

                // Calculate new row selection upper bound for active view
                let visible_rows_upper_bound = self.table
                    .count_visible_rows(
                        self.views.active_view().table_state().row_sort(),
                        self.views.active_view().table_state().filter_ast()
                    );

                // Update the active view's row selection
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_row_selection()
                    .update_selection(visible_rows_upper_bound);
            }

            (Key::Right, ProcessTableViewFocus::Filter) => {
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_filter_string()
                    .inc_cursor()
            }

            (Key::Left, ProcessTableViewFocus::Filter) => {
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_filter_string()
                    .dec_cursor()
            }

            (Key::Enter, ProcessTableViewFocus::Filter) => {
                self.views
                    .mut_active_view()
                    .mut_view_focus()
                    .set_to_rows()
            }
            
            _ => return Ok(ProcessTableEventState::NotConsumed)
        }
        
        Ok(ProcessTableEventState::Consumed)
    }
}


