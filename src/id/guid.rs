use super::{GenerateId, IdGen};
use std::fmt;
use uuid::Uuid;

impl GenerateId for IdGen {
    type Id = Uuid;

    fn nil() -> Self::Id {
        Uuid::nil()
    }

    fn generate() -> Self::Id {
        Uuid::new_v4()
    }

    fn reset() {}

    fn display(id: Self::Id) -> impl fmt::Display {
        id.simple()
    }
}
