use crate::enums::check_variant_array;
use const_default::ConstDefault;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumCount, EnumIter, IntoStaticStr, VariantArray};

#[derive(
    Debug,
    Clone,
    PartialEq,
    AsRefStr,
    IntoStaticStr,
    EnumIter,
    EnumCount,
    VariantArray,
    Serialize,
    Deserialize,
)]
pub enum AbilityState {
    Pressed,
    Pending,
}

const _: () = check_variant_array::<AbilityState>();

impl ConstDefault for AbilityState {
    const DEFAULT: Self = Self::Pressed;
}

impl Default for AbilityState {
    fn default() -> Self {
        Self::DEFAULT
    }
}
