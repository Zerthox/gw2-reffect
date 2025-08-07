mod bar;
mod duration_text;
mod stacks_text;
mod text;

pub use self::{bar::*, duration_text::*, stacks_text::*, text::*};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconSettings {
    pub stack_text: StackTextSettings,
    pub duration_text: DurationTextSettings,
    pub duration_bar: DurationBarSettings,
}

impl IconSettings {
    #[inline]
    pub const fn new() -> Self {
        Self {
            stack_text: StackTextSettings::new(),
            duration_text: DurationTextSettings::new(),
            duration_bar: DurationBarSettings::new(),
        }
    }
}
