use enumflags2::{BitFlag, BitFlags};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, VariantArray};

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
    EnumIter,
    VariantArray,
    Serialize,
    Deserialize,
)]
pub enum Value {
    #[strum(serialize = "Any of")]
    Any,

    #[strum(serialize = "All of")]
    All,

    #[strum(serialize = "None of")]
    #[serde(alias = "NotAny")]
    None,

    #[strum(serialize = "Not all of")]
    NotAll,
}

impl Value {
    pub fn check<T>(&self, target: BitFlags<T>, other: BitFlags<T>) -> bool
    where
        T: BitFlag,
    {
        match self {
            Self::Any => target.intersects(other),
            Self::All => target.contains(other),
            Self::None => !target.intersects(other),
            Self::NotAll => !target.contains(other),
        }
    }
}
