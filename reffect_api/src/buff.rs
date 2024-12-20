use std::collections::BTreeMap;

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

    /// Most recent application timestamp or `0` if time not visible.
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

    /// Checks whether the buff is infinite duration.
    #[inline]
    pub const fn is_infinite(&self) -> bool {
        self.runout_time == u32::MAX
    }

    /// Returns the total buff duration in milliseconds.
    #[inline]
    pub const fn duration(&self) -> u32 {
        self.runout_time - self.apply_time
    }

    /// Returns the remaining buff duration in milliseconds.
    #[inline]
    pub const fn remaining(&self, now: u32) -> u32 {
        if self.is_infinite() {
            u32::MAX
        } else {
            self.runout_time.saturating_sub(now)
        }
    }

    /// Returns the buff progress.
    #[inline]
    pub const fn progress(&self, now: u32) -> f32 {
        self.remaining(now) as f32 / self.duration() as f32
    }
}
