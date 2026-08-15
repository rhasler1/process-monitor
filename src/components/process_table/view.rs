use process_table::ProcessTableState;

use crate::components::utils::scroll::Scroll;

use std::slice::Iter;

#[derive(Debug, Default, Clone)]
pub struct ProcessTableVisualRowSelection {
    selection: Option<usize>
}

impl ProcessTableVisualRowSelection {
    /// Selection invariant for rows in a ProcessTable.
    ///
    /// # Behavior
    /// - If `upper_bound` is 0, `selection` is None.
    /// - If `upper_bound` is > 0 and `selection` is
    ///   Some(_) < `upper_bound`, then `selection`
    ///   is unchanged.
    /// - If `upper_bound` is > 0, and `selection` is
    ///   >= `upper_bound`, then `selection` is set to
    ///   > `upper_bound - 1`.
    /// - If `upper_bound` is > 0, and `selection` is 
    ///   None, then `selection is set to `Some(0)`.
    fn selection_invariant(&mut self, upper_bound: usize) {
        self.selection = if upper_bound == 0 {
            None
        } else {
            match self.selection {
                Some(visual_idx)
                    if visual_idx < upper_bound => Some(visual_idx),
                Some(_) => Some(upper_bound - 1),
                None => Some(0)
            }
        }
    }

    /// Updates the selection by applying the
    /// invariant.
    pub fn update(&mut self, upper_bound: usize) {
        self.selection_invariant(upper_bound);
    }

    /// Advances the selection by 1.
    ///
    /// Selection is clamped by argued upper_bound.
    pub fn inc_selection(&mut self, upper_bound: usize) {
        if let Some(visual_selection) = self.selection {
            self.selection = Some(visual_selection + 1);
        }

        self.selection_invariant(upper_bound);
    }

    /// Moves the selection back by 1.
    pub fn dec_selection(&mut self, upper_bound: usize) {
        if let Some(visual_selection) = self.selection {
            self.selection = 
                Some(visual_selection.saturating_sub(1));
        }

        self.selection_invariant(upper_bound);
    }

    // Getter
    pub fn selection(&self) -> Option<usize> {
        self.selection
    }
}

#[derive(Debug, Default, Clone)]
pub enum ProcessTableViewFocus {
    #[default]
    Table,
    Filter
}

impl ProcessTableViewFocus {
    pub fn set_to_table(&mut self) {
        *self = Self::Table;
    }

    pub fn set_to_filter(&mut self) {
        *self = Self::Filter;
    }
}

#[derive(Debug, Default, Clone)]
pub struct ProcessTableView {
    table_state:    ProcessTableState,
    row_selection:  ProcessTableVisualRowSelection,
    row_scroll:     Scroll,
    /// Table | Filter
    focus:          ProcessTableViewFocus
}

impl ProcessTableView {
    pub fn new_from_existing(&self) -> Self {
        Self {
            table_state: self.table_state.clone(),
            row_selection: self.row_selection.clone(),
            row_scroll: self.row_scroll.clone(),
            focus: self.focus.clone()
        }
    }

    pub fn table_state(&self) -> &ProcessTableState {
        &self.table_state
    }

    pub fn mut_table_state(&mut self) -> &mut ProcessTableState {
        &mut self.table_state
    }

    pub fn visual_row_selection(&self) -> &ProcessTableVisualRowSelection {
        &self.row_selection
    }

    pub fn mut_visual_row_selection(&mut self) -> &mut ProcessTableVisualRowSelection {
        &mut self.row_selection
    }

    pub fn row_scroll(&self) -> &Scroll {
        &self.row_scroll
    }

    pub fn mut_row_scroll(&mut self) -> &mut Scroll {
        &mut self.row_scroll
    }

    pub fn view_focus(&self) -> &ProcessTableViewFocus {
        &self.focus
    }

    pub fn mut_view_focus(&mut self) -> &mut ProcessTableViewFocus {
        &mut self.focus
    }
}

pub struct ProcessTableViews {
    views:          Vec<ProcessTableView>,
    /// Index into views
    view_selection: usize,
}

impl Default for ProcessTableViews {
    fn default() -> Self {
        Self {
            views:          vec![ProcessTableView::default()],
            view_selection: 0,
        }
    }
}

impl ProcessTableViews {
    pub fn create_new_view_from_existing(&mut self, table_view: &ProcessTableView) {
        self.views.push(ProcessTableView::new_from_existing(table_view));
    }

    pub fn remove_selected_view(&mut self) {
        if self.views.len() == 1 {
            // Invariant: At all times views must be non-empty.
            return
        }

        self.views.remove(self.view_selection);

        self.view_selection = if self.view_selection == 0 {
            0
        } else {
            self.view_selection - 1
        };
    }

    // Len can never be 1.
    pub fn inc_selection(&mut self) {
        let len = self.views.len();
        let sel = self.view_selection;

        self.view_selection = if sel + 1 < len { sel + 1 }
        else { 0 };
    }

    // Len can never be 1
    pub fn dec_selection(&mut self) {
        let len = self.views.len();
        let sel = self.view_selection;

        self.view_selection = if sel > 0 { sel - 1 }
        else { len - 1 };
    }

    // Invariant: selection is always valid, unwrap is safe
    pub fn active_view(&self) -> &ProcessTableView {
        self.views.get(self.view_selection).unwrap()
    }

    pub fn mut_active_view(&mut self) -> &mut ProcessTableView {
        self.views.get_mut(self.view_selection).unwrap()
    }

    
    // Iter
    pub fn views(&self) -> impl Iterator<Item = &ProcessTableView> {
        self.views.iter()
    }

    pub fn mut_views(&mut self) -> impl Iterator<Item = &mut ProcessTableView> {
        self.views.iter_mut()
    }
}

