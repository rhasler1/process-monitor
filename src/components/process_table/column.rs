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

#[derive(PartialEq)]
pub enum ColumnID {
    PID,
    Name,
    CPU(CPUUnitOptions),
    Mem(MemUnitOptions)
}

impl ColumnID {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColumnID::PID       => "pid",
            ColumnID::Name      => "name",
            ColumnID::CPU(unit) => unit.as_str(),
            ColumnID::Mem(unit) => unit.as_str()
        }
    }
}

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

pub enum Direction {
    Left,
    Right
}

pub enum ColumnEvent {
    MoveSelection(Direction),
    InsertColumn(Column),
    RemoveColumn,
    RotateUnit
}

pub struct Columns {
    columns:   Vec<Column>,
    // capacity: max column count
    capacity:  usize,
    selection: Option<usize>
}

impl Columns {
    pub const DEFAULT_CAPACITY: usize = 10;

    pub fn event(&mut self, event: ColumnEvent) {
        match event {
            ColumnEvent::MoveSelection(Direction::Left) => {
                self.move_selection(Direction::Left);
            }
            ColumnEvent::MoveSelection(Direction::Right) => {
                self.move_selection(Direction::Right);
            }
            ColumnEvent::InsertColumn(col) => {
                self.insert_col(col);
            }
            ColumnEvent::RemoveColumn => {
                self.remove_col();
            }
            ColumnEvent::RotateUnit => {
                self.rotate_unit();
            }
        }
    }

    fn apply_selection_invariant(&mut self) {
        let col_count = self.columns.len();

        self.selection = if col_count == 0 {
            None
        } else {
            match self.selection {
                Some(selection) if selection < col_count => Some(selection),
                Some(_) => Some(col_count - 1),
                None => Some(0)
            }
        };
    }

    fn move_selection(&mut self, dir: Direction) {
        if let Some(selection) = self.selection {
            match dir {
                Direction::Left  => { self.selection = Some(selection.saturating_sub(1)); }
                Direction::Right => { self.selection = Some(selection.saturating_add(1)); }
            }
            self.apply_selection_invariant();
        } 
    }
 
    fn insert_col(&mut self, col: Column) {
        let col_count = self.columns.len();
        
        if col_count < self.capacity {
            let insert_pos = 
                if let Some(selection) = self.selection {
                    selection
                } else {
                    0
                };
            self.columns.insert(insert_pos, col);
            self.apply_selection_invariant();
        }
    }

    fn remove_col(&mut self) {
        if let Some(selection) = self.selection {
            self.columns.remove(selection);
            self.apply_selection_invariant();
        }
    }

    fn rotate_unit(&mut self) {
        if let Some(selection) = self.selection {
            if let Some(col) = self.columns.get(selection) {
                match &col.id {
                    ColumnID::Mem(old_unit) => {
                        let new_unit = match old_unit {
                            MemUnitOptions::B  => MemUnitOptions::KB,
                            MemUnitOptions::KB => MemUnitOptions::MB,
                            MemUnitOptions::MB => MemUnitOptions::GB,
                            MemUnitOptions::GB => MemUnitOptions::B
                        };
                        self.columns.remove(selection);
                        self.columns.insert(selection, Column::from(ColumnID::Mem(new_unit)));
                    }
                    ColumnID::CPU(old_unit) => {
                        let new_unit = match old_unit {
                            CPUUnitOptions::Avg => CPUUnitOptions::Tot,
                            CPUUnitOptions::Tot => CPUUnitOptions::Avg
                        };
                        self.columns.remove(selection);
                        self.columns.insert(selection, Column::from(ColumnID::CPU(new_unit)));
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn get_count(&self) -> usize {
        self.columns.len()
    }

    /*pub fn get_selection(&self) -> Option<usize> {
        self.selection
    }*/

    pub fn iter(&self) -> impl Iterator<Item = (&Column, bool)> {
        self.columns.iter().enumerate().map(|(idx, col)| (col, Some(idx) == self.selection))
    }
}

impl From<Vec<Column>> for Columns {
    fn from(columns: Vec<Column>) -> Self {
        let selection = if columns.len() == 0 {
            None
        } else {
            Some(0)
        };

        Self {
            columns,
            capacity:  Self::DEFAULT_CAPACITY,
            selection
        }
    }
}

impl Default for Columns {
    fn default() -> Self {
        let columns = vec![
            Column::from(ColumnID::PID),
            Column::from(ColumnID::Name), 
            Column::from(ColumnID::CPU(CPUUnitOptions::Avg)), 
            Column::from(ColumnID::Mem(MemUnitOptions::B))];
        
        let selection = if columns.len() == 0 {
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

#[cfg(test)]
pub mod test {
    use super::{ColumnID, Column, ColumnEvent, Columns, CPUUnitOptions, Direction};

    #[test]
    fn test_event() {
        let mut columns: Columns = Columns::default();
        assert!(columns.selection == Some(0));
        assert!(columns.get_count() == columns.columns.len());
        assert!(columns.get_count() == 4);

        // BVA RemoveColumn
        for _ in 0..(columns.get_count() + 1) {
            columns.event(ColumnEvent::RemoveColumn);
        }
        assert!(columns.selection == None);
        assert!(columns.get_count() == 0);

        // BVA for InsertColumn
        for _ in 0..(columns.capacity + 1) {
            columns.event(ColumnEvent::InsertColumn(Column::from(ColumnID::CPU(CPUUnitOptions::Avg))));
        }
        assert!(columns.selection == Some(0));
        assert!(columns.get_count() == columns.capacity);

        // BVA MoveSelection
        for _ in 0..(columns.get_count() + 1) {
            columns.move_selection(Direction::Right);
        }
        assert!(columns.selection == Some(columns.get_count() - 1));
        for _ in 0..(columns.get_count() + 1) {
            columns.move_selection(Direction::Left);
        }
        assert!(columns.selection == Some(0));
    }
}