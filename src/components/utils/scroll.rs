// 1D scroll
#[derive(Default)]
pub struct Scroll {
    start:  usize,
    count:  usize,
    cursor: usize
}

impl Scroll {
    /// `calc_start` does NOT check if the argued cursor is valid over the data
    /// it is indexing; that is the responsibility of the Component state | model
    /// that uses `Scroll`.
    /// `calc_start` does ensure that the argued cursor is in the range of values
    /// starting from `start`..count
    ///
    /// Args
    ///      - [`count`: usize]  the len of the "data window"
    ///      - [`cursor`: usize] the focus point in the "data window"
    ///      - Important: A 1:1 relationship between the cursor->element and count is assumed
    /// 
    /// Return value- `start`: usize
    pub fn calc_start(&mut self, count: usize, cursor: usize) -> usize {
        if count == 0 {
            self.count = 0;
            self.start = 0;
            self.cursor = cursor;
            return 0;
        }

        // If count got bigger shift left by the delta, clamp to 0
        if self.count > count {
            self.start = self.start.saturating_sub(self.count - count);
        }

        // Keep cursor in range start..count
        if cursor < self.start {
            self.start = cursor;
        } else if cursor > self.start + (count - 1) {
            self.start = cursor - (count - 1);
        }

        // Update scoll state
        self.count = count;
        self.cursor = cursor;
        // Return start
        self.start
    }
}

#[cfg(test)]
pub mod test {
    use super::Scroll;

    #[test]
    fn test_utils_scroll() {
        let mut scroll = Scroll::default();
        let height = 10;
        let cursor = 0;
        let start = scroll.calc_start(height, cursor);
        assert_eq!(start, 0 as usize);
        
        let cursor = 1;
        let start = scroll.calc_start(height, cursor);
        assert_eq!(start, 0 as usize);
        
        let cursor = 10;
        let start = scroll.calc_start(height, cursor);
        assert_eq!(start, 1 as usize);

        let height = 15; // 7,8,9,10,11,12,13,14,15,16,17,18,19,20,21
        let cursor = 21;
        let start = scroll.calc_start(height, cursor);
        assert_eq!(start, 7);
        
        let cursor = 10;
        let start = scroll.calc_start(height, cursor);
        assert_eq!(start, 7);

        let cursor = 1;
        let start = scroll.calc_start(height, cursor);
        assert_eq!(start, 1);

        let cursor = 0;
        let start = scroll.calc_start(height, cursor);
        assert_eq!(start, 0);

        let cursor = 100;
        let start = scroll.calc_start(height, cursor);
        assert_eq!(start, 86);

        let height = 5;
        let start = scroll.calc_start(height, cursor);
        assert_eq!(start, 96);
        
        let height = 1;
        let start = scroll.calc_start(height, cursor);
        assert_eq!(start, 100);
    }
}
