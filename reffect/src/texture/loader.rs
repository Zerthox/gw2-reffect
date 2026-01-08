use super::{TextureManager, TextureSource};
use crate::worker::Worker;
use std::sync::mpsc;

#[derive(Debug)]
pub(super) struct TextureLoader {
    worker: Worker,
    sender: mpsc::Sender<TextureSource>,
}

impl TextureLoader {
    pub fn spawn() -> Option<Self> {
        let (sender, receiver) = mpsc::channel();
        let worker = Worker::spawn("reffect-texture-loader", move || {
            while let Ok(source) = receiver.recv()
                && TextureManager::is_active()
            {
                TextureManager::load_source(source);
            }
        })?;
        Some(Self { worker, sender })
    }

    pub fn send(&self, source: TextureSource) -> bool {
        self.sender
            .send(source)
            .inspect_err(|_| log::error!("Texture loader receiver disconnected"))
            .is_ok()
    }

    pub fn exit_and_wait(self) {
        let Self { worker, sender } = self;
        drop(sender);
        worker.wait();
    }
}
