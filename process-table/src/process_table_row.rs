use super::{Process, ProcessStats, RowSort};

use std::cmp::Ordering;

pub struct ProcessTableRow<'a> {
    process:    &'a Process,
    statistics: &'a ProcessStats,
}

impl<'a> ProcessTableRow<'a> {
    pub fn new(
        process: &'a Process,
        statistics: &'a ProcessStats
        ) -> Self {
        Self { process, statistics }
    }

    pub fn process(&self) -> &'a Process {
        self.process
    }

    pub fn statistics(&self) -> &'a ProcessStats {
        self.statistics
    }

    /// Only supporting current `Process` based sorting.
    /// Sorting by historical `ProcessStats` is a hot path.
    /// In the future, ProcessStats can be optimized to maintian
    /// rolling statistics such that updating becomes O(1) rather
    /// than the current O(N).
    pub fn cmp(&self, other: &Self, sort: &RowSort) -> Ordering {
        match sort {
            RowSort::PidDec => other.process.pid().cmp(self.process.pid()),
            RowSort::PidInc => self.process.pid().cmp(other.process.pid()),

            RowSort::CpuDec => other.process.cpu_total()
                .partial_cmp(self.process.cpu_total())
                .unwrap_or(Ordering::Equal),

            RowSort::CpuInc => self.process.cpu_total()
                .partial_cmp(other.process.cpu_total())
                .unwrap_or(Ordering::Equal),

            RowSort::MemDec => other.process.mem().cmp(self.process.mem()),
            RowSort::MemInc => self.process.mem().cmp(other.process.mem()),

            RowSort::NameDec => other.process.name().cmp(self.process.name()),
            RowSort::NameInc => self.process.name().cmp(other.process.name()),
        }
    }
}

