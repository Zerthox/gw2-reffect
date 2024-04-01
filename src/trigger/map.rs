use super::Trigger;
use crate::context::{MapCategory, RenderContext};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum MapTrigger {
    #[default]
    Any,
    Category(MapCategory),
    Ids(Vec<u32>),
}

impl Trigger for MapTrigger {
    fn is_active(&self, ctx: &RenderContext) -> bool {
        match self {
            Self::Any => true,
            Self::Category(category) => ctx.map.category == *category,
            Self::Ids(ids) => ids.iter().copied().any(|id| ctx.map.is_on_map(id)),
        }
    }
}
