
use process_table::{
    ProcessTable, ProcessTableViewState, ProcessEntry as Row
};

use crate::components::Event;
use crate::events::EventState;
use crate::adapters::crossterm::input::Key;
use crate::config::app_config::Config;
use crate::domain::process::model::ProcessSnapShot;


#[derive(Debug, Default, Clone)]
pub enum Focus {
    #[default]
    Table,
    Filter
}

#[derive(Debug, Default)]
pub struct ProcessTableView {
    view_state: ProcessTableViewState,
    /// Table | Filter
    focus:      Focus
}

pub struct ProcessTableViews {
    views:      Vec<ProcessTableView>,
    /// Index into views
    selection:  usize,
}

impl Default for ProcessTableViews {
    fn default() -> Self {
        Self {
            views:      vec![ProcessTableView::default()],
            selection:  0,
        }
    }
}
// TODO: Document & Test
impl ProcessTableViews {
    pub fn create_new_view_from_existing(&mut self, view: &ProcessTableViewState) {
        let new_view = view.clone();

        self.views.push(ProcessTableView { view_state: new_view, focus: Focus::Table });
    }

    pub fn remove_selected_view(&mut self) {
        if self.views.len() == 1 {
            // Invariant: At all times views must be non-empty.
            return
        }

        self.views.remove(self.selection);

        self.selection = if self.selection == 0 {
            0
        } else {
            self.selection - 1
        };
    }

    // Len can never be 1.
    pub fn inc_selection(&mut self) {
        let len = self.views.len();
        let sel = self.selection;

        self.selection = if sel + 1 < len { sel + 1 }
        else { 0 };
    }

    // Len can never be 1
    pub fn dec_selection(&mut self) {
        let len = self.views.len();
        let sel = self.selection;

        self.selection = if sel > 0 { sel - 1 }
        else { len - 1 };
    }

    // Invariant: selection is always valid, unwrap is safe
    pub fn active_view(&self) -> &ProcessTableView {
        self.views.get(self.selection).unwrap()
    }

    pub fn mut_active_view(&mut self) -> &mut ProcessTableView {
        self.views.get_mut(self.selection).unwrap()
    }
}


pub struct TableComponent {
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

impl TableComponent {
    pub fn new(snapshot: &ProcessSnapShot, _config: &Config) -> Self {
        Self {
            table: ProcessTable::new(snapshot.into()),
            views: ProcessTableViews::default()
        }
    }

    pub fn new_snapshot(&mut self, snapshot: &ProcessSnapShot) {
        self.table.update_rows(snapshot.into());
    }

    pub fn table_and_views(&mut self) -> (&ProcessTable, &mut ProcessTableViews) {
        (&self.table, &mut self.views)
    }
}

// TODO: Figure out how to capture Ctrl+Tab w/ crossterm
impl Event for TableComponent {
    fn event(&mut self, key: Key) -> EventState {
        let focus = self.views.active_view().focus.clone();
        
        match (key, focus) {
            // Change active view's focus to Filter
            (Key::Char('/'), Focus::Table) => self.views.mut_active_view().focus = Focus::Filter,

            /* Events that change active view*/
            // Go to next active view
            (Key::PageUp, Focus::Table) => self.views.inc_selection(),
            // Go to prev active view
            (Key::PageDown, Focus::Table) => self.views.dec_selection(),
            

            /* Events that cahnge the active view's view state columns */
            // Go to next column
            (Key::Tab, Focus::Table) => self.views.mut_active_view().view_state.columns_inc_selection(),
            // TODO: KeyModifier: Ctrl + Char for column events

            
            /* Events that change the active view's view state sort*/
            // Sort by pid decreasing
            (Key::Char('p'), Focus::Table) => self.views.mut_active_view().view_state.row_sort_by_pid_dec(),
            // Sort by pid increasing
            (Key::Char('P'), Focus::Table) => self.views.mut_active_view().view_state.row_sort_by_pid_inc(),

            // Sort by cpu decreasing
            (Key::Char('c'), Focus::Table) => self.views.mut_active_view().view_state.row_sort_by_cpu_dec(),
            // Sort by cpu increasing
            (Key::Char('C'), Focus::Table) => self.views.mut_active_view().view_state.row_sort_by_cpu_inc(),
            
            // Sort by mem decreasing
            (Key::Char('m'), Focus::Table) => self.views.mut_active_view().view_state.row_sort_by_mem_dec(),
            // Sort by mem increasing
            (Key::Char('M'), Focus::Table) => self.views.mut_active_view().view_state.row_sort_by_mem_inc(),
            
            // Sort by name decreasing
            (Key::Char('n'), Focus::Table) => self.views.mut_active_view().view_state.row_sort_by_name_dec(),
            // Sort by name increasing
            (Key::Char('N'), Focus::Table) => self.views.mut_active_view().view_state.row_sort_by_name_inc(),

            // TODO: Terminate

            /* Events that change the active view's view state filter */
            // Insert char into filter
            (Key::Char(c), Focus::Filter) => {
                self.views.mut_active_view().view_state.filter_string_insert_ch(c);
                self.views.mut_active_view().view_state.update_filter_ast();
            },
            // Remove char from filter
            (Key::Backspace, Focus::Filter) => {
                self.views.mut_active_view().view_state.filter_string_remove_ch();
                self.views.mut_active_view().view_state.update_filter_ast();
            },
            // Move cursor forward
            (Key::Right, Focus::Filter) => self.views.mut_active_view().view_state.filter_string_inc_cursor(),
            // Move cursor backwards
            (Key::Left, Focus::Filter) => self.views.mut_active_view().view_state.filter_string_dec_cursor(),
            // Move active view's focus to Table
            (Key::Enter, Focus::Filter) => self.views.mut_active_view().focus = Focus::Table,
            
            _ => return EventState::NotConsumed
        }
        
        EventState::Consumed
    }
}





