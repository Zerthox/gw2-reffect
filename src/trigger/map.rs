use super::Trigger;
use crate::context::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MapTrigger {
    pub ids: Vec<u32>,
}

impl Trigger for MapTrigger {
    fn is_active(&self, ctx: &Context) -> bool {
        self.ids.is_empty() || self.ids.iter().copied().any(|id| ctx.player.is_on_map(id))
    }
}
