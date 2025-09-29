use enumflags2::bitflags;
use reffect_core::colors::{Color, Colored};
use reffect_core::named::Named;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

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
    AsRefStr,
    IntoStaticStr,
    Display,
    EnumIter,
    VariantArray,
    Serialize,
    Deserialize,
)]
#[repr(u8)]
#[bitflags]
pub enum AbilityState {
    #[default]
    Pressed = 1 << 0,

    Pending = 1 << 1,
}

impl Named for AbilityState {
    fn name(&self) -> &'static str {
        match self {
            Self::Pressed => "Pressed",
            Self::Pending => "Pending",
        }
    }
}

impl Colored for AbilityState {
    fn colored(&self) -> Option<Color> {
        None
    }
}
