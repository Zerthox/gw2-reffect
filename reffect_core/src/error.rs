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

    #[error("No handler")]
    NoHandler,

    #[error("No Mumble link")]
    NoMumble,

    #[error("Unavailable in competitive mode")]
    CompetitiveMode,

    #[error("Main thread not found")]
    MainThreadNotFound,

    #[error("Context not found")]
    ContextNotFound,

    #[error("World not found")]
    WorldNotFound,

    #[error("Content not found")]
    ContentNotFound,

    #[error("Skill not found")]
    SkillNotFound,

    #[error("Failed to get user")]
    UserNotFound,

    #[error("No character")]
    NoCharacter,

    #[error("Unavailable for character state")]
    CharacterState,

    #[error("Character buffs not found")]
    BuffsNotFound,

    #[error("Character skillbar not found")]
    SkillbarNotFound,

    #[error("Character health not found")]
    HealthNotFound,

    #[error("Character endurance not found")]
    EnduranceNotFound,

    #[error("Character inventory not found")]
    InventoryNotFound,

    #[error("Character profession not found")]
    ProfNotFound,

    #[error("Character specialization not found")]
    SpecNotFound,

    #[error("Kennel not found")]
    KennelNotFound,

    #[error("No target")]
    NoTarget,

    #[error("Unavailable for target state")]
    TargetState,

    #[error("Group not found")]
    GroupNotFound,

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
