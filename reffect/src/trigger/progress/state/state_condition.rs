use crate::enums::check_variant_array;
use const_default::ConstDefault;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumCount, EnumIter, IntoStaticStr, VariantArray};

#[derive(
    Debug, Clone, PartialEq, AsRefStr, IntoStaticStr, EnumIter, EnumCount, Serialize, Deserialize,
)]
pub enum StateCondition {
    #[strum(serialize = "True")]
    True,
    #[strum(serialize = "False")]
    False,
}

impl VariantArray for StateCondition {
    const VARIANTS: &'static [Self] = &[Self::True, Self::False];
}

const _: () = check_variant_array::<StateCondition>();

impl ConstDefault for StateCondition {
    const DEFAULT: Self = Self::True;
}

impl Default for StateCondition {
    fn default() -> Self {
        Self::DEFAULT
    }
}
