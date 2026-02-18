use std::sync::mpsc;
use crate::core::process::model::ProcessSnapShot;
use crate::core::process::ProcessSnapShotSource;
use crate::adapters::sysinfo::sysinfo_datasource::SysinfoDataSource;

/// `Messages` from main thread to worker thread
pub enum CallerMessage {
    BuildProcessSnapShot,
    //Terminate // TODO 2/17/2026 - When implementing terminate
}

/// `Messages` from worker thread to main thread
pub enum WorkerMessage {
    Done(ProcessSnapShot),
    //Err(String) // 2/18/2026 - SysinfoDataSource cannot return an err
}

/// This structure is for `main` event loop
pub struct SysinfoWorker {
    rx: mpsc::Receiver<WorkerMessage>,
    tx: mpsc::SyncSender<CallerMessage>
}

impl SysinfoWorker {
    pub fn default() -> Self {
        /// TODO 2/18/2026 Revise this documentation -
        /// Rendezvous channel -  send will not return until a recv is paried with it.
        /// 
        /// Why this works for sysinfoworker:
        /// The Worker only has one task if the caller is not ready to receive than
        /// worker blocking is OK.
        ///
        /// The Caller sends messages on RefreshEvents; the duration
        /// between RefreshEvents is much greater than the Duration the worker thread spends
        /// processing a message and completing it's `work`--meaning the Caller should never
        /// block on send.
        const CHANNEL_SIZE: usize = 0;
        /// This channel is used for communication from `main` thread to `worker` thread
        let (to_caller_tx, from_worker_rx) = mpsc::sync_channel(CHANNEL_SIZE);
        /// This channel is used for communication from `worker` thread to `main` thread 
        let (to_worker_tx, from_caller_rx) = mpsc::sync_channel(CHANNEL_SIZE);

        let mut data_source = SysinfoDataSource::default();

        std::thread::spawn(move || {
            loop {
                // poll message from caller
                // `recv()` function will always block the current thread if there is no data available and it’s possible for more data to be sent (at least one sender still exists)
                let message = from_caller_rx.recv();
                match message {
                    Ok(CallerMessage::BuildProcessSnapShot) => {
                        data_source.refresh_all();
                        let snapshot = data_source.fetch_process_snapshot();
                        // `send` will only error if the receiving end of the channel
                        // has been disconnected
                        if to_caller_tx.send(WorkerMessage::Done(snapshot)).is_err() {
                            // return terminates the thread
                            return;
                        }
                    } // Err here only occurs if the channel has hung up
                    Err(_) => {
                        // TODO 2/18/2026 - terminate thread; there might be a better option.
                        // This occurs when to_worker_tx,from_caller_rx channel has hung up, to
                        // caller_tx, from_worker_rx channel might still be operational; in this
                        // case a return message could be sent
                        // What can problem be done is to try and send a message to the caller
                        // that the channel communication from caller -> Recv has hung up,
                        // if this send fails, then terminate the thread
                        return;
                    }
                }
            }
        });

        // Return channel ends used by caller
        SysinfoWorker {rx: from_worker_rx, tx: to_worker_tx}
    }

    /// `try_next` is not blocking
    /// Return: Ok(WorkerMessage) or Err(TryRecvError::Empty) or Err(TryRecvError::Disconnected)
    pub fn try_next(&self) -> anyhow::Result<WorkerMessage, mpsc::TryRecvError> {
        self.rx.try_recv()
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
        worker.tx.send(CallerMessage::BuildProcessSnapShot);
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
            Err(mpsc::TryRecvError::Empty) => {
                assert!(false);
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                assert!(false)
            }
        }
    }
}
// SIGNOFF 2/18/2026 - initial implementation & testing of sysinfo worker is
// almost complete. Check TODO
