use super::{memo::Memo, MapTrigger, PlayerTrigger, Trigger};
use crate::{
    context::RenderContext,
    traits::{Leaf, RenderOptions},
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MetaTrigger {
    pub player: PlayerTrigger, // player not memoized due to combat & mount
    pub map: Memo<MapTrigger>, // memoize map
}

impl Trigger for MetaTrigger {
    fn is_active(&mut self, ctx: &RenderContext) -> bool {
        self.player.is_active(ctx) && self.map.is_active(ctx)
    }
}

impl Leaf for MetaTrigger {
    fn load(&mut self) {}

    fn context_update(&mut self, ctx: &RenderContext) {
        self.map.update(ctx);
    }
}

impl RenderOptions for MetaTrigger {
    fn render_options(&mut self, ui: &Ui) {
        self.player.render_options(ui);

        self.map.render_options(ui);
    }
}
