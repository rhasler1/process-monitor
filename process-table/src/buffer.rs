use super::BufError;

use std::str::Chars;

/// Guarantees internal buffer is ascii
#[derive(Default)]
pub struct AsciiString {
    /// Internal buffer
    buffer: String,
    /// Insert position into buffer
    cursor: usize
}

impl AsciiString {
    // Buffer mut methods
    pub fn insert_ch(&mut self, ch: char) -> Result<(), BufError> {
        if !ch.is_ascii() {
            return Err(BufError::NonAsciiInsertCh(ch))
        }

        self.buffer.insert(self.cursor, ch);
        self.cursor += 1;
        
        Ok(())
    }

    pub fn remove_ch(&mut self) -> Result<(), BufError> {
        if self.cursor  == 0 {
            // Nothing to remove
            return Ok(())
        }

        self.cursor -= 1;
        self.buffer.remove(self.cursor);

        Ok(())
    }

    pub fn insert_str(&mut self, s: &str) -> Result<(), BufError> {
        if !s.is_ascii() {
            return Err(BufError::NonAsciiInsertStr(s.to_string()))
        }

        self.buffer.insert_str(self.cursor, s);
        self.cursor += s.len();

        Ok(())
    }

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
    pub fn inc_cursor(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor += 1;
        }
    }

    pub fn dec_cursor(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    // Cursor get methods
    pub fn cursor(&self) -> usize {
        self.cursor
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert_ch() {
        let mut ascii_string = AsciiString::default();
        ascii_string.insert_ch('h').unwrap();
        ascii_string.insert_ch('e').unwrap();
        ascii_string.insert_ch('l').unwrap();
        ascii_string.insert_ch('l').unwrap();
        ascii_string.insert_ch('o').unwrap();
        ascii_string.insert_ch(' ').unwrap();
        ascii_string.insert_ch('w').unwrap();
        ascii_string.insert_ch('o').unwrap();
        ascii_string.insert_ch('r').unwrap();
        ascii_string.insert_ch('l').unwrap();
        ascii_string.insert_ch('d').unwrap();
        ascii_string.insert_ch('!').unwrap();
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
        ascii_string.remove_ch().unwrap();
        assert!(ascii_string.is_empty());
        assert_eq!(ascii_string.cursor(), 0);

        ascii_string.insert_ch('h').unwrap();
        assert!(!ascii_string.is_empty());
        assert_eq!(ascii_string.cursor(), 1);
        ascii_string.remove_ch().unwrap();
        assert!(ascii_string.is_empty());
        assert_eq!(ascii_string.cursor(), 0);
    }

    #[test]
    fn test_insert_str() {
        let mut ascii_string = AsciiString::default();
        
        assert!(ascii_string.is_empty());
        ascii_string.insert_str("hello world!").unwrap();
        assert!(!ascii_string.is_empty());
        assert_eq!(ascii_string.as_str(), "hello world!");
        assert_eq!(ascii_string.cursor(), ascii_string.as_str().len());
    }

    #[test]
    fn dec_cursor() {
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
    fn inc_cursor() {
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

