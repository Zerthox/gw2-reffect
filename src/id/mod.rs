#[cfg_attr(feature = "guid", path = "guid.rs")]
mod simple;

use core::fmt;
use std::hash::Hash;

/// Helper to generate ids.
///
/// The backing id implementation is selected using features.
pub struct IdGen;

impl IdGen {
    pub fn nil() -> Id {
        Id(<Self as GenerateId>::nil())
    }

    pub fn generate() -> Id {
        Id(<Self as GenerateId>::generate())
    }
}

/// Opaque wrapper around an id type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Id(<IdGen as GenerateId>::Id);

impl Default for Id {
    fn default() -> Self {
        IdGen::nil()
    }
}

impl fmt::Display for Id {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        IdGen::display(self.0).fmt(formatter)
    }
}

/// Helper interface for implementing id generation.
trait GenerateId {
    type Id: fmt::Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash;

    fn nil() -> Self::Id;

    fn generate() -> Self::Id;

    fn display(id: Self::Id) -> impl fmt::Display;
}
