mod view;
pub use view::{ProcessTableViewFocus, ProcessTableView, ProcessTableViews};

use process_table::{
    ProcessTable, ProcessTableState, ProcessEntry as Row
};

use crate::components::Event;
use crate::events::EventState;
use crate::adapters::crossterm::input::Key;
use crate::domain::process::model::ProcessSnapShot;


pub struct ProcessTableComponent {
    table: ProcessTable,
    views: ProcessTableViews,
}

impl From<&ProcessSnapShot> for Vec<Row> {
    fn from(snapshot: &ProcessSnapShot) -> Vec<Row> {
        snapshot.iter().map(|s| {
            Row::new(
                s.pid(),
                s.total_cpu_usage(),
                s.mem_usage(),
                s.name().to_string_lossy().to_string())
        }).collect()
    }
}

impl ProcessTableComponent {
    pub fn new(snapshot: &ProcessSnapShot) -> Self {
        Self {
            table: ProcessTable::new(snapshot.into()),
            views: ProcessTableViews::default()
        }
    }

    pub fn new_snapshot(&mut self, snapshot: &ProcessSnapShot) {
        // Update table rows
        self.table.update_rows(snapshot.into());

        // Update all views selections
        for view in self.views.mut_views() {
            // Calculate new row selection upper bound for each view
            let visible_rows_upper_bound = self.table.count_visible_rows(
                view.table_state().row_sort(),
                view.table_state().filter_ast()
            );

            // Update views row selection
            view.mut_visual_row_selection().update(visible_rows_upper_bound);
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
            (Key::Char('/'), ProcessTableViewFocus::Table) =>
                self.views.mut_active_view().mut_view_focus().set_to_filter(),

            /* Events that change active view*/
            // Go to next active view
            (Key::PageUp, ProcessTableViewFocus::Table) =>
                self.views.inc_selection(),
            // Go to prev active view
            (Key::PageDown, ProcessTableViewFocus::Table) =>
                self.views.dec_selection(),
            

            /* Events that cahnge the active view's view state columns */
            // Go to next column
            (Key::Tab, ProcessTableViewFocus::Table) =>
                self.views.mut_active_view().mut_table_state().columns_inc_selection(),
            // TODO: KeyModifier: Ctrl + Char for column events

            
            /* Events that change the active view's table state sort*/
            // Sort by pid decreasing
            (Key::Char('p'), ProcessTableViewFocus::Table) =>
                self.views.mut_active_view().mut_table_state().row_sort_by_pid_dec(),
            // Sort by pid increasing
            (Key::Char('P'), ProcessTableViewFocus::Table) =>
                self.views.mut_active_view().mut_table_state().row_sort_by_pid_inc(),

            // Sort by cpu decreasing
            (Key::Char('c'), ProcessTableViewFocus::Table) =>
                self.views.mut_active_view().mut_table_state().row_sort_by_cpu_dec(),
            // Sort by cpu increasing
            (Key::Char('C'), ProcessTableViewFocus::Table) =>
                self.views.mut_active_view().mut_table_state().row_sort_by_cpu_inc(),
            
            // Sort by mem decreasing
            (Key::Char('m'), ProcessTableViewFocus::Table) =>
                self.views.mut_active_view().mut_table_state().row_sort_by_mem_dec(),
            // Sort by mem increasing
            (Key::Char('M'), ProcessTableViewFocus::Table) =>
                self.views.mut_active_view().mut_table_state().row_sort_by_mem_inc(),
            
            // Sort by name decreasing
            (Key::Char('n'), ProcessTableViewFocus::Table) =>
                self.views.mut_active_view().mut_table_state().row_sort_by_name_dec(),
            // Sort by name increasing
            (Key::Char('N'), ProcessTableViewFocus::Table) =>
                self.views.mut_active_view().mut_table_state().row_sort_by_name_inc(),

            /* Events that change the active view's visual row selection */
            (Key::Up, ProcessTableViewFocus::Table) => {
                let visible_rows_upper_bound = self.table.count_visible_rows(
                    self.views.active_view().table_state().row_sort(),
                    self.views.active_view().table_state().filter_ast()
                );
                self.views.mut_active_view().mut_visual_row_selection().dec_selection(visible_rows_upper_bound);
            },

            (Key::Down, ProcessTableViewFocus::Table) => {
                let visible_rows_upper_bound = self.table.count_visible_rows(
                    self.views.active_view().table_state().row_sort(),
                    self.views.active_view().table_state().filter_ast()
                );
                self.views.mut_active_view().mut_visual_row_selection().inc_selection(visible_rows_upper_bound);
            }

            // TODO: Terminate

            /* Events that change the active view's view state filter */
            // Insert char into filter
            (Key::Char(c), ProcessTableViewFocus::Filter) => {
                // Update filer_string & ast
                self.views.mut_active_view().mut_table_state().filter_string_insert_ch(c);
                self.views.mut_active_view().mut_table_state().update_filter_ast();
        
                // Calculate new row selection upper bound for active view
                let visible_rows_upper_bound = self.table.count_visible_rows(
                    self.views.active_view().table_state().row_sort(),
                    self.views.active_view().table_state().filter_ast()
                );

                // Update the active view's row selection
                self.views.mut_active_view().mut_visual_row_selection().update(visible_rows_upper_bound);
            },
            // Remove char from filter
            (Key::Backspace, ProcessTableViewFocus::Filter) => {
                self.views.mut_active_view().mut_table_state().filter_string_remove_ch();
                self.views.mut_active_view().mut_table_state().update_filter_ast();

                // Calculate new row selection upper bound for active view
                let visible_rows_upper_bound = self.table.count_visible_rows(
                    self.views.active_view().table_state().row_sort(),
                    self.views.active_view().table_state().filter_ast()
                );

                // Update the active view's row selection
                self.views.mut_active_view().mut_visual_row_selection().update(visible_rows_upper_bound);
            },
            // Move cursor forward
            (Key::Right, ProcessTableViewFocus::Filter) =>
                self.views.mut_active_view().mut_table_state().filter_string_inc_cursor(),
            // Move cursor backwards
            (Key::Left, ProcessTableViewFocus::Filter) =>
                self.views.mut_active_view().mut_table_state().filter_string_dec_cursor(),
            // Move active view's focus to Table
            (Key::Enter, ProcessTableViewFocus::Filter) =>
                self.views.mut_active_view().mut_view_focus().set_to_table(),
            
            _ => return EventState::NotConsumed
        }
        
        EventState::Consumed
    }
}





