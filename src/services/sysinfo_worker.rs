use anyhow::Result;
// Log
use log::{debug, info, warn, error};
// Communication channel
use std::sync::mpsc::{
    Receiver, TryRecvError, RecvError,
    SyncSender, TrySendError, SendError,
    sync_channel
};
use std::sync::mpsc::TrySendError::{Disconnected, Full};
// App
use crate::domain::process::model::ProcessSnapShot;
use crate::domain::process::ProcessSnapShotSource;
use crate::adapters::sysinfo::sysinfo_datasource::SysinfoDataSource;

/// `Messages` from main thread to worker thread
pub enum CallerMessage {
    BuildProcessSnapShot,
    TerminateProcess(u32)
}

/// `Messages` from worker thread to main thread
pub enum WorkerMessage {
    Done(ProcessSnapShot),
    DoneTerminateProcess
}

// This structure is for `main` event loop
pub struct SysinfoWorker {
    rx: Receiver<WorkerMessage>,
    tx: SyncSender<CallerMessage>
}

impl SysinfoWorker {
    pub fn default() -> Self {
        // [5/4/26] No longer using rendezvous channels because the worker 
        // can receive multiple tasks, requiring a queue. The channel does
        // not need to be large, in a use case, at most, the channel contains
        // 1 BuildProcessSnapShot and 1 TerminateProcess(u32) message.
        const CHANNEL_CAPACITY: usize = 2;
        // This channel is used for communication from `main` thread to `worker` thread
        let (to_caller_tx, from_worker_rx) = sync_channel(CHANNEL_CAPACITY);
        // This channel is used for communication from `worker` thread to `main` thread 
        let (to_worker_tx, from_caller_rx) = sync_channel(CHANNEL_CAPACITY);

        let mut data_source = SysinfoDataSource::default();

        std::thread::spawn(move || {
            loop {
                // `recv()` function will always block the current thread if there is no data available
                // and it’s possible for more data to be sent (at least one sender still exists).
                // This is intended, the worker should wait here for more work.
                let message = from_caller_rx.recv();
                match message {
                    Ok(CallerMessage::BuildProcessSnapShot) => {
                        debug!("`Worker`: received CallerMessage::BuildProcessSnapShot");
                        
                        data_source.refresh_all();
                        let snapshot = data_source.fetch_process_snapshot();
                        match to_caller_tx.try_send(WorkerMessage::Done(snapshot)) {
                            Ok(_) => {
                                debug!("`Worker` try_send(snapshot) successful");
                            }
                            Err(try_send_err) => {
                                match try_send_err {
                                    Full(_snapshot) => {
                                        warn!("`Worker`: mpsc channel is FULL, could not send snapshot");
                                    }
                                    Disconnected(_snapshot) => {
                                        error!("`Worker`: mpsc channel is DISCONNECTED, exiting worker loop");
                                        return;
                                    }
                                }
                            }
                        }
                    }
                    Ok(CallerMessage::TerminateProcess(pid)) => {
                        debug!("`Worker`: received CallerMessage::TerminateProcess(pid)");
                        data_source.terminate_process(pid);
                        match to_caller_tx.try_send(WorkerMessage::DoneTerminateProcess) {
                            Ok(_) => {}
                            Err(try_send_err) => {
                                match try_send_err {
                                    Full(_) => {
                                        warn!("`Worker`: mpsc channel is FULL, could not send Done Terminate message");
                                    }
                                    Disconnected(_) => {
                                        error!("`Worker`: mpsc channel is DISCONNECTED, exiting worker loop...");
                                        return;
                                    }
                                }
                            }
                        }
                    }
                    // `RecvError` occurs if the channel has hung up
                    Err(_recv_err) => {
                        info!("`Worker`: Channel has hung up, exiting worker loop");
                        return;
                    }
                }
            }
        });
        // Return channel ends used by caller
        SysinfoWorker {rx: from_worker_rx, tx: to_worker_tx}
    }

    // `try_next` is not blocking
    // Return: Ok(WorkerMessage) or Err(TryRecvError::Empty) or Err(TryRecvError::Disconnected)
    pub fn try_next(&self) -> Result<WorkerMessage, TryRecvError> {
        self.rx.try_recv()
    }

    pub fn next(&self) -> Result<WorkerMessage, RecvError> {
        self.rx.recv()
    }

    // `send` is blocking
    // Returns Error if the receiver is disconnected
    pub fn send(&self, msg: CallerMessage) -> Result<(), SendError<CallerMessage>> {
        self.tx.send(msg)
    }
    
    pub fn try_send(&self, msg: CallerMessage) -> Result<(), TrySendError<CallerMessage>> {
        self.tx.try_send(msg)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn test_sysinfo_worker() {
        let worker = SysinfoWorker::default();
        let _ = worker.send(CallerMessage::BuildProcessSnapShot);

        let give_worker_time = std::time::Duration::from_millis(100);
        std::thread::sleep(give_worker_time);

        let message = worker.try_next();
        match message {
            Ok(WorkerMessage::Done(snapshot)) => {
                assert!(snapshot.count() > 0);
            }
            Ok(_) => {}
            // Worker has no work completed
            Err(mpsc::TryRecvError::Empty) => {
                assert!(false);
            }
            // Worker to caller channel closed
            Err(mpsc::TryRecvError::Disconnected) => {
                assert!(false)
            }
        }
    }
}
