use crate::domain::process::primitive::ProcessItem;
use crate::domain::process::model::ProcessSnapShot;
use crate::events::EventState;

#[derive(Clone, PartialEq)]
pub enum Direction {
    Up,
    Down
}

#[derive(Clone, PartialEq)]
pub enum RowOrder {
    PIDDec,
    PIDInc,
    NameDec,
    NameInc,
    CPUDec,
    CPUInc,
    MemDec,
    MemInc
}

#[derive(Clone, PartialEq)]
pub enum RowsEvent {
    MoveSelection(Direction),
    TerminateSelection,
    Sort(RowOrder)
}

pub struct Rows {
    rows:      Vec<Row>,
    filter:    Option<String>,
    selection: Option<usize>,
    order:     RowOrder
}

impl Rows {
    pub fn row_event(&mut self, event: RowsEvent) -> EventState {
        match event {
            RowsEvent::MoveSelection(Direction::Up) => {
                self.move_selection(Direction::Up)
            }
            RowsEvent::MoveSelection(Direction::Down) => {
                self.move_selection(Direction::Down)
            }
            RowsEvent::Sort(RowOrder::PIDDec) => {
                self.order = RowOrder::PIDDec
            }
            RowsEvent::Sort(RowOrder::PIDInc) => {
                self.order = RowOrder::PIDInc
            }
            RowsEvent::Sort(RowOrder::NameDec) => {
                self.order = RowOrder::NameDec
            }
            RowsEvent::Sort(RowOrder::NameInc) => {
                self.order = RowOrder::NameInc
            }
            RowsEvent::Sort(RowOrder::CPUDec) => {
                self.order = RowOrder::CPUDec;
            }
            RowsEvent::Sort(RowOrder::CPUInc) => {
                self.order = RowOrder::CPUInc;
            }
            RowsEvent::Sort(RowOrder::MemDec) => {
                self.order = RowOrder::MemDec;
            }
            RowsEvent::Sort(RowOrder::MemInc) => {
                self.order = RowOrder::MemInc;
            }
            RowsEvent::TerminateSelection => {
                if let Some(pid) = self.get_selected_value() {
                    return EventState::ReturnPID(pid)
                }
            }
        }
        EventState::Consumed
    }

    /*pub fn row_event_term(&self) -> Option<u32> {
        self.get_selected_value()
    }*/

    fn move_selection(&mut self, dir: Direction) {
        if let Some(selection) = self.selection {
            match dir {
                Direction::Up   => { self.selection = Some(selection.saturating_sub(1)); }
                Direction::Down => { self.selection = Some(selection.saturating_add(1)); }
            }
            self.enforce_invariant_on_selection();
        } 
    }

    fn enforce_invariant_on_selection(&mut self) {
        let row_count: usize = self.filter_and_sort_indices().len();

        self.selection = if row_count == 0 {
            None
        } else {
            match self.selection {
                Some(row) if row < row_count => Some(row),
                Some(_) => Some(row_count - 1),
                None => Some(0)
            }
        };
    }

    fn get_selected_value(&self) -> Option<u32> {
        if let Some(selection) = self.selection {
            if let Some(idx) = self.filter_and_sort_indices().get(selection) {
                if let Some(item) = self.rows.get(*idx) {
                    return Some(item.pid)
                }
                return None
            }
            return None
        }
        None
    }

    /// Returns iterator over visible rows after applying filter
    /// and sorting. The Iterator Item includes a Row reference and
    /// a flag indicating if the row is the selected row
    pub fn iter(&self) -> impl Iterator<Item = (&Row, bool)> {
        // Turn the vector returned by visible() into an iterator that references &Row
        // Note: into_iter() consumes the collection returned by visible();
        // rendering the collection unusable afterwards
        self.filter_and_sort_indices().into_iter().enumerate().map(|(idx, visible_value)| (&self.rows[visible_value], Some(idx) == self.selection))
    }
   
    /// Return indices of visible rows after applying filter and sorting
    fn filter_and_sort_indices(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.rows.len()).collect();
        
        if let Some(filter) = &self.filter {
            indices.retain(|&i| self.rows[i].filter(&filter));
        }
        
        indices.sort_by(|&i, &j| self.rows[i].order(&self.rows[j], &self.order));
        
        indices
    }

    // Setter
    pub fn replace_rows(&mut self, rows: Vec<Row>) {
        self.rows = rows;
        self.enforce_invariant_on_selection();
    }

    pub fn set_filter(&mut self, filter_str: &str) {
        let filter_string = String::from(filter_str);
        self.filter = Some(filter_string);
        self.enforce_invariant_on_selection();
    }
    
    // Getter
    pub fn get_selection(&self) -> Option<usize> {
        self.selection
    }
}

/// Create inner rows data
impl From<&ProcessSnapShot> for Vec<Row> {
    fn from(snapshot: &ProcessSnapShot) -> Vec<Row> {
       let rows: Vec<Row> = snapshot.iter().map(|item| Row::from(item)).collect();
       rows
    }
}

/// Create Rows structure
impl From<&ProcessSnapShot> for Rows {
    // Transfer ownership
    fn from(snapshot: &ProcessSnapShot) -> Self {
       let rows: Vec<Row> = snapshot.iter().map(|item| Row::from(item)).collect();
       let selection = if rows.len() > 0 {
            Some(0)
       } else {
            None
       };

       Self {
            rows,
            filter: None,
            selection,
            order:  RowOrder::CPUDec,
        }
    }
}

