// Following imports are used for [name](ProcessRow::name)
use std::ffi::{OsString, OsStr};
use std::borrow::Cow;

/// Creates a fixed column schema.
pub struct ProcessRow {
    pid:       u32,
    name:      OsString,
    cpu_usage: f32,
    mem_usage: u64
}

impl ProcessRow {
    /// Number of columns in ProcessRow schema
    pub const COL_COUNT: usize = 4;
    /// Default column headers
    pub const COL_HEADERS: [&str; Self::COL_COUNT] = ["pid","name","cpu","mem"];
    
    /// Creates a ProcessRow with [pid] [name] [cpu_usage] [memory_usage]
    pub fn new(
        pid:       u32,
        name:      OsString,
        cpu_usage: f32,
        mem_usage: u64) -> Self {
        Self {
            pid,
            name,
            cpu_usage,
            mem_usage
        }
    }

    /// Returns [pid](ProcessRow::pid) to the `ProcessRow` it is called on
    pub fn pid(&self) -> u32 {
        self.pid
    }
    
    /// Returns [name](ProcessRow::name) to the `ProcessRow` it is called on
    pub fn name(&self) -> &OsStr {
        self.name.as_os_str()
    }

    /// Returns [name](ProcessRow::name) as a Cow<str> to the `ProcessRow` it is called on;
    ///     Cow<str> is guaranteed to be valid utf-8
    pub fn name_to_string_lossy(&self) -> Cow<str> {
        self.name.to_string_lossy()
    }

    /// Returns [cpu_usage](ProcessRow::cpu_usage) as f32 to the `ProcessRow` it is called on
    pub fn cpu_usage(&self) -> f32 {
        self.cpu_usage
    }

    /// Returns [mem_usage](ProcessRow::mem_usage) as u64 to the `ProcessRow` it is called on
    pub fn mem_usage(&self) -> u64 {
        self.mem_usage
    }
}

impl PartialEq for ProcessRow {
    /// Compares ProcessRow by [pid](ProcessRow::pid)
    fn eq(&self, other: &Self) -> bool {
        self.pid.eq(&other.pid)
    }
}
// TODO 2/15/2026 [16:37]
/// System processes are represened as a table here
pub struct ProcessTable {
    table: Vec<ProcessRow>
}

impl ProcessTable {
    /// Creates a ProcessTable with the provided ProcessRows
    pub fn new(rows: Vec<ProcessRow>) -> Self {
        Self {
            table: rows
        }
    }

    /// Returns the number of rows in the table
    pub fn count_rows(&self) -> usize {
        self.table.len()
    }

    /// Returns the number of cols in the table
    pub fn count_cols(&self) -> usize {
        ProcessRow::COL_COUNT
    }

    pub fn iter(&self) -> std::slice::Iter<'_, ProcessRow> {
        self.table.iter()
    }
}
