use serde::{Deserialize, Serialize};
// Import primitive
use crate::domain::process::primitive::ProcessItem;
// Import used by `ProcessSnapShotHistory`
use crate::domain::common::bounded_queue::BoundedQueue;
// DomainModel trait
//use crate::domain::DomainModel;

/// Stores one or more `ProcessItem` in [data](ProcessSnapShot::data)
/// `ProcessItem's` stored in ProcessSnapShot are to be sampled at
/// the same time. The timestamp when sampling occurred is stored in
/// [ts](ProcessSnapShot::ts)
/// [ts] Set ts by chrono::Local::now().timestamp();
#[derive(Default, Serialize, Deserialize)]
pub struct ProcessSnapShot {
    data: Vec<ProcessItem>,
    // Returns the number of non-leap seconds since January 1, 1970 0:00:00 UTC (aka “UNIX timestamp”).
    ts:   i64
}

impl ProcessSnapShot {
    /// Creates `ProcessSnapShot` with the provided [data](Vec<ProcessItem>) and [ts](DateTime<Utc>)
    pub fn new(data: Vec<ProcessItem>, ts: i64) -> Self {
        Self {
            data,
            ts
        }
    }

    /// Returns the number `ProcessItem` in the `ProcessSnapShot` it is called on
    pub fn count(&self) -> usize {
        self.data.len()
    }

    /// Returns the timestamp of the `ProcessSnapShot` is is called on 
    pub fn ts(&self) -> i64 {
        self.ts
    }

    /// Returns an immutable iterator over [data](ProcessSnapShot::data)
    pub fn iter(&self) -> std::slice::Iter<'_, ProcessItem> {
        self.data.iter()
    }

    // Use to create test fixtures
    #[cfg(test)]
    pub fn serialize(&self) -> String {
        toml::to_string(&self).unwrap()
    }

    #[cfg(test)]
    pub fn deserialize(s: &str) -> Self {
        toml::from_str(s).unwrap()
    }
}


/// Stores `ProcessSnapShot`s up to the provided capacity
pub struct ProcessSnapShotHistory {
    // ProcessSnapShotHistory capacity field is stored in the bounded queue
    data: BoundedQueue<ProcessSnapShot>
}

impl ProcessSnapShotHistory {
    pub fn new(capacity: usize) -> Self {
        let data: BoundedQueue<ProcessSnapShot> = BoundedQueue::new(capacity);
        Self {
            data
        }
    }

    pub fn push_back(&mut self, snap_shot: ProcessSnapShot) {
        self.data.push_back(snap_shot);
    }

    pub fn back(&self) -> Option<&ProcessSnapShot> {
        self.data.back()
    }

    pub fn front(&self) -> Option<&ProcessSnapShot> {
        self.data.front()
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, ProcessSnapShot> {
        self.data.iter()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_deserialize_process_fixture_count_1() {
        let proc_str = include_str!("../../../test/fixtures/process_snapshot_count_1.toml");
        let proc_snapshot = ProcessSnapShot::deserialize(proc_str);
        assert_eq!(proc_snapshot.count(), 1);
        assert_eq!(proc_snapshot.ts(), 1978609300);
        assert_eq!(proc_snapshot.iter().last().unwrap().pid(), 10);
        assert_eq!(proc_snapshot.iter().last().unwrap().avg_cpu_usage(), 0.0);
        assert_eq!(proc_snapshot.iter().last().unwrap().mem_usage(), 141230080);
        assert_eq!(proc_snapshot.iter().last().unwrap().name_to_string_lossy(), "process_a");
    }

    #[test]
    fn test_deserialize_process_fixture_count_22() {
        let proc_str = include_str!("../../../test/fixtures/process_snapshot_count_22.toml");
        let proc_snapshot = ProcessSnapShot::deserialize(proc_str);
        assert_eq!(proc_snapshot.count(), 22);
        assert_eq!(proc_snapshot.ts(), 1778609300);
        assert_eq!(proc_snapshot.iter().last().unwrap().pid(), 31);
        assert_eq!(proc_snapshot.iter().last().unwrap().avg_cpu_usage(), 0.0);
        assert_eq!(proc_snapshot.iter().last().unwrap().mem_usage(), 427196416);
        assert_eq!(proc_snapshot.iter().last().unwrap().name_to_string_lossy(), "process_v");
    }

    #[test]
    fn test_snapshot_constructor() {
        let pid = 10;
        let name = "process_a".into();
        let avg_cpu_usage = 0.0;
        let total_cpu_usage = 0.0;
        let mem_usage = 141230080;
        let ts = 1978609300;
        
        let data = vec![ProcessItem::new(pid, name, avg_cpu_usage, total_cpu_usage, mem_usage)];
        let proc_snapshot = ProcessSnapShot::new(data, ts);

        assert_eq!(proc_snapshot.count(), 1);
        assert_eq!(proc_snapshot.ts(), 1978609300);
        assert_eq!(proc_snapshot.iter().last().unwrap().pid(), 10);
        assert_eq!(proc_snapshot.iter().last().unwrap().avg_cpu_usage(), 0.0);
        assert_eq!(proc_snapshot.iter().last().unwrap().mem_usage(), 141230080);
        assert_eq!(proc_snapshot.iter().last().unwrap().name_to_string_lossy(), "process_a");
    }
}
