use process_table::ProcessTableState;

use serde::{Deserialize, Serialize};

// TODO: 
// 1. Write validate_deserialize()
// 2. Write config component:
//      - That can: change current configuration (e.g., refresh rate, theme)
//

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum ProcessTableViewFocus {
    Columns,
    #[default]
    Rows,
    Filter
}

impl ProcessTableViewFocus {
    pub fn set_to_columns(&mut self) {
        *self = Self::Columns;
    }

    pub fn set_to_rows(&mut self) {
        *self = Self::Rows;
    }

    pub fn set_to_filter(&mut self) {
        *self = Self::Filter;
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProcessTableView {
    table_state:    ProcessTableState,
    /// Table | Filter
    filter_err_msg: Option<String>,
    focus:          ProcessTableViewFocus,
}

impl ProcessTableView {
    pub fn new_from_existing(&self) -> Self {
        Self {
            table_state:    self.table_state.clone(),
            filter_err_msg: self.filter_err_msg.clone(),
            focus:          self.focus.clone()
        }
    }

    pub fn table_state(&self) -> &ProcessTableState {
        &self.table_state
    }

    pub fn mut_table_state(&mut self) -> &mut ProcessTableState {
        &mut self.table_state
    }

    pub fn view_focus(&self) -> &ProcessTableViewFocus {
        &self.focus
    }

    pub fn mut_view_focus(&mut self) -> &mut ProcessTableViewFocus {
        &mut self.focus
    }

    pub fn set_filter_err_msg(&mut self, s: &str) {
        self.filter_err_msg = Some(s.to_string());
    }

    pub fn set_filter_err_msg_to_none(&mut self) {
        self.filter_err_msg = None;
    }

    pub fn filter_err_msg(&self) -> Option<&String> {
        self.filter_err_msg.as_ref()
    }
}

#[derive(Serialize, Deserialize)]
pub enum ViewsOrientation {
    SplitHorizontal,
    SplitVertical
}

impl ViewsOrientation {
    pub fn set_to_split_horizontal(&mut self) {
        *self = Self::SplitHorizontal;
    }

    pub fn set_to_split_vertical(&mut self) {
        *self = Self::SplitVertical;
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProcessTableViews {
    /// Collection of views; can never be empty
    views:              Vec<ProcessTableView>,
    /// Index into views; methods guarantee
    /// `views_selection` is always valid
    views_selection:    usize,
    /// Description of how views should
    /// be oriented on the screen
    views_orientation:  ViewsOrientation,
    /// Maximum number of ProcessTableView in views
    capacity: usize

}

impl Default for ProcessTableViews {
    fn default() -> Self {
        Self {
            views:              vec![ProcessTableView::default()],
            views_selection:    0,
            views_orientation:  ViewsOrientation::SplitVertical,
            capacity: Self::DEFAULT_CAPACITY
        }
    }
}

impl ProcessTableViews {
    const DEFAULT_CAPACITY: usize = 2;

    pub fn create_new_view_from_active(&mut self) {
        if self.views.len() == self.capacity {
            return
        }

        self.views.push(
            ProcessTableView::new_from_existing(self.active_view())
        );
    }

    pub fn remove_active_view(&mut self) {
        if self.views.len() == 1 {
            // Invariant: At all times views must be non-empty.
            return
        }

        self.views.remove(self.views_selection);

        self.views_selection = if self.views_selection == 0 {
            0
        } else {
            self.views_selection - 1
        };
    }

    // Len can never be 1.
    pub fn inc_selection(&mut self) {
        let len = self.views.len();
        let sel = self.views_selection;

        self.views_selection = if sel + 1 < len { sel + 1 }
        else { 0 };
    }

    // Len can never be 1
    pub fn dec_selection(&mut self) {
        let len = self.views.len();
        let sel = self.views_selection;

        self.views_selection = if sel > 0 { sel - 1 }
        else { len - 1 };
    }

    // Invariant: selection is always valid, unwrap is safe
    pub fn active_view(&self) -> &ProcessTableView {
        self.views.get(self.views_selection).unwrap()
    }

    pub fn mut_active_view(&mut self) -> &mut ProcessTableView {
        self.views.get_mut(self.views_selection).unwrap()
    }
    
    // Iter
    pub fn views(&self) -> impl Iterator<Item = &ProcessTableView> {
        self.views.iter()
    }

    pub fn mut_views(&mut self) -> impl Iterator<Item = &mut ProcessTableView> {
        self.views.iter_mut()
    }

    pub fn views_selection(&self) -> usize {
        self.views_selection
    }

    pub fn views_orientation(&self) -> &ViewsOrientation {
        &self.views_orientation
    }

    pub fn mut_views_orientation(&mut self) -> &mut ViewsOrientation {
        &mut self.views_orientation
    }

    pub fn count_views(&self) -> usize {
        self.views.len()
    }
}

