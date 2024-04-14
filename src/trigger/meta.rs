use super::{MapTrigger, PlayerTrigger, Trigger};
use crate::{
    context::RenderContext,
    traits::{Leaf, RenderOptions},
};
use nexus::imgui::Ui;
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

impl Leaf for MetaTrigger {
    fn load(&mut self) {
        self.player.load();
        self.map.load();
    }
}

impl RenderOptions for MetaTrigger {
    fn render_options(&mut self, ui: &Ui) {
        self.player.render_options(ui);

        self.map.render_options(ui);
    }
}
