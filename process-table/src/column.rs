/// Supported cpu formats.
#[derive(Debug, Default, Clone, PartialEq)]
pub enum CpuUnitOptions {
    #[default]
    Average,
    Total
}

/// Supported memory formats.
#[derive(Debug, Default, Clone, PartialEq)]
pub enum MemoryUnitOptions {
    #[default]
    B,
    KB,
    MB,
    GB
}

/// Supported column types.
#[derive(Debug, Clone, PartialEq)]
pub enum Column {
    Pid,
    Cpu(CpuUnitOptions),
    Memory(MemoryUnitOptions),
    Name
}

/// ColumnConfig associates a Column with it's width.
///
/// Width's relation to #Terminal cells is 1:1
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnConfig {
    column: Column,
    width: usize
}

impl ColumnConfig {
    pub fn new(column: Column, width: usize) -> Self {
        Self {
            column,
            width
        }
    }

    pub fn column(&self) -> &Column {
        &self.column
    }

    pub fn set_column(&mut self, new_col: Column) {
        self.column = new_col;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn set_width(&mut self, new_width: usize) {
        self.width = new_width;
    }
}

/// Manages column configurations and
/// the currently selected column.
pub struct Columns {
    col_configs: Vec<ColumnConfig>,
    selection: Option<usize>
}

impl Default for Columns {
    /// Default implementation
    fn default() -> Self {
        Self {
            col_configs: vec![
                ColumnConfig {
                    column: Column::Pid,
                    width: 10 
                },
                ColumnConfig {
                    column: Column::Cpu(CpuUnitOptions::Total),
                    width: 10
                },
                    ColumnConfig {
                    column: Column::Memory(MemoryUnitOptions::B),
                    width: 10
                },
                ColumnConfig {
                    column: Column::Name,
                    width: 10
                }
            ],
            selection: None
        }
    }
}

impl Columns {
    /// Creates an empty Columns structure.
    pub fn new_empty() -> Self {
        Self {
            col_configs: vec![],
            selection: None
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

        self.col_configs.remove(selection);

        if self.col_configs.is_empty() {
            self.selection = None
        } else if selection >= self.col_configs.len() {
            self.selection = Some(self.col_configs.len() - 1)
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
        col_config: ColumnConfig
        ) {
        match self.selection {
            Some(selection) => {
                self.col_configs.insert(selection, col_config);
            }
            None => {
                self.col_configs.insert(0, col_config);
                self.selection = Some(0)
            }
        }
    }

    /// Rotates the selected Columns units.
    pub fn rotate_unit(&mut self) {
        let Some(selection) = self.selection else {
            return
        };

        // Safe to unwrap here
        let config = self.col_configs.get_mut(selection).unwrap();

        let rotated = match config.column() {
            Column::Cpu(CpuUnitOptions::Total)      => Column::Cpu(CpuUnitOptions::Average),
            Column::Cpu(CpuUnitOptions::Average)    => Column::Cpu(CpuUnitOptions::Total),

            Column::Memory(MemoryUnitOptions::B)    => Column::Memory(MemoryUnitOptions::KB),
            Column::Memory(MemoryUnitOptions::KB)   => Column::Memory(MemoryUnitOptions::MB),
            Column::Memory(MemoryUnitOptions::MB)   => Column::Memory(MemoryUnitOptions::GB),
            Column::Memory(MemoryUnitOptions::GB)   => Column::Memory(MemoryUnitOptions::B),

            _ => return
        };

        config.set_column(rotated);
    }

    // Column_configs getters

    /// Gets reference to the selected column_config.
    pub fn get_column_config(&self) -> Option<&ColumnConfig> {
        if let Some(selection) = self.selection {
            self.col_configs.get(selection)
        } else {
            None
        }
    }

    // Helpers

    /// Wrapper over `self.col_configs.len()`
    fn count_columns(&self) -> usize {
        self.col_configs.len()
    }
}

#[cfg(test)]
mod test {
use super::*;

    #[test]
    fn test_remove_column() {
        // Using default constructor.
        let mut columns = Columns::default(); 
        
        assert_eq!(columns.count_columns(), 4);
        
        assert!(columns.selection().is_none());

        // Column not removed when selection is None & Vec is not empty.
        columns.remove_column();
        
        assert_eq!(columns.count_columns(), 4);

        // Set selection to Some(0)
        columns.selection = Some(0);

        // Column is removed when selection is Some & vec is not empty.
        columns.remove_column();
        
        assert_eq!(columns.count_columns(), 3);
        
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
        columns.insert_column(ColumnConfig { column: Column::Pid, width: 10 });

        // Columns count is 1
        assert_eq!(columns.count_columns(), 1);

        // Selection moved to Some(0)
        assert_eq!(columns.selection(), Some(0));

        // Insert into non empty columns
        columns.insert_column(ColumnConfig { column: Column::Pid, width: 10 });
        
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

        assert_eq!(columns.count_columns(), 4);

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

        assert_eq!(columns.count_columns(), 4);

        assert!(columns.selection.is_none());

        columns.dec_selection();

        // Selection is set to Some(len-1)
        assert_eq!(columns.selection(), Some(columns.count_columns() - 1));

        // BVA
        for _ in 0..=columns.count_columns() {
            columns.dec_selection();
        }

        // Selection wraps to Some(2)
        assert_eq!(columns.selection(), Some(2));
    }
}
