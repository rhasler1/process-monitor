// TODO SIGNOFF 2/21/2026 - Separate ratatui as much as possible from the Components
//

// Make this static for now, I'll work on generic UI components at a later time
const PID_HEADER:  &'static str = "pid";
const NAME_HEADER: &'static str = "name";
const CPU_HEADER:  &'static str = "cpu";
const MEM_HEADER:  &'static str = "mem";

// Column property builder
#[derive(Default, Clone, Copy)]
pub struct ColumnProp {
    pub header: &'static str, // header names
    pub width:  u16
} impl ColumnProp {
    pub fn header(mut self, header: &'static str) -> Self {
        self.header = header;
        self
    }
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }
}

// Row property builder
/*#[derive(Default)]
struct RowProp {
    pub index:  usize,
    pub height: u16
} impl RowProp {
    pub fn index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }
    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }
}*/

// TODO [2/23/26] Move StyleToken back to adapters/ratatui/theme.rs
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum StyleToken {
    // Token 
    Header, Row, Cell, Select, // Component inner
    Focus, NotFocus            // Component outter
}

// Common table elements that can be styled
#[allow(dead_code)]
pub struct TableStyle {
    pub header:    StyleToken,
    pub row:       StyleToken,
    pub select:    StyleToken,
    pub focus:     StyleToken,
    pub not_focus: StyleToken
} impl Default for TableStyle {
    fn default() -> Self {
        Self {
            header:    StyleToken::Header,
            row:       StyleToken::Row,
            select:    StyleToken::Select,
            focus:     StyleToken::Focus,
            not_focus: StyleToken::NotFocus
        }
    }
}

/// Builder
pub struct ProcessTableView {
    pub headers: Vec<ColumnProp>,
    pub height:  u16,
    pub width:   u16,
    pub style:   TableStyle
} impl Default for ProcessTableView {
    fn default() -> Self {
        let mut headers: Vec<ColumnProp> = Vec::with_capacity(Self::COLUMNS.into());
        headers.push(ColumnProp::default().header(PID_HEADER).width(0));
        headers.push(ColumnProp::default().header(NAME_HEADER).width(0));
        headers.push(ColumnProp::default().header(CPU_HEADER).width(0));
        headers.push(ColumnProp::default().header(MEM_HEADER).width(0));
        Self {
            headers,
            height: 0,
            width: 0,
            style: TableStyle::default()
        }
    }
} impl ProcessTableView {
    pub const COLUMNS: u16 = 4;
    pub fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }
}


// Ratatui is only used in the handle_draw() function;
// I am ok with this for now. I don't think I am that far
// off from being able to remove ratatui from the module.
// Once ratatui is removed from the module, I can write
// tests for this. Removing ratatui from the components
// implys that ratatui needs to be removed from the App.
//
// Import ratatui
use ratatui::prelude::{Layout,Direction,Constraint};
use ratatui::widgets::{Cell,Row,Table};
// Import model and state
use crate::components::process_table::state::ProcessTableState;
use crate::domain::process::model::ProcessSnapShot;
impl ProcessTableView {
    pub fn handle_draw(&mut self,
        frame: &mut ratatui::prelude::Frame,
        area:  ratatui::prelude::Rect,
        _focus: bool,
        process_snapshot: &ProcessSnapShot,
        _state: &ProcessTableState) -> anyhow::Result<()>
    {
        let _row_height = 1; // TODO make this a RowProp

        // Ratatui
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1)
            ]).split(area);

        // Set internal width
        if self.width != chunks[0].width {
            self.width = chunks[0].width;
            let column_width = self.width / Self::COLUMNS;
            self.headers = self.headers.iter()
                .map(|header| header.width(column_width)).collect();
        }

        // Ratatui
        let widths: Vec<_> = self.headers.iter()
            .map(|header| Constraint::Length(header.width)).collect();
       
        // Set internal height
        if self.height != chunks[0].height {
            self.height = chunks[0].height;
        }

        // Ratatui
        let header: Row = self.headers.iter()
            .map(|header| Cell::from(header.header)).collect::<Row>();

        // Ratatui
        //
        let rows = process_snapshot.iter()
            .skip(0)
            .take(self.height.into())
            .map(|process_item| {
                let cells = vec![Cell::from(format!("{}",process_item.pid())),
                Cell::from(format!("{}",process_item.name_to_string_lossy())),
                Cell::from(format!("{}",process_item.cpu_usage())),
                Cell::from(format!("{}",process_item.mem_usage()))
                ];
                Row::new(cells)
            }).collect::<Vec<_>>();
        /*let rows = process_snapshot.iter()
            .skip(0)
            .take(self.height.saturating_sub(row_height).into())        // sub 1 for the header
            .map(|process_item| {
                let mut cells = Vec::with_capacity(Self::COLUMNS.into());
                // Idea for reordering columns; column order should be part of state
                // so that it might be changed by the controller
                self.headers.iter().map(|header| {
                    let cell = match header.header {
                        PID_HEADER  => Cell::from(format!("{}", process_item.pid())),
                        NAME_HEADER => Cell::from(process_item.name_to_string_lossy()),
                        CPU_HEADER  => Cell::from(format!("{}", process_item.cpu_usage())),
                        MEM_HEADER  => Cell::from(format!("{}", process_item.mem_usage())),
                        _           => Cell::default()
                    };
                    cells.push(cell);
                });
                Row::new(cells)}).collect::<Vec<_>>();*/

        let table = Table::new(rows,widths).header(header);
        frame.render_widget(table, chunks[0]);
        Ok(())
    }
}
