use crate::{
    fmt::Pretty,
    internal::{Ability, Buff, Recharge, Resource, Skillbar},
};

#[derive(Debug, Clone)]
pub enum ProgressActive {
    Fixed {
        current: u32,
        max: u32,
    },
    Buff {
        id: u32,
        stacks: u32,
        duration: u32,
        end: u32,
    },
    Ability {
        id: u32,
        ammo: u32,
        rate: f32,
        duration: u32,
        end: u32,
        ammo_duration: u32,
        ammo_end: u32,
    },
}

impl ProgressActive {
    /// Creates a dummy active progress.
    pub const fn dummy() -> Self {
        Self::Fixed { current: 1, max: 1 }
    }

    /// Creates an empty timed active progress.
    pub const fn empy_buff(id: u32) -> Self {
        Self::Buff {
            id,
            stacks: 0,
            duration: 0,
            end: 0,
        }
    }

    /// Creates new timed active progress from a buff.
    pub fn from_buff(id: u32, buff: &Buff) -> Self {
        Self::Buff {
            id,
            stacks: buff.stacks,
            duration: if buff.is_infinite() {
                u32::MAX
            } else {
                Self::time_between(buff.apply_time, buff.runout_time)
            },
            end: buff.runout_time,
        }
    }

    /// Creates new timed active progress from a recharge.
    pub fn from_recharge(recharge: &Recharge) -> Self {
        let duration = recharge.recharge;
        Self::Ability {
            id: 0,
            ammo: if duration > 0 { 1 } else { 0 },
            duration,
            end: recharge.end(),
            ammo_duration: 0,
            ammo_end: 0,
            rate: 1.0,
        }
    }

    /// Creates new timed active progress from a skillbar and ability.
    pub fn from_ability(skillbar: &Skillbar, ability: &Ability) -> Self {
        Self::Ability {
            id: ability.id,
            ammo: ability.ammo,
            duration: ability.recharge,
            end: skillbar.last_update
                + Self::unscale(ability.recharge_remaining, skillbar.recharge_rate),
            ammo_duration: ability.ammo_recharge,
            ammo_end: skillbar.last_update
                + Self::unscale(ability.ammo_recharge_remaining, skillbar.recharge_rate),
            rate: skillbar.recharge_rate,
        }
    }

    /// Creates a resource progress for edit mode.
    pub const fn edit_resource(progress: f32, max: u32) -> Self {
        Self::Fixed {
            current: (progress * max as f32) as u32,
            max,
        }
    }

    /// Creates a uff progress for edit mode.
    pub const fn edit_buff(id: u32, progress: f32, now: u32) -> Self {
        Self::Buff {
            id,
            stacks: (25.0 * progress) as u32,
            duration: 5000,
            end: now + (5000.0 * progress) as u32,
        }
    }

    /// Creates an ability progress for edit mode.
    pub const fn edit_ability(id: u32, progress: f32, now: u32) -> Self {
        // half speed
        let slow = if progress < 0.5 {
            2.0 * progress
        } else {
            2.0 * progress - 1.0
        };
        Self::Ability {
            id,
            ammo: (5.0 * progress) as u32,
            duration: 5000,
            end: now + (5000.0 * progress) as u32,
            rate: 1.0,
            ammo_duration: 10_000,
            ammo_end: now + (10000.0 * slow) as u32,
        }
    }

    pub const fn id(&self) -> Option<u32> {
        match *self {
            Self::Fixed { .. } => None,
            Self::Buff { id, .. } | Self::Ability { id, .. } => Some(id),
        }
    }

    /// Whether the progress uses timestamps.
    pub const fn is_timed(&self) -> bool {
        matches!(self, Self::Buff { .. } | Self::Ability { .. })
    }

    /// Returns the intensity (alternative progress).
    pub const fn intensity(&self) -> u32 {
        match *self {
            Self::Fixed { current, .. } => current,
            Self::Buff { stacks, .. } => stacks,
            Self::Ability { ammo, .. } => ammo,
        }
    }

