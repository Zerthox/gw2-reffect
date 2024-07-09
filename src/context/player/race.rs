use enumflags2::bitflags;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

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
#[repr(u8)]
pub enum Race {
    Asura = 1 << 0,
    Charr = 1 << 1,
    Human = 1 << 2,
    Norn = 1 << 3,
    Sylvari = 1 << 4,
}

impl TryFrom<u8> for Race {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Asura),
            1 => Ok(Self::Charr),
            2 => Ok(Self::Human),
            3 => Ok(Self::Norn),
            4 => Ok(Self::Sylvari),
            _ => Err(value),
        }
    }
}
