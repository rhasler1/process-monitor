use crate::components::process_table::table::{TableEvent, TableFocus, TableModel};
use crate::components::process_table::row::{RowsEvent, Direction as RowDirection, RowOrder};
use crate::components::process_table::column::{Column, ColumnID, ColumnsEvent, MemUnitOptions, CPUUnitOptions, Direction as ColsDirection};
use crate::adapters::crossterm::input::Key;
use crate::components::text_line::model::{TextLineEvent, MoveDirection as TextLineMoveDirection};

#[derive(Default)]
pub struct TableController;
impl TableController {
    pub fn key_event(&self, key: Key, model: &TableModel) -> Option<TableEvent> { 
        let focus = model.focus();
        match (key, focus) {
            // Change focus
            (Key::Char('/'), TableFocus::Rows | TableFocus::Columns)   => Some(TableEvent::MoveFocus(TableFocus::Filter)),
            (Key::Enter,     TableFocus::Filter | TableFocus::Columns) => Some(TableEvent::MoveFocus(TableFocus::Rows)),
            (Key::Tab,       TableFocus::Filter | TableFocus::Rows)    => Some(TableEvent::MoveFocus(TableFocus::Columns)),

            // Row
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
            (Key::Char('T'), TableFocus::Rows) => Some(TableEvent::OnRows(RowsEvent::TerminateSelection)),

            // Column
            (Key::Left,      TableFocus::Columns) => Some(TableEvent::OnCols(ColumnsEvent::MoveSelection(ColsDirection::Left))),
            (Key::Right,     TableFocus::Columns) => Some(TableEvent::OnCols(ColumnsEvent::MoveSelection(ColsDirection::Right))),
            (Key::Char('p'), TableFocus::Columns) => Some(TableEvent::OnCols(ColumnsEvent::InsertColumn(Column::from(ColumnID::PID)))),
            (Key::Char('n'), TableFocus::Columns) => Some(TableEvent::OnCols(ColumnsEvent::InsertColumn(Column::from(ColumnID::Name)))),
            (Key::Char('c'), TableFocus::Columns) => Some(TableEvent::OnCols(ColumnsEvent::InsertColumn(Column::from(ColumnID::CPU(CPUUnitOptions::Avg))))),
            (Key::Char('m'), TableFocus::Columns) => Some(TableEvent::OnCols(ColumnsEvent::InsertColumn(Column::from(ColumnID::Mem(MemUnitOptions::B))))),
            (Key::Backspace, TableFocus::Columns) => Some(TableEvent::OnCols(ColumnsEvent::RemoveColumn)),
            (Key::Char('u'), TableFocus::Columns) => Some(TableEvent::OnCols(ColumnsEvent::RotateUnit)),

            // Filter
            (Key::Char(c),   TableFocus::Filter) => Some(TableEvent::OnFilter(TextLineEvent::InsertCharacter(c))),
            (Key::Backspace, TableFocus::Filter) => Some(TableEvent::OnFilter(TextLineEvent::RemoveCharacter)),
            (Key::Left,      TableFocus::Filter) => Some(TableEvent::OnFilter(TextLineEvent::MoveCursor(TextLineMoveDirection::Left))),
            (Key::Right,     TableFocus::Filter) => Some(TableEvent::OnFilter(TextLineEvent::MoveCursor(TextLineMoveDirection::Right))),

            (_, _) => None 
        }
    }
}

