use super::Error;

use serde::{Serialize, Deserialize};

/// Supported memory formats.
/// 
/// Does not include runtime architecture.
#[derive(Debug, Default, Clone,
    PartialEq, Serialize, Deserialize)]
pub enum MemoryUnitOptions {
    #[default]
    B,
    KB,
    MB,
    GB
}

impl MemoryUnitOptions {
    pub fn as_str(&self) -> &str {
        match self {
            Self::B =>  "Mem B",
            Self::KB => "Mem KB",
            Self::MB => "Mem MB",
            Self::GB => "Mem GB"
        }
    }
}

/// Supported column types.
///
/// Does not include runtime architecture.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ColumnOptions {
    // Can be derived from `Process`
    Pid,
    CpuTotal,
    CpuAverage,
    Memory(MemoryUnitOptions),
    Name,

    // Can be derived from `ProcessStats`
    MeanCpuUsageOverLastMinute,
    MeanCpuUsageAsTotalOverLastMinute,
}

impl ColumnOptions{
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pid => "Pid",
            Self::CpuTotal => "CpuT%",
            Self::CpuAverage => "CpuA%",
            Self::Memory(unit) => unit.as_str(),
            Self::MeanCpuUsageAsTotalOverLastMinute => "CpuT%/60s",
            Self::MeanCpuUsageOverLastMinute => "CpuA%/60s",
            Self::Name => "Name"
        }
    }
}

/// Configuration structure that decouples
/// persistence from runtime architecture.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColumnsConfig {
    columns: Vec<ColumnOptions>
}

impl From<&Columns> for ColumnsConfig {
    fn from(columns: &Columns) -> Self {
        Self {
            columns: columns.columns.clone()
        }
    }
}

/// Manages selection, insertion, and
/// deletion into a vector of columns.
///
/// Couples persistent and runtime architecture.
#[derive(Debug, Clone)]
pub struct Columns {
    columns:    Vec<ColumnOptions>,
    selection:  Option<usize>,
    capacity:   usize
}

impl TryFrom<&ColumnsConfig> for Columns {
    type Error = Error;
    
    fn try_from(config: &ColumnsConfig) -> Result<Self, Error> {
        Ok(Self {
            columns:    config.columns.clone(),
            selection:  None,
            capacity:   Self::DEFAULT_MAX_CAPACITY
        })
    }
}

impl Default for Columns {
    /// Default implementation
    fn default() -> Self {
        Self {
            columns: vec![
                ColumnOptions::Pid,
                ColumnOptions::CpuAverage,
                ColumnOptions::MeanCpuUsageOverLastMinute,
                ColumnOptions::Memory(MemoryUnitOptions::B),
                ColumnOptions::Name,
            ],
            selection:  None,
            capacity:   Self::DEFAULT_MAX_CAPACITY
        }
    }
}

impl Columns {
    const DEFAULT_MAX_CAPACITY: usize = 10;
    
    /// Creates an empty Columns structure.
    pub fn new_empty() -> Self {
        Self {
            columns:        Vec::with_capacity(Self::DEFAULT_MAX_CAPACITY),
            selection:      None,
            capacity:       Self::DEFAULT_MAX_CAPACITY,
        }
    }

    // Selection mutators

    /// Advances the selection to the next column.
    ///
    /// Selection wraps around when the end of `col_configs` is reached.
    ///
    /// # Behavior
    /// - If `col_configs` is empty, `selection` is set to `None`.
    /// - If `selection` is `None` and `col_configs` is non-empty,
    ///   `selection` is set to `Some(0)`.
    /// - If `selection` points to a column other than the last,
    ///   it is advanced by one.
    /// - If `selection` points to the last column, it wraps around to Some(0).
    ///
    /// # Invariant
    /// After this method returns, `selection` is either `None` when `col_configs`
    /// is empty, or a valid index into `col_configs`.
    pub fn inc_selection(&mut self) {
        let len = self.count_columns();

        if len == 0 {
            self.selection = None;
            return
        }

        self.selection = Some(match self.selection {
            Some(i) if i + 1 < len => i + 1,
            Some(_) | None => 0,
        });
    }

    /// Moves the selection to the previous column.
    ///
    /// Selection wraps when the beginning of `col_configs` is reached.
    ///
    /// # Behavior
    ///
    /// - If `col_configs` is empty, `selection` is set to `None`.
    /// - If `selection` is `None` and `col_configs` is non-empty,
    ///   `selection` is set to `Some(col_configs.len() - 1)`.
    /// - If `selection` points to a column other than the first,
    ///   it is moved back by one.
    /// - If `selection` points to the first column,
    ///   it wraps around to Some(col_configs.len()-1).
    pub fn dec_selection(&mut self) {
        let len = self.count_columns();

        if len == 0 {
            self.selection = None;
            return
        }

        self.selection = Some(match self.selection {
            Some(i) if i > 0 => i - 1,
            _ => len - 1
        });
    }

    /// Sets the selection to None.
    pub fn deselect(&mut self) {
        self.selection = None
    }

    // Selection getters

    /// Gets the selection.
    pub fn selection(&self) -> Option<usize> {
        self.selection
    }

    // Column_configs mutators

    /// Removes the selected column.
    ///
    /// # Behavior
    /// - If `selection` is None, method does not
    ///   remove a column and returns.
    /// - If `col_configs` is empty after removal,
    ///   `selection` is set to None.
    /// - If `selection` is out of bounds after removal,
    ///   it is moved back by one.
    pub fn remove_column(&mut self) {
        let Some(selection) = self.selection else {
            return
        };

        self.columns.remove(selection);

        if self.columns.is_empty() {
            self.selection = None
        } else if selection >= self.columns.len() {
            self.selection = Some(self.columns.len() - 1)
        }
    }

