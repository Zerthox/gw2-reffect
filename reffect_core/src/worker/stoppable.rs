use super::Worker;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

#[derive(Debug)]
pub struct StoppableWorker<T = ()> {
    worker: Worker<T>,
    token: Token,
}

impl<T> StoppableWorker<T> {
    pub fn spawn(name: &'static str, work: impl FnOnce(Token) -> T + Send + 'static) -> Option<Self>
    where
        T: Send + 'static,
    {
        let token = Token::new();
        let worker_token = token.clone();
        Worker::spawn(name, move || work(worker_token)).map(|worker| Self { worker, token })
    }

    pub fn exit_and_wait(self) -> Option<T> {
        self.token.request_stop();
        self.worker.wait()
    }
}

#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct Token {
    stop: Arc<AtomicBool>,
}

impl Token {
    #[inline]
    pub fn new() -> Self {
        Self {
            stop: Arc::new(AtomicBool::new(false)),
        }
    }

    #[inline]
    pub fn stop_requested(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }

    #[inline]
    pub fn request_stop(self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

impl Default for Token {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
