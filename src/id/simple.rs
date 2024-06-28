use super::{GenerateId, IdGen};
use std::{
    fmt,
    sync::atomic::{AtomicU32, Ordering},
};

// TODO: reclaim on drop if previous id?

const INITIAL: u32 = 1;

static ID: AtomicU32 = AtomicU32::new(INITIAL);

impl GenerateId for IdGen {
    type Id = u32;

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
