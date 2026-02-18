// Import primitive
use crate::core::process::primitive::ProcessItem;
// Import used by `ProcessSnapShotHistory`
use crate::core::common::bounded_queue::BoundedQueue;

/// Stores one or more `ProcessItem` in [data](ProcessSnapShot::data)
/// `ProcessItem's` stored in ProcessSnapShot are to be sampled at
/// the same time. The timestamp when sampling occurred is stored in
/// [ts](ProcessSnapShot::ts)
/// [ts] Set ts by chrono::Local::now().timestamp();
pub struct ProcessSnapShot {
    data: Vec<ProcessItem>,
    //Returns the number of non-leap seconds since January 1, 1970 0:00:00 UTC (aka “UNIX timestamp”).
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
}

/// Stores `ProcessSnapShot`s up to the provided capacity
pub struct ProcessSnapShotHistory {
    // ProcessSnapShotHistory capacity field is stored in the bounded queue
    data: BoundedQueue<ProcessSnapShot>
}

impl ProcessSnapShotHistory {
    pub fn default() -> Self {
        const DEFAULT_CAPACITY: usize = 10;
        let data: BoundedQueue<ProcessSnapShot> = BoundedQueue::new(DEFAULT_CAPACITY);
        Self {
            data
        }
    }

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
    use super::ProcessSnapShot;
    use chrono::{DateTime, Local, Utc};
    use crate::core::process::primitive::ProcessItem;
    use std::ffi::OsString;

    
    #[test]
    fn test_process_snap_shot() {
        let item1 = ProcessItem::new(2, OsString::from("pm"), 5 as f32, 10 as u64);
        let item2 = ProcessItem::new(3, OsString::from("pm"), 5 as f32, 10 as u64);
        let item3 = ProcessItem::new(4, OsString::from("pd"), 5 as f32, 10 as u64);
        
        /// [ts] Set ts by chrono::Local::now().timestamp();
        let ts = chrono::Local::now().timestamp();
        let snap_shot = ProcessSnapShot::new(vec![item1,item2,item3], ts);
        assert_eq!(snap_shot.count(), 3);
        let iter = snap_shot.iter();
        let item1 = ProcessItem::new(2, OsString::from("pm"), 5 as f32, 10 as u64);
        let pids: Vec<_> = iter.map(|item| {item.pid()}).collect();
        assert_eq!(pids[0],2);
        assert_eq!(pids[1],3);
        assert_eq!(pids[2],4);
    }

    use super::ProcessSnapShotHistory;
    #[test]
    fn test_process_snap_shot_history() {
        let item1 = ProcessItem::new(2, OsString::from("pm"), 5 as f32, 10 as u64);
        let item2 = ProcessItem::new(3, OsString::from("pm"), 5 as f32, 10 as u64);
        let item3 = ProcessItem::new(4, OsString::from("pd"), 5 as f32, 10 as u64);
        
        /// [ts] Set ts by chrono::Local::now().timestamp();
        let ts = chrono::Local::now().timestamp();
        let snapshot1 = ProcessSnapShot::new(vec![item1,item2,item3], ts);

        let item1 = ProcessItem::new(2, OsString::from("pm"), 6 as f32, 11 as u64);
        let item2 = ProcessItem::new(3, OsString::from("pm"), 7 as f32, 12 as u64);
        let item3 = ProcessItem::new(4, OsString::from("pd"), 8 as f32, 13 as u64);
        let snapshot2 = ProcessSnapShot::new(vec![item1,item2,item3], 10 as i64);

        let mut history = ProcessSnapShotHistory::new(2);
        assert_eq!(history.capacity(), 2);
        history.push_back(snapshot1);
        history.push_back(snapshot2);

        // Getting history of pids and checking
        let mut storage: Vec<Vec<u32>> = Vec::new();
        for snapshot in history.iter().rev() {
            let local_storage: Vec<u32> = snapshot.iter().map(|pitem| pitem.pid()).collect();
            storage.push(local_storage);
        }
        assert!(storage.len() == 2);
        for pids in storage {
            assert!(pids[0] == 2 as u32);
            assert!(pids[1] == 3 as u32);
            assert!(pids[2] == 4 as u32);
        }
    }
}

