use nexus::gamebind::GameBind;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumCount, EnumIter, IntoStaticStr, VariantArray};

/// Skillbar slot.
#[derive(
    Debug,
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
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Slot {
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
    pub const DEFAULT: Self = Self::WeaponSwap;

    #[inline]
    pub const fn bind(&self) -> GameBind {
        match self {
            Self::WeaponSwap => GameBind::SkillWeaponSwap,
            Self::Weapon1 => GameBind::SkillWeapon1,
            Self::Weapon2 => GameBind::SkillWeapon2,
            Self::Weapon3 => GameBind::SkillWeapon3,
            Self::Weapon4 => GameBind::SkillWeapon4,
            Self::Weapon5 => GameBind::SkillWeapon5,
            Self::Heal => GameBind::SkillHeal,
            Self::Utility1 => GameBind::SkillUtility1,
            Self::Utility2 => GameBind::SkillUtility2,
            Self::Utility3 => GameBind::SkillUtility3,
            Self::Elite => GameBind::SkillElite,
            Self::Profession1 => GameBind::SkillProfession1,
            Self::Profession2 => GameBind::SkillProfession2,
            Self::Profession3 => GameBind::SkillProfession3,
            Self::Profession4 => GameBind::SkillProfession4,
            Self::Profession5 => GameBind::SkillProfession5,
            Self::SpecialAction => GameBind::SkillSpecialAction,
            Self::Mount => GameBind::MountToggle,
        }
    }
}

impl Default for Slot {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl From<Slot> for GameBind {
    #[inline]
    fn from(slot: Slot) -> Self {
        slot.bind()
    }
}
