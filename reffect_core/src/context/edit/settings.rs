use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditSettings {
    /// Whether edit mode is allowed in combat.
    pub during_combat: bool,

    /// Whether to show all elements of a pack in edit mode.
    pub show_all: bool,
}

impl EditSettings {
    #[inline]
    pub const fn new() -> Self {
        Self {
            during_combat: false,
            show_all: false,
        }
    }
}

impl Default for EditSettings {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
