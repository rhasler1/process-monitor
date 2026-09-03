/// Stores process PID
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcessPid(u32);

impl ProcessPid {
    pub fn new(pid: u32) -> Self {
        Self(pid)
    }

    pub fn as_u32(&self) -> u32 {
        self.0
    }

    pub fn as_u64(&self) -> u64 {
        self.0 as u64
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

/// Stores process CPU usage as total
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct ProcessCpuTotal(f32);

impl ProcessCpuTotal {
    pub fn new(cpu: f32) -> Self {
        Self(cpu)
    }

    pub fn total(&self) -> f32 {
        self.as_f32()
    }

    pub fn as_f32(&self) -> f32 {
        self.0
    }

    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
}

/// Stores process CPU usage as an average across all cores.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct ProcessCpuAverage(f32);

impl ProcessCpuAverage {
    pub fn new(cpu: f32) -> Self {
        Self(cpu)
    }

    pub fn average(&self) -> f32 {
        self.as_f32()
    }

    pub fn as_f32(&self) -> f32 {
        self.0
    }

    pub fn as_f64(&self) -> f64 {
        self.0 as f64
    }
}

/// Stores process memory usage as bytes
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessMemory(u64);

impl ProcessMemory {
    pub fn new(memory: u64) -> Self {
        Self(memory)
    }

    pub fn as_bytes(&self) -> u64 {
        self.as_u64()
    }

    pub fn as_kb(&self) -> u64 {
        self.as_u64() / 1024
    }

    pub fn as_mb(&self) -> u64 {
        self.as_u64() / 1048576
    }

    pub fn as_gb(&self) -> u64 {
        self.as_u64() / 1073741824
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

/// Stores process name as String
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ProcessName(String);

impl ProcessName {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Represents the current state of a process.
#[derive(Debug, Default, PartialEq)]
pub struct Process {
    pid:        ProcessPid,
    cpu_tot:    ProcessCpuTotal,
    cpu_avg:    ProcessCpuAverage,
    mem:        ProcessMemory,
    name:       ProcessName
}

impl Process {
    pub fn new(
        pid:        u32,
        cpu_tot:    f32,
        cpu_avg:    f32,
        mem:        u64,
        name:       String) -> Self {
        Self {
            pid:        ProcessPid(pid),
            cpu_tot:    ProcessCpuTotal(cpu_tot),
            cpu_avg:    ProcessCpuAverage(cpu_avg),
            mem:        ProcessMemory(mem),
            name:       ProcessName(name)
        }
    }

    pub fn pid(&self) -> &ProcessPid {
        &self.pid
    }

    pub fn cpu_total(&self) -> &ProcessCpuTotal {
        &self.cpu_tot
    }

    pub fn cpu_average(&self) -> &ProcessCpuAverage {
        &self.cpu_avg
    }

    pub fn mem(&self) -> &ProcessMemory {
        &self.mem
    }

    pub fn name(&self) -> &ProcessName {
        &self.name
    }

    /*pub fn statistic_fields(&self) -> (
        &ProcessPid,
        &ProcessCpu,
        &ProcessMemory
        ) {
        (self.pid(), self.cpu(), self.mem())
    }*/
}

