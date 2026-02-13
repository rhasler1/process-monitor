pub struct ProcessItem {
    pid: u32,
    name: String,
    cpu_usage: f32,
    memory_usage: u64,
    path: String
}

impl ProcessItem {
    pub fn new(
        pid: u32,
        name: String,
        cpu_usage: f32,
        memory_usage: u64,
        path: String) -> Self {
        Self {
            pid,
            name,
            cpu_usage,
            memory_usage,
            path,
        }
    }
}

impl PartialEq for ProcessItem {
    fn eq(&self, other: &Self) -> bool {
        self.pid.eq(&other.pid)
    }
}

pub struct ProcessDataModel {
    model: Vec<ProcessItem>
}

impl ProcessDataModel {
    pub fn new(model: Vec<ProcessItem>) -> Self {
        Self {
            model
        }
    }

    pub fn replace(&mut self, new_model: Vec<ProcessItem>) {
        self.model = new_model;
    }

    pub fn clear(&mut self) {
        self.model.clear();
    }
}
