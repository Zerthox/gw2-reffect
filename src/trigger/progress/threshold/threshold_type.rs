use serde::{Deserialize, Serialize};
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

    /// Minimum amount.
    #[strum(serialize = "Min amount")]
    Min(f32),

    /// Maximum amount.
    #[strum(serialize = "Max amount")]
    Max(f32),

    /// Exact amount.
    #[strum(serialize = "Exact amount")]
    Exact(f32),

    /// Range of amounts.
    #[strum(serialize = "Amount between")]
    Between(f32, f32),
}

impl VariantArray for ThresholdType {
    const VARIANTS: &'static [Self] = &[
        Self::Always,
        Self::Present,
        Self::Missing,
        Self::Min(1.0),
        Self::Max(1.0),
        Self::Exact(1.0),
        Self::Between(0.0, 1.0),
    ];
}
