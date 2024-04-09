use super::Trigger;
use crate::context::{Mount, Profession, RenderContext, Specialization};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerTrigger {
    pub combat: Option<bool>,
    pub profs: Vec<Profession>,
    pub specs: Vec<Specialization>,
    pub mounts: Vec<Mount>,
}

impl Trigger for PlayerTrigger {
    fn is_active(&self, ctx: &RenderContext) -> bool {
        self.combat
            .map(|combat| combat == ctx.ui.combat)
            .unwrap_or(true)
            && empty_or_contains(&self.profs, &ctx.player.prof)
            && empty_or_contains(&self.specs, &ctx.player.spec)
            && empty_or_contains(&self.mounts, &ctx.player.mount)
    }
}

fn empty_or_contains<T>(slice: &[T], el: &T) -> bool
where
    T: PartialEq,
{
    slice.is_empty() || slice.contains(el)
}
