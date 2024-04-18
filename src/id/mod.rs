use core::fmt;

#[cfg_attr(feature = "guid", path = "guid.rs")]
mod simple;

pub trait GenerateId {
    type Id;

    fn nil() -> Self::Id;

    fn generate() -> Self::Id;

    fn display(id: Self::Id) -> impl fmt::Display;
}

/// Helper to generate ids.
///
/// The backing id implementation is selected using features.
pub struct IdGen;

pub type Id = <IdGen as GenerateId>::Id;

impl IdGen {
    pub fn nil() -> Id {
        <Self as GenerateId>::nil()
    }

    pub fn generate() -> Id {
        <Self as GenerateId>::generate()
    }

    pub fn display(id: Id) -> impl fmt::Display {
        <Self as GenerateId>::display(id)
    }
}
