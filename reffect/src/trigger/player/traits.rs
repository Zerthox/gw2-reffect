use const_default::ConstDefault;
use serde::{Deserialize, Serialize};

/// A trait requirement.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct TraitRequirement {
    /// Trait id.
    pub id: u32,

    /// Whether to check for present or missing.
    pub present: bool,
}

impl TraitRequirement {
    /// Checks whether the trait requirement is met.
    pub fn is_met(&self, traits: impl IntoIterator<Item = u32>) -> bool {
        let contains = traits.into_iter().any(|id| id == self.id);
        if self.present { contains } else { !contains }
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
