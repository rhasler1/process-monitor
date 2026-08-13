use std::ops::Div;

use super::{Column, Sort};

/// Stores process PID
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessPid(u32);

impl ProcessPid {
    pub fn new(pid: u32) -> Self {
        Self(pid)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn as_u64(&self) -> u64 {
        self.0 as u64
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

/// Stores process CPU usage as total
#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct ProcessCpu(f32);

impl ProcessCpu {
    pub fn new(cpu: f32) -> Self {
        Self(cpu)
    }

    pub fn total(&self) -> f32 {
        self.as_f32()
    }

    pub fn average(&self, core_count: usize) -> f32 {
        self.as_f32().div(core_count as f32)
    }

    pub fn as_f32(&self) -> f32 {
        self.0
    }

    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
}

/// Stores process memory usage as bytes
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessMemory(u64);

impl ProcessMemory {
    pub fn new(memory: u64) -> Self {
        Self(memory)
    }

    pub fn as_bytes(&self) -> u64 {
        self.as_u64()
    }

    pub fn as_kb(&self) -> u64 {
        self.as_u64() / 1024
    }

    pub fn as_mb(&self) -> u64 {
        self.as_u64() / 1048576
    }

    pub fn as_gb(&self) -> u64 {
        self.as_u64() / 1073741824
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

/// Stores process name as String
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessName(String);

impl ProcessName {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct ProcessEntry {
    pid:    ProcessPid,
    cpu:    ProcessCpu,
    mem:    ProcessMemory,
    name:   ProcessName
}

pub enum Cell<'a> {
    Pid(&'a ProcessPid),
    Cpu(&'a ProcessCpu),
    Memory(&'a ProcessMemory),
    Name(&'a ProcessName)
}

impl ProcessEntry {
    pub fn new(
        pid: u32,
        cpu: f32,
        mem: u64,
        name: String) -> Self {
        Self {
            pid:  ProcessPid(pid),
            cpu:  ProcessCpu(cpu),
            mem:  ProcessMemory(mem),
            name: ProcessName(name)
        }
    }

    // Currently, not being used
    pub fn cell(&self, column: Column) -> Cell<'_> {
        match column {
            Column::Pid => Cell::Pid(&self.pid),
            Column::Cpu(_) => Cell::Cpu(&self.cpu),
            Column::Memory(_) => Cell::Memory(&self.mem),
            Column::Name => Cell::Name(&self.name),
        }
    }

    pub fn cmp(&self, other: &Self, sort: &Sort) -> std::cmp::Ordering {
        match sort {
            Sort::PidDec    => other.pid.cmp(&self.pid),
            Sort::PidInc    => self.pid.cmp(&other.pid),
            Sort::CpuDec    => other.cpu.partial_cmp(&self.cpu).unwrap_or(std::cmp::Ordering::Equal),
            Sort::CpuInc    => self.cpu.partial_cmp(&other.cpu).unwrap_or(std::cmp::Ordering::Equal),
            Sort::MemDec    => other.mem.cmp(&self.mem),
            Sort::MemInc    => self.mem.cmp(&other.mem),
            Sort::NameDec   => other.name.cmp(&self.name),
            Sort::NameInc   => self.name.cmp(&other.name),
        }
    }

    // Getters

    pub fn pid(&self) -> &ProcessPid {
        &self.pid
    }

    pub fn cpu(&self) -> &ProcessCpu {
        &self.cpu
    }

    pub fn mem(&self) -> &ProcessMemory {
        &self.mem
    }

    pub fn name(&self) -> &ProcessName {
        &self.name
    }
}

#[cfg(test)]
mod test {
    use crate::Sort;
    use super::{ProcessEntry as Row};

    #[test]
    fn test_row_cmp() {
        let row1 = Row::new(1, 3.0, 5, "a".to_string());
        let row2 = Row::new(2, 4.0, 6, "b".to_string());

        assert_eq!(
            row1.cmp(&row2, &Sort::PidDec),
            std::cmp::Ordering::Greater
        );

        assert_eq!(
            row1.cmp(&row2, &Sort::PidInc),
            std::cmp::Ordering::Less
        );
        
        assert_eq!(
            row1.cmp(&row2, &Sort::CpuDec),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            row1.cmp(&row2, &Sort::CpuInc),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            row1.cmp(&row2, &Sort::MemDec),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            row1.cmp(&row2, &Sort::MemInc),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            row1.cmp(&row2, &Sort::NameDec),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            row1.cmp(&row2, &Sort::NameInc),
            std::cmp::Ordering::Less
        );
    }
}

