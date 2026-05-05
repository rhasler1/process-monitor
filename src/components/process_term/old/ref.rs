pub struct ProcInfo {
    pid:  u32,
    name: String
}

pub enum TerminateEvent {
    Initialize(ProcInfo),
    Confirm,
    Deny
}

pub struct Terminate {
    visible: bool,
    proc:    Option<ProcInfo>
}

impl Terminate {
    pub fn event(&mut self, event: TerminateEvent) {
        match event {
            // Problem: info will not be available here
            TerminateEvent::Initialize(info) {

            }
        }
    }
}

impl Default for Terminate {
    fn default() -> Self {
        Self {
            visible: false,
            pid:     None,
            name:    None
        }
    }
}
