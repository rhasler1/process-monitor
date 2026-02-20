use std::sync::mpsc;
use crate::domain::process::model::ProcessSnapShot;
use crate::domain::process::ProcessSnapShotSource;
use crate::adapters::sysinfo::sysinfo_datasource::SysinfoDataSource;

/// `Messages` from main thread to worker thread
pub enum CallerMessage {
    BuildProcessSnapShot,
}

/// `Messages` from worker thread to main thread
pub enum WorkerMessage {
    Done(ProcessSnapShot),
    Error(mpsc::RecvError)
}

/// This structure is for `main` event loop
pub struct SysinfoWorker {
    rx: mpsc::Receiver<WorkerMessage>,
    tx: mpsc::SyncSender<CallerMessage>
}

impl SysinfoWorker {
    pub fn default() -> Self {
        /// 2/18/2026
        /// Two rendezvous channel's (0 buffer size) are created: `tx.send` is blocking and will not return until a recv is paried with it.
        /// 
        /// Why this works for SysinfoWorker:
        /// - The Worker only has one task if the caller is not ready to receive than worker blocking is OK (it has no other work todo).
        ///
        /// - The `caller` sends the `worker` messages on RefreshEvents; the duration between RefreshEvents
        /// is much greater than the Duration the worker thread spends processing a message and completing it's `work`
        /// --meaning the Caller should never block on send. There is a `tx.try_send()` function
        /// that is a non-blocking send--this is something that can be experimented with.
        const CHANNEL_SIZE: usize = 0;
        /// This channel is used for communication from `main` thread to `worker` thread
        let (to_caller_tx, from_worker_rx) = mpsc::sync_channel(CHANNEL_SIZE);
        /// This channel is used for communication from `worker` thread to `main` thread 
        let (to_worker_tx, from_caller_rx) = mpsc::sync_channel(CHANNEL_SIZE);

        let mut data_source = SysinfoDataSource::default();

        std::thread::spawn(move || {
            loop {
                /// `recv()` function will always block the current thread if there is no data available
                /// and it’s possible for more data to be sent (at least one sender still exists).
                /// This is intended, the worker should wait here for more work.
                let message = from_caller_rx.recv();
                match message {
                    Ok(CallerMessage::BuildProcessSnapShot) => {
                        /// Refresh data source & build new domain models
                        data_source.refresh_all();
                        let snapshot = data_source.fetch_process_snapshot();
                        /// `send` returns error if the channel is disconnected; thread is
                        /// terminated in this case
                        if to_caller_tx.send(WorkerMessage::Done(snapshot)).is_err() {
                            return;
                        }
                    } // `RecvError` occurs if the channel has hung up
                    Err(mpsc::RecvError) => {
                        /// If `caller` => `worker` mpsc channel is disconnected; attempt to notify
                        /// caller via `worker` => `caller` mpsc channel. The worker thread is terminated if
                        /// the `notification` fails.
                        if to_caller_tx.send(WorkerMessage::Error(mpsc::RecvError)).is_err() {
                            return;
                        }
                    }
                }
            }
        });
        /// Return channel ends used by caller
        SysinfoWorker {rx: from_worker_rx, tx: to_worker_tx}
    }

    /// `try_next` is not blocking
    /// Return: Ok(WorkerMessage) or Err(TryRecvError::Empty) or Err(TryRecvError::Disconnected)
    pub fn try_next(&self) -> anyhow::Result<WorkerMessage, mpsc::TryRecvError> {
        self.rx.try_recv()
    }

    pub fn next(&self) -> anyhow::Result<WorkerMessage, mpsc::RecvError> {
        self.rx.recv()
    }

    /// `send` is blocking
    /// Returns Error if the receiver is disconnected
    pub fn send(&self, msg: CallerMessage) -> anyhow::Result<(), mpsc::SendError<CallerMessage>> {
        self.tx.send(msg)
    }
}

#[cfg(test)]
pub mod test {
    use std::sync::mpsc;
    use crate::core::process::model::ProcessSnapShot;
    use crate::core::process::ProcessSnapShotSource;
    use crate::adapters::sysinfo::sysinfo_datasource::SysinfoDataSource;
    use super::{CallerMessage,WorkerMessage,SysinfoWorker};    

    #[test]
    fn test_sysinfo_worker() {
        let worker = SysinfoWorker::default();
        let _ = worker.tx.send(CallerMessage::BuildProcessSnapShot);
        /// INFO 2/18/2026 - experiment with `give_worker_time` to get an idea of how
        /// long it takes for the worker to complete it's task. Err(...Empty) will 
        /// execute if worker did not complete it's task in the time alotted.
        let give_worker_time = std::time::Duration::from_millis(10);
        std::thread::sleep(give_worker_time);

        let message = worker.try_next();
        match message {
            Ok(WorkerMessage::Done(snapshot)) => {
                assert!(snapshot.count() > 0);
            }
            // Caller to worker channel closed
            Ok(WorkerMessage::Error(mpsc::RecvError)) => {
                assert!(false);
            }
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
