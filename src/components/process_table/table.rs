use crate::events::EventState;
use crate::config::config::Config;
use crate::domain::process::model::ProcessSnapShot;
use crate::components::process_table::{
    row::{Row, RowOrder, Rows, RowsEvent},
    column::{Column, Columns, ColumnEvent}
};
use crate::components::text_line::model::{TextLineModel, TextLineEvent};

#[derive(Clone, PartialEq)]
pub enum TableFocus {
    Rows,
    Columns,
    Filter
}

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
    pub fn new(snapshot: &ProcessSnapShot, config: &Config) -> Self {
        Self {
            rows:    Rows::from(snapshot),
            columns: Columns::from(config),
            filter:  TextLineModel::default(),
            focus:   TableFocus::Rows
        }
    }

    pub fn event(&mut self, event: TableEvent) -> EventState {
        match event {
            TableEvent::MoveFocus(table_focus) => {
                self.focus = table_focus;
            }
            TableEvent::OnRows(row_event) => {
                // TODO if row event is a sort, then the column header should be updated
                // `row_event` may return EventState::ReturnPID(pid)
                return self.rows.event(row_event);
            }
            TableEvent::OnCols(col_event) => {
                // `col_event` may return EventState::ReturnColumns
                return self.columns.event(col_event);
            }
            TableEvent::OnFilter(filter_event) => {
                self.filter.handle_event(filter_event);
                let filter_str = self.filter.buffer();
                self.rows.set_filter(filter_str);
            }
        }
        EventState::Consumed
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
        self.rows.iter_filter_and_sort()
    }

    pub fn row_selection(&self) -> Option<usize> {
        self.rows.get_selection()
    }

    pub fn row_order(&self) -> &RowOrder {
        self.rows.get_order()
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
