pub mod process_data_model;
// Implement in adapters/sysinfo/
use crate::app::models::process_data_model::ProcessItem;
pub trait ProcessDataModelSource {
    fn fetch_model(&self) -> Vec<ProcessItem>;
}
