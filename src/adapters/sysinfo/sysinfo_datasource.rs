use crate::app::models::process_data_model::ProcessItem;
use crate::app::models::ProcessDataModelSource;

pub struct SysinfoDataSource {
    system: sysinfo::System
}

impl SysinfoDataSource {
    pub fn default() -> Self {
        Self {
            system: sysinfo::System::new_all()
        }
    }

    // note: sysinfo::MINIMUM_CPU_UPDATE_INTERVAL = 200 ms
    pub fn refresh_all(&mut self) {
        self.system.refresh_all();
    }
}

impl ProcessDataModelSource for SysinfoDataSource {
    fn fetch_model(&self) -> Vec<ProcessItem> {
        let len = self.system.processes().len();
        let mut processes: Vec<ProcessItem> = Vec::with_capacity(len);

        for (pid, process) in self.system.processes() {
            let name = if let Some(name) = process.name().to_str() {
                String::from(name)
            } else {
                String::from("Name Not found")
            };
            let cpu_usage = if let Some(core_count) = sysinfo::System::physical_core_count() {
                process.cpu_usage() / core_count as f32
            } else {
                process.cpu_usage()
            };
            let memory_usage = process.memory();
            let path = if let Some(path) = process.exe() {
                if let Some(path) = path.to_str() {
                    path.to_string()
                } else {
                    String::from("Path non-valid unicode")
                }
            } else {
                String::from("Path permission denied")
            };
            let process_item = ProcessItem::new(
                pid.as_u32(),
                name,
                cpu_usage,
                memory_usage,
                path
                );
            processes.push(process_item);
        }
        return processes;
    }
}
