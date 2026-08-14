// Testing for this module is in tests/integration_test.rs

use super::{AST, Sort, ProcessEntry as Row};

#[derive(Default)]
pub struct ProcessTable {
    rows:       Vec<Row>,
}

// TODO: Document

impl ProcessTable {
    pub fn new(
        rows: Vec<Row>,
        ) -> Self {
        Self {
            rows,
        }
    }

    fn sort_indices(&self, sort: &Sort) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.rows.len()).collect();

        indices.sort_by(|&i, &j| self.rows[i].cmp(&self.rows[j], sort));
        
        indices
    }

    fn rows_sorted(&self, sort: &Sort) -> impl Iterator<Item = &Row> {
        self.sort_indices(sort)
            .into_iter()
            .map(|visual_idx| &self.rows[visual_idx])
    }


    pub fn visible_rows(
        &self,
        sort: &Sort,
        ast: &Option<AST>
    ) -> impl Iterator<Item = &Row> {
        self.rows_sorted(sort)
            .filter(|row| {
                match ast {
                    Some(ast) => ast.matches(row),
                    None => true
                }
            })
    }

    pub fn get_row(
        &self,
        sort: &Sort,
        ast: &Option<AST>,
        visual_selection: Option<usize>
        ) -> Option<&Row> {
        if let Some(visual_selection) = visual_selection {
            self.visible_rows(sort, ast).nth(visual_selection)
        } else {
            None
        }
    }

    pub fn count_visible_rows(&self, sort: &Sort, ast: &Option<AST>) -> usize {
        self.visible_rows(sort, ast).count()
    }
}

