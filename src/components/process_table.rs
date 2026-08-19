mod view;
use std::time::Duration;

pub use view::{ProcessTableViewFocus, ProcessTableView, ProcessTableViews, ViewsOrientation};

use process_table::{
    ProcessTable, ProcessTableState, Process, ProcessTableRow, RowSort
};

use crate::components::Event;
use crate::events::EventState;
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

        Ok(Self {
            table,
            views: ProcessTableViews::default()
        })
    }

    pub fn new_snapshot(&mut self, snapshot: &ProcessSnapShot) {
        // Update table rows
        self.table.update(snapshot.into());

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
    }

    pub fn table_and_views(&mut self) -> (&ProcessTable, &mut ProcessTableViews) {
        (&self.table, &mut self.views)
    }
}

// TODO: Figure out how to capture Ctrl+Tab w/ crossterm
impl Event for ProcessTableComponent {
    fn event(&mut self, key: Key) -> EventState {
        let focus = self.views.active_view().view_focus().clone();
        
        match (key, focus) {
            // Change active view's focus to Filter
            (Key::Char('/'), ProcessTableViewFocus::Rows) =>
                self.views.mut_active_view().mut_view_focus().set_to_filter(),

            // Change avtive view's focus to Columns
            (Key::Tab, ProcessTableViewFocus::Rows) => {
                self.views.mut_active_view().mut_view_focus().set_to_columns();
            }

            (Key::Tab, ProcessTableViewFocus::Columns) => {
                self.views.mut_active_view().mut_table_state().mut_columns().inc_selection();
            }

            (Key::Enter, ProcessTableViewFocus::Columns) => {
                self.views.mut_active_view().mut_view_focus().set_to_rows();
            }

            /* Events that change active view*/
            // Go to next active view
            (Key::PageUp, ProcessTableViewFocus::Rows) =>
                self.views.inc_selection(),
            // Go to prev active view
            (Key::PageDown, ProcessTableViewFocus::Rows) =>
                self.views.dec_selection(),
            
            /* Events that change the active view's table state sort*/
            // Sort by pid decreasing
            (Key::Char('p'), ProcessTableViewFocus::Rows) =>
                self.views.mut_active_view().mut_table_state().row_sort_by(RowSort::PidDec),
            // Sort by pid increasing
            (Key::Char('P'), ProcessTableViewFocus::Rows) =>
                self.views.mut_active_view().mut_table_state().row_sort_by(RowSort::PidInc),

            // Sort by cpu decreasing
            (Key::Char('c'), ProcessTableViewFocus::Rows) =>
                self.views.mut_active_view().mut_table_state().row_sort_by(RowSort::CpuDec),
            // Sort by cpu increasing
            (Key::Char('C'), ProcessTableViewFocus::Rows) =>
                self.views.mut_active_view().mut_table_state().row_sort_by(RowSort::CpuInc),
            
            // Sort by mem decreasing
            (Key::Char('m'), ProcessTableViewFocus::Rows) =>
                self.views.mut_active_view().mut_table_state().row_sort_by(RowSort::MemDec),
            // Sort by mem increasing
            (Key::Char('M'), ProcessTableViewFocus::Rows) =>
                self.views.mut_active_view().mut_table_state().row_sort_by(RowSort::MemInc),
            
            // Sort by name decreasing
            (Key::Char('n'), ProcessTableViewFocus::Rows) =>
                self.views.mut_active_view().mut_table_state().row_sort_by(RowSort::NameDec),
            // Sort by name increasing
            (Key::Char('N'), ProcessTableViewFocus::Rows) =>
                self.views.mut_active_view().mut_table_state().row_sort_by(RowSort::NameInc),

            /* Events that change the active view's visual row selection */
            (Key::Up, ProcessTableViewFocus::Rows) => {
                let visible_rows_upper_bound = self.table.count_visible_rows(
                    self.views.active_view().table_state().row_sort(),
                    self.views.active_view().table_state().filter_ast()
                );
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_row_selection()
                    .dec_selection(visible_rows_upper_bound);
            },

            (Key::Down, ProcessTableViewFocus::Rows) => {
                let visible_rows_upper_bound = self.table.count_visible_rows(
                    self.views.active_view().table_state().row_sort(),
                    self.views.active_view().table_state().filter_ast()
                );
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_row_selection()
                    .inc_selection(visible_rows_upper_bound);
            }

            (Key::Char('s'), ProcessTableViewFocus::Rows) => {
                // create new view
                self.views.create_new_view_from_active();
            }

            (Key::Delete, ProcessTableViewFocus::Rows) => {
                self.views.remove_active_view();
            }

            (Key::Char('h'), ProcessTableViewFocus::Rows) => {
                self.views.mut_views_orientation().set_to_split_horizontal();
            }

            (Key::Char('v'), ProcessTableViewFocus::Rows) => {
                self.views.mut_views_orientation().set_to_split_vertical();
            }

            (Key::Left, ProcessTableViewFocus::Rows) => {
                self.views.dec_selection();
            }

            (Key::Right, ProcessTableViewFocus::Rows) => {
                self.views.inc_selection();
            }

            // TODO: Terminate

            /* Events that change the active view's view state filter */
            // Insert char into filter
            (Key::Char(c), ProcessTableViewFocus::Filter) => {
                // Update filer_string & ast
                self.views.mut_active_view().mut_table_state().mut_filter_string().insert_ascii_ch(c);
                self.views.mut_active_view().mut_table_state().update_filter_ast();
        
                // Calculate new row selection upper bound for active view
                let visible_rows_upper_bound = self.table.count_visible_rows(
                    self.views.active_view().table_state().row_sort(),
                    self.views.active_view().table_state().filter_ast()
                );

                // Update the active view's row selection
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_row_selection()
                    .update_selection(visible_rows_upper_bound);
            },
            // Remove char from filter
            (Key::Backspace, ProcessTableViewFocus::Filter) => {
                self.views.mut_active_view().mut_table_state().mut_filter_string().remove_ch();
                self.views.mut_active_view().mut_table_state().update_filter_ast();

                // Calculate new row selection upper bound for active view
                let visible_rows_upper_bound = self.table.count_visible_rows(
                    self.views.active_view().table_state().row_sort(),
                    self.views.active_view().table_state().filter_ast()
                );

                // Update the active view's row selection
                self.views
                    .mut_active_view()
                    .mut_table_state()
                    .mut_row_selection()
                    .update_selection(visible_rows_upper_bound);
            },
            // Move cursor forward
            (Key::Right, ProcessTableViewFocus::Filter) =>
                self.views.mut_active_view().mut_table_state().mut_filter_string().inc_cursor(),
            // Move cursor backwards
            (Key::Left, ProcessTableViewFocus::Filter) =>
                self.views.mut_active_view().mut_table_state().mut_filter_string().dec_cursor(),
            // Move active view's focus to Table
            (Key::Enter, ProcessTableViewFocus::Filter) =>
                self.views.mut_active_view().mut_view_focus().set_to_rows(),
            
            _ => return EventState::NotConsumed
        }
        
        EventState::Consumed
    }
}


