// Deprecated 2/15/2026
/// A fixed byte array bounded by a capacity.
/// This structure has one implementation representing valid utf-8.
struct FixedByteArray<const CAPACITY: usize> {
    buf: [u8; CAPACITY],
    len: usize
}

/// Implements a fixed byte array representing valid utf-8.
impl<const CAPACITY: usize> FixedByteArray<CAPACITY> { 
    /// Creates a fixed byte array from a string slice.
    /// Truncates on the character-level; graphmemes can be visually split.
    pub fn from_str_trunc(s: &str) -> Self {
        let mut buf = [0u8; CAPACITY];
        let end = s.floor_char_boundary(s.len().min(CAPACITY));
        buf[0..end].copy_from_slice(&s.as_bytes()[0..end]);
        Self { buf, len: end }
    }
    
    /// Returns a string slice 
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.buf[0..self.len]).unwrap()
    }

    pub fn len(&self) -> usize {
        self.len
    }
    
    pub fn capacity(&self) -> usize {
        CAPACITY
    }
}
