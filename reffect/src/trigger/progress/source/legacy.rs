use crate::{
    context::Slot,
    trigger::{Combatant, ProgressSource},
};
use serde::{Deserialize, Serialize};
use serde_with::{OneOrMany, formats::PreferMany, serde_as};

#[serde_as]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum ProgressSourceLegacy {
    /// Inherit from above.
    #[default]
    Inherit,

    /// Always active, no associated progress.
    #[serde(alias = "None")]
    Always,

    /// Buff ids, multiple matches are merged.
    #[serde(alias = "Single")]
    #[serde(alias = "Has")]
    #[serde(alias = "Any")]
    #[serde(alias = "AnyBuff")]
    #[serde(alias = "Effect")]
    Buff(#[serde_as(as = "OneOrMany<_, PreferMany>")] Vec<u32>),

    /// Ability ids, first match is used.
    Ability(Vec<u32>),

    /// Skillbar slot.
    SkillbarSlot(Slot),

    /// Health.
    Health,

    /// Barrier.
    Barrier,

    // Defiance
    Defiance,

    /// Endurance.
    Endurance,

    /// Primary profession resource.
    PrimaryResource,

    /// Secondary profession resource.
    SecondaryResource,
}

impl From<ProgressSourceLegacy> for ProgressSource {
    fn from(source: ProgressSourceLegacy) -> Self {
        match source {
            ProgressSourceLegacy::Inherit => Self::Inherit,
            ProgressSourceLegacy::Always => Self::Always,
            ProgressSourceLegacy::Buff(ids) => Self::Buff {
                combatant: Combatant::default(),
                ids,
            },
            ProgressSourceLegacy::Ability(ids) => Self::Ability { ids },
            ProgressSourceLegacy::SkillbarSlot(slot) => Self::SkillbarSlot { slot },
            ProgressSourceLegacy::Health => Self::Health {
                combatant: Combatant::default(),
            },
            ProgressSourceLegacy::Barrier => Self::Barrier {
                combatant: Combatant::default(),
            },
            ProgressSourceLegacy::Defiance => Self::Defiance {
                combatant: Combatant::default(),
            },
            ProgressSourceLegacy::Endurance => Self::Endurance,
            ProgressSourceLegacy::PrimaryResource => Self::PrimaryResource,
            ProgressSourceLegacy::SecondaryResource => Self::SecondaryResource,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::Migrate;

    #[test]
    fn migrate() {
        let json = r#"{ "Buff": 123 }"#;
        let result = serde_json::from_str::<Migrate<ProgressSource, ProgressSourceLegacy>>(&json)
            .expect("failed to deserialize");
        let source = result.inner;
        assert_eq!(
            source,
            ProgressSource::Buff {
                combatant: Combatant::default(),
                ids: vec![123]
            }
        );
    }
}
