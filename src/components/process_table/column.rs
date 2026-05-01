// TODO [3/30/26] It could be interesting to change both units and decimal places
#[derive(PartialEq)]
pub enum MemUnitOptions {
    B,
    KB,
    MB,
    GB
}

impl MemUnitOptions {
    pub fn as_str(&self) -> &'static str {
        match self {
            MemUnitOptions::B  => "mem (B)",
            MemUnitOptions::KB => "mem (KB)",
            MemUnitOptions::MB => "mem (MB)",
            MemUnitOptions::GB => "mem (GB)",
        }
    }
}

#[derive(PartialEq)]
pub enum CPUUnitOptions {
    Avg,
    Tot
}

impl CPUUnitOptions {
    pub fn as_str(&self) -> &'static str {
        match self {
            CPUUnitOptions::Avg => "cpu (avg)",
            CPUUnitOptions::Tot => "cpu (tot)"
        }
    }
}

/// ColumnID: Identifier
#[derive(PartialEq)]
pub enum ColumnID {
    PID,
    Name,
    CPU(CPUUnitOptions),
    Mem(MemUnitOptions)
}

impl ColumnID {
    /// ColumnID::as_str() -> &'static str: Identifier String representation 
    pub fn as_str(&self) -> &'static str {
        match self {
            ColumnID::PID       => "pid",
            ColumnID::Name      => "name",
            ColumnID::CPU(unit) => unit.as_str(),
            ColumnID::Mem(unit) => unit.as_str()
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
        Self { id }
    }
}

/// Direction: Signals which way to move selection
pub enum Direction {
    Left,
    Right
}

/// ColumnsEvent: Events that can act on `Columns`
pub enum ColumnsEvent {
    MoveSelection(Direction),
    InsertColumn(Column),
    RemoveColumn,
    RotateUnit
}

pub struct Columns {
    columns:   Vec<Column>,
    capacity:  usize,
    // There must always be at least 1 col, hence selection should not be optional
    selection: usize
}

impl Columns {
    pub const DEFAULT_CAPACITY: usize = 10;

    pub fn iter(&self) -> impl Iterator<Item = (&Column, bool)> {
        self.columns.iter().enumerate().map(|(idx, col)| (col, idx == self.selection))
    }

    pub fn count(&self) -> usize {
        self.columns.len()
    }

    pub fn get_selection(&self) -> usize {
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
            ColumnsEvent::RotateUnit => {
                self.rotate_unit();
            }
        }
    }

    fn rotate_unit(&mut self) -> bool {
        let col = self.columns.get(self.selection);
        if let Some(col) = col {
            match &col.id {
                ColumnID::Mem(old_unit) => {
                    let new_unit = match old_unit {
                        MemUnitOptions::B  => MemUnitOptions::KB,
                        MemUnitOptions::KB => MemUnitOptions::MB,
                        MemUnitOptions::MB => MemUnitOptions::GB,
                        MemUnitOptions::GB => MemUnitOptions::B
                    };
                    self.columns.remove(self.selection);
                    self.columns.insert(self.selection, Column::from(ColumnID::Mem(new_unit)));
                    return true
                }
                ColumnID::CPU(old_unit) => {
                    let new_unit = match old_unit {
                        CPUUnitOptions::Avg => CPUUnitOptions::Tot,
                        CPUUnitOptions::Tot => CPUUnitOptions::Avg
                    };
                    self.columns.remove(self.selection);
                    self.columns.insert(self.selection, Column::from(ColumnID::CPU(new_unit)));
                }
                _ => return false
            }
        }
        false
    }

    fn move_selection(&mut self, dir: Direction) {
        match dir {
            Direction::Left  => { self.selection = self.selection.saturating_sub(1); }
            Direction::Right => { self.selection = self.selection.saturating_add(1); }
        }
        self.enforce_invariant_on_selection();
    }

    fn enforce_invariant_on_selection(&mut self) {
        let col_count: usize = self.columns.len();
        let selection = self.selection;
        self.selection = if selection < col_count {
            selection
        } else {
            selection - 1
        };
    }

    fn insert_invariant(&mut self, col: Column) -> bool {
        if self.columns.len() < self.capacity {
            self.columns.insert(self.selection, col);
            self.selection = self.selection + 1;
            return true
        } else {
            return false
        }
    }

    fn remove_invariant(&mut self) -> bool {
        if self.columns.len() == 0 {
            panic!("Columns length invariant broken");
        } else if self.columns.len() == 1 {
            return false
        } else {
            self.columns.remove(self.selection);
            self.selection = self.selection.saturating_sub(1);
            return true
        }
    }
}

/// Constructor
impl From<Vec<Column>> for Columns {
    fn from(columns: Vec<Column>) -> Self {
        if columns.is_empty() {
            panic!("Columns length invariant broken");
        }

        Self {
            columns,
            capacity:  Self::DEFAULT_CAPACITY,
            selection: 0
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
                Column::from(ColumnID::CPU(CPUUnitOptions::Avg)), 
                Column::from(ColumnID::Mem(MemUnitOptions::B))],
            capacity:  Self::DEFAULT_CAPACITY,
            selection: 0
        }
    }
}
