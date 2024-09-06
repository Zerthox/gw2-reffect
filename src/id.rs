use std::{
    fmt,
    sync::atomic::{AtomicU32, Ordering},
};

/// Id generator.
#[derive(Debug)]
#[repr(transparent)]
pub struct IdGen(AtomicU32);

impl IdGen {
    const INITIAL: u32 = 1;

    /// Creates a new id generator.
    pub const fn new() -> Self {
        Self(AtomicU32::new(Self::INITIAL))
    }

    /// Generates a fresh id.
    pub fn generate(&self) -> Id {
        Id(self.0.fetch_add(1, Ordering::Relaxed))
    }

    /// Resets the id generator.
    pub fn reset(&self) {
        self.0.store(Self::INITIAL, Ordering::Relaxed)
    }
}

/// Opaque wrapper around an id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Id(u32);

impl Id {
    /// Null element id.
    pub const NIL: Self = Self(0);

    /// Returns whether the id is nil.
    pub fn is_nil(&self) -> bool {
        *self == Self::NIL
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::NIL
    }
}

impl fmt::Display for Id {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}
