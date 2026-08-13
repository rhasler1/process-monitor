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

    fn rows(&self) -> impl Iterator<Item = &Row> {
        self.rows.iter()
    }

    fn rows_sorted(&self, sort: &Sort) -> impl Iterator<Item = &Row> {
        self.sort_indices(sort)
            .into_iter()
            .map(|visual_idx| &self.rows[visual_idx])
    }

    fn rows_filtered(&self, ast: &AST) -> impl Iterator<Item = &Row> {
        self.rows.iter().filter(|row| ast.matches(row))
    }

    /*pub fn rows_sorted_and_filtered(&self, sort: &Sort, ast: &AST) -> impl Iterator<Item = &Row> {
        self.rows_sorted(sort).filter(|row| ast.matches(row))
    }*/

    fn sort_indices(&self, sort: &Sort) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.rows.len()).collect();

        indices.sort_by(|&i, &j| self.rows[i].cmp(&self.rows[j], sort));
        
        indices
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

    pub fn count_visible_rows(&self, ast: &Option<AST>) -> usize {
        if let Some(ast) = ast {
            self.rows_filtered(ast).count()
        } else {
            self.rows().count()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Lexer, Parser};

    #[test]
    fn test_valid_rows_sorted_and_filtered() {
        let rows = vec![
            Row::new(1, 1.0, 1, "a".to_string()),
            Row::new(2, 2.0, 2, "b".to_string()),
            Row::new(3, 3.0, 3, "c".to_string()),
            Row::new(4, 4.0, 4, "d".to_string()),
        ];

        let table = ProcessTable::new(rows);

        let filter_string = "pid = 1 | pid = 2 & cpu < 2".to_string();
        let mut lexer = Lexer::default();
        let tokens = lexer.process_line(&filter_string).unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let sort = Sort::PidDec;

        let ast = Some(ast);
        let mut row_iter = table.visible_rows(&sort, &ast);
        //assert_eq!(row_iter.next(), Some(&Row::new(2, 2.0, 2, "b".to_string())));
        assert_eq!(row_iter.next(), Some(&Row::new(1, 1.0, 1, "a".to_string())));
        assert!(row_iter.next().is_none());
    }

    #[test]
    fn test_invalid_rows_sorted_and_filtered() {
        let rows = vec![
            Row::new(1, 1.0, 1, "a".to_string()),
            Row::new(2, 2.0, 2, "b".to_string()),
            Row::new(3, 3.0, 3, "c".to_string()),
            Row::new(4, 4.0, 4, "d".to_string()),
        ];

        let table = ProcessTable::new(rows);

        let filter_string = "pi d = 1".to_string();
        let mut lexer = Lexer::default();
        let tokens = lexer.process_line(&filter_string).unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();
        assert!(ast.is_err());

        let mut row_iter = table.rows();
        assert_eq!(row_iter.next(), Some(&Row::new(1, 1.0, 1, "a".to_string())));
        assert_eq!(row_iter.next(), Some(&Row::new(2, 2.0, 2, "b".to_string())));
        assert_eq!(row_iter.next(), Some(&Row::new(3, 3.0, 3, "c".to_string())));
        assert_eq!(row_iter.next(), Some(&Row::new(4, 4.0, 4, "d".to_string())));
        assert!(row_iter.next().is_none());
    }
}


