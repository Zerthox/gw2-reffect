mod stoppable;

use std::thread::{self, JoinHandle};

pub use self::stoppable::*;

#[derive(Debug)]
pub struct Worker<T = ()> {
    name: &'static str,
    handle: JoinHandle<T>,
}

impl<T> Worker<T> {
    pub fn spawn(name: &'static str, work: impl FnOnce() -> T + Send + 'static) -> Option<Self>
    where
        T: Send + 'static,
    {
        thread::Builder::new()
            .name(name.into())
            .spawn(move || {
                log::debug!("Thread {name} spawn");
                let result = work();
                log::debug!("Thread {name} exit");
                result
            })
            .inspect_err(|err| log::error!("Failed to spawn {name}: {err}"))
            .ok()
            .map(|handle| Self { name, handle })
    }

    pub fn wait(self) -> Option<T> {
        self.handle
            .join()
            .inspect_err(|_| log::error!("Failed to join {}", self.name))
            .ok()
    }
}
