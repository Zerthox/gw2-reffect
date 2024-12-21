use crate::{
    fmt::Pretty,
    internal::{Buff, Resource},
};
use reffect_internal::{Ability, Skillbar};

#[derive(Debug, Clone)]
pub enum ProgressActive {
    Fixed {
        current: u32,
        max: u32,
    },
    Timed {
        id: u32,
        intensity: u32,
        duration: u32,
        end: u32,
        rate: f32,
    },
}

impl ProgressActive {
    /// Creates a dummy active progress.
    pub const fn dummy() -> Self {
        Self::Fixed { current: 1, max: 1 }
    }

    /// Creates a new resource progress from percent & maximum.
    pub const fn from_percent(progress: f32, max: u32) -> Self {
        Self::Fixed {
            current: (progress * max as f32) as u32,
            max,
        }
    }

    /// Creates an empty timed active progress.
    pub const fn empy_timed(id: u32) -> Self {
        Self::Timed {
            id,
            intensity: 0,
            duration: 0,
            end: 0,
            rate: 1.0,
        }
    }

    /// Creates new timed active progress from a buff.
    pub fn from_buff(id: u32, buff: &Buff) -> Self {
        Self::Timed {
            id,
            intensity: buff.stacks,
            duration: if buff.is_infinite() {
                u32::MAX
            } else {
                Self::time_between(buff.apply_time, buff.runout_time)
            },
            end: buff.runout_time,
            rate: 1.0,
        }
    }

    /// Creates new timed active progress from a skillbar and ability.
    pub fn from_ability(skillbar: &Skillbar, ability: &Ability) -> Self {
        Self::Timed {
            id: ability.id,
            intensity: ability.ammo,
            duration: ability.recharge,
            end: skillbar.last_update
                + Self::unscale(ability.recharge_remaining, skillbar.recharge_rate),
            rate: skillbar.recharge_rate,
        }
    }

    /// Creates new timed active progress from a skillbar and ability.
    pub fn from_ability_ammo(skillbar: &Skillbar, ability: &Ability) -> Self {
        Self::Timed {
            id: ability.id,
            intensity: ability.ammo,
            duration: ability.ammo_recharge,
            end: skillbar.last_update
                + Self::unscale(ability.ammo_recharge_remaining, skillbar.recharge_rate),
            rate: skillbar.recharge_rate,
        }
    }

    pub const fn id(&self) -> Option<u32> {
        match *self {
            Self::Fixed { .. } => None,
            Self::Timed { id, .. } => Some(id),
        }
    }

    /// Whether the progress uses timestamps.
    pub const fn is_timed(&self) -> bool {
        matches!(self, Self::Timed { .. })
    }

    /// Returns the intensity (alternative progress).
    pub const fn intensity(&self) -> u32 {
        match *self {
            Self::Fixed { current, .. } => current,
            Self::Timed { intensity, .. } => intensity,
        }
    }

    /// Returns the current progress between `0.0` and `1.0`.
    pub fn progress(&self, now: u32) -> Option<f32> {
        self.current(now).map(|current| {
            match (current, self.max()) {
                (0, 0) => 0.0, // treat 0/0 as 0% progress
                (_, 0) => 1.0, // treat x/0 as 100% progress
                (current, max) => current as f32 / max as f32,
            }
        })
    }

    /// Returns the current progress between `0.0` and `1.0`.
    pub fn progress_or_default(&self, now: u32) -> f32 {
        self.progress(now).unwrap_or(1.0)
    }

    pub fn progress_rate(&self) -> f32 {
        match *self {
            Self::Fixed { .. } => 1.0,
            Self::Timed { rate, .. } => rate,
        }
    }

    /// Returns the current amount in its native unit.
    pub fn current(&self, now: u32) -> Option<u32> {
        match *self {
            Self::Fixed { current, .. } => Some(current),
            Self::Timed { end, rate, .. } => {
                (end != u32::MAX).then(|| Self::time_between_scaled(now, end, rate))
            }
        }
    }

    /// Returns the current amount as text.
    pub fn current_text(&self, now: u32, pretty: bool) -> String {
        match *self {
            Self::Fixed { current, .. } => Pretty::string_if(current, pretty),
            Self::Timed { end, rate, .. } => {
                if end == u32::MAX {
                    "?".into()
                } else {
                    let time = Self::time_between_scaled(now, end, rate);
                    if time > 0 {
                        Self::format_seconds(time)
                    } else {
                        String::new()
                    }
                }
            }
        }
    }

    /// Returns the maximum amount in its native unit.
    pub fn max(&self) -> u32 {
        match *self {
            Self::Fixed { max, .. } => max,
            Self::Timed { duration, .. } => duration,
        }
    }

    /// Returns the maximum amount as text.
    pub fn max_text(&self, pretty: bool) -> String {
        match *self {
            Self::Fixed { max, .. } => Pretty::string_if(max, pretty),
            Self::Timed { duration, .. } => {
                if duration != u32::MAX {
                    Self::format_seconds(duration)
                } else {
                    "?".into()
                }
            }
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
