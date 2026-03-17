// ProcessTableModel
pub struct ProcessTableModel {
    // TableHeader
    // Rows
}

pub struct Row {
    // From ProcessItem
}
impl Row {
    // Equality by PID
}
pub struct Rows {
    // Vec<Row>
    // selection: Option<usize>
    // order:     RowOrder
    // (filter:    Option<String>) This will come from App as a &Filter, do not store as field
}
impl Rows {
    //
}
pub enum Header {
    // PID, Name, etc.
}
pub struct TableHeader {
    // Vec<Header>
    // selection: Option<usize>
}


pub enum ColumnHeader {
    PID,
    Name,
    CPU,
    Mem
}

pub struct TableHeader {
    pub col_headers:   Vec<ColumnHeader>,
    pub col_selection: Option<usize>,
    pub col_capacity:  usize
}

impl Default for TableHeader {
    fn default() -> Self {
        let col_headers: Vec<ColumnHeader> = vec![
            ColumnHeader::PID,
            ColumnHeader::Name,
            ColumnHeader::CPU,
            ColumnHeader::Mem
        ];

        Self {
            col_headers,
            col_selection: None,
            col_capacity:  Self::TABLE_HEADER_CAPACITY
        }
    }
}

impl TableHeader {
    pub const TABLE_HEADER_CAPACITY: usize = 10;

    pub fn insert(&mut self, col_header: ColumnHeader) -> bool {
        if self.col_headers.len() >= self.col_capacity {
            return false
        }

        if let Some(selection) = self.col_selection {
            self.col_headers.insert(selection, col_header);
            return true
        }

        // Cannot insert when selection is None
        return false
    }

    pub fn remove(&mut self) -> bool {
        // There must be at least 2 columns to remove a single column
        if self.col_headers.len() <= 1 {
            return false
        }

        if let Some(selection) = self.col_selection {
            let _ = self.col_headers.remove(selection);
            self.col_selection = Some(selection.saturating_sub(1));
            return true
        }

        // Cannot remove when selection is None
        return false
    }

    pub fn count(&self) -> usize {
        self.col_headers.len()
    }
}

pub enum RowOrder {
    PidDec,
    PidInc,
    NameDec,
    NameInc,
    CpuDec,
    CpuInc,
    MemDec,
    MemInc
}

impl Default for RowOrder {
    fn default() -> Self {
        RowOrder::CpuDec
    }
}

pub enum MoveRowDirection {
    Down,
    Up
}

#[derive(Default)]
pub struct TableRowsState {
    order:     RowOrder,
    selection: Option<usize>,
    filter:    Option<String>
}

impl TableRowsState {
    pub fn new(rows: &Vec<ProcessItem>) -> Self {
        let order = RowOrder::default();
        let selection = if rows.len() > 0 {
            Some(0)
        } else {
            None
        };
        let filter = None;
        Self { order, selection, filter }
    }

    pub fn model_update(mut self, rows: &TableRows) {
        self.enforce_selection_invariant(&rows);
    }

    fn enforce_selection_invariant(&mut self, rows: &TableRows) {
        let row_count: usize = rows.count();
        self.selection = if row_count == 0 {
            None
        } else {
            match self.selection {
                Some(row) if row < row_count => Some(row),
                Some(_) => Some(row_count - 1),
                None => Some(0)
            }
        };
    }

    pub fn move_row_selection(&mut self, dir: MoveRowDirection, rows: &TableRows) {
        if let Some(selection) = self.selection {
            match dir {
                MoveRowDirection::Up   => { self.selection = Some(selection.saturating_sub(1)); }
                MoveRowDirection::Down => { self.selection = Some(selection.saturating_add(1)); }
            }
            self.enforce_selection_invariant(&rows);
        } 
    }

}

use crate::domain::process::primitive::ProcessItem;
use crate::domain::process::model::ProcessSnapShot;
#[derive(Default)]
pub struct TableRows {
    rows:  Vec<ProcessItem>,
    state: TableRowsState
}

impl From<&ProcessSnapShot> for TableRows {
    // TODO [3/4/26] Consider mapping to a TableRow structure
    fn from(snapshot: &ProcessSnapShot) -> Self {
        let rows: Vec<ProcessItem> = snapshot.iter().map(|item| item.clone()).collect();
        let state = TableRowsState::new(&rows);
        Self { rows, state }
    }
}

impl TableRows {
    pub fn count(&self) -> usize {
        self.rows.len()
    }
}

pub struct ProcessTableModel {
    table_header: TableHeader,
    table_rows:   TableRows
}
