use std::result;
use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

/// Error information.
#[derive(Debug, Clone, Default, Error)]
pub enum Error {
    #[default]
    #[error("Disabled")]
    Disabled,

    #[error("No Mumble link")]
    NoMumble,

    #[error("Unavailable in competitive mode")]
    CompetitiveMode,

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

    #[error("Character gear not found")]
    GearNotFound,

    #[error("Character specialization not found")]
    SpecNotFound,

    #[error("No target")]
    NoTarget,

    #[error("Unavailable for target state")]
    TargetState,

    #[error("Windows error: {0}")]
    Windows(windows::core::Error),
}
