/// Information about a currently applied buff.
///
/// Time related information is only given if currently visible.
/// Always visible for Boons & Conditions (border around them).
/// Visible for other effects starting from 5 seconds left (icon blinking).
#[derive(Debug, Clone)]
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

impl Buff {
    #[inline]
    pub const fn duration(&self) -> u32 {
        self.runout_time - self.apply_time
    }
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
