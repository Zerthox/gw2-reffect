use super::Trigger;
use crate::{
    context::RenderContext,
    player::{Profession, Specialization},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerTrigger {
    pub profs: Vec<Profession>,
    pub specs: Vec<Specialization>,
    pub combat: Option<bool>,
}

impl Trigger for PlayerTrigger {
    fn is_active(&self, ctx: &RenderContext) -> bool {
        (self.profs.is_empty() || self.profs.contains(&ctx.player.prof))
            && (self.specs.is_empty() || self.specs.contains(&ctx.player.spec))
            && self
                .combat
                .map(|combat| combat == ctx.ui.combat)
                .unwrap_or(true)
    }
}
