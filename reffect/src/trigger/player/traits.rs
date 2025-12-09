use crate::context::Traits;
use const_default::ConstDefault;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TraitRequirement {
    pub id: u32,
    pub present: bool,
}

impl TraitRequirement {
    pub fn is_met(&self, traits: &Traits) -> bool {
        let contains = traits.contains(&self.id);
        match self.present {
            true => contains,
            false => !contains,
        }
    }
}

impl ConstDefault for TraitRequirement {
    const DEFAULT: Self = Self {
        id: 0,
        present: true,
    };
}

impl Default for TraitRequirement {
    fn default() -> Self {
        Self::DEFAULT
    }
}
