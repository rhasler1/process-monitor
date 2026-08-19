
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum RowSort {
    // Process
    PidDec,
    PidInc,
    #[default]
    CpuDec,
    CpuInc,
    MemDec,
    MemInc,
    NameDec,
    NameInc,
    
    // Sorting by process stats is
    // a hot path. This can be optimized
    // in the future, e.g., modify
    // process_stats to maintain rolling
    // statistics. For now, Not supporting
    // sort by Process Stats
    //
    /*// Process Stats
    CpuAvgDec,
    CpuAvgInc,*/
}

