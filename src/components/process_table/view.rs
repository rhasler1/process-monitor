use std::borrow::Cow;

// Make this static for now, I'll work on generic UI componenets at a later time
const PID_HEADER:  &'static str = "pid";
const NAME_HEADER: &'static str = "name";
const CPU_HEADER:  &'static str = "cpu";
const MEM_HEADER:  &'static str = "mem";

#[derive(Default)]
struct ProcessTableColumnProp {
    pub header: &'static str,
    //pub width:  u16
}

/// Builder
impl ProcessTableColumnProp {
    pub fn header(mut self, header: &'static str) -> Self {
        self.header = header;
        self
    }

    //pub fn width(&mut self, width: u16) -> Self {
    //    self.width = width;
    //    self
    //}
}

pub struct ProcessTableView {
    headers: Vec<ProcessTableColumnProp>,
    //style: // Ratatui? // I'd like to make the view in some way independant of Ratatui
}

// TODO 2/20/26
impl Default for ProcessTableView {
    fn default() -> Self {
        const CAPACITY: usize = 4;
        let mut headers: Vec<ProcessTableColumnProp> = Vec::with_capacity(CAPACITY);
        headers.push(ProcessTableColumnProp::default());
        headers.push(ProcessTableColumnProp::default());
        headers.push(ProcessTableColumnProp::default());
        headers.push(ProcessTableColumnProp::default());
        Self {
            headers
        }
    }
}

// Import ratatui
use ratatui::prelude::{Frame,Rect,Layout,Direction,Constraint};
use ratatui::widgets::{Cell,Row,Table};
use crate::components::process_table::state::ProcessTableState;
use crate::domain::process::model::ProcessSnapShot;

impl ProcessTableView {
    pub fn handle_draw(&self,
        frame: &mut Frame,
        area: Rect,
        focused: bool,
        process_snapshot: &ProcessSnapShot,
        state: &ProcessTableState) -> anyhow::Result<()>
    {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1)    // Process table
            ]).split(area);
        let process_table_height = vertical_chunks[0].height;
        let rows = process_snapshot.iter()
            .skip(0)
            .take(process_table_height.into())
            .map(|process_item| {
                let cells = vec![Cell::from(format!("{}",process_item.pid())),
                Cell::from(format!("{}",process_item.name_to_string_lossy())),
                Cell::from(format!("{}",process_item.cpu_usage())),
                Cell::from(format!("{}",process_item.mem_usage()))
                ];
                Row::new(cells)
            }).collect::<Vec<_>>();
        let widths = [Constraint::Length(5),Constraint::Length(5),Constraint::Length(5),Constraint::Length(5)];
        let table = Table::new(rows,widths);
        frame.render_widget(table, vertical_chunks[0]);
        Ok(())
    }
}

