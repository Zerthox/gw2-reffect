use crate::context::Ability;
use const_default::ConstDefault;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressInfo {
    pub available: bool,
    pub pressed: bool,
    pub pending: bool,
}

impl ProgressInfo {
    pub const fn new() -> Self {
        Self {
            available: true,
            pressed: false,
            pending: false,
        }
    }
}

impl ConstDefault for ProgressInfo {
    const DEFAULT: Self = Self::new();
}

impl Default for ProgressInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl From<&Ability> for ProgressInfo {
    fn from(ability: &Ability) -> Self {
        Self {
            available: ability.is_available,
            pressed: ability.is_pressed,
            pending: ability.is_pending,
        }
    }
}
