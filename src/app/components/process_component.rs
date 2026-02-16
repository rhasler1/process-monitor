pub mod process_model;
pub mod process_state;
pub mod process;

/// Any ProcessTable producer must implement this trait; see adapters/sysinfo/sysinfo_datasource.rs
use crate::app::components::process_component::process_model::ProcessTable;
pub trait ProcessTableSource {
    fn build_table(&self) -> ProcessTable;
}
