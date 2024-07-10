use super::{memo::Memo, MapTrigger, PlayerTrigger, Trigger};
use crate::{
    context::{Context, ContextUpdate},
    traits::RenderOptions,
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct MetaTrigger {
    pub player: PlayerTrigger, // player not memoized due to combat & mount
    pub map: Memo<MapTrigger>, // memoize map
}

impl MetaTrigger {
    pub fn load(&mut self) {
        self.player.load();
    }
}

impl Trigger for MetaTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        if ctx.has_update(ContextUpdate::Map) {
            self.map.update(ctx);
        }

        self.player.is_active(ctx) && self.map.is_active(ctx)
    }
}

impl RenderOptions for MetaTrigger {
    fn render_options(&mut self, ui: &Ui) {
        self.player.render_options(ui);

        ui.spacing();
        let changed = self.map.render_options(ui);
        if changed {
            // map trigger changed, ensure fresh state next access
            self.map.clear();
        }
    }
}
