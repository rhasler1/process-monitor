use log::{debug, error};
use serde::{Deserialize, Serialize};
use crate::{config::config::{Config, write_config}, events::EventState};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
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

#[derive(PartialEq, Debug, Serialize, Deserialize)]
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

#[derive(PartialEq, Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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
    RotateUnit,
    SaveColumnConfig
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Columns {
    columns:   Vec<Column>,
    #[serde(skip)]
    capacity: usize,
    #[serde(skip)]
    selection: Option<usize>
}

impl Columns {
    pub const DEFAULT_CAPACITY: usize = 10;

    pub fn event(&mut self, event: ColumnEvent) -> EventState {
        /*// TODO REMOVE
        let selection = self.selection;
        let capacity = self.capacity;
        debug!("`Columns` selection = {selection:?}\n capacity = {capacity:?}\n");*/

        match event {
            ColumnEvent::MoveSelection(Direction::Left) => {
                self.move_selection(Direction::Left);
                EventState::Consumed
            }
            ColumnEvent::MoveSelection(Direction::Right) => {
                self.move_selection(Direction::Right);
                EventState::Consumed
            }
            ColumnEvent::InsertColumn(col) => {
                self.insert_col(col);
                EventState::Consumed
            }
            ColumnEvent::RemoveColumn => {
                self.remove_col();
                EventState::Consumed
            }
            ColumnEvent::RotateUnit => {
                self.rotate_unit();
                EventState::Consumed
            }
            ColumnEvent::SaveColumnConfig => {
                // TODO propagate config string to main and do IO on separate thread
                let config: String = self.serialize_columns();
                match write_config(config) {
                    Ok(_) => {}
                    Err(e) => { error!("`Columns:` event: error: {e} when writing column config") }
                }
                EventState::Consumed
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

    pub fn iter(&self) -> impl Iterator<Item = (&Column, bool)> {
        self.columns.iter().enumerate().map(|(idx, col)| (col, Some(idx) == self.selection))
    }

    pub fn serialize_columns(&self) -> String {
        toml::to_string(&self).unwrap_or_default()
    }
}

impl From<&Config> for Columns {
    fn from(config: &Config) -> Self {
        let mut columns = 
        if let Some(col_config) = config.get_columns_config() {
            toml::from_str(col_config).unwrap_or_default()
        } else {
            Columns::default()
        };
        
        columns.capacity  = Self::DEFAULT_CAPACITY;
        
        columns.selection = if columns.columns.len() == 0 {
            None
        } else {
            Some(0)
        };

        columns
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
