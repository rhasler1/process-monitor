// Process model
use crate::app::models::process_model::{ProcessRow,ProcessTable};
// Model trait
use crate::app::models::ProcessTableSource;
// Component trait
use crate::app::components::DrawableComponent;

/// An enumerator describing tabular move directions
pub enum MoveDirection {
    Down,
    Up,
    Left,
    Right
}

/// Encapsulates process table state
struct ProcessTableState {
    select_row: Option<usize>
}

impl ProcessTableState {
    /// Creates a ProcessTableState with the provided ProcessTable
    pub fn new(table: &ProcessTable) -> Self {
        let select_row = if table.count_rows() > 0 {
            Some(0)
        } else {
            None
        };
        Self {
            select_row
        }
    }

    pub fn move_selection(&mut self, dir: MoveDirection, max_idx: usize) {
        if self.select_row.is_none() {
            return
        }
        match dir {
            MoveDirection::Down  => {
                self.select_row = Some(move_selection_down(self.select_row.unwrap(), max_idx));
            }
            MoveDirection::Up    => {
                self.select_row = Some(move_selection_up(self.select_row.unwrap()));
            }
            MoveDirection::Left  => {}
            MoveDirection::Right => {}
        }
    }

    /*TODO
    pub fn view(&self, model: &ProcessDataModel) -> Vec<usize> {
        let v = model.len();
    }
    */
}

const fn move_selection_down(selection_idx: usize, max_idx: usize) -> usize {
    let mut new_selection_idx = selection_idx;
    if selection_idx < max_idx {
        new_selection_idx = selection_idx + 1;
    }
    new_selection_idx
}

const fn move_selection_up(selection_idx: usize) -> usize {
    selection_idx.saturating_sub(1)
}

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
