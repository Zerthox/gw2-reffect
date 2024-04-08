use super::{MapTrigger, PlayerTrigger, Trigger};
use crate::context::RenderContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MetaTrigger {
    pub player: PlayerTrigger,
    pub map: MapTrigger,
}

impl Trigger for MetaTrigger {
    fn is_active(&self, ctx: &RenderContext) -> bool {
        self.player.is_active(ctx) && self.map.is_active(ctx)
    }
}
