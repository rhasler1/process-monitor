const PID_HEADER:  &'static str = "pid";
const NAME_HEADER: &'static str = "name";
const CPU_HEADER:  &'static str = "cpu";
const MEM_HEADER:  &'static str = "mem";

// Column property builder
#[derive(Default, Clone, Copy)]
pub struct ColumnProp {
    pub header: &'static str, // header names
    pub width:  u16
}

impl ColumnProp {
    pub fn header(mut self, header: &'static str) -> Self {
        self.header = header;
        self
    }

    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }
}

pub struct TableView {
    pub headers: Vec<ColumnProp>,
    pub height:  u16,
    pub width:   u16,
    pub scroll:  Scroll
}

impl Default for TableView {
    fn default() -> Self {
        let mut headers: Vec<ColumnProp> = Vec::with_capacity(Self::COLUMNS.into());
        headers.push(ColumnProp::default().header(PID_HEADER).width(0));
        headers.push(ColumnProp::default().header(NAME_HEADER).width(0));
        headers.push(ColumnProp::default().header(CPU_HEADER).width(0));
        headers.push(ColumnProp::default().header(MEM_HEADER).width(0));
        Self {
            headers,
            height: 0,
            width:  0,
            scroll: Scroll::default()
        }
    }
}

impl TableView {
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

// Import ratatui
use ratatui::prelude::{Layout,Direction,Constraint};
use ratatui::widgets::{Cell,Row,Table};
use ratatui::style::{Style, Color};
use crate::components::process_table::table::TableModel;
use crate::components::utils::scroll::Scroll;

impl TableView {
    pub fn draw(&mut self,
        frame: &mut ratatui::prelude::Frame,
        area:  ratatui::prelude::Rect,
        _focus: bool,
        table: &TableModel) -> anyhow::Result<()>
    {
        //let _row_height = 1; // TODO make this a RowProp

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

        // Subtract header height of 1 from self.height to get the count of rows that can be viewed
        let row_count = self.height.saturating_sub(1);

        // Ratatui
        let header: Row = self.headers.iter()
            .map(|header| Cell::from(header.header)).collect::<Row>();

        let start = if let Some(selection) = table.row_selection() {
            self.scroll.calc_start(row_count.into(), selection)
        } else {
            0
        };

        // Ratatui
        //
        let rows = table.row_iter()
            .skip(start)
            .take(row_count.into())
            .map(|(process_item, s)| {
                let cells = vec![Cell::from(format!("{}",process_item.pid)),
                Cell::from(format!("{}",&process_item.name)),
                Cell::from(format!("{}",process_item.cpu_usage)),
                Cell::from(format!("{}",process_item.mem_usage))
                ];
                let style = if s {
                    Style::default().bg(Color::Blue)
                } else {
                    Style::default().fg(Color::White)
                };
                Row::new(cells).style(style)
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
