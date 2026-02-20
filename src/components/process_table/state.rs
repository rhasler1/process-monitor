use crate::domain::process::model::ProcessSnapShot;
use crate::components::process_table::controller::ProcessTableAction;
use crate::events::EventState;

/// ProcessTable sort options
pub enum ProcessTableSort {
    PidDec,
    PidInc,
    NameDec,
    NameInc,
    CpuDec,
    CpuInc,
    MemDec,
    MemInc
}

impl Default for ProcessTableSort {
    fn default() -> Self {
        ProcessTableSort::CpuDec
    }
}

/// Move directions supported by ProcessTable
pub enum MoveDirection {
    Down,
    Up
}

/// Encapsulates process table state
#[derive(Default)]
pub struct ProcessTableState {
    select_row: Option<usize>,
}

impl ProcessTableState {
    /// Creates a ProcessTableState with the provided ProcessTable
    pub fn handle_model_update(&mut self, process_snapshot: &ProcessSnapShot) {
        self.enforce_select_row_invariant(&process_snapshot);
    }
    
    fn enforce_select_row_invariant(&mut self, process_snapshot: &ProcessSnapShot) {
        let row_count: usize = process_snapshot.count();
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

    /// Response to controller
    pub fn handle_action(&mut self,
        action: ProcessTableAction,
        process_snapshot: &ProcessSnapShot) {
        match action {
            ProcessTableAction::Move(dir) => {
                self.move_selection(dir, &process_snapshot);
            }
            ProcessTableAction::Sort(sort) => {}
        }
    }

    pub fn move_selection(&mut self, dir: MoveDirection, process_snapshot: &ProcessSnapShot) {
        if let Some(selection) = self.select_row {
            match dir {
                MoveDirection::Up   => { self.select_row = Some(selection.saturating_sub(1)); }
                MoveDirection::Down => { self.select_row = Some(selection.saturating_add(1)); }
            }
            self.enforce_select_row_invariant(&process_snapshot);
        } 
    }
}

/*const fn move_selection_down(selection_idx: usize, max_idx: usize) -> usize {
    let mut new_selection_idx = selection_idx;
    if selection_idx < max_idx {
        new_selection_idx = selection_idx + 1;
    }
    new_selection_idx
}

const fn move_selection_up(selection_idx: usize) -> usize {
    selection_idx.saturating_sub(1)
}
*/
