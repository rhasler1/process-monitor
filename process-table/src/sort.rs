
#[derive(Debug, Default, PartialEq, Eq)]
pub enum Sort {
    PidDec,
    PidInc,
    #[default]
    CpuDec,
    CpuInc,
    MemDec,
    MemInc,
    NameDec,
    NameInc
}

