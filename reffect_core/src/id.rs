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
    #[inline]
    pub const fn new() -> Self {
        Self(AtomicU32::new(Self::INITIAL))
    }

    /// Generates a fresh id.
    #[inline]
    pub fn generate(&self) -> Id {
        Id(self.0.fetch_add(1, Ordering::Relaxed))
    }

    /// Resets the id generator.
    #[inline]
    pub fn reset(&self) {
        self.0.store(Self::INITIAL, Ordering::Relaxed)
    }

    /// Attempts to reclaim the id.
    /// Returns `true` on success.
    #[inline]
    pub fn try_reclaim(&self, id: Id) -> bool {
        let Id(val) = id;
        self.0
            .compare_exchange(val + 1, val, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
    }
}

impl Default for IdGen {
    #[inline]
    fn default() -> Self {
        Self::new()
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
    #[inline]
    pub fn is_nil(&self) -> bool {
        *self == Self::NIL
    }
}

impl Default for Id {
    #[inline]
    fn default() -> Self {
        Self::NIL
    }
}

impl fmt::Display for Id {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}
