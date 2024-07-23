use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoStaticStr, VariantArray};

#[derive(Debug, Default, Clone, AsRefStr, IntoStaticStr, EnumIter, Serialize, Deserialize)]
pub enum ThresholdType {
    /// Always met.
    Always,

    /// Must be present.
    #[default]
    Present,

    /// Must be missing.
    Missing,

    /// Minimum amount.
    #[strum(serialize = "Min amount")]
    Min(u32),

    /// Maximum amount.
    #[strum(serialize = "Max amount")]
    Max(u32),

    /// Exact amount.
    #[strum(serialize = "Exact amount")]
    Exact(u32),

    /// Range of amounts.
    #[strum(serialize = "Amount between")]
    Between(u32, u32),
}

impl VariantArray for ThresholdType {
    const VARIANTS: &'static [Self] = &[
        Self::Always,
        Self::Present,
        Self::Missing,
        Self::Min(1),
        Self::Max(1),
        Self::Exact(1),
        Self::Between(0, 1),
    ];
}
