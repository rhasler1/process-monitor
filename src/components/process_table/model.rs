use crate::domain::process::primitive::ProcessItem;
use crate::domain::process::model::ProcessSnapShot;

pub enum Direction {
    Up,
    Down
}

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

pub struct Rows {
    rows:      Vec<Row>,
    filter:    Option<String>,
    selection: Option<usize>,
    order:     RowOrder
}

impl Rows {
    pub fn move_selection(&mut self, dir: Direction) {
        if let Some(selection) = self.selection {
            match dir {
                Direction::Up   => { self.selection = Some(selection.saturating_sub(1)); }
                Direction::Down => { self.selection = Some(selection.saturating_add(1)); }
            }
            self.enforce_invariant_on_selection();
        } 
    }

    pub fn enforce_invariant_on_selection(&mut self) {
        let row_count: usize = self.visible().len();

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

    /// Returns iterator over visible rows after applying filter
    /// and sorting. The Iterator Item includes a Row reference and
    /// a flag indicating if the row is the selected row
    pub fn iter(&self) -> impl Iterator<Item = (&Row, bool)> {
        let indices = self.visible();
        
        indices.into_iter().map(move |i| {
            (&self.rows[i], Some(i) == self.selection)
        })
    }
   
    /// Return indices of visible rows after applying filter and sorting
    fn visible(&self) -> Vec<usize> {
        let mut indices: Vec<usize> = (0..self.rows.len()).collect();
        
        if let Some(filter) = &self.filter {
            indices.retain(|&i| self.rows[i].filter(&filter));
        }
        
        indices.sort_by(|&i, &j| self.rows[i].order(&self.rows[j], &self.order));
        
        indices
    }

    /// Builder
    pub fn filter(self, filter: &str) -> Self {
        // Ownership of self.* is transferred
        Self {
            rows:      self.rows,
            filter:    Some(String::from(filter)),
            selection: self.selection,
            order:     self.order
        }
    }

    /// Builder
    pub fn selection(self, selection: Option<usize>) -> Self {
        Self {
            rows:      self.rows,
            filter:    self.filter,
            selection,
            order:     self.order
        }
    }

    /// Builder
    pub fn order(self, order: RowOrder) -> Self {
        Self {
            rows:      self.rows,
            filter:    self.filter,
            selection: self.selection,
            order
        }
    }

    /// Builder
    pub fn rows(self, rows: Vec<Row>) -> Self {
        Self {
            rows,
            filter:    self.filter,
            selection: self.selection,
            order:     self.order,
        }
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
    pid:       u32,
    name:      String,
    cpu_usage: f32,
    mem_usage: u64
}

impl From<&ProcessItem> for Row {
    fn from(item: &ProcessItem) -> Self {
        Self {
            pid:       item.pid(),
            name:      item.name_to_string_lossy().to_string(),
            cpu_usage: item.cpu_usage(),
            mem_usage: item.mem_usage()
        }
    }
}

impl Row {
    // TODO [3/7/26] Work on more advanced filtering options
    pub fn filter(&self, filter: &str) -> bool {
        self.name.to_lowercase().contains(&filter.to_lowercase())
    }

    pub fn order(&self, other: &Self, order: &RowOrder) -> std::cmp::Ordering {
        match order {
            RowOrder::PIDDec =>  other.pid.cmp(&self.pid),
            RowOrder::PIDInc =>  self.pid.cmp(&other.pid),
            RowOrder::NameDec => other.name.cmp(&self.name),
            RowOrder::NameInc => self.name.cmp(&other.name),
            RowOrder::CPUDec =>  other.cpu_usage.partial_cmp(&self.cpu_usage).unwrap_or(std::cmp::Ordering::Equal),
            RowOrder::CPUInc =>  self.cpu_usage.partial_cmp(&other.cpu_usage).unwrap_or(std::cmp::Ordering::Equal),
            RowOrder::MemDec =>  other.mem_usage.cmp(&self.mem_usage),
            RowOrder::MemInc =>  self.mem_usage.cmp(&other.mem_usage)
        }
    }
}
