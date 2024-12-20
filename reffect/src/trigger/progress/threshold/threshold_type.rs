use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{AsRefStr, EnumIter, IntoStaticStr, VariantArray};

#[derive(
    Debug, Default, Clone, PartialEq, AsRefStr, IntoStaticStr, EnumIter, Serialize, Deserialize,
)]
pub enum ThresholdType {
    /// Always met.
    #[default]
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

impl ThresholdType {
    pub fn no_amount(&self) -> bool {
        matches!(self, Self::Always | Self::Present | Self::Missing)
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
