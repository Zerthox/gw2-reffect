#![allow(unused)]

/// Result returned.
///
/// **Important:** the information is only valid to read until the next update.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct SelfResult {
    /// Whether there has been an error.
    pub error: Error,

    /// Pointer to the buffs array.
    pub buffs: *const Buff,

    /// Length of the buffs array.
    pub len: usize,

    /// Profession resources.
    pub resources: Resources,
}

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
    ResourceNotFound = 8,
    Windows = u8::MAX,
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

/// Information about profession resources.
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct Resources {
    /// Primary profession resource.
    pub primary: Resource,

    /// Secondary profession resource.
    pub secondary: Resource,
}

/// Information about a profession resource.
#[derive(Debug, Clone, Default)]
#[repr(C)]
pub struct Resource {
    /// Current amount.
    pub current: u32,

    /// Maximum amount.
    pub max: u32,
}
