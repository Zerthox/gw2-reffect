use crate::{
    internal::{Interface, Internal},
    render::Validation,
};
use serde::{Deserialize, Serialize};

/// An item mapped to its hidden buff.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct Item {
    /// Item id.
    pub item: u32,

    /// Hidden buff id.
    #[serde(skip)]
    pub buff: u32,
}

impl Item {
    /// Creates an empty/invalid item.
    pub const fn empty() -> Self {
        Self { item: 0, buff: 0 }
    }

    /// Updates the item buff.
    pub fn update(&mut self) {
        self.buff = Internal::get_item_info(self.item)
            .ok()
            .and_then(|info| info.buff())
            .unwrap_or(0);
    }

    /// Validates the item buff.
    pub fn validate(&self) -> Validation<String> {
        let Self { item, .. } = *self;
        if let Ok(info) = Internal::get_item_info(item)
            && let Some(buff) = info.buff()
        {
            Validation::Confirm(format!(
                "{} {item} corresponds to hidden effect {buff}",
                info.as_ref()
            ))
        } else {
            Validation::Error(format!("Item {item} is invalid"))
        }
    }
}
