// Make the column identifier include unit options
// E.g., Mem in Bytes, Kilobytes, Megabytes, Gigabytes

// TODO [3/16/26]
pub enum MemUnitOptions {
    B,
    KB,
    MB,
    GB
}

/// ColumnID: Identifier
#[derive(PartialEq)]
pub enum ColumnID {
    PID,
    Name,
    CPU,
    Mem
}

impl ColumnID {
    /// ColumnID::as_str() -> &'static str: Identifier String representation 
    pub fn as_str(&self) -> &'static str {
        match self {
            ColumnID::PID  => "pid",
            ColumnID::Name => "name",
            ColumnID::CPU  => "cpu",
            ColumnID::Mem  => "mem"
        }
    }
}

/// Column
pub struct Column {
    pub id:  ColumnID,
}

impl Column {
    pub fn header(&self) -> &'static str {
        self.id.as_str()
    }
}

impl From<ColumnID> for Column {
    fn from(id: ColumnID) -> Self {
        // resource: https://docs.rs/ratatui/latest/ratatui/layout/enum.Constraint.html
        Self { id }
    }
}

/// Direction: Signals which way to move selection
pub enum Direction {
    Left,
    Right
}

/// ColumnsEvent: Events that can act on `Columns`
//#[derive(Clone)]
pub enum ColumnsEvent {
    MoveSelection(Direction),
    InsertColumn(Column),
    RemoveColumn
}

pub struct Columns {
    columns:   Vec<Column>,
    capacity:  usize,
    selection: Option<usize>
}

impl Columns {
    pub const DEFAULT_CAPACITY: usize = 10;

    pub fn iter(&self) -> impl Iterator<Item = (&Column, bool)> {
        self.columns.iter().enumerate().map(|(idx, col)| (col, Some(idx) == self.selection))
    }

    pub fn count(&self) -> usize {
        self.columns.len()
    }

    pub fn get_selection(&self) -> Option<usize> {
        self.selection
    }

    // TODO: Can probably rename this to `event`
    pub fn cols_event(&mut self, event: ColumnsEvent) {
        match event {
            ColumnsEvent::MoveSelection(Direction::Left) => {
                self.move_selection(Direction::Left);
            }
            ColumnsEvent::MoveSelection(Direction::Right) => {
                self.move_selection(Direction::Right);
            }
            ColumnsEvent::InsertColumn(col) => {
                let _ = self.insert_invariant(col);
            }
            ColumnsEvent::RemoveColumn => {
                let _ = self.remove_invariant();
            }
        }
    }

    // Same selection logic from Rows
    fn move_selection(&mut self, dir: Direction) {
        if let Some(selection) = self.selection {
            match dir {
                Direction::Left  => { self.selection = Some(selection.saturating_sub(1)); }
                Direction::Right => { self.selection = Some(selection.saturating_add(1)); }
            }
            self.enforce_invariant_on_selection();
        }
    }

    // Same selection logic from Rows
    fn enforce_invariant_on_selection(&mut self) {
        let col_count: usize = self.columns.len();

        self.selection = if col_count == 0 {
            None
        } else {
            match self.selection {
                Some(row) if row < col_count => Some(row),
                Some(_) => Some(col_count - 1),
                None => Some(0)
            }
        };
    }

    // Same insertion logic from TextLineModel
    fn insert_invariant(&mut self, col: Column) -> bool {
        if let Some(selection) = self.selection {
            if self.columns.len() < self.capacity {
                self.columns.insert(selection, col);
                self.selection = Some(selection + 1);
                return true
            } else {
                return false
            }
        }
        false
    }

    // Same remove logic from TextLineModel
    fn remove_invariant(&mut self) -> bool {
        if let Some(selection) = self.selection {
            // There must be at least one column
            if self.columns.len() <= 1 {
                return false
            } else {
                self.columns.remove(selection);
                self.selection = Some(selection.saturating_sub(1)); // Saturate sub b/c no guard on selection value being 0
                return true
            }
        }
        false
    }
}

/// Constructor
impl From<Vec<Column>> for Columns {
    fn from(columns: Vec<Column>) -> Self {
        let selection = if columns.is_empty() {
            None
        } else {
            Some(0)
        };

        Self {
            columns,
            capacity: Self::DEFAULT_CAPACITY,
            selection
        }
    }
}

/// Constructor
impl Default for Columns {
    fn default() -> Self {
        Self {
            columns:
                vec![Column::from(ColumnID::PID),
                Column::from(ColumnID::Name), 
                Column::from(ColumnID::CPU), 
                Column::from(ColumnID::Mem)],
            capacity:  Self::DEFAULT_CAPACITY,
            selection: Some(0)
        }
    }
}

