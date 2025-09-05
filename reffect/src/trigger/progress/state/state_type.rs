use crate::enums::check_variant_array;
use const_default::ConstDefault;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumCount, EnumIter, IntoStaticStr, VariantArray};

#[derive(
    Debug, Clone, PartialEq, AsRefStr, IntoStaticStr, EnumIter, EnumCount, Serialize, Deserialize,
)]
pub enum StateType {
    #[strum(serialize = "Available")]
    Available,
    #[strum(serialize = "Pressed")]
    Pressed,
    #[strum(serialize = "Pending")]
    Pending,
}

impl VariantArray for StateType {
    const VARIANTS: &'static [Self] = &[Self::Available, Self::Pressed, Self::Pending];
}

const _: () = check_variant_array::<StateType>();

impl ConstDefault for StateType {
    const DEFAULT: Self = Self::Available;
}

impl Default for StateType {
    fn default() -> Self {
        Self::DEFAULT
    }
}
