use super::ColumnError;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum CpuUnitOptions {
    #[default]
    Average,
    Total
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum MemoryUnitOptions {
    #[default]
    B,
    KB,
    MB,
    GB
}

#[derive(Debug, Clone, PartialEq)]
pub enum Column {
    Pid,
    Cpu(CpuUnitOptions),
    Memory(MemoryUnitOptions),
    Name
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColumnConfig {
    column: Column,
    width: usize
}

impl ColumnConfig {
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

// TODO: Have Columns maintain it's own selection | insert_position
pub struct Columns {
    col_configs: Vec<ColumnConfig>,
    selection: Option<usize>
}

impl Default for Columns {
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
            selection: Some(0)
        }
    }
}

impl Columns {
    fn count_columns(&self) -> usize {
        self.col_configs.len()
    }

    pub fn rotate_unit(&mut self) -> Result<(), ColumnError> {
        if self.selection.is_none() {
            return Ok(())
        }

        let selection = self.selection.unwrap();

        if selection >= self.count_columns() {
            Err(ColumnError::BadSelection(selection))
        } else {
            // Unwrap is safe here
            let new_col: Option<Column> = match self.col_configs.get(selection).unwrap().column() {
                Column::Cpu(unit) => {
                    match unit {
                        CpuUnitOptions::Total => {
                            Some(Column::Cpu(CpuUnitOptions::Average))
                        }
                        CpuUnitOptions::Average => {
                            Some(Column::Cpu(CpuUnitOptions::Total))
                        }
                    }
                }
                Column::Memory(unit) => {
                    match unit {
                        MemoryUnitOptions::B => {
                            Some(Column::Memory(MemoryUnitOptions::KB))
                        }
                        MemoryUnitOptions::KB => {
                            Some(Column::Memory(MemoryUnitOptions::MB))
                        }
                        MemoryUnitOptions::MB => {
                            Some(Column::Memory(MemoryUnitOptions::GB))
                        }
                        MemoryUnitOptions::GB => {
                            Some(Column::Memory(MemoryUnitOptions::B))
                        }
                    }
                }
                _ => { None }
            };

            if let Some(col) = new_col {
                self.col_configs.get_mut(selection).unwrap().set_column(col);
            }

            Ok(())
        }
    }

    // TODO: Leaving off here...
    //
    // This Structure will be made to maintain selection invariant internally,
    // no need to return Results
    //
    /// Column at remove_position is removed on Ok(()), caller should update selection.
    /// TODO: This will return an Err if attempting to remove a column when columns is empty;
    /// in this case return Ok(()) and do not remove.
    ///
    /// This can use similar logic to AsciiString
    pub fn remove_column(&mut self) {
        if self.count_columns() == 0 {
            // No column to remove
            return
        }

        if self.selection.is_none() {
            return
        }

        let selection = self.selection.unwrap();

        self.col_configs.remove(selection);

        if self.count_columns() == 0 {
            self.selection = None;
            return
        }

        self.selection = Some(selection.saturating_sub(1));
    }

    /// Column is inserted at insert_position on Ok(()), caller should update selection.
    pub fn insert_column(
        &mut self,
        insert_position: usize,
        col_config: ColumnConfig
        ) -> Result<(), ColumnError> {
        if insert_position > self.count_columns() {
            Err(ColumnError::BadSelection(insert_position))
        } else {
            self.col_configs.insert(insert_position, col_config);
            Ok(())
        }
    }

    pub fn get_column_config(&self, selection: usize) -> Option<&ColumnConfig> {
        self.col_configs.get(selection)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_column() {
        let mut columns = Columns::default();
        
        assert_eq!(columns.count_columns(), 4);
        columns.remove_column();
        let visual_selection = 0;
        assert_eq!(columns.count_columns(), 3);
        assert_eq!(
            *columns.get_column_config(visual_selection).unwrap(),
            ColumnConfig {
                column: Column::Cpu(CpuUnitOptions::Total),
                width: 10
        });

        columns.remove_column();
        assert_eq!(columns.count_columns(), 2);
        assert_eq!(
            *columns.get_column_config(visual_selection).unwrap(),
            ColumnConfig {
                column: Column::Memory(MemoryUnitOptions::B),
                width: 10
        });
    }

    #[test]
    fn test_insert_column() {
        let mut columns = Columns::default();
        let visual_selection = 0;
        let insert_position = visual_selection;

        assert_eq!(columns.count_columns(), 4);
        columns.insert_column(
            insert_position,
            ColumnConfig {
                column: Column::Name,
                width: 5
            }
        ).unwrap();
        assert_eq!(columns.count_columns(), 5);
        
        let visual_selection  = 0;
        assert_eq!(
            *columns.get_column_config(visual_selection).unwrap(),
            ColumnConfig {
                column: Column::Name,
                width: 5
            }
        );
    }
}
