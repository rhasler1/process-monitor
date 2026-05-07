use crate::components::process_table::row::{Row, Rows, RowsEvent};
use crate::components::process_table::column::{Column, Columns, ColumnEvent};
use crate::components::text_line::model::{TextLineModel, TextLineEvent};
use crate::domain::process::model::ProcessSnapShot;
use crate::events::EventState;
// A model receives an action and does something with it

#[derive(Clone, PartialEq)]
pub enum TableFocus {
    Rows,
    Columns,
    Filter
}

//#[derive(Clone, PartialEq)]
pub enum TableEvent {
    MoveFocus(TableFocus),
    OnRows(RowsEvent),
    OnCols(ColumnEvent),
    OnFilter(TextLineEvent)
}

pub struct TableModel {
    rows:    Rows,
    columns: Columns,
    filter:  TextLineModel,
    focus:   TableFocus
}

impl TableModel {
    pub fn table_event(&mut self, event: TableEvent) -> EventState {
        match event {
            TableEvent::MoveFocus(table_focus) => {
                self.focus = table_focus;
            }
            TableEvent::OnRows(row_event) => {
                // `row_event` may return EventState::ConsumedWithReturnPayload(pid)
                return self.rows.row_event(row_event);
            }
            TableEvent::OnCols(col_event) => {
                self.columns.event(col_event);
            }
            TableEvent::OnFilter(filter_event) => {
                self.filter.handle_event(filter_event);
                let filter_str = self.filter.buffer();
                self.rows.set_filter(filter_str);
            }
        }
        EventState::Consumed
    }

    /*pub fn table_event_term(&self) -> Option<u32> {
        if matches!(self.focus, TableFocus::Rows) {
            self.rows.row_event_term()
        }
        else {
            None
        }
    }*/

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
        self.columns.get_count()
    }

    // Filter methods
    pub fn filter_str(&self) -> &str {
        self.filter.buffer()
    }

    pub fn filter_len(&self) -> usize {
        self.filter.len()
    }

    pub fn filter_cursor(&self) -> usize {
        self.filter.cursor()
    }
}

impl From<&ProcessSnapShot> for TableModel {
    fn from(snapshot: &ProcessSnapShot) -> Self {
        Self {
            rows:    Rows::from(snapshot),
            columns: Columns::default(),
            filter:  TextLineModel::default(),
            focus:   TableFocus::Rows
        }
    }
}
