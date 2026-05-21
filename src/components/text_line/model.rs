pub enum MoveDirection {
    Left,
    Right
}

pub enum TextLineEvent {
    InsertCharacter(char),
    MoveCursor(MoveDirection),
    RemoveCharacter
}

pub struct TextLineModel {
    buffer:   String,
    capacity: usize,
    cursor:   usize
}

impl Default for TextLineModel {
    fn default() -> Self {
        Self {
            buffer:   String::with_capacity(Self::BUFFER_CAPACITY),
            capacity: Self::BUFFER_CAPACITY,
            cursor:   0 
        }
    }
}

impl TextLineModel {
    pub const BUFFER_CAPACITY: usize = 100;
    
    pub fn handle_event(&mut self, event: TextLineEvent) -> bool {
        match event {
            TextLineEvent::InsertCharacter(c) => self.insert_invariant(c),
            TextLineEvent::MoveCursor(dir)    => self.move_invariant(dir),
            TextLineEvent::RemoveCharacter    => self.remove_invariant()
        }
    }
    
    /// Safely moves the cursor
    ///
    /// Returns: true if cursor is moved
    ///          false if cursor could not be moved
    fn move_invariant(&mut self, dir: MoveDirection) -> bool {
        let len = self.buffer.len();
        if len == 0 {
            false
        } else {
            match dir {
                MoveDirection::Left => {
                    if self.cursor > 0 {
                        self.cursor -= 1;
                        true
                    } else {
                        false
                    }
                }
                MoveDirection::Right => {
                    if self.cursor < len {
                        self.cursor += 1;
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }

    fn insert_invariant(&mut self, c: char) -> bool {
        if self.buffer.len() < Self::BUFFER_CAPACITY {
            self.buffer.insert(self.cursor, c);
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    fn remove_invariant(&mut self) -> bool {
        if self.buffer.is_empty() || self.cursor == 0 {
            false
        } else {
            self.cursor -= 1;
            self.buffer.remove(self.cursor);
            true
        }
    }

    pub fn buffer(&self) -> &str {
        &self.buffer
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }
}

#[cfg(test)]
pub mod test {
    use super::{TextLineModel, TextLineEvent, MoveDirection};
    #[test]
    fn test_text_line_model() {
        let mut model = TextLineModel::default();
        model.handle_event(TextLineEvent::MoveCursor(MoveDirection::Left));
        assert_eq!(model.cursor(), 0);
        model.handle_event(TextLineEvent::MoveCursor(MoveDirection::Right));
        assert_eq!(model.cursor(), 0);
        for _i in 0..200 {
            model.handle_event(TextLineEvent::InsertCharacter('c'));
        }
        assert_eq!(model.len(), 100);
        assert_eq!(model.cursor(), 100);
        model.handle_event(TextLineEvent::MoveCursor(MoveDirection::Right));
        assert_eq!(model.len(), 100);
        assert_eq!(model.cursor(), 100);
        model.handle_event(TextLineEvent::MoveCursor(MoveDirection::Left));
        assert_eq!(model.len(), 100);
        assert_eq!(model.cursor(), 99);
        model.handle_event(TextLineEvent::MoveCursor(MoveDirection::Right));
        assert_eq!(model.len(), 100);
        assert_eq!(model.cursor(), 100);
        for _i in 0..10 {
            model.handle_event(TextLineEvent::MoveCursor(MoveDirection::Left));
        }
        model.handle_event(TextLineEvent::RemoveCharacter);
        assert_eq!(model.len(), 99);
        assert_eq!(model.cursor(), 89);
        for _i in 0..200 {
            model.handle_event(TextLineEvent::RemoveCharacter);
        }
        assert_eq!(model.len(), 10);
        assert_eq!(model.cursor(), 0);
        for _i in 0..200 {
            model.handle_event(TextLineEvent::MoveCursor(MoveDirection::Right));
            model.handle_event(TextLineEvent::RemoveCharacter); 
        }
        assert_eq!(model.len(), 0);
        assert_eq!(model.cursor(), 0);
    }
}
