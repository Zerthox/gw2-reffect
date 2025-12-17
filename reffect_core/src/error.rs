use microseh::{Exception, ExceptionCode};
use std::result;
use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

/// Error information.
#[derive(Debug, Clone, Default, Error)]
pub enum Error {
    #[default]
    #[error("Disabled")]
    Disabled,

    #[error("Not initialized")]
    NotInitialized,

    #[error("No handler")]
    Handler,

    #[error("No Mumble link")]
    Mumble,

    #[error("Unavailable in competitive mode")]
    CompetitiveMode,

    #[error("Main thread not found")]
    MainThread,

    #[error("Context not found")]
    Context,

    #[error("World not found")]
    World,

    #[error("Content not found")]
    Content,

    #[error("Skill not found")]
    Skill,

    #[error("Item not found")]
    Item,

    #[error("Failed to get user")]
    User,

    #[error("Character not found")]
    Character,

    #[error("Unavailable for character state")]
    CharacterState,

    #[error("Character buffs not found")]
    Buffs,

    #[error("Character skillbar not found")]
    Skillbar,

    #[error("Character health not found")]
    Health,

    #[error("Character endurance not found")]
    Endurance,

    #[error("Character inventory not found")]
    Inventory,

    #[error("Character profession not found")]
    Profession,

    #[error("Character specialization not found")]
    Specialization,

    #[error("Character build not found")]
    Build,

    #[error("Kennel not found")]
    Kennel,

    #[error("Pet not found")]
    Pet,

    #[error("Owner not found")]
    Owner,

    #[error("Target not found")]
    Target,

    #[error("Unavailable for target state")]
    TargetState,

    #[error("Group not found")]
    Group,

    #[error("Windows error: {0}")]
    Windows(windows::core::Error),

    #[error("Exception at {address:?}: {code}")]
    Exception {
        code: ExceptionCode,
        address: *mut (),
    },
}

unsafe impl Send for Error {}

impl From<Exception> for Error {
    #[inline]
    fn from(exception: Exception) -> Self {
        Self::Exception {
            code: exception.code(),
            address: exception.address().cast(),
        }
    }
}