    /// Returns the current progress rate.
    pub fn progress_rate(&self) -> f32 {
        match *self {
            Self::Fixed { .. } | Self::Buff { .. } => 1.0,
            Self::Ability { rate, .. } => rate,
        }
    }

    /// Returns the current progress between `0.0` and `1.0`.
    pub fn progress(&self, value: ProgressValue, now: u32) -> Option<f32> {
        self.current(value, now).map(|current| {
            match (current, self.max(value)) {
                (0, 0) => 0.0, // treat 0/0 as 0% progress
                (_, 0) => 1.0, // treat x/0 as 100% progress
                (current, max) => current as f32 / max as f32,
            }
        })
    }

    /// Returns the current progress between `0.0` and `1.0`.
    pub fn progress_or_default(&self, value: ProgressValue, now: u32) -> f32 {
        self.progress(value, now).unwrap_or(1.0)
    }

    /// Returns the current amount in its native unit.
    pub fn current(&self, value: ProgressValue, now: u32) -> Option<u32> {
        match *self {
            Self::Fixed { current, .. } => Some(current),
            Self::Buff { end, .. } => (end != u32::MAX).then(|| Self::time_between(now, end)),
            Self::Ability {
                end,
                ammo_end,
                rate,
                ..
            } => {
                let end = match value {
                    ProgressValue::Primary => end,
                    ProgressValue::Secondary => ammo_end,
                };
                Some(Self::time_between_scaled(now, end, rate))
            }
        }
    }

    /// Returns the current amount as text.
    pub fn current_text(&self, value: ProgressValue, now: u32, pretty: bool) -> String {
        match *self {
            Self::Fixed { current, .. } => Pretty::string_if(current, pretty),
            Self::Buff { end, .. } => {
                if end == u32::MAX {
                    "?".into()
                } else {
                    Self::duration_text(Self::time_between(now, end))
                }
            }
            Self::Ability {
                end,
                ammo_end,
                rate,
                ..
            } => {
                let end = match value {
                    ProgressValue::Primary => end,
                    ProgressValue::Secondary => ammo_end,
                };
                Self::duration_text(Self::time_between_scaled(now, end, rate))
            }
        }
    }

    /// Returns the maximum amount in its native unit.
    pub fn max(&self, value: ProgressValue) -> u32 {
        match *self {
            Self::Fixed { max, .. } => max,
            Self::Buff { duration, .. } => duration,
            Self::Ability {
                duration,
                ammo_duration,
                ..
            } => match value {
                ProgressValue::Primary => duration,
                ProgressValue::Secondary => ammo_duration,
            },
        }
    }

    /// Returns the maximum amount as text.
    pub fn max_text(&self, value: ProgressValue, pretty: bool) -> String {
        match *self {
            Self::Fixed { max, .. } => Pretty::string_if(max, pretty),
            Self::Buff { duration, .. } => {
                if duration != u32::MAX {
                    Self::format_seconds(duration)
                } else {
                    "?".into()
                }
            }
            Self::Ability {
                duration,
                ammo_duration,
                ..
            } => Self::format_seconds(match value {
                ProgressValue::Primary => duration,
                ProgressValue::Secondary => ammo_duration,
            }),
        }
    }

    fn time_between(start: u32, end: u32) -> u32 {
        end.saturating_sub(start)
    }

    fn time_between_scaled(start: u32, end: u32, rate: f32) -> u32 {
        (Self::time_between(start, end) as f32 * rate) as u32
    }

    fn unscale(time: u32, rate: f32) -> u32 {
        (time as f32 / rate) as u32
    }

    fn format_seconds(time: u32) -> String {
        format!("{:.1}", time as f32 / 1000.0)
    }

    fn duration_text(time: u32) -> String {
        if time > 0 {
            Self::format_seconds(time)
        } else {
            String::new()
        }
    }
}

impl TryFrom<Resource> for ProgressActive {
    type Error = ();

    fn try_from(resource: Resource) -> Result<Self, Self::Error> {
        let Resource { current, max } = resource;
        if max != 0 {
            Ok(Self::Fixed { current, max })
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProgressValue {
    Primary,
    Secondary,
}
