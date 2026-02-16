pub mod process_model;
use crate::app::models::process_model::ProcessTable;
pub trait ProcessTableSource {
    fn build_table(&self) -> ProcessTable;
}