    /// Inserts the argued column_config.
    ///
    /// # Behavior
    /// - If selection is None, column is inserted at Some(0)
    ///   and selection is set to Some(0).
    /// - If selection is Some(_), column is inserted at
    ///   selection.
    pub fn insert_column(
        &mut self,
        col: ColumnOptions
        ) {
        if self.columns.len() < self.capacity {
            match self.selection {
                Some(selection) => {
                    self.columns.insert(selection, col);
                }
                None => {
                    self.columns.insert(0, col);
                    self.selection = Some(0)
                }
            }
        }
    }

    /// Rotates the selected Columns units.
    pub fn rotate_unit(&mut self) {
        let Some(selection) = self.selection else {
            return
        };

        // Safe to unwrap here
        let column = self.columns.get_mut(selection).unwrap();

        let rotated = match column {
            ColumnOptions::Memory(MemoryUnitOptions::B)    => ColumnOptions::Memory(MemoryUnitOptions::KB),
            ColumnOptions::Memory(MemoryUnitOptions::KB)   => ColumnOptions::Memory(MemoryUnitOptions::MB),
            ColumnOptions::Memory(MemoryUnitOptions::MB)   => ColumnOptions::Memory(MemoryUnitOptions::GB),
            ColumnOptions::Memory(MemoryUnitOptions::GB)   => ColumnOptions::Memory(MemoryUnitOptions::B),

            _ => return
        };

        *column = rotated;

        //config.set_column(rotated);
    }

    // Column_configs getters

    /// Gets reference to the selected column_config.
    pub fn get_column(&self) -> Option<&ColumnOptions> {
        if let Some(selection) = self.selection {
            self.columns.get(selection)
        } else {
            None
        }
    }

    // Helpers

    /// Wrapper over `self.col_configs.len()`
    fn count_columns(&self) -> usize {
        self.columns.len()
    }

    
    // Iterators
    
    pub fn columns(&self) -> impl Iterator<Item = &ColumnOptions> {
        self.columns.iter()
    }
}

#[cfg(test)]
mod test {
use super::*;

    #[test]
    fn test_remove_column() {
        // Using default constructor.
        let mut columns = Columns::default(); 
        
        assert_eq!(columns.count_columns(), 5);
        
        assert!(columns.selection().is_none());

        // Column not removed when selection is None & Vec is not empty.
        columns.remove_column();
        
        assert_eq!(columns.count_columns(), 5);

        // Set selection to Some(0)
        columns.selection = Some(0);

        // Column is removed when selection is Some & vec is not empty.
        columns.remove_column();
        
        assert_eq!(columns.count_columns(), 4);
        
        // Selection is not moved.
        assert_eq!(columns.selection(), Some(0));

        // BVA
        for _ in 0..=columns.count_columns() {
            columns.remove_column();
        }

        // All columns have been removed.
        assert_eq!(columns.count_columns(), 0);

        // Selection is set to None.
        assert!(columns.selection().is_none())
    }

    #[test]
    fn test_insert_column() {
        // Using empty columns
        let mut columns = Columns::new_empty();

        assert_eq!(columns.count_columns(), 0);

        assert!(columns.selection.is_none());

        // Insert into empty columns
        columns.insert_column(ColumnOptions::Pid);

        // Columns count is 1
        assert_eq!(columns.count_columns(), 1);

        // Selection moved to Some(0)
        assert_eq!(columns.selection(), Some(0));

        // Insert into non empty columns
        columns.insert_column(ColumnOptions::Pid);
        
        // Coulmns count is 2
        assert_eq!(columns.count_columns(), 2);

        // Selection is still Some(0)
        assert_eq!(columns.selection(), Some(0));
    }

    #[test]
    fn test_inc_selection_empty() {
        // Using empty columns
        let mut columns = Columns::new_empty();

        assert_eq!(columns.count_columns(), 0);

        assert!(columns.selection.is_none());
    
        columns.inc_selection();

        // Selection is not moved
        assert!(columns.selection.is_none());
    }

    #[test]
    fn test_inc_selection_nonempty() {
        let mut columns = Columns::default();

        assert_eq!(columns.count_columns(), 5);

        assert!(columns.selection.is_none());

        columns.inc_selection();

        // Selection is set to Some(0)
        assert_eq!(columns.selection(), Some(0));

        // BVA
        for _ in 0..=columns.count_columns() {
            columns.inc_selection();
        }

        // Selection wraps to Some(1)
        assert_eq!(columns.selection(), Some(1));
    }

    #[test]
    fn test_dec_selection_empty() {
        // Using empty columns
        let mut columns = Columns::new_empty();

        assert_eq!(columns.count_columns(), 0);

        assert!(columns.selection.is_none());
    
        columns.dec_selection();

        // Selection is not moved
        assert!(columns.selection.is_none());
    }

    #[test]
    fn test_dec_selection_nonempty() {
        let mut columns = Columns::default();

        assert_eq!(columns.count_columns(), 5);

        assert!(columns.selection.is_none());

        columns.dec_selection();

        // Selection is set to Some(len-1)
        assert_eq!(columns.selection(), Some(columns.count_columns() - 1));

        // BVA
        for _ in 0..=columns.count_columns() {
            columns.dec_selection();
        }

        // Selection wraps to Some(3)
        assert_eq!(columns.selection(), Some(3));
    }
}
