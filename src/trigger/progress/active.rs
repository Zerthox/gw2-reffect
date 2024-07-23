use crate::internal::{Buff, Resource};

#[derive(Debug, Clone)]
pub enum ProgressActive {
    Buff {
        stacks: u32,
        apply: u32,
        runout: u32,
    },
    Resource(Resource),
}

impl ProgressActive {
    /// Returns the intensity (alternative progress).
    pub fn intensity(&self) -> u32 {
        match self {
            Self::Buff { stacks, .. } => *stacks,
            Self::Resource(resource) => resource.current,
        }
    }

    /// Returns the current progress between `0.0` and `1.0`.
    pub fn progress(&self, now: u32) -> Option<f32> {
        let current = self.current(now)?;
        let max = self.max();
        (max != 0).then(|| current as f32 / max as f32)
    }

    /// Returns the current progress between `0.0` and `1.0`.
    pub fn progress_or_default(&self, now: u32) -> f32 {
        if let Some(current) = self.current(now) {
            let max = self.max();
            if max != 0 {
                current as f32 / max as f32
            } else {
                0.0 // default to 0 for no max
            }
        } else {
            1.0 // default to 1 for no current
        }
    }

    /// Returns the current amount in its native unit.
    pub fn current(&self, now: u32) -> Option<u32> {
        match self {
            Self::Buff { runout, .. } => Self::time_between_checked(now, *runout),
            Self::Resource(resource) => Some(resource.current),
        }
    }

    /// Returns the current amount as text.
    pub fn current_text(&self, now: u32) -> String {
        match self {
            Self::Buff { runout, .. } => Self::time_between_checked(now, *runout)
                .map(Self::format_seconds)
                .unwrap_or_else(|| "?".into()),
            Self::Resource(res) => {
                if res.max != 0 {
                    res.current.to_string()
                } else {
                    "?".into()
                }
            }
        }
    }

    /// Returns the maximum amount in its native unit.
    pub fn max(&self) -> u32 {
        match self {
            Self::Buff { apply, runout, .. } => runout.saturating_sub(*apply),
            Self::Resource(resource) => resource.max,
        }
    }

    /// Returns the maximum amount as text.
    pub fn max_text(&self) -> String {
        match self {
            Self::Buff { apply, runout, .. } => Self::time_between_checked(*apply, *runout)
                .map(Self::format_seconds)
                .unwrap_or_else(|| "?".into()),
            Self::Resource(resource) => {
                if resource.max != 0 {
                    resource.max.to_string()
                } else {
                    "?".into()
                }
            }
        }
    }

    fn time_between(start: u32, end: u32) -> u32 {
        end.saturating_sub(start)
    }

    fn time_between_checked(now: u32, end: u32) -> Option<u32> {
        (end != u32::MAX).then(|| Self::time_between(now, end))
    }

    fn format_seconds(time: u32) -> String {
        format!("{:.1}", time as f32 / 1000.0)
    }
}

impl From<&Buff> for ProgressActive {
    fn from(buff: &Buff) -> Self {
        Self::Buff {
            stacks: buff.stacks,
            apply: buff.apply_time,
            runout: buff.runout_time,
        }
    }
}

impl TryFrom<Resource> for ProgressActive {
    type Error = ();

    fn try_from(resource: Resource) -> Result<Self, Self::Error> {
        if resource.max != 0 {
            Ok(Self::Resource(resource))
        } else {
            Err(())
        }
    }
}
