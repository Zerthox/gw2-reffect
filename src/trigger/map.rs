use super::Trigger;
use crate::context::{Context, MapCategory};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum MapTrigger {
    #[default]
    Any,
    Category(MapCategory),
    Ids(Vec<u32>),
}

impl Trigger for MapTrigger {
    fn is_active(&self, ctx: &Context) -> bool {
        match self {
            Self::Any => true,
            Self::Category(category) => ctx.player.map.category == *category,
            Self::Ids(ids) => ids.iter().copied().any(|id| ctx.player.is_on_map(id)),
        }
    }
}
