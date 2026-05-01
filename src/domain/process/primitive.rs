/*
TODO [3/2/26]
- Disk usage
- Runtime
- Status
*/
// https://docs.rs/sysinfo/latest/sysinfo/struct.Process.html

// Following imports are used by [name](ProcessItem::name)
use std::ffi::{OsString, OsStr};
use std::borrow::Cow;

/// Represents a single system process
#[derive(Default, Clone)]
pub struct ProcessItem {
    /// [pid](ProcessItem::pid) is a unique process identifier
    pid:             u32,
    name:            OsString,
    avg_cpu_usage:   f32,
    total_cpu_usage: f32,
    mem_usage:       u64
}

impl ProcessItem {
    /// Creates a `ProcessItem` with [pid] [name] [cpu_usage] [memory_usage]
    pub fn new(
        pid:             u32,
        name:            OsString,
        avg_cpu_usage:   f32,
        total_cpu_usage: f32,
        mem_usage: u64) -> Self {
        Self {
            pid,
            name,
            avg_cpu_usage,
            total_cpu_usage,
            mem_usage
        }
    }

    /// Returns [pid](ProcessItem::pid) to the `ProcessItem` it is called on
    pub fn pid(&self) -> u32 {
        self.pid
    }
    
    /// Returns [name](ProcessItem::name) to the `ProcessItem` it is called on
    pub fn name(&self) -> &OsStr {
        self.name.as_os_str()
    }

    /// Returns [name](ProcessItem::name) as a Cow<str> to the `ProcessItem` it is called on;
    ///     Cow<str> is guaranteed to be valid utf-8
    pub fn name_to_string_lossy(&self) -> Cow<str> {
        self.name.to_string_lossy()
    }

    /// Returns [avg_cpu_usage](ProcessItem::cpu_usage) as f32 to the `ProcessItem` it is called on
    pub fn avg_cpu_usage(&self) -> f32 {
        self.avg_cpu_usage
    }

    /// Returns [total_cpu_usage](ProcessItem::cpu_usage) as f32 to the `ProcessItem` it is called on
    pub fn total_cpu_usage(&self) -> f32 {
        self.total_cpu_usage
    }

    /// Returns [mem_usage](ProcessItem::mem_usage) as u64 to the `ProcessItem` it is called on
    pub fn mem_usage(&self) -> u64 {
        self.mem_usage
    }
}

impl PartialEq for ProcessItem {
    /// Compares ProcessItem by [pid](ProcessItem::pid)
    fn eq(&self, other: &Self) -> bool {
        self.pid.eq(&other.pid)
    }
}

#[cfg(test)]
pub mod test {
    use super::ProcessItem;
    use std::ffi::OsString;
    
    #[test]
    fn test_process_item_eq() {
        let item1 = ProcessItem::new(2, OsString::from("pm"), 5 as f32, 10 as u64);
        let item2 = ProcessItem::new(2, OsString::from("pm"), 5 as f32, 10 as u64);
        assert!(item1.eq(&item2));
        let item3 = ProcessItem::new(3, OsString::from("pd"), 5 as f32, 10 as u64);
        assert!(!item2.eq(&item3));
    }

    #[test]
    fn test_process_item_name_to_string_lossy() {
        let pname = "pm";
        let item1 = ProcessItem::new(2, OsString::from(pname), 5 as f32, 10 as u64);
        let cow = item1.name_to_string_lossy();
        assert_eq!(cow.into_owned(), String::from(pname));
    }
}
