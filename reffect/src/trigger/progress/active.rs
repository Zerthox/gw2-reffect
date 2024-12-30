use reffect_internal::{Skillbar, Slot};

use crate::{
    fmt::{Time, Unit},
    internal::{Ability, Buff, Resource},
    settings::FormatSettings,
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
        skill: Skill,
        ammo: u32,
        rate: f32,
        recharge: u32,
        end: u32,
        ammo_recharge: u32,
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

    /// Creates new timed active progress from an ability.
    pub fn from_ability(skill: Skill, ability: &Ability) -> Self {
        let Ability {
            id: _,
            ammo,
            last_update,
            recharge_rate,
            recharge,
            recharge_remaining,
            ammo_recharge,
            ammo_recharge_remaining,
        } = *ability;
        Self::Ability {
            skill,
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
    pub const fn edit_ability(skill: Skill, progress: f32, now: u32) -> Self {
        Self::Ability {
            skill,
            ammo: (5.0 * progress) as u32,
            recharge: 5000,
            end: now + (5000.0 * progress) as u32,
            rate: 1.0,
            ammo_recharge: 5000,
            ammo_end: now + (5000.0 * progress) as u32,
        }
    }

    /// Returns the assoicated skill.
    pub const fn skill(&self) -> Skill {
        match *self {
            Self::Fixed { .. } => Skill::Unknown,
            Self::Buff { id, .. } => Skill::Id(id),
            Self::Ability { skill, .. } => skill,
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

    /// Returns the intensity (alternative progress).
    pub const fn intensity(&self) -> u32 {
        match *self {
            Self::Fixed { current, .. } => current,
            Self::Buff { stacks, .. } => stacks,
            Self::Ability { ammo, .. } => ammo,
        }
    }

    /// Returns the current progress rate.
    pub const fn progress_rate(&self) -> f32 {
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
                (current, max) => {
                    let progress = current as f32 / max as f32;
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
    pub fn current(&self, value: ProgressValue, now: u32) -> Option<u32> {
        match *self {
            Self::Fixed { current, .. } => Some(current),
            Self::Buff { end, .. } => (end != u32::MAX).then(|| Self::time_between(now, end)),
            Self::Ability {
                end,
                ammo_end,
                rate,
                ..
            } => Some(Self::time_between_scaled(
                now,
                value.pick(end, ammo_end),
                rate,
            )),
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
            Self::Fixed { current, .. } => Unit::format_if(current, unit),
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
    pub fn max(&self, value: ProgressValue) -> u32 {
        match *self {
            Self::Fixed { max, .. } => max,
            Self::Buff { duration, .. } => duration,
            Self::Ability {
                recharge,
                ammo_recharge,
                ..
            } => value.pick(recharge, ammo_recharge),
        }
    }

    /// Returns the maximum amount as text.
    pub fn max_text(&self, value: ProgressValue, unit: bool, settings: &FormatSettings) -> String {
        match *self {
            Self::Fixed { max, .. } => Unit::format_if(max, unit),
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
    PreferPrimary,
    PreferSecondary,
}

impl ProgressValue {
    pub fn pick(&self, primary: u32, secondary: u32) -> u32 {
        match self {
            Self::Primary => primary,
            Self::Secondary => secondary,
            Self::PreferPrimary => {
                if primary > 0 {
                    primary
                } else {
                    secondary
                }
            }
            Self::PreferSecondary => {
                if secondary > 0 {
                    secondary
                } else {
                    primary
                }
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Skill {
    #[default]
    Unknown,
    WeaponSwap,
    BundleDrop,
    Id(u32),
}

impl Skill {
    pub fn from_slot(skillbar: &Skillbar, slot: Slot) -> Self {
        if slot == Slot::WeaponSwap {
            if skillbar.has_bundle {
                Skill::BundleDrop
            } else {
                Skill::WeaponSwap
            }
        } else {
            skillbar
                .slot(slot)
                .map(|ability| ability.id.into())
                .unwrap_or_default()
        }
    }
}

impl From<u32> for Skill {
    fn from(id: u32) -> Self {
        Self::Id(id)
    }
}
