use thiserror::Error;

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

    #[error("No character")]
    NoCharacter,

    #[error("Unavailable for character state")]
    CharacterState,

    #[error("Failed to get character buffs")]
    BuffsNotFound,

    #[error("Failed to get character health")]
    HealthNotFound,

    #[error("Failed to get character endurance")]
    EnduranceNotFound,

    #[error("Failed to get character gear")]
    GearNotFound,

    #[error("Failed to get character specializations")]
    SpecNotFound,

    #[error("Windows error: {0}")]
    Windows(windows::core::Error),
}
