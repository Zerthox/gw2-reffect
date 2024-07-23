use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoStaticStr, VariantArray};

#[derive(
    Debug, Default, Clone, AsRefStr, IntoStaticStr, EnumIter, Serialize, Deserialize, VariantArray,
)]
pub enum AmountType {
    // Intensity.
    #[default]
    Intensity,

    // Duration.
    Duration,
}
