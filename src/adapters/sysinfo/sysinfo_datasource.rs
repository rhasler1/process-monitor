// Import internal application process representation
use crate::domain::process::primitive::ProcessItem;
use crate::domain::process::model::ProcessSnapShot;
use crate::domain::process::ProcessSnapShotSource;
use sysinfo::{ProcessesToUpdate, ProcessRefreshKind, UpdateKind};

/// Adapter for internal application to communicate with sysinfo API
pub struct SysinfoDataSource {
    system: sysinfo::System
}

impl Default for SysinfoDataSource {
    /// Creates a default sysinfo::System instance that can be used
    /// to fetch system process information
    fn default() -> Self {
        Self {
            system: sysinfo::System::new_all()
        }
    }
}

impl SysinfoDataSource {
    /// Refreshes sysinfo internal structures
    //pub fn refresh_all(&mut self) {
    //    self.system.refresh_all();
    //}

    pub fn refresh_all(&mut self) {
        self.system.refresh_cpu_all();
        self.system.refresh_memory();

        // 1. Configure the filter to only fetch what you need (starting with nothing)
        let process_filter = ProcessRefreshKind::nothing()
            .with_cpu()
            .with_memory()
            .with_cmd(UpdateKind::Always);

        // 2. Refresh the processes in your struct
        self.system.refresh_processes_specifics(
            ProcessesToUpdate::All, 
            true, 
            process_filter
        );

        /*let process_filter = ProcessRefreshKind::new()
            .with_cpu()
            .with_memory()
            .with_cmd(UpdateKind::Always); // Tracks command names/arguments
            
        self.system.refresh_processes_specifics(
            ProcessesToUpdate::All, 
            true, 
            process_filter);*/
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
    /// Gets system process information via sysinfo::System
    /// Formats system process information to internal application
    ///
    fn fetch_process_snapshot(&self) -> ProcessSnapShot {
        let len = self.system.processes().len();
        let mut processes: Vec<ProcessItem> = Vec::with_capacity(len);

        for (pid, process) in self.system.processes() {
            // Get ownership of [name](sysinfo::process:name)
            let name = process.name().to_os_string();
            // sysinfo cpu_usage returns total usage over all cores; dividing by core_count to get
            // an avg usage over all cores
            let avg_cpu_usage = if let Some(core_count) = sysinfo::System::physical_core_count() {
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
