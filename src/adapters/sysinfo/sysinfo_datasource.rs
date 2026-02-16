// Process model
use crate::app::components::process_component::process_model::{ProcessRow,ProcessTable};
// Trait to build model
use crate::app::components::process_component::ProcessTableSource;

/// Wraps sysinfo::System
pub struct SystemDataSource {
    system: sysinfo::System
}

impl SystemDataSource {
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

impl ProcessTableSource for SystemDataSource {
    /// Builds a ProcessTable using sysinfo API calls
    fn build_table(&self) -> ProcessTable {
        let len = self.system.processes().len();
        let mut processes: Vec<ProcessRow> = Vec::with_capacity(len);

        for (pid, process) in self.system.processes() {
            let name = process.name().to_os_string();
            // sysinfo cpu_usage returns total usage over all cores; dividing by core_count to get
            // an avg usage over all cores
            let cpu_avg_core_usage = if let Some(core_count) = sysinfo::System::physical_core_count() {
                process.cpu_usage() / core_count as f32
            } else {
                process.cpu_usage()
            };
            let memory_usage = process.memory();
            let process_row = ProcessRow::new(
                pid.as_u32(),
                name,
                cpu_avg_core_usage,
                memory_usage
                );
            processes.push(process_row);
        }

        ProcessTable::new(processes)
    }
}
