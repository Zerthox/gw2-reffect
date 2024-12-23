use super::ProgressActive;
use crate::internal::{self, Skillbar};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumCount, EnumIter, IntoStaticStr, VariantArray};

/// Skillbar slot.
#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    AsRefStr,
    IntoStaticStr,
    EnumCount,
    EnumIter,
    VariantArray,
    Serialize,
    Deserialize,
)]
pub enum Slot {
    #[default]
    #[strum(serialize = "Weapon Swap")]
    WeaponSwap,

    #[strum(serialize = "Weapon 1")]
    Weapon1,

    #[strum(serialize = "Weapon 2")]
    Weapon2,

    #[strum(serialize = "Weapon 3")]
    Weapon3,

    #[strum(serialize = "Weapon 4")]
    Weapon4,

    #[strum(serialize = "Weapon 5")]
    Weapon5,

    Heal,

    #[strum(serialize = "Utility 1")]
    Utility1,

    #[strum(serialize = "Utility 2")]
    Utility2,

    #[strum(serialize = "Utility 3")]
    Utility3,

    Elite,

    #[strum(serialize = "Profession 1")]
    Profession1,

    #[strum(serialize = "Profession 2")]
    Profession2,

    #[strum(serialize = "Profession 3")]
    Profession3,

    #[strum(serialize = "Profession 4")]
    Profession4,

    #[strum(serialize = "Profession 5")]
    Profession5,

    #[strum(serialize = "Special Action")]
    SpecialAction,

    Mount,
}

impl Slot {
    pub fn get_id(&self, skillbar: &Skillbar) -> Option<u32> {
        let slot = (*self).try_into().ok()?;
        Some(skillbar.slot(slot)?.id)
    }

    pub fn get_progress(&self, skillbar: &Skillbar) -> Option<ProgressActive> {
        match *self {
            Self::WeaponSwap => {
                let swap = skillbar.weapon_swap.as_ref()?;
                Some(ProgressActive::from_recharge(swap))
            }
            Self::Profession1 => {
                if let Some(swap) = &skillbar.legend_swap {
                    Some(ProgressActive::from_recharge(swap))
                } else {
                    let ability = skillbar.slot(internal::Slot::Profession1)?;
                    Some(ProgressActive::from_ability(skillbar, ability))
                }
            }
            slot => {
                let ability = skillbar.slot(slot.try_into().expect("failed to convert slot"))?;
                Some(ProgressActive::from_ability(skillbar, ability))
            }
        }
    }
}

impl TryFrom<Slot> for internal::Slot {
    type Error = ();

    fn try_from(slot: Slot) -> Result<Self, Self::Error> {
        match slot {
            Slot::WeaponSwap => Err(()),
            Slot::Weapon1 => Ok(Self::Weapon1),
            Slot::Weapon2 => Ok(Self::Weapon2),
            Slot::Weapon3 => Ok(Self::Weapon3),
            Slot::Weapon4 => Ok(Self::Weapon4),
            Slot::Weapon5 => Ok(Self::Weapon5),
            Slot::Heal => Ok(Self::Heal),
            Slot::Utility1 => Ok(Self::Utility1),
            Slot::Utility2 => Ok(Self::Utility2),
            Slot::Utility3 => Ok(Self::Utility3),
            Slot::Elite => Ok(Self::Elite),
            Slot::Profession1 => Ok(Self::Profession1),
            Slot::Profession2 => Ok(Self::Profession2),
            Slot::Profession3 => Ok(Self::Profession3),
            Slot::Profession4 => Ok(Self::Profession4),
            Slot::Profession5 => Ok(Self::Profession5),
            Slot::SpecialAction => Ok(Self::SpecialAction),
            Slot::Mount => Ok(Self::Mount),
        }
    }
}
