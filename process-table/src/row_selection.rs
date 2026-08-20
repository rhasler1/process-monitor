
#[derive(Debug, Default, Clone)]
pub struct VisualRowSelection {
    selection: Option<usize>
}

impl VisualRowSelection {


    /// Selection invariant for rows in a ProcessTable.
    ///
    /// # Behavior
    /// - If `upper_bound` is 0, `selection` is None.
    /// - If `upper_bound` is > 0 and `selection` is
    ///   Some(_) < `upper_bound`, then `selection`
    ///   is unchanged.
    /// - If `upper_bound` is > 0, and `selection` is
    ///   >= `upper_bound`, then `selection` is set to
    ///   > `upper_bound - 1`.
    /// - If `upper_bound` is > 0, and `selection` is 
    ///   None, then `selection is set to `Some(0)`.
    fn selection_invariant(&mut self, upper_bound: usize) {
        self.selection = if upper_bound == 0 {
            None
        } else {
            match self.selection {
                Some(visual_idx)
                    if visual_idx < upper_bound => Some(visual_idx),
                Some(_) => Some(upper_bound - 1),
                None => Some(0)
            }
        }
    }

    /// Updates the selection by applying the invariant.
    pub fn update_selection(&mut self, upper_bound: usize) {
        self.selection_invariant(upper_bound);
    }

    /// Advances the selection by 1.
    ///
    /// Selection is clamped by argued upper_bound.
    pub fn inc_selection(&mut self, upper_bound: usize) {
        if let Some(visual_selection) = self.selection {
            self.selection = Some(visual_selection + 1);
        }

        self.selection_invariant(upper_bound);
    }

    /// Moves the selection back by 1.
    pub fn dec_selection(&mut self, upper_bound: usize) {
        if let Some(visual_selection) = self.selection {
            self.selection = 
                Some(visual_selection.saturating_sub(1));
        }

        self.selection_invariant(upper_bound);
    }

    pub fn selection(&self) -> Option<usize> {
        self.selection
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_inc_visual_row_selection() {
        let row_count = 5;

        let mut visual_row_selection = VisualRowSelection::default();
        
        assert!(visual_row_selection.selection().is_none());

        visual_row_selection.inc_selection(row_count);

        assert_eq!(visual_row_selection.selection(), Some(0));

        visual_row_selection.inc_selection(row_count);

        assert_eq!(visual_row_selection.selection(), Some(1));

        visual_row_selection.inc_selection(row_count);

        assert_eq!(visual_row_selection.selection(), Some(2));

        visual_row_selection.inc_selection(row_count);

        assert_eq!(visual_row_selection.selection(), Some(3));

        visual_row_selection.inc_selection(row_count);

        assert_eq!(visual_row_selection.selection(), Some(4));
         
        // BVA
        visual_row_selection.inc_selection(row_count);

        assert_eq!(visual_row_selection.selection(), Some(4));
    }

    #[test]
    fn test_dec_visual_row_selection() {
        let row_count = 5;

        let mut visual_row_selection = VisualRowSelection::default();

        assert!(visual_row_selection.selection().is_none());

        visual_row_selection.dec_selection(row_count);

        assert_eq!(visual_row_selection.selection(), Some(0));

        // BVA
        visual_row_selection.dec_selection(row_count);

        assert_eq!(visual_row_selection.selection(), Some(0));
    }

    #[test]
    fn test_update_selection() {
        let row_count = 5;

        let mut visual_row_selection = VisualRowSelection::default();
    
        assert!(visual_row_selection.selection().is_none());

        visual_row_selection.update_selection(row_count);

        assert_eq!(visual_row_selection.selection(), Some(0));

        let row_count = 0;

        visual_row_selection.update_selection(row_count);

        assert!(visual_row_selection.selection().is_none());
    }
}

