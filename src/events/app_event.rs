/// Crossterm mouse and key events are adapted to corresponding AppEvent variants
use crate::adapters::crossterm::input::*;
/// Instant is used to track a time delta for sending Refresh events
use std::time::{Instant,Duration};
/// MPSC channel
use std::sync::mpsc::{Receiver,SyncSender,sync_channel};

/// Defines app event & it's variants
pub enum AppEvent {
    KeyInputEvent(KeyInput),
    MouseInputEvent(MouseInput),
    Refresh,
    Tick
}

/// Bounded MPSC channel where producer sends AppEvent to receiver (located at src/main.rs)
/// The sending thread will block if the bounded MPSC channel is saturated
pub struct AppEvents {
    rx:  Receiver<AppEvent>,
    _tx: SyncSender<AppEvent>

}
impl AppEvents {
    pub const TICK_RATE:    Duration = Duration::from_millis(256);  // ~0.25s
    pub const REFRESH_RATE: Duration = Duration::from_millis(2048); // ~2s
    pub const CHANNEL_SIZE: usize = 16;

    pub fn default() -> Self {    
        let (tx, rx) = sync_channel(Self::CHANNEL_SIZE);
        let event_tx = tx.clone();

        // https://doc.rust-lang.org/std/time/struct.Instant.html
        let mut last = Instant::now();
        // move || means capture anything used inside the closure by value
        std::thread::spawn(move || {
            loop {
                // Refresh rate time delta check
                let now = Instant::now();
                if now.duration_since(last) >= Self::REFRESH_RATE {
                    last = now;
                    if event_tx.send(AppEvent::Refresh).is_err() {
                        return;
                    } // Crossterm key | mouse event check
                } else if let Ok(true) = crossterm::event::poll(Self::TICK_RATE) {
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
                    } // No refresh & no key | mouse event then send tick event
                } else {
                    // 'send' will only error if the receiving end of the channel has been disconnected
                    if event_tx.send(AppEvent::Tick).is_err() {
                        return; 
                    }
                }
            }
        });
        AppEvents {rx, _tx: tx}
    }

    pub fn next(&self) -> anyhow::Result<AppEvent, std::sync::mpsc::RecvError> {
        self.rx.recv()
    }
}

#[cfg(test)]
mod test {
    use crate::adapters::crossterm::input::*;
    use std::time::{Instant,Duration};
    use std::sync::mpsc::{Receiver,SyncSender,sync_channel};
    use super::{AppEvent,AppEvents};

    /// Test checks refresh events occur between REFRESH_RATE and REFRESH_RATE + TICK_RATE
    #[test]
    fn test_app_events() {
        let app_events = AppEvents::default();
        let mut last_refresh = Instant::now();
        let mut last_tick = Instant::now();
        let refresh_iters = 2;
        let mut refresh_count = 0;
        loop {
            let _ = match app_events.next() {
                Ok(AppEvent::Refresh) => {
                    let now_refresh = Instant::now();
                    let delta_refresh = now_refresh.duration_since(last_refresh);
                    assert!(delta_refresh >= AppEvents::REFRESH_RATE && delta_refresh <= AppEvents::REFRESH_RATE + AppEvents::TICK_RATE);
                    last_refresh = now_refresh;
                    refresh_count = refresh_count + 1;
                    if refresh_count >= refresh_iters {
                        break;
                    }
                }
                Ok(AppEvent::Tick) => {
                    let now_tick = Instant::now();
                    let delta_tick = now_tick.duration_since(last_tick);
                    assert!(delta_tick >= AppEvents::TICK_RATE && delta_tick <= AppEvents::TICK_RATE + AppEvents::TICK_RATE);
                    last_tick = now_tick;
                }
                // Not testing crossterm mouse & key events
                _ => continue
            };
        }
    }
// TODO 2/17/2026 TEST channel size?
}
