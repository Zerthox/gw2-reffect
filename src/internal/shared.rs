use std::mem::MaybeUninit;

/// Error codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Error {
    None = 0,
    Outdated = 1,
    NoMumble = 2,
    CompetitiveMode = 3,
    ContextNotFound = 4,
    NoCharacter = 5,
    CharacterState = 6,
    BuffsNotFound = 7,
    HealthNotFound = 8,
    SpecNotFound = 9,
    Windows = u8::MAX,
}

/// Result representing success or failures.
///
/// FFI-safe version of [`std::result::Result`].
#[derive(Debug, Clone)]
#[repr(C)]
#[must_use]
pub struct Result<T> {
    /// Whether there has been an error.
    pub error: Error,

    /// Only valid if [`Error::None`].
    pub value: T,
}

impl<T> From<std::result::Result<T, Error>> for Result<T> {
    fn from(result: std::result::Result<T, Error>) -> Self {
        match result {
            Ok(value) => Self {
                error: Error::None,
                value,
            },
            Err(error) => Self {
                error,
                value: unsafe { MaybeUninit::uninit().assume_init() },
            },
        }
    }
}

impl<T> From<Result<T>> for std::result::Result<T, Error> {
    fn from(result: Result<T>) -> Self {
        let Result { error, value } = result;
        if let Error::None = error {
            Ok(value)
        } else {
            Err(error)
        }
    }
}

/// Result returned for own character.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct SelfResult {
    /// Buffs.
    pub buffs: Result<Buffs>,

    /// Profession resources.
    pub resources: Result<Resources>,
}

/// Current buffs.
///
/// **Important:** pointers are only valid to read until the next update.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Buffs {
    /// Pointer to the buffs array.
    pub buffs: *const Buff,

    /// Length of the buffs array.
    pub len: usize,
}

/// Information about a currently applied buff.
///
/// Time related information is only given if currently visible.
/// Always visible for Boons & Conditions (border around them).
/// Visible for other effects starting from 5 seconds left (icon blinking).
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Buff {
    /// Skill id of the buff.
    pub id: u32,

    /// Category of the buff.
    pub category: Category,

    /// Whether the buff stacks in duration.
    pub duration_stacking: bool,

    /// Number of stacks or `1` if not intensity-stacking.
    pub stacks: u32,

    /// Most recent application timestamp or [`u32::MAX`] if time not visible.
    // TODO: default to 0 instead?
    pub apply_time: u32,

    /// Predicted runout timestamp or [`u32::MAX`] if time not visible.
    pub runout_time: u32,
}

/// Category of the buff.
///
/// Any category except for Boon and Condition is mapped to [`Category::Generic`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Category {
    /// Buff is a Boon.
    Boon = 0,

    /// Buff is an uncategorized effect.
    Generic = 1,

    /// Buff is a Condition.
    Condition = 2,

    /// Buff is hidden but gives a screen border.
    ScreenBorder = 3,
}

/// Information about resources.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Resources {
    /// Health.
    pub health: Resource,

    /// Barrier.
    pub barrier: Resource,

    /// Primary profession resource.
    // TODO: separate error state for profession resources?
    pub primary: Resource,

    /// Secondary profession resource.
    pub secondary: Resource,
}

impl Resources {
    /// Creates empty resources.
    pub const fn empty() -> Self {
        Self {
            health: Resource::empty(),
            barrier: Resource::empty(),
            primary: Resource::empty(),
            secondary: Resource::empty(),
        }
    }
}

/// Information about a resource.
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct Resource {
    /// Current amount.
    pub current: u32,

    /// Maximum amount.
    pub max: u32,
}

impl Resource {
    /// Creates an empty resource.
    pub const fn empty() -> Self {
        Self::new(0, 0)
    }

    /// Creates a resources with the given values.
    pub const fn new(current: u32, max: u32) -> Self {
        Self { current, max }
    }
}

/// Player traits.
pub type Traits = [u32; 9];
