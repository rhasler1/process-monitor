// Testing for this module is in tests/integration_test.rs

use super::{AST, Sort, ProcessEntry as Row};

#[derive(Default)]
pub struct ProcessTable {
    rows:       Vec<Row>,
}

impl ProcessTable {
    pub fn new(
        rows: Vec<Row>,
        ) -> Self {
        Self {
            rows,
        }
    }

    pub fn update_rows(&mut self, rows: Vec<Row>) {
        self.rows = rows
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

    // TODO: Should this take in columns and return a different type?
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

/*
 * TODO: integrate idea into row.rs
 *
 * pub struct ProcessTableRow<'a> {
    process: &'a Process,
    columns: &'a Columns,
}
 *
 * impl<'a> ProcessTableRow<'a> {
    pub fn cells(&self) -> impl Iterator<Item = Cell<'a>> {
        self.columns.iter().map(|column| {
            column.cell(self.process)
        })
    }
}

 *let rows = self.process_table
    .visible_rows(
        state.active_view().table_state().row_sort(),
        state.active_view().table_state().filter_ast(),
    )
    .skip(row_offset)
    .take(area.height as usize)
    .map(|process| {
        state.active_view()
            .columns()
            .cells(process)
    });
 *
 * */
