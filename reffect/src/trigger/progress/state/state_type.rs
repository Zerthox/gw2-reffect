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
pub enum StateType {
    Available,
    Pressed,
    Pending,
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
