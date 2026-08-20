// Import internal application process representation
use crate::domain::process::primitive::ProcessItem;
use crate::domain::process::model::ProcessSnapShot;
use crate::domain::process::ProcessSnapShotSource;
use sysinfo::{
    System,
    ProcessesToUpdate
};

/// Adapter for internal application to communicate with sysinfo API
pub struct SysinfoDataSource {
    system: System
}

impl Default for SysinfoDataSource {
    /// Creates a default sysinfo::System instance that can be used
    /// to fetch system process information
    fn default() -> Self {
        Self {
            // Creates a new System instance with nothing loaded.
            system: System::new()
        }
    }
}

impl SysinfoDataSource {

    /// Refreshes all processes
    pub fn refresh_all(&mut self) {
        self.system.refresh_processes(
            ProcessesToUpdate::All,
            true
        );
    }

    /// Terminate process
    pub fn terminate_process(&self, pid: u32) {
        let pid: sysinfo::Pid = sysinfo::Pid::from_u32(pid);
        if let Some(process) = self.system.process(pid) {
            process.kill();
        }
    }
}

impl ProcessSnapShotSource for SysinfoDataSource {
    /// Fetches snapshot of current system processes.
    fn fetch_process_snapshot(&self) -> ProcessSnapShot {
        let mut processes = Vec::with_capacity(
            self.system
                .processes()
                .len()
        );

        for (pid, process) in self.system.processes() {
            // Get ownership of [name](sysinfo::process:name)
            let name = process.name().to_os_string();
            // sysinfo cpu_usage returns total usage over all cores;
            // dividing by core_count to get an avg usage over all cores
            let avg_cpu_usage = 
                if let Some(core_count) = sysinfo::System::physical_core_count() {
                    process.cpu_usage() / core_count as f32
            } else {
                process.cpu_usage()
            };
            let total_cpu_usage = process.cpu_usage();
            let memory_usage = process.memory();
            let process_row = ProcessItem::new(
                pid.as_u32(),
                name,
                avg_cpu_usage,
                total_cpu_usage,
                memory_usage
                );
            processes.push(process_row);
        }

        let ts = chrono::Local::now().timestamp();
        ProcessSnapShot::new(processes, ts)
    }
}
