use super::Trigger;
use crate::context::Context;
use nexus::data_link::mumble::Profession;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PlayerTrigger {
    pub profs: Vec<Profession>,
    pub specs: Vec<u32>,
}

impl Trigger for PlayerTrigger {
    fn is_active(&self, ctx: &Context) -> bool {
        (self.profs.is_empty() || self.profs.contains(&ctx.player.prof))
            && (self.specs.is_empty() || self.specs.contains(&ctx.player.spec))
    }
}
