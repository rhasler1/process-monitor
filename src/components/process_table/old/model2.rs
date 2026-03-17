// TODO [3/3/26] Build a TableModel
// UI component model
use crate::domain::process::model::ProcessSnapShot;

pub enum ProcessOrder {
    PidDec,
    PidInc,
    NameDec,
    NameInc,
    CpuDec,
    CpuInc,
    MemDec,
    MemInc
}

impl Default for ProcessOrder {
    fn default() -> Self {
        ProcessOrder::CpuDec
    }
}

pub enum Direction {
    Down,
    Up,
    Left,
    Right
}

pub enum Action {
    Sort(ProcessOrder),
    Move(Direction),
    Insert(ColumnHeader),
    Remove(ColumnHeader)
}

pub enum ColumnHeader {
    PID,
    Name,
    CPU,
    Mem
}

pub struct TableHeader {
    headers:   Vec<ColumnHeader>,
    selection: Option<usize>
}

impl Default for TableHeader {
    fn default() -> Self {
        let headers: Vec<ColumnHeader> = vec![
            ColumnHeader::PID,
            ColumnHeader::Name,
            ColumnHeader::CPU,
            ColumnHeader::Mem
        ];
        Self {
            headers
        }
    }
}

impl TableHeader {
    pub const TABLE_HEADER_CAPACITY: usize = 10;

    pub fn insert(&mut self, header: ColumnHeader) -> bool {
        if self.headers.len() < TABLE_HEADER_CAPACITY {
            self.headers.push(header);
            true
        }

        false
    }
}


#[derive(Default)]
pub struct ProcessTableModel {
    header:     TableHeader, // TODO [3/3/26]
    rows:       TableRows,
    //select_row: Option<usize>,
    //order:      ProcessOrder
}

impl ProcessTableModel {
    /// Creates a ProcessTableModel with the provided ProcessTable
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
                self.move_row_selection(dir, &process_snapshot);
            }
            ProcessTableAction::Sort(_sort) => {}
        }
    }

    pub fn move_row_selection(&mut self, dir: Direction, process_snapshot: &ProcessSnapShot) {
        if let Some(selection) = self.select_row {
            match dir {
                Direction::Up   => { self.select_row = Some(selection.saturating_sub(1)); }
                Direction::Down => { self.select_row = Some(selection.saturating_add(1)); }
            }
            self.enforce_select_row_invariant(&process_snapshot);
        } 
    }

    pub fn select_row(&self) -> Option<usize> {
        self.select_row
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
