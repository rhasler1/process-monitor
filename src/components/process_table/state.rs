use crate::core::process::model::ProcessSnapShot;

#[derive(Default)]
pub struct ProcessTableState {
    selection: Option<usize>
}

impl ProcessTableState {
    pub fn update(&mut self, process_snapshot: &ProcessSnapShot) {
        if process_snapshot.count() > 0 {
            self.selection = Some(0);
        }
    }
}
