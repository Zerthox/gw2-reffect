use crate::{
    colors::{Color, Colored},
    named::Named,
};
use enumflags2::{BitFlags, bitflags};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

/// Player weapon sets.
pub type Weapons = BitFlags<Weapon>;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    AsRefStr,
    IntoStaticStr,
    Display,
    EnumIter,
    VariantArray,
    Serialize,
    Deserialize,
)]
#[bitflags]
#[repr(u32)]
pub enum Weapon {
    #[strum(serialize = "Axe (main)")]
    AxeMain = 1 << 0,

    #[strum(serialize = "Axe (off)")]
    AxeOff = 1 << 1,

    #[strum(serialize = "Dagger (main)")]
    DaggerMain = 1 << 2,

    #[strum(serialize = "Dagger (off)")]
    DaggerOff = 1 << 3,

    #[strum(serialize = "Mace (main)")]
    MaceMain = 1 << 4,

    #[strum(serialize = "Mace (off)")]
    MaceOff = 1 << 5,

    #[strum(serialize = "Pistol (main)")]
    PistolMain = 1 << 6,

    #[strum(serialize = "Pistol (off)")]
    PistolOff = 1 << 7,

    #[strum(serialize = "Sword (main)")]
    SwordMain = 1 << 8,

    #[strum(serialize = "Sword (off)")]
    SwordOff = 1 << 9,

    Scepter = 1 << 10,

    Focus = 1 << 11,
    Shield = 1 << 12,
    Torch = 1 << 13,
    Warhorn = 1 << 14,

    Greatsword = 1 << 15,
    Hammer = 1 << 16,
    Longbow = 1 << 17,
    Rifle = 1 << 18,
    Shortbow = 1 << 19,
    Staff = 1 << 20,
    Spear = 1 << 21,
}

impl Named for Weapon {
    fn name(&self) -> &'static str {
        self.into()
    }
}

impl Colored for Weapon {
    fn colored(&self) -> Option<Color> {
        None
    }
}
