use crate::AsciiString;
use crate::Lexer;
use crate::Sort;
use crate::column::Columns;
use super::Error;
use super::Parser;

use super::{Scroll, AST, ColumnConfig};

/*
 * TODO: ProcessTableVisualSelection has been moved to view,
 * remove from here
 * */
#[derive(Debug, Default, Clone)]
struct ProcessTableVisualSelection {
    selection: Option<usize>
}

impl ProcessTableVisualSelection {
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

    /// Updates the selection by applying the
    /// invariant.
    fn update(&mut self, upper_bound: usize) {
        self.selection_invariant(upper_bound);
    }

    /// Advances the selection by 1.
    ///
    /// Selection is clamped by argued upper_bound.
    fn inc_selection(&mut self, upper_bound: usize) {
        if let Some(visual_selection) = self.selection {
            self.selection = Some(visual_selection + 1);
        }

        self.selection_invariant(upper_bound);
    }

    /// Moves the selection back by 1.
    fn dec_selection(&mut self, upper_bound: usize) {
        if let Some(visual_selection) = self.selection {
            self.selection = 
                Some(visual_selection.saturating_sub(1));
        }

        self.selection_invariant(upper_bound);
    }
}

#[derive(Debug, Default, Clone)]
pub struct ProcessTableState {
    /// AsciiString manages it's own cursor
    filter_string:  AsciiString,
    /// AST that can be derived from AsciiString's buffer
    filter:         Option<AST>,
    /// Columns manages it's own selection
    columns:        Columns,
    /// Table row scroll offset
    row_scroll:     Scroll,
    /// Table row selection
    row_selection:  ProcessTableVisualSelection,
    /// Table row sort order
    row_sort:       Sort
}

impl ProcessTableState {
    // Filter_string: AsciiString mutators
    
    pub fn filter_string_insert_ch(&mut self, ch: char) -> Result<(), Error> {
        self.filter_string
            .insert_ascii_ch(ch).map_err(Error::from)
    }

    pub fn filter_string_remove_ch(&mut self) {
        self.filter_string
            .remove_ch()
    }

    pub fn filter_string_insert_str(&mut self, s: &str) -> Result<(), Error> {
        self.filter_string
            .insert_ascii_str(s).map_err(Error::from)
    }

    pub fn filter_string_inc_cursor(&mut self) {
        self.filter_string.inc_cursor();
    }

    pub fn filter_string_dec_cursor(&mut self) {
        self.filter_string.dec_cursor();
    }

    // Filter: Option<AST> mutators

    /// The caller is responsible for updating `row_selection`
    /// after calling this method. This method cannot update
    /// `row_selection` because `ProcessTableState` is
    /// unaware of the row count.
    pub fn update_filter_ast(&mut self) -> Result<(), Error> {
        // Lex
        let mut lexer = Lexer::default();
        let tokens = lexer
            .process_line(self.filter_string.as_str())
            .inspect_err(|_| {
                self.filter = None
            })?;

        // Parse
        let mut parser = Parser::new(tokens);
        let ast = parser
            .parse()
            .inspect_err(|_| {
                self.filter = None
            })?;

        self.filter = Some(ast);
        Ok(())
    }

    // Filter: Option<AST> getters
    pub fn filter_ast(&self) -> &Option<AST> {
        &self.filter
    }
    
    // Row_selection: ProcessTableVisualSelection mutators

    /// Clamps `row_selection` to be within the argued `row_upper_bound`.
    /// 
    /// Call this method after Self::update_filer_ast()
    pub fn update_row_selection(
        &mut self,
        row_upper_bound: usize
        ) {
        self.row_selection.update(row_upper_bound);
    }

    pub fn inc_visual_row_selection(
        &mut self,
        row_upper_bound: usize
        ) {
        self.row_selection.inc_selection(row_upper_bound);
    }

    pub fn dec_visual_row_selection(
        &mut self,
        row_upper_bound: usize
        ) {
        self.row_selection.dec_selection(row_upper_bound);
    }

    // Row_selection: ProcessTableVisualSelection getters
    
    pub fn visual_row_selection(&self) -> Option<usize> {
        self.row_selection.selection
    }

    // columns: Columns mutators

