use std::collections::{BTreeMap, HashMap};

pub type BuffMap = BTreeMap<u32, Buff>;

/// Currently applied buff.
///
/// Times are only given if currently visible.
/// Always visible for Boons & Conditions (border around them).
/// Visible for other effects starting from 5 seconds left (icon blinking).
#[derive(Debug, Clone)]
pub struct Buff {
    /// Number of stacks or `1` if not intensity-stacking.
    pub stacks: u32,

    /// Most recent application timestamp or [`u32::MAX`] if time not visible.
    // TODO: default to 0 instead?
    pub apply_time: u32,

    /// Predicted runout timestamp or [`u32::MAX`] if time not visible.
    pub runout_time: u32,
}

impl Buff {
    #[inline]
    pub const fn empty() -> Self {
        Self {
            stacks: 0,
            apply_time: 0,
            runout_time: 0,
        }
    }

    #[inline]
    pub const fn duration(&self) -> u32 {
        self.runout_time - self.apply_time
    }
}

pub type BuffInfoMap = HashMap<u32, BuffInfo>;

/// Information about a buff.
#[derive(Debug, Clone)]
pub struct BuffInfo {
    /// Id of the buff.
    pub id: u32,

    /// Category of the buff.
    pub category: Category,

    /// Stacking behavior of the buff.
    pub stacking: Stacking,
}

/// Category of the buff.
///
/// Any category except for Boon and Condition is mapped to [`Category::Generic`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

/// Stacking behavior of the buff.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Stacking {
    // Other/unknown stacking type.
    Other,

    /// Buff stacks in intenstity.
    Intensity,

    /// Buff stacks in duration.
    Duration,
}
