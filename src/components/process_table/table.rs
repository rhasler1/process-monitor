use crate::components::process_table::row::{Row, Rows, RowsEvent};
use crate::components::process_table::column::{Column, Columns, ColumnsEvent};
use crate::domain::process::model::ProcessSnapShot;
// A model receives an action and does something with it

#[derive(Clone, PartialEq)]
pub enum TableFocus {
    Rows,
    Columns,
    Filter
}

//#[derive(Clone, PartialEq)]
pub enum TableEvent {
    MoveFocus,
    OnRows(RowsEvent),
    OnCols(ColumnsEvent)
}

pub struct TableModel {
    rows:    Rows,
    columns: Columns,
    focus:   TableFocus
}

impl TableModel {
    pub fn table_event(&mut self, event: TableEvent) {
        match event {
            TableEvent::MoveFocus => {
                self.move_focus();
            }
            TableEvent::OnRows(row_event) => {
                self.rows.row_event(row_event);
            }
            TableEvent::OnCols(col_event) => {
                self.columns.cols_event(col_event);
            }
            //_ => {}
        }
    }

    fn move_focus(&mut self) {
        match self.focus {
            TableFocus::Rows =>    { self.focus = TableFocus::Filter }
            TableFocus::Filter =>  { self.focus = TableFocus::Columns }
            TableFocus::Columns => { self.focus = TableFocus::Rows }
        }
    }

    pub fn new_snapshot(&mut self, snapshot: &ProcessSnapShot) {
        let new_rows = Vec::<Row>::from(snapshot);
        self.rows.replace_rows(new_rows);
    }


    pub fn focus(&self) -> TableFocus {
        self.focus.clone()
    }

    // Row methods
    pub fn rows_iter(&self) -> impl Iterator<Item = (&Row, bool)> {
        self.rows.iter()
    }

    pub fn row_selection(&self) -> Option<usize> {
        self.rows.get_selection()
    }

    // Column methods
    pub fn cols_iter(&self) -> impl Iterator<Item = (&Column, bool)> {
        self.columns.iter()
    }

    pub fn cols_count(&self) -> usize {
        self.columns.count()
    }
}

impl From<&ProcessSnapShot> for TableModel {
    fn from(snapshot: &ProcessSnapShot) -> Self {
        Self {
            rows:    Rows::from(snapshot),
            columns: Columns::default(),
            focus:   TableFocus::Rows
        }
    }
}
