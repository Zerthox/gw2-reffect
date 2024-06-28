#![allow(unused)]

/// Result returned.
///
/// **Important:** the array is only valid to read until the next update.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct BuffsResult {
    /// Whether there has been an error.
    pub error: bool, // TODO: error enum

    /// Pointer to the buffs array.
    pub buffs: *mut Buff,

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

    /// Number of stacks or `1` if not intensity-stacking.
    pub stacks: u32,

    /// Initially applied duration of the buff or [`u32::MAX`] if time not visible.
    pub duration: u32,

    /// Predicted runout timestamp or [`u32::MAX`] if time not visible.
    pub runout_time: u32,
}

/// Category of the buff.
///
/// Any category except for Boon and Condition is mapped to [`Category::Generic`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub enum Category {
    /// Buff is a Boon.
    Boon = 0,

    /// Buff is an uncategorized effect.
    Generic = 1,

    /// Buff is a Condition.
    Condition = 2,
}
