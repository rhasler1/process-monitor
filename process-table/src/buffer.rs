use super::{BufError, Error};

use std::str::Chars;

use serde::{Deserialize, Serialize};

/// Guarantees internal buffer is ascii
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsciiString {
    /// Internal buffer
    buffer: String,
    /// Insert position into buffer
    cursor: usize,
    /// Buffer capacity
    capacity: usize
}

impl Default for AsciiString {
    fn default() -> Self {
        Self {
            buffer:     String::with_capacity(Self::DEFAULT_MAX_CAPACITY),
            cursor:     0,
            capacity:   Self::DEFAULT_MAX_CAPACITY
        }
    }
}

impl AsciiString {
    const DEFAULT_MAX_CAPACITY: usize = 100;

    /// Validates internal state.
    ///
    /// Call after deserializing.
    pub fn validate_deserialization(&self) -> Result<(), Error> {
        // Check invariant 1: capacity is less than or equal to the defined max.
        if self.capacity > Self::DEFAULT_MAX_CAPACITY {
            return Err(
                BufError::BadCapacity(
                    self.capacity
                ).into()
            );
        }

        // Check invariant 2: buffer length is less than or equal to the capacity.
        if self.buffer.len() > self.capacity {
            return Err(
                BufError::BadCapacity(
                    self.capacity
                ).into()
            );
        }

        // Check invariant 3: buffer is ascii.
        if !self.buffer.is_ascii() {
            return Err(
                BufError::NonAsciiInsertStr(
                    self.buffer.to_owned()
                ).into()
            );
        }

        // Check invariant 4: cursor position is less than or equal to collection length.
        if self.cursor > self.buffer.len() {
            return Err(
                BufError::InsertPositionOutOfBounds(
                    self.cursor()
                ).into()
            );
        }

        Ok(())
    }

    /// Inserts ascii character at cursor
    /// position into the buffer.
    ///
    /// If insert is successfuly, cursor is
    /// advanced by 1.
    ///
    /// # Returns
    /// BufError on non-ascii character.
    pub fn insert_ascii_ch(&mut self, ch: char) -> Result<(), BufError> {
        if !ch.is_ascii() {
            return Err(BufError::NonAsciiInsertCh(ch))
        }

        if self.buffer.len() < self.buffer.capacity() {
            self.buffer.insert(self.cursor, ch);
            self.cursor += 1;
        }

        Ok(())
    }

    /// Removes character previous to cursor
    /// from the buffer.
    ///
    /// If no previous character, nothing is removed.
    ///
    /// Cursor is moved back by one on successful removal.
    pub fn remove_ch(&mut self) {
        if self.cursor == 0 {
            // Nothing to remove
            return
        }

        self.cursor -= 1;
        self.buffer.remove(self.cursor);
    }


    /// Inserts ascii str at cursor
    /// position into the buffer.
    ///
    /// If insert is successfuly, cursor is
    /// advanced by str.len().
    ///
    /// # Returns
    /// BufError on non-ascii str.
    pub fn insert_ascii_str(&mut self, s: &str) -> Result<(), BufError> {
        if !s.is_ascii() {
            return Err(BufError::NonAsciiInsertStr(s.to_string()))
        }

        if self.buffer.len() + s.len() < self.buffer.capacity() {
            self.buffer.insert_str(self.cursor, s);
            self.cursor += s.len();
        }


        Ok(())
    }

    /// Clears the buffer and sets cursor
    /// to 0.
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.cursor = 0;
    }

    // Buffer get methods
    pub fn as_str(&self) -> &str {
        &self.buffer
    }

    pub fn chars(&self) -> Chars<'_> {
        self.buffer.chars()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    // Cursor mut methods
    
    /// Increments cursor by 1.
    ///
    /// Cursor is clamped by buffer length.
    pub fn inc_cursor(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor += 1;
        }
    }

    /// Decrements cursor by 1.
    pub fn dec_cursor(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    // Cursor get methods
    
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert_ascii_ch() {
        let mut ascii_string = AsciiString::default();
        
        ascii_string.insert_ascii_ch('h').unwrap();
        ascii_string.insert_ascii_ch('e').unwrap();
        ascii_string.insert_ascii_ch('l').unwrap();
        ascii_string.insert_ascii_ch('l').unwrap();
        ascii_string.insert_ascii_ch('o').unwrap();
        ascii_string.insert_ascii_ch(' ').unwrap();
        ascii_string.insert_ascii_ch('w').unwrap();
        ascii_string.insert_ascii_ch('o').unwrap();
        ascii_string.insert_ascii_ch('r').unwrap();
        ascii_string.insert_ascii_ch('l').unwrap();
        ascii_string.insert_ascii_ch('d').unwrap();
        ascii_string.insert_ascii_ch('!').unwrap();
        
        assert_eq!(
            ascii_string.as_str(),
            "hello world!"
        );
        
        assert_eq!(
            ascii_string.cursor(),
            ascii_string.as_str().len()
        );
    }

    #[test]
    fn test_remove_ch() {
        let mut ascii_string = AsciiString::default();
        
        // BVA
        assert!(ascii_string.is_empty());
        
        assert_eq!(ascii_string.cursor(), 0);
        
        ascii_string.remove_ch();
        
        assert!(ascii_string.is_empty());
        
        assert_eq!(ascii_string.cursor(), 0);
    }

    #[test]
    fn test_insert_ascii_str() {
        let mut ascii_string = AsciiString::default();
        
        assert!(ascii_string.is_empty());
        
        ascii_string.insert_ascii_str("hello world!").unwrap();
        
        assert!(!ascii_string.is_empty());
        
        assert_eq!(ascii_string.as_str(), "hello world!");
        
        assert_eq!(ascii_string.cursor(), ascii_string.as_str().len());
    }

    #[test]
    fn test_dec_cursor() {
        let mut ascii_string = AsciiString::default();
        
        assert!(ascii_string.is_empty());
        
        assert_eq!(ascii_string.cursor, 0); 
        
        // BVA
        for _ in 0..1 {
            ascii_string.dec_cursor();
        }

        assert_eq!(ascii_string.cursor, 0); 
    }

    #[test]
    fn test_inc_cursor() {
        let mut ascii_string = AsciiString::default();
        assert!(ascii_string.is_empty());
        assert_eq!(ascii_string.cursor, 0); 
        
        // BVA
        for _ in 0..1 {
            ascii_string.inc_cursor();
        }

        assert_eq!(ascii_string.cursor, 0); 
    }
}