pub struct Row {
    pub pid:             u32,
    pub name:            String,
    pub avg_cpu_usage:   f32,
    pub total_cpu_usage: f32,
    pub mem_usage:       u64      // From ProcessSnapShot memory usage is in bytes
}

impl From<&ProcessItem> for Row {
    fn from(item: &ProcessItem) -> Self {
        Self {
            pid:             item.pid(),
            name:            item.name_to_string_lossy().to_string(),
            avg_cpu_usage:   item.avg_cpu_usage(),
            total_cpu_usage: item.total_cpu_usage(),
            mem_usage:       item.mem_usage()
        }
    }
}

impl Row {
    // TODO: Add more filtering options
    fn filter(&self, filter: &str) -> bool {
        self.name.to_lowercase().contains(&filter.to_lowercase())
    }

    pub fn order(&self, other: &Self, order: &RowOrder) -> std::cmp::Ordering {
        match order {
            RowOrder::PIDDec =>  other.pid.cmp(&self.pid),
            RowOrder::PIDInc =>  self.pid.cmp(&other.pid),
            RowOrder::NameDec => other.name.cmp(&self.name),
            RowOrder::NameInc => self.name.cmp(&other.name),
            RowOrder::CPUDec =>  other.total_cpu_usage.partial_cmp(&self.total_cpu_usage).unwrap_or(std::cmp::Ordering::Equal),
            RowOrder::CPUInc =>  self.total_cpu_usage.partial_cmp(&other.total_cpu_usage).unwrap_or(std::cmp::Ordering::Equal),
            RowOrder::MemDec =>  other.mem_usage.cmp(&self.mem_usage),
            RowOrder::MemInc =>  self.mem_usage.cmp(&other.mem_usage)
        }
    }

    pub fn mem_usage_as_b(&self) -> u64 {
        self.mem_usage
    }
    
    pub fn mem_usage_as_kb(&self) -> u64 {
        self.mem_usage / 1024
    }

    pub fn mem_usage_as_mb(&self) -> u64 {
        self.mem_usage / 1048576
    }

    pub fn mem_usage_as_gb(&self) -> u64 {
        self.mem_usage / 1073741824
    }
}

#[cfg(test)]
pub mod test {
    use super::{Row, Rows, Direction};
    use crate::domain::process::primitive::ProcessItem;
    use std::ffi::OsString;
    use crate::domain::process::model::ProcessSnapShot;

    #[test]
    fn test_row_model_selection() {
        // Creating ProcessSnapShot to create Rows from
        let item1 = ProcessItem::new(2, OsString::from("pm"), 5 as f32, 5 as f32, 10 as u64);
        let item2 = ProcessItem::new(3, OsString::from("pm"), 5 as f32, 5 as f32, 10 as u64);
        let item3 = ProcessItem::new(4, OsString::from("pd"), 5 as f32, 5 as f32, 10 as u64);
        let ts = chrono::Local::now().timestamp();
        let snap_shot = ProcessSnapShot::new(vec![item1,item2,item3], ts);

        // Attempt to move selection out of bounds
        let mut rows = Rows::from(&snap_shot);
        for _i in 0..10 {
            rows.move_selection(Direction::Down);
        }
        assert!(rows.selection == Some(2));
        for _i in 0..10 {
            rows.move_selection(Direction::Up);
        }
        assert!(rows.selection == Some(0));

        // Snapshot increase in size
        let item1 = ProcessItem::new(2, OsString::from("pm"), 5 as f32, 5 as f32, 10 as u64);
        let item2 = ProcessItem::new(3, OsString::from("pm"), 5 as f32, 5 as f32, 10 as u64);
        let item3 = ProcessItem::new(4, OsString::from("pd"), 5 as f32, 5 as f32, 10 as u64);
        let item4 = ProcessItem::new(5, OsString::from("ps"), 5 as f32, 5 as f32, 10 as u64);
        let ts = chrono::Local::now().timestamp();
        let snap_shot = ProcessSnapShot::new(vec![item1,item2,item3,item4], ts);
        let new_rows: Vec<Row> = Vec::<Row>::from(&snap_shot);
        rows.replace_rows(new_rows);

        assert!(rows.selection == Some(0));
        for _i in 0..10 {
            rows.move_selection(Direction::Down);
        }
        assert!(rows.selection == Some(3));

        // Snapshot decrease in size
        let item1 = ProcessItem::new(2, OsString::from("pm"), 5 as f32, 5 as f32, 10 as u64);
        let item2 = ProcessItem::new(3, OsString::from("pm"), 5 as f32, 5 as f32, 10 as u64);
        let ts = chrono::Local::now().timestamp();
        let snap_shot = ProcessSnapShot::new(vec![item1,item2], ts);
        let new_rows: Vec<Row> = Vec::<Row>::from(&snap_shot);
        rows.replace_rows(new_rows);
        assert!(rows.selection == Some(1));
        
        // Snapshot empty
        let snap_shot = ProcessSnapShot::new(vec![], ts);
        let new_rows: Vec<Row> = Vec::<Row>::from(&snap_shot);
        rows.replace_rows(new_rows);
        assert!(rows.selection == None);
    }
}