    pub fn columns_insert_column(
        &mut self,
        col_config: ColumnConfig
        ) {
        self.columns.insert_column(col_config);
    }

    pub fn columns_remove_column(&mut self) {
        self.columns.remove_column();
    }

    pub fn columns_rotate_unit(&mut self) {
        self.columns.rotate_unit();
    }

    pub fn columns_inc_selection(&mut self) {
        self.columns.inc_selection();
    }

    pub fn columns_dec_selection(&mut self) {
        self.columns.dec_selection();
    }

    // Columns: Columns getters

    pub fn columns_get_column_config(&self) -> Option<&ColumnConfig> {
        self.columns.get_column_config()
    }

    /*TODO: Most of the getters & mutators here can just return
     * references to the underlying structures instead of explicitly
     * delegating. */
    pub fn columns(&self) -> &Columns {
        &self.columns
    }

    // row_sort_order: Sort mutators
    
    pub fn row_sort_by_pid_dec(&mut self) {
        self.row_sort = Sort::PidDec;
    }

    pub fn row_sort_by_pid_inc(&mut self) {
        self.row_sort = Sort::PidInc;
    }

    pub fn row_sort_by_cpu_dec(&mut self) {
        self.row_sort = Sort::CpuDec;
    }

    pub fn row_sort_by_cpu_inc(&mut self) {
        self.row_sort = Sort::CpuInc;
    }

    pub fn row_sort_by_mem_dec(&mut self) {
        self.row_sort = Sort::MemDec;
    }

    pub fn row_sort_by_mem_inc(&mut self) {
        self.row_sort = Sort::MemInc;
    }

    pub fn row_sort_by_name_dec(&mut self) {
        self.row_sort = Sort::NameDec;
    }

    pub fn row_sort_by_name_inc(&mut self) {
        self.row_sort = Sort::NameInc;
    }

    // row_sort_order: Sort getters
    
    pub fn row_sort(&self) -> &Sort {
        &self.row_sort
    }

    // row_scroll: Scroll mutators

    /// Calculates the visual_index of the first visible row needed
    /// to keep the selected row within the visual window.
    ///
    /// Each row occupies exactly one terminal cell veritcally, so
    /// `visual_window` represents the number of rows that can be
    /// displayed.
    ///
    /// Returns `0` when no row is selected.
    pub fn row_scroll_calc_start(&mut self, visual_window: usize) -> usize {
        let Some(visual_row_selection) = self.row_selection.selection else {
            return 0
        };

        self.row_scroll.calc_start(
            visual_window,
            visual_row_selection
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_visual_row_selection_invariant_bound_eq_0() {
        let row_count = 0;

        let mut view_state = ProcessTableState::default();

        assert!(view_state.visual_row_selection().is_none());

        view_state.update_row_selection(row_count);

        assert!(view_state.visual_row_selection().is_none());
    }

    #[test]
    fn test_inc_visual_row_selection() {
        let row_count = 5;

        let mut view_state = ProcessTableState::default();
        
        assert!(view_state.visual_row_selection().is_none());

        view_state.inc_visual_row_selection(row_count);

        assert_eq!(view_state.visual_row_selection(), Some(0));

        view_state.inc_visual_row_selection(row_count);

        assert_eq!(view_state.visual_row_selection(), Some(1));

        view_state.inc_visual_row_selection(row_count);

        assert_eq!(view_state.visual_row_selection(), Some(2));

        view_state.inc_visual_row_selection(row_count);

        assert_eq!(view_state.visual_row_selection(), Some(3));

        view_state.inc_visual_row_selection(row_count);

        assert_eq!(view_state.visual_row_selection(), Some(4));
         
        // BVA
        view_state.inc_visual_row_selection(row_count);

        assert_eq!(view_state.visual_row_selection(), Some(4));
    }

    #[test]
    fn test_dec_visual_row_selection() {
        let row_count = 5;

        let mut view_state = ProcessTableState::default();

        assert!(view_state.visual_row_selection().is_none());

        view_state.update_row_selection(row_count);

        assert_eq!(view_state.visual_row_selection(), Some(0));
        
        // BVA
        view_state.dec_visual_row_selection(row_count);

        assert_eq!(view_state.visual_row_selection(), Some(0));
    }
}

