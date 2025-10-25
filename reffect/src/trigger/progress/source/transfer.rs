use super::{Combatant, ProgressSource};
use const_default::ConstDefault;

#[derive(Debug, Clone)]
pub struct Transfer {
    pub combatant: Combatant,
    pub ids: Vec<u32>,
}

impl Default for Transfer {
    fn default() -> Self {
        Self {
            combatant: Combatant::DEFAULT,
            ids: vec![0],
        }
    }
}

impl From<ProgressSource> for Transfer {
    fn from(source: ProgressSource) -> Self {
        match source {
            ProgressSource::Buff { combatant, ids } => Self { combatant, ids },
            ProgressSource::Ability { ids } => Self {
                ids,
                ..Self::default()
            },
            ProgressSource::Health { combatant }
            | ProgressSource::Barrier { combatant }
            | ProgressSource::Defiance { combatant } => Self {
                combatant,
                ..Self::default()
            },
            ProgressSource::Inherit
            | ProgressSource::Always
            | ProgressSource::SkillbarSlot { .. }
            | ProgressSource::Endurance
            | ProgressSource::PrimaryResource
            | ProgressSource::SecondaryResource => Self::default(),
        }
    }
}

impl Transfer {
    pub fn transfer(from: ProgressSource, to: &mut ProgressSource) {
        Self::from(from).apply(to);
    }

    pub fn apply(self, source: &mut ProgressSource) {
        match source {
            ProgressSource::Buff { combatant, ids } => {
                *combatant = self.combatant;
                *ids = self.ids;
            }
            ProgressSource::Ability { ids } => *ids = self.ids,
            ProgressSource::Health { combatant }
            | ProgressSource::Barrier { combatant }
            | ProgressSource::Defiance { combatant } => *combatant = self.combatant,
            ProgressSource::Inherit
            | ProgressSource::Always
            | ProgressSource::SkillbarSlot { .. }
            | ProgressSource::Endurance
            | ProgressSource::PrimaryResource
            | ProgressSource::SecondaryResource => {}
        }
    }
}
