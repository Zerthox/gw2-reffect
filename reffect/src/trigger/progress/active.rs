use super::ProgressValue;
use crate::{
    context::{Ability, AbilityState, Buff, Resource, ResourceType, SkillId},
    fmt::{Time, Unit},
    settings::FormatSettings,
};
use enumflags2::BitFlags;

#[derive(Debug, Clone)]
pub enum ProgressActive {
    Resource {
        current: f32,
        max: f32,
        resource: ResourceType,
    },
    Buff {
        id: u32,
        stacks: u32,
        duration: u32,
        end: u32,
    },
    Ability {
        id: SkillId,
        ammo: u32,
        rate: f32,
        recharge: u32,
        end: u32,
        ammo_recharge: u32,
        ammo_end: u32,
        state: BitFlags<AbilityState>,
    },
}

impl ProgressActive {
    /// Creates a dummy active progress for always trigger.
    pub const fn always() -> Self {
        Self::Resource {
            current: 1.0,
            max: 1.0,
            resource: ResourceType::DEFAULT,
        }
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

    /// Creates simple progress percent.
    pub const fn percent(current: f32, resource_type: ResourceType) -> Self {
        Self::Resource {
            current,
            max: 100.0,
            resource: resource_type,
        }
    }

    /// Creates simple progress percent.
    pub const fn from_resource(resource: &Resource, resource_type: ResourceType) -> Option<Self> {
        let Resource { current, max } = *resource;
        if max != 0.0 {
            Some(Self::Resource {
                current,
                max,
                resource: resource_type,
            })
        } else {
            None
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

    /// Creates new timed active progress from an ability.
    pub fn from_ability(ability: &Ability) -> Self {
        let Ability {
            id,
            ammo,
            last_update,
            recharge_rate,
            recharge,
            recharge_remaining,
            ammo_recharge,
            ammo_recharge_remaining,
            state,
            ..
        } = *ability;
        Self::Ability {
            id,
            ammo,
            recharge,
            end: if recharge > 0 {
                last_update + Self::unscale(recharge_remaining, recharge_rate)
            } else {
                0
            },
            ammo_recharge,
            ammo_end: if ammo_recharge > 0 {
                last_update + Self::unscale(ammo_recharge_remaining, recharge_rate)
            } else {
                0
            },
            rate: recharge_rate,
            state,
        }
    }

    /// Creates a resource progress for edit mode.
    pub fn edit_resource(progress: f32, max: f32, resource_type: ResourceType) -> Self {
        Self::Resource {
            current: (progress * max).round_ties_even(),
            max,
            resource: resource_type,
        }
    }

    /// Creates a uff progress for edit mode.
    pub const fn edit_buff(id: u32, progress: f32, now: u32) -> Self {
        let decreasing = 1.0 - progress;
        Self::Buff {
            id,
            stacks: (25.0 * progress) as u32,
            duration: 5000,
            end: now + (5000.0 * decreasing) as u32,
        }
    }

    /// Creates an ability progress for edit mode.
    pub const fn edit_ability(id: SkillId, progress: f32, now: u32) -> Self {
        let decreasing = 1.0 - progress;
        Self::Ability {
            id,
            ammo: (5.0 * progress) as u32,
            recharge: 5000,
            end: now + (5000.0 * decreasing) as u32,
            rate: 1.0,
            ammo_recharge: 5000,
            ammo_end: now + (5000.0 * decreasing) as u32,
            state: BitFlags::EMPTY,
        }
    }

    /// Returns the assoicated skill.
    pub const fn skill(&self) -> SkillId {
        match *self {
            Self::Resource { .. } => SkillId::Unknown,
            Self::Buff { id, .. } => SkillId::Id(id),
            Self::Ability { id, .. } => id,
        }
    }

    /// Whether the progress uses timestamps.
    pub const fn is_timed(&self) -> bool {
        matches!(self, Self::Buff { .. } | Self::Ability { .. })
    }

    /// Whether the progress is inverted.
    pub const fn is_inverted(&self) -> bool {
        matches!(self, Self::Ability { .. })
    }

    /// Returns the intensity (stacks/ammo).
    pub const fn intensity(&self) -> u32 {
        match *self {
            Self::Resource { current, .. } => current as u32,
            Self::Buff { stacks, .. } => stacks,
            Self::Ability { ammo, .. } => ammo,
        }
    }

    /// Returns the current progress rate.
    pub const fn progress_rate(&self) -> f32 {
        match *self {
            Self::Resource { .. } | Self::Buff { .. } => 1.0,
            Self::Ability { rate, .. } => rate,
        }
    }

    /// Returns the current progress between `0.0` and `1.0`.
    pub fn progress(&self, value: ProgressValue, now: u32) -> Option<f32> {
        self.current(value, now).map(|current| {
            match (current, self.max(value)) {
                (0.0, 0.0) => 0.0, // treat 0/0 as 0% progress
                (_, 0.0) => 1.0,   // treat x/0 as 100% progress
                (current, max) => {
                    let progress = current / max;
                    if self.is_inverted() {
                        1.0 - progress
                    } else {
                        progress
                    }
                }
            }
        })
    }

    /// Returns the current progress between `0.0` and `1.0`.
    pub fn progress_or_default(&self, value: ProgressValue, now: u32) -> f32 {
        self.progress(value, now).unwrap_or(1.0)
    }

    /// Returns the current amount in its native unit.
    pub fn current(&self, value: ProgressValue, now: u32) -> Option<f32> {
        match *self {
            Self::Resource { current, .. } => Some(current),
            Self::Buff { end, .. } => {
                (end != u32::MAX).then(|| Self::time_between(now, end) as f32)
            }
            Self::Ability {
                end,
                ammo_end,
                rate,
                ..
            } => Some(Self::time_between_scaled(now, value.pick(end, ammo_end), rate) as f32),
        }
    }

    /// Returns the current amount as text.
    pub fn current_text(
        &self,
        value: ProgressValue,
        now: u32,
        unit: bool,
        settings: &FormatSettings,
    ) -> String {
        match *self {
            Self::Resource { current, .. } => {
                if unit {
                    Unit::format(current)
                } else {
                    current.round_ties_even().to_string()
                }
            }
            Self::Buff { end, .. } => {
                if end == u32::MAX {
                    "?".into()
                } else {
                    Self::duration_text(Self::time_between(now, end), settings)
                }
            }
            Self::Ability {
                end,
                ammo_end,
                rate,
                ..
            } => Self::duration_text(
                Self::time_between_scaled(now, value.pick(end, ammo_end), rate),
                settings,
            ),
        }
    }

    /// Returns the maximum amount in its native unit.
    pub fn max(&self, value: ProgressValue) -> f32 {
        match *self {
            Self::Resource { max, .. } => max,
            Self::Buff { duration, .. } => duration as f32,
            Self::Ability {
                recharge,
                ammo_recharge,
                ..
            } => value.pick(recharge, ammo_recharge) as f32,
        }
    }

    /// Returns the maximum amount as text.
    pub fn max_text(&self, value: ProgressValue, unit: bool, settings: &FormatSettings) -> String {
        match *self {
            Self::Resource { max, .. } => {
                if unit {
                    Unit::format(max)
                } else {
                    max.round_ties_even().to_string()
                }
            }
            Self::Buff { duration, .. } => {
                if duration != u32::MAX {
                    Self::duration_text(duration, settings)
                } else {
                    "?".into()
                }
            }
            Self::Ability {
                recharge,
                ammo_recharge,
                ..
            } => Self::duration_text(value.pick(recharge, ammo_recharge), settings),
        }
    }

    /// Retruns the resource type for the progress.
    pub const fn resource_type(&self) -> ResourceType {
        match *self {
            Self::Resource { resource, .. } => resource,
            _ => ResourceType::DEFAULT,
        }
    }

    /// Retruns the ability state for the progress.
    pub const fn ability_state(&self) -> BitFlags<AbilityState> {
        match *self {
            Self::Ability { state, .. } => state,
            _ => BitFlags::EMPTY,
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

    fn duration_text(time: u32, settings: &FormatSettings) -> String {
        if time > 0 {
            Time::format(time, settings.minutes_threshold, settings.millis_threshold)
        } else {
            String::new()
        }
    }
}
