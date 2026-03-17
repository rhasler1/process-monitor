use crate::components::process_table::table::{TableEvent, TableFocus, TableModel};
use crate::components::process_table::row::{RowsEvent, Direction as RowDirection, RowOrder};
use crate::components::process_table::column::{Column, ColumnID, ColumnsEvent, Direction as ColsDirection};
use crate::adapters::crossterm::input::Key;

#[derive(Default)]
pub struct TableController;
impl TableController {
    pub fn key_event(&self, key: Key, model: &TableModel) -> Option<TableEvent> { 
        let focus = model.focus();
        match (key, focus) {
            (Key::Char('/'), _)                => Some(TableEvent::MoveFocus),

            (Key::Up,        TableFocus::Rows) => Some(TableEvent::OnRows(RowsEvent::MoveSelection(RowDirection::Up))),
            (Key::Down,      TableFocus::Rows) => Some(TableEvent::OnRows(RowsEvent::MoveSelection(RowDirection::Down))),
            (Key::Char('p'), TableFocus::Rows) => Some(TableEvent::OnRows(RowsEvent::Sort(RowOrder::PIDDec))),
            (Key::Char('P'), TableFocus::Rows) => Some(TableEvent::OnRows(RowsEvent::Sort(RowOrder::PIDInc))),
            (Key::Char('n'), TableFocus::Rows) => Some(TableEvent::OnRows(RowsEvent::Sort(RowOrder::NameDec))),
            (Key::Char('N'), TableFocus::Rows) => Some(TableEvent::OnRows(RowsEvent::Sort(RowOrder::NameInc))),
            (Key::Char('c'), TableFocus::Rows) => Some(TableEvent::OnRows(RowsEvent::Sort(RowOrder::CPUDec))),
            (Key::Char('C'), TableFocus::Rows) => Some(TableEvent::OnRows(RowsEvent::Sort(RowOrder::CPUInc))),
            (Key::Char('m'), TableFocus::Rows) => Some(TableEvent::OnRows(RowsEvent::Sort(RowOrder::MemDec))),
            (Key::Char('M'), TableFocus::Rows) => Some(TableEvent::OnRows(RowsEvent::Sort(RowOrder::MemInc))),
            
            (Key::Left,      TableFocus::Columns) => Some(TableEvent::OnCols(ColumnsEvent::MoveSelection(ColsDirection::Left))),
            (Key::Right,     TableFocus::Columns) => Some(TableEvent::OnCols(ColumnsEvent::MoveSelection(ColsDirection::Right))),
            (Key::Char('p'), TableFocus::Columns) => Some(TableEvent::OnCols(ColumnsEvent::InsertColumn(Column::from(ColumnID::PID)))),
            (Key::Char('n'), TableFocus::Columns) => Some(TableEvent::OnCols(ColumnsEvent::InsertColumn(Column::from(ColumnID::Name)))),
            (Key::Char('c'), TableFocus::Columns) => Some(TableEvent::OnCols(ColumnsEvent::InsertColumn(Column::from(ColumnID::CPU)))),
            (Key::Char('m'), TableFocus::Columns) => Some(TableEvent::OnCols(ColumnsEvent::InsertColumn(Column::from(ColumnID::Mem)))),
            (Key::Delete,    TableFocus::Columns) => Some(TableEvent::OnCols(ColumnsEvent::RemoveColumn)),

            (_, _)         => None 
        }
    }
}

