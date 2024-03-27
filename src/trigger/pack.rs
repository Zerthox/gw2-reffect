use super::{MapTrigger, PlayerTrigger, Trigger};
use crate::context::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PackTrigger {
    pub player: PlayerTrigger,
    pub map: MapTrigger,
}

impl Trigger for PackTrigger {
    fn is_active(&self, ctx: &Context) -> bool {
        self.player.is_active(ctx) && self.map.is_active(ctx)
    }
}
