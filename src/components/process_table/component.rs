// import ratatui
use ratatui::prelude::{Frame,Rect,Layout,Direction,Constraint};
use ratatui::widgets::{Cell,Row,Table};
//
use anyhow::Result;
use crate::components::process_table::state::ProcessTableState;
use crate::components::{Component,DrawableComponent};
use crate::core::process::model::ProcessSnapShot;
use crate::core::process::primitive::ProcessItem;

#[derive(Default)]
pub struct ProcessTable {
    state: ProcessTableState,
}

impl ProcessTable {
    pub fn update(&mut self, process_snapshot: &ProcessSnapShot) {
        self.state.update(&process_snapshot);
    }

    pub fn draw(&self,
        process_snapshot: &ProcessSnapShot,
        frame: &mut Frame,
        area: Rect,
        focused: bool) -> anyhow::Result<()>
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
