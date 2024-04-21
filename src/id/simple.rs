use super::{GenerateId, IdGen};
use std::{
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};

const INITIAL: usize = 1;

static ID: AtomicUsize = AtomicUsize::new(INITIAL);

impl GenerateId for IdGen {
    type Id = usize;

    fn nil() -> Self::Id {
        0
    }

    fn generate() -> Self::Id {
        ID.fetch_add(1, Ordering::Relaxed)
    }

    fn reset() {
        ID.store(INITIAL, Ordering::Relaxed)
    }

    fn display(id: Self::Id) -> impl fmt::Display {
        id
    }
}
