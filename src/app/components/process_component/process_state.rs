use crate::app::components::process_component::process_model::ProcessTable;

/// An enumerator describing tabular move directions
pub enum MoveDirection {
    Down,
    Up,
    Left,
    Right
}

/// Encapsulates process table state
pub struct ProcessTableState {
    // None is reserved for an empty table; if table is non-empty there must be a selection
    select_row: Option<usize>
}

impl ProcessTableState {
    /// Creates a ProcessTableState with the provided ProcessTable
    pub fn new(table: &ProcessTable) -> Self {
        let select_row = if table.count_rows() > 0 {
            Some(0)
        } else {
            None
        };
        Self {
            select_row
        }
    }

    pub fn move_selection(&mut self, dir: MoveDirection, max_idx: usize) {
        if self.select_row.is_none() {
            return
        }
        match dir {
            MoveDirection::Down  => {
                self.select_row = Some(move_selection_down(self.select_row.unwrap(), max_idx));
            }
            MoveDirection::Up    => {
                self.select_row = Some(move_selection_up(self.select_row.unwrap()));
            }
            MoveDirection::Left  => {}
            MoveDirection::Right => {}
        }
    }

    /// Enforces a valid [select_row](Option<usize>) selection invariant
    /// Rules:
    ///     (1) Empty table -> no selection
    ///     (2) If selection out of bounds -> clamp to last row
    ///     (3) If no selection but row exists -> select first row
    pub fn enforce_select_row_invariant(&mut self, table: &ProcessTable) {
        let row_count: usize = table.count_rows();
        self.select_row = if row_count == 0 {
            None
        } else {
            match self.select_row {
                Some(row) if row < row_count => Some(row),
                Some(_) => Some(row_count - 1),
                None => Some(0)
            }
        };
    }

    /*TODO
    pub fn view(&self, model: &ProcessDataModel) -> Vec<usize> {
        let v = model.len();
    }
    */
}

const fn move_selection_down(selection_idx: usize, max_idx: usize) -> usize {
    let mut new_selection_idx = selection_idx;
    if selection_idx < max_idx {
        new_selection_idx = selection_idx + 1;
    }
    new_selection_idx
}

const fn move_selection_up(selection_idx: usize) -> usize {
    selection_idx.saturating_sub(1)
}
