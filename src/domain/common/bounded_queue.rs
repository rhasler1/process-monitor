use std::collections::VecDeque;

/// Description:
/// Bounds a VecDeque by a capacity; when an element is added to the queue that will
/// cause the queue to surpass it's capacity, the element at the front of the queue 
/// is popped off and the new element is placed at the back of the queue.
///
/// - The most recent element added is found using back()
/// - Reverse the iterator to iterate beginning at the most recent item
pub struct BoundedQueue<T> {
    queue:    VecDeque<T>,
    capacity: usize
}

impl<T> BoundedQueue<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            capacity
        }
    }
    
    pub fn push_back(&mut self, element: T) {
        let len = self.queue.len();
        if len < self.capacity {
            self.queue.push_back(element);
        } else if len == self.capacity {
            self.queue.pop_front();
            self.queue.push_back(element);
        } else {
            unreachable!("Invalid state reached")
        }
    }

    pub fn front(&self) -> Option<&T> {
        self.queue.front()
    }

    pub fn back(&self) -> Option<&T> {
        self.queue.back()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, T> {
        self.queue.iter()
    }
}

#[cfg(test)]
mod test {
    use super::BoundedQueue;
    #[test]
    fn test_bounded_queue() {
        let mut q = BoundedQueue::<u64>::new(2);
        assert_eq!(q.capacity, 2);
        q.push_back(1);
        q.push_back(2);
        assert_eq!(1, *q.front().unwrap());
        assert_eq!(2, *q.back().unwrap());
        q.push_back(3);
        assert_eq!(2, q.queue.len());
        assert_eq!(2, *q.front().unwrap());
        assert_eq!(3, *q.back().unwrap());
        let q_iter = q.iter();
        let v: Vec<_> = q_iter.rev().map(|element| {*element}).collect();
        assert_eq!(v[0], 3);
        assert_eq!(v[1], 2);
    }

    use crate::core::process::model::ProcessSnapShot;
    use chrono::{DateTime, Local, Utc};
    use crate::core::process::primitive::ProcessItem;
    use std::ffi::OsString;
    #[test]
    fn test_bounded_queue_containing_snapshots() {
        let item1 = ProcessItem::new(2, OsString::from("pm"), 5 as f32, 10 as u64);
        let item2 = ProcessItem::new(3, OsString::from("pm"), 5 as f32, 10 as u64);
        let item3 = ProcessItem::new(4, OsString::from("pd"), 5 as f32, 10 as u64);
        let snapshot1 = ProcessSnapShot::new(vec![item1,item2,item3], 10 as i64);

        let item1 = ProcessItem::new(2, OsString::from("pm"), 6 as f32, 11 as u64);
        let item2 = ProcessItem::new(3, OsString::from("pm"), 7 as f32, 12 as u64);
        let item3 = ProcessItem::new(4, OsString::from("pd"), 8 as f32, 13 as u64);
        let snapshot2 = ProcessSnapShot::new(vec![item1,item2,item3], 10 as i64);

        let mut q = BoundedQueue::<ProcessSnapShot>::new(2);
        assert_eq!(q.capacity, 2);
        q.push_back(snapshot1);
        q.push_back(snapshot2);
        
        let mut storage: Vec<Vec<u32>> = Vec::new();
        for snap in q.iter().rev() {
            let local_storage: Vec<u32> = snap.iter().map(|pitem| pitem.pid()).collect();
            storage.push(local_storage);
        }
        assert!(storage.len() == 2);
        for snap_pids in storage {
            assert!(snap_pids[0] == 2 as u32);
            assert!(snap_pids[1] == 3 as u32);
            assert!(snap_pids[2] == 4 as u32);
        }
    }
}
