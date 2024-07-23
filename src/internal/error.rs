use super::shared::Error as InternalError;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum Error {
    #[error("Failed to extract internal")]
    Extract,

    #[error("Failed to load internal")]
    Load,

    #[error("{0}")]
    Internal(InternalError),
}

impl Default for Error {
    fn default() -> Self {
        Self::Internal(InternalError::None)
    }
}

impl From<InternalError> for Error {
    fn from(value: InternalError) -> Self {
        Self::Internal(value)
    }
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Outdated => write!(f, "Outdated"),
            Self::NoMumble => write!(f, "No mumble"),
            Self::CompetitiveMode => write!(f, "Unavailable in competitive modes"),
            Self::ContextNotFound => write!(f, "Context not found"),
            Self::NoCharacter => write!(f, "No character"),
            Self::CharacterState => write!(f, "Unavailable for current character state"),
            Self::BuffsNotFound => write!(f, "Failed go get character effects"),
            Self::HealthNotFound => write!(f, "Failed go get character health"),
            Self::SpecNotFound => write!(f, "Failed go get character specs"),
            Self::Windows => write!(f, "Windows error"),
        }
    }
}
