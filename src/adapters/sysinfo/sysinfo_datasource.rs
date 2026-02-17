// Import internal application process representation
use crate::core::process::primitive::ProcessItem;

/// Adapter for internal application to communicate with sysinfo API
pub struct SysinfoDataSource {
    system: sysinfo::System
}

impl SysinfoDataSource {
    /// Creates a default sysinfo::System instance that can be used
    /// to fetch system process information
    pub fn default() -> Self {
        Self {
            system: sysinfo::System::new_all()
        }
    }

    /// Refreshes sysinfo internal structures
    pub fn refresh_all(&mut self) {
        self.system.refresh_all();
    }
}
//TODO
impl ProcessSource for SysinfoDataSource {
    /// Gets system process information via sysinfo::System
    /// Formats system process information to internal application
    ///
    fn fetch_processes(&self) -> Vec<Process> {
        let len = self.system.processes().len();
        let mut processes: Vec<Process> = Vec::with_capacity(len);

        for (pid, process) in self.system.processes() {
            // Get ownership of [name](sysinfo::process:name)
            let name = process.name().to_os_string();
            // sysinfo cpu_usage returns total usage over all cores; dividing by core_count to get
            // an avg usage over all cores
            let cpu_avg_core_usage = if let Some(core_count) = sysinfo::System::physical_core_count() {
                process.cpu_usage() / core_count as f32
            } else {
                process.cpu_usage()
            };
            let memory_usage = process.memory();
            let process_row = ProcessItem::new(
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
