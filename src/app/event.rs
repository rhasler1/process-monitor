// Bounded MPSC channel where producer sends AppEvent to receiver (located at src/main.rs)
// The sending thread will block if the bounded MPSC channel is saturated
// Crossterm mouse and key events are adapted to corresponding AppEvent variants
use crate::adapters::crossterm::input::*;

pub enum AppEvent {
    KeyInputEvent(KeyInput),
    MouseInputEvent(MouseInput),
    Tick
}

pub struct AppEvents {
    rx: std::sync::mpsc::Receiver<AppEvent>,
    _tx: std::sync::mpsc::SyncSender<AppEvent>
}

impl AppEvents {
    pub fn default() -> Self {
        const TICK_RATE: std::time::Duration = std::time::Duration::from_millis(256);
        const CHANNEL_SIZE: usize = 128;
        
        let (tx, rx) = std::sync::mpsc::sync_channel(CHANNEL_SIZE);
        let event_tx = tx.clone();
        
        std::thread::spawn(move || loop {
            // Handling crossterm key and mouse events
            if let Ok(true) = crossterm::event::poll(TICK_RATE) {
                if let Ok(event) = crossterm::event::read() {
                    if let crossterm::event::Event::Key(key) = event {
                        // Check needed for Windows
                        if key.kind == crossterm::event::KeyEventKind::Press {
                            // 'send' will only error if the receiving end of the channel has been disconnected
                            if event_tx.send(AppEvent::KeyInputEvent(KeyInput::from(key))).is_err() {
                                return;
                            }
                        }
                    }
                    if let crossterm::event::Event::Mouse(mouse) = event {
                        // 'send' will only error if the receiving end of the channel has been disconnected
                        if event_tx.send(AppEvent::MouseInputEvent(MouseInput::from(mouse))).is_err() {
                            return;
                        }
                    }
                }
            }
            // 'send' will only error if the receiving end of the channel has been disconnected
            if event_tx.send(AppEvent::Tick).is_err() {
                return;
            }
        });
        AppEvents {rx, _tx: tx}
    }

    pub fn next(&self) -> anyhow::Result<AppEvent, std::sync::mpsc::RecvError> {
        self.rx.recv()
    }
}
