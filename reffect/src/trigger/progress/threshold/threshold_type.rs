use crate::enums::check_variant_array;
use const_default::ConstDefault;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{AsRefStr, EnumCount, EnumIter, IntoStaticStr, VariantArray};

#[derive(
    Debug, Clone, PartialEq, AsRefStr, IntoStaticStr, EnumIter, EnumCount, Serialize, Deserialize,
)]
pub enum ThresholdType {
    /// Always met.
    Always,

    /// Must be present.
    Present,

    /// Must be missing.
    Missing,

    /// Amount below.
    #[serde(alias = "Max")]
    Below(f32),

    /// Amount above.
    #[serde(alias = "Min")]
    Above(f32),

    /// Exact amount.
    Exact(f32),

    /// Amount in range.
    Between(f32, f32),
}

impl ConstDefault for ThresholdType {
    const DEFAULT: Self = Self::Always;
}

impl Default for ThresholdType {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl VariantArray for ThresholdType {
    const VARIANTS: &'static [Self] = &[
        Self::Always,
        Self::Present,
        Self::Missing,
        Self::Above(1.0),
        Self::Below(1.0),
        Self::Exact(1.0),
        Self::Between(0.0, 1.0),
    ];
}

const _: () = check_variant_array::<ThresholdType>();

impl ThresholdType {
    pub fn no_amount(&self) -> bool {
        matches!(self, Self::Always | Self::Present | Self::Missing)
    }
}

impl fmt::Display for ThresholdType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Always => write!(f, "Always"),
            Self::Present => write!(f, "Present"),
            Self::Missing => write!(f, "Missing"),
            Self::Above(value) => write!(f, ">= {value}"),
            Self::Below(value) => write!(f, "<= {value}"),
            Self::Exact(value) => write!(f, "= {value}"),
            Self::Between(min, max) => write!(f, "{min} - {max}"),
        }
    }
}
