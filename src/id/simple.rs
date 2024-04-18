use super::{GenerateId, IdGen};
use std::{
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};

impl GenerateId for IdGen {
    type Id = usize;

    fn nil() -> Self::Id {
        0
    }

    fn generate() -> Self::Id {
        static ID: AtomicUsize = AtomicUsize::new(1);

        ID.fetch_add(1, Ordering::Relaxed)
    }

    fn display(id: Self::Id) -> impl fmt::Display {
        id
    }
}
