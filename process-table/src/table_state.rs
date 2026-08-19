use crate::VisualRowScroll;

use super::{AsciiString, AST, Parser, Lexer, VisualRowSelection,
    Columns, RowSort, Error};

#[derive(Debug, Default, Clone)]
pub struct ProcessTableState {
    /// AsciiString manages it's own cursor
    filter_string:  AsciiString,
    /// AST that can be derived from AsciiString's buffer
    filter:         Option<AST>,
    /// Column orientation, manages it's own selection
    columns:        Columns,
    /// Row selection (index) in Table's visible rows
    row_selection:  VisualRowSelection,
    /// Calculate row offset
    row_scroll:     VisualRowScroll,
    /// Table row sort order
    row_sort:       RowSort
}

impl ProcessTableState {
    /* Filter string */
    pub fn mut_filter_string(&mut self) -> &mut AsciiString {
        &mut self.filter_string
    }

    pub fn filter_string(&self) -> &AsciiString {
        &self.filter_string
    }

    /* Filter AST */
    
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

    pub fn filter_ast(&self) -> &Option<AST> {
        &self.filter
    }
    
    /* Columns */

    pub fn mut_columns(&mut self) -> &mut Columns {
        &mut self.columns
    }

    pub fn columns(&self) -> &Columns {
        &self.columns
    }

    /* Row selection */
   
    pub fn mut_row_selection(&mut self) -> &mut VisualRowSelection {
        &mut self.row_selection
    }

    pub fn row_selection(&self) -> &VisualRowSelection {
        &self.row_selection
    }

    /* Row scroll */
    pub fn mut_row_scroll(&mut self) -> &mut VisualRowScroll {
        &mut self.row_scroll
    }

    pub fn row_scroll(&self) -> &VisualRowScroll {
        &self.row_scroll
    }

    /* Row sort */

    pub fn row_sort_by(&mut self, sort: RowSort) {
        self.row_sort = sort;
    }

    pub fn row_sort(&self) -> &RowSort {
        &self.row_sort
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update_filter_ast_valid_string() {
        let mut table_state = ProcessTableState::default();

        assert!(table_state.filter_string().is_empty());

        assert!(table_state.filter_ast().is_none());

        let result = table_state
            .mut_filter_string()
            .insert_ascii_str("pid = 1 | pid = 2");

        assert!(result.is_ok());

        let result = table_state.update_filter_ast();

        assert!(result.is_ok());

        assert!(table_state.filter_ast().is_some());
    }

    #[test]
    fn test_update_filter_ast_invalid_string() {
        let mut table_state = ProcessTableState::default();

        assert!(table_state.filter_string().is_empty());

        assert!(table_state.filter_ast().is_none());

        let result = table_state
            .mut_filter_string()
            .insert_ascii_str("pid = 1 | pid ! 2");

        assert!(result.is_ok());

        let result = table_state.update_filter_ast();

        assert!(result.is_err());

        assert!(table_state.filter_ast().is_none());
    }
}

