use strum::{AsRefStr, Display, IntoStaticStr};

/// Information about a skill.
#[derive(Debug, Clone)]
pub enum SkillInfo {
    /// Ability.
    Ability,

    /// Buff.
    Buff {
        /// Category of the buff.
        category: Category,

        /// Stacking behavior of the buff.
        stacking: Stacking,
    },
}

/// Category of the buff.
///
/// Any category except for Boon and Condition is mapped to [`Category::Effect`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, AsRefStr, IntoStaticStr,
)]
pub enum Category {
    /// Buff is a Boon.
    Boon = 0,

    /// Buff is an uncategorized effect.
    Effect = 1,

    /// Buff is a Condition.
    Condition = 2,

    /// Buff is hidden but gives a screen border.
    #[strum(serialize = "Screen Border")]
    ScreenBorder = 3,
}

/// Stacking behavior of the buff.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Display, AsRefStr, IntoStaticStr,
)]
pub enum Stacking {
    // Other/unknown stacking type.
    Other,

    /// Buff stacks in intenstity.
    Intensity,

    /// Buff stacks in duration.
    Duration,
}
