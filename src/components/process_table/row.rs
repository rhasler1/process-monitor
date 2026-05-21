use crate::events::EventState;
use crate::domain::process::{
    primitive::ProcessItem,
    model::ProcessSnapShot
};

pub struct Row {
    pub pid:             u32,
    pub name:            String,
    pub avg_cpu_usage:   f32,
    pub total_cpu_usage: f32,
    pub mem_usage:       u64
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
    fn filter(&self, filter: &str) -> bool {
        self.name.to_lowercase().contains(&filter.to_lowercase())
    }

    pub fn order(&self, other: &Self, order: &RowOrder) -> std::cmp::Ordering {
        match order {
            RowOrder::PIDDec  => other.pid.cmp(&self.pid),
            RowOrder::PIDInc  => self.pid.cmp(&other.pid),
            RowOrder::NameDec => other.name.cmp(&self.name),
            RowOrder::NameInc => self.name.cmp(&other.name),
            RowOrder::CPUDec  => other.total_cpu_usage.partial_cmp(&self.total_cpu_usage).unwrap_or(std::cmp::Ordering::Equal),
            RowOrder::CPUInc  => self.total_cpu_usage.partial_cmp(&other.total_cpu_usage).unwrap_or(std::cmp::Ordering::Equal),
            RowOrder::MemDec  => other.mem_usage.cmp(&self.mem_usage),
            RowOrder::MemInc  => self.mem_usage.cmp(&other.mem_usage)
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

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down
}

#[derive(Clone, PartialEq, Debug)]
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

#[derive(Clone, PartialEq, Debug)]
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
    pub fn event(&mut self, event: RowsEvent) -> EventState {
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

    // Returns iterator over filtered & sorted rows. The Iterator 
    // Item includes a Row reference and a flag indicating if the row is the selected row
    pub fn iter_filter_and_sort(&self) -> impl Iterator<Item = (&Row, bool)> {
        // Turn the vector returned by filter_and_sort_indices() into an iterator that references &Row
        // Note: into_iter() consumes the collection returned by filter_and_sort_indices();
        // rendering the collection unusable afterwards
        self.filter_and_sort_indices().into_iter().enumerate().map(|(idx, visible_value)| (&self.rows[visible_value], Some(idx) == self.selection))
    }
   
    // Return row indices after applying filter and sort
    fn filter_and_sort_indices(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.rows.len()).collect();
        
        if let Some(filter) = &self.filter {
            indices.retain(|&i| self.rows[i].filter(filter));
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

    pub fn get_row_count_after_filter(&self) -> usize {
        self.filter_and_sort_indices().len()
    }

    pub fn get_order(&self) -> &RowOrder {
        &self.order
    } 
}


// Create inner rows data
impl From<&ProcessSnapShot> for Vec<Row> {
    fn from(snapshot: &ProcessSnapShot) -> Vec<Row> {
       let rows: Vec<Row> = snapshot.iter().map(Row::from).collect();
       rows
    }
}

// Create Rows structure
impl From<&ProcessSnapShot> for Rows {
    fn from(snapshot: &ProcessSnapShot) -> Self {
       let rows: Vec<Row> = snapshot.iter().map(Row::from).collect();
       let selection = if !rows.is_empty() {
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

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::events::EventState;
    use crate::domain::process::model::ProcessSnapShot;

    #[test]
    fn test_enforce_invariant_on_selection() {
        let proc_str = include_str!("../../../test/fixtures/process_snapshot_count_22.toml");
        let proc_snapshot = ProcessSnapShot::deserialize(proc_str);
        let mut rows = Rows::from(&proc_snapshot);
        let count = rows.get_row_count_after_filter();

        // BVA set rows selection past upper bound
        rows.selection = Some(count);
        rows.enforce_invariant_on_selection();
        assert_eq!(rows.selection, Some(count - 1));

        // BVA set rows selection "below" lower bound
        rows.selection = None;
        rows.enforce_invariant_on_selection();
        assert_eq!(rows.selection, Some(0));
    }

    #[test]
    fn test_event_move_selection() {
        let proc_str = include_str!("../../../test/fixtures/process_snapshot_count_22.toml");
        let proc_snapshot = ProcessSnapShot::deserialize(proc_str);
        let mut rows = Rows::from(&proc_snapshot);
        let count = rows.get_row_count_after_filter();

        // BVA attempt to move selection past the lower bound
        for _ in 0..(count + 1) {
            assert_eq!(rows.event(RowsEvent::MoveSelection(Direction::Up)), EventState::Consumed);
        }
        assert_eq!(rows.get_selection(), Some(0));

        // BVA attempt to move selection past the upper bound
        for _ in 0..(count + 1) {
            assert_eq!(rows.event(RowsEvent::MoveSelection(Direction::Down)), EventState::Consumed);
        }
        assert_eq!(rows.get_selection(), Some(count - 1));
    }

    #[test]
    fn test_event_sort() {
        let proc_str = include_str!("../../../test/fixtures/process_snapshot_count_22.toml");
        let proc_snapshot = ProcessSnapShot::deserialize(proc_str);
        let mut rows = Rows::from(&proc_snapshot);

        // Sort PID inc
        assert_eq!(rows.event(RowsEvent::Sort(RowOrder::PIDInc)), EventState::Consumed);
        let smallest_pid = rows.iter_filter_and_sort().next().unwrap().0.pid;
        let greatest_pid = rows.iter_filter_and_sort().last().unwrap().0.pid;
        assert!(smallest_pid < greatest_pid);

        // Sort PID dec
        assert_eq!(rows.event(RowsEvent::Sort(RowOrder::PIDDec)), EventState::Consumed);
        assert_eq!(rows.iter_filter_and_sort().next().unwrap().0.pid, greatest_pid);
        assert_eq!(rows.iter_filter_and_sort().last().unwrap().0.pid, smallest_pid);
    }

    #[test]
    fn test_event_terminate_selection() {
        let proc_str = include_str!("../../../test/fixtures/process_snapshot_count_22.toml");
        let proc_snapshot = ProcessSnapShot::deserialize(proc_str);
        let mut rows = Rows::from(&proc_snapshot);
        let pid = rows.iter_filter_and_sort().next().unwrap().0.pid;
        assert_eq!(rows.event(RowsEvent::TerminateSelection), EventState::ReturnPID(pid));
    }

    #[test]
    fn test_replace_rows() {
        let proc_str = include_str!("../../../test/fixtures/process_snapshot_count_22.toml");
        let proc_snapshot = ProcessSnapShot::deserialize(proc_str);
        let mut rows = Rows::from(&proc_snapshot);
        
        let proc_str = include_str!("../../../test/fixtures/process_snapshot_count_0.toml");
        let proc_snapshot = ProcessSnapShot::deserialize(proc_str);
        let new_rows = Vec::<Row>::from(&proc_snapshot);
        let count = new_rows.len();
        rows.replace_rows(new_rows);
        assert_eq!(rows.rows.len(), count);
        assert_eq!(rows.get_selection(), None);

        let proc_str = include_str!("../../../test/fixtures/process_snapshot_count_1.toml");
        let proc_snapshot = ProcessSnapShot::deserialize(proc_str);
        let new_rows = Vec::<Row>::from(&proc_snapshot);
        let count = new_rows.len();

        rows.replace_rows(new_rows);
        assert_eq!(rows.rows.len(), count);
        assert_eq!(rows.get_selection(), Some(0));
    }

    #[test]
    fn test_filter_rows() {
        let proc_str = include_str!("../../../test/fixtures/process_snapshot_count_22.toml");
        let proc_snapshot = ProcessSnapShot::deserialize(proc_str);
        let mut rows = Rows::from(&proc_snapshot);
        rows.set_filter("process_a");
        for row in rows.iter_filter_and_sort() {
            assert_eq!(row.0.name, "process_a");
        }
    }

    #[test]
    fn test_get_row_count_after_filter() {
        let proc_str = include_str!("../../../test/fixtures/process_snapshot_count_22.toml");
        let proc_snapshot = ProcessSnapShot::deserialize(proc_str);
        let rows = Rows::from(&proc_snapshot);
        assert_eq!(rows.get_row_count_after_filter(), 22);
    }
}
