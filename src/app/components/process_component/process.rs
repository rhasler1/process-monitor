// Process model
use crate::app::components::process_component::process_model::{ProcessRow,ProcessTable};
// Model trait
use crate::app::components::process_component::ProcessTableSource;
// Process state import
use crate::app::components::process_component::process_state::ProcessTableState;
// Component trait
use crate::app::components::{Component,DrawableComponent};
use crate::app::EventState;
// Adapter imports
use crate::adapters::crossterm::input::{KeyInput,MouseInputKind,MouseInput};

pub struct ProcessComponent {
    table:       ProcessTable,
    table_state: ProcessTableState
}

impl ProcessComponent {
    pub fn new<T>(table_source: &T) -> Self where T: ProcessTableSource
    {
        let table = table_source.build_table();
        let table_state = ProcessTableState::new(&table);
        Self {
            table,
            table_state
        }
    }

    pub fn refresh_event<T>(&mut self, table_source: &T) where T: ProcessTableSource {
        self.replace_table(table_source);
    }

    //TODO
    fn replace_table<T>(&mut self, table_source: &T) where T: ProcessTableSource {
        let new_table = table_source.build_table();
        self.table_state.enforce_select_row_invariant(&new_table);
        self.table = new_table;
    }
}

impl Component for ProcessComponent {
    fn key_event(&mut self, key: KeyInput) -> EventState {/*TODO*/ EventState::NotConsumed}
    fn mouse_event(&mut self, mouse: MouseInput) -> EventState {/*TODO*/ EventState::NotConsumed}
}

use ratatui::prelude::{Frame,Rect,Layout,Direction,Constraint};
use ratatui::widgets::{Cell,Row,Table};
impl DrawableComponent for ProcessComponent {
    fn draw(
        &mut self,
        frame:   &mut Frame,
        area:    Rect,
        focused: bool) -> anyhow::Result<()>
    {
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1)    // Process table
            ]).split(area);
        let process_table_height = vertical_chunks[0].height;
        let rows = self.table.iter()
            .skip(0)
            .take(process_table_height.into())
            .map(|process_row| {
                let cells = vec![Cell::from(format!("{}",process_row.pid())),
                Cell::from(format!("{}",process_row.name_to_string_lossy())),
                Cell::from(format!("{}",process_row.cpu_usage())),
                Cell::from(format!("{}",process_row.mem_usage()))
                ];
                Row::new(cells)
            })
            .collect::<Vec<_>>();
        
        let widths = [Constraint::Length(5),Constraint::Length(5),Constraint::Length(5),Constraint::Length(5)];
        let table = Table::new(rows,widths);
        frame.render_widget(table, vertical_chunks[0]);

        Ok(())
    }
}
