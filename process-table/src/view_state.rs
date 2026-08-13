use crate::AsciiString;
use crate::Lexer;
use crate::column::Columns;
use super::Error;
use super::Parser;

use super::{Scroll, AST, ColumnConfig};

#[derive(Default)]
struct VisualSelection {
    selection: Option<usize>
}

impl VisualSelection {
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
    filter_string:  AsciiString,
    filter:         Option<AST>,
    columns:        Columns,
    row_scroll:     Scroll,
    row_selection:  VisualSelection,
    col_selection:  VisualSelection
}

impl ProcessTableViewState {
    pub fn insert_ch_filter_str(&mut self, ch: char) -> Result<(), Error> {
        self.filter_string
            .insert_ch(ch).map_err(Error::from)
    }

    pub fn remove_ch_filter_str(&mut self) -> Result<(), Error> {
        self.filter_string
            .remove_ch().map_err(Error::from)
    }

    pub fn insert_str_filter_str(&mut self, s: &str) -> Result<(), Error> {
        self.filter_string
            .insert_str(s).map_err(Error::from)
    }

    /// Selection needs to be updated after calling
    pub fn update_row_filter(&mut self) -> Result<(), Error> {
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

    /// Clamps row_selection to be within row_upper_bound
    /// Delegates this task to VisualSelection::update()
    pub fn update_row_selection(&mut self, row_upper_bound: usize) {
        self.row_selection.update(row_upper_bound);
    }

    /// Get current visual row selection
    pub fn visual_row_selection(&self) -> Option<usize> {
        self.row_selection.selection
    }

    pub fn inc_visual_row_selection(&mut self, row_upper_bound: usize) {
        self.row_selection.inc_selection(row_upper_bound);
    }

    pub fn dec_visual_row_selection(&mut self, row_upper_bound: usize) {
        self.row_selection.dec_selection(row_upper_bound);
    }

    pub fn filter_ast(&self) -> &Option<AST> {
        &self.filter
    }

    // TODO: Remove / Insert col
}


