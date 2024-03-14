#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, strum::Display)]
pub enum GetBuffsError {
    #[strum(to_string = "received null pointer")]
    Null,
    #[strum(to_string = "{err}")]
    Internal { err: RawError },
}

impl From<RawError> for GetBuffsError {
    fn from(err: RawError) -> Self {
        Self::Internal { err }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    num_enum::TryFromPrimitive,
    num_enum::IntoPrimitive,
    strum::Display,
)]
#[repr(i32)]
pub enum RawError {
    #[strum(to_string = "character context not found")]
    CharacterContextNotFound = -1,

    #[strum(to_string = "inactive in competitive mode")]
    CompetitiveMode = -2,

    #[strum(to_string = "current character not set")]
    NoCharacter = -3,

    #[strum(to_string = "error while iterating through buff table")]
    BuffIterationError = -4,

    #[strum(to_string = "failed to take threads snapshot")]
    ThreadsSnapshotError = -11,

    #[strum(to_string = "failed to load ntdll.dll")]
    DllLoadError = -12,

    #[strum(to_string = "failed to find NtQueryInformationThread")]
    QueryInformationThreadError = -13,

    #[strum(to_string = "no matching thread found")]
    NoMatchingThread = -14,
}
