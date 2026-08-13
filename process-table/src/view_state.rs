use crate::AsciiString;
use crate::Lexer;
use crate::Sort;
use crate::column::Columns;
use crate::row;
use super::Error;
use super::Parser;

use super::{Scroll, AST, ColumnConfig};

#[derive(Default)]
struct ProcessTableVisualSelection {
    selection: Option<usize>
}

impl ProcessTableVisualSelection {
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

    fn update(&mut self, upper_bound: usize) {
        self.selection_invariant(upper_bound);
    }

    fn inc_selection(&mut self, upper_bound: usize) {
        if let Some(visual_selection) = self.selection {
            self.selection = Some(visual_selection + 1);
        }

        self.selection_invariant(upper_bound);
    }

    fn dec_selection(&mut self, upper_bound: usize) {
        if let Some(visual_selection) = self.selection {
            self.selection = 
                Some(visual_selection.saturating_sub(1));
        }

        self.selection_invariant(upper_bound);
    }
}

#[derive(Default)]
pub struct ProcessTableViewState {
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

impl ProcessTableViewState {
    // Filter_string: AsciiString mutators
    
    pub fn filter_string_insert_ch(&mut self, ch: char) -> Result<(), Error> {
        self.filter_string
            .insert_ch(ch).map_err(Error::from)
    }

    pub fn filter_string_remove_ch(&mut self) {
        self.filter_string
            .remove_ch()
    }

    pub fn filter_string_insert_str(&mut self, s: &str) -> Result<(), Error> {
        self.filter_string
            .insert_str(s).map_err(Error::from)
    }

    // Filter: Option<AST> mutators

    /// The caller is responsible for updating `row_selection`
    /// after calling this method. This method cannot update
    /// `row_selection` because `ProcessTableViewState` is
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

    // Columns: Columns getters

    pub fn columns_get_column_config(&self) -> Option<&ColumnConfig> {
        self.columns.get_column_config()
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

// TODO: Write unit tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {

    }
}


