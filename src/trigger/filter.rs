use super::{map_old::MapTriggerOld, memo::Memo, MapTrigger, PlayerTrigger, Trigger};
use crate::{
    context::{Context, ContextUpdate, EditState},
    render_util::debug_optional,
    serde_migrate::migrate,
    traits::{RenderDebug, RenderOptions},
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FilterTrigger {
    pub player: PlayerTrigger, // player not memoized due to combat & mount

    #[serde(deserialize_with = "migrate::<_, _, MapTriggerOld>")]
    pub map: Memo<MapTrigger>, // memoize map
}

impl FilterTrigger {
    pub fn load(&mut self) {
        self.player.load();
    }
}

impl Trigger for FilterTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        if ctx.has_update(ContextUpdate::Map) {
            self.map.update(ctx);
        }

        self.player.is_active(ctx) && self.map.is_active(ctx)
    }
}

impl RenderOptions for FilterTrigger {
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        self.player.render_options(ui, state);

        ui.spacing();
        let changed = self.map.render_options(ui, state);
        if changed {
            // map trigger changed, ensure fresh state next access
            self.map.clear();
        }
    }
}

impl RenderDebug for FilterTrigger {
    fn render_debug(&mut self, ui: &Ui) {
        ui.text(format!("Trait filter: {}", self.player.traits.memo()));
        debug_optional(ui, "Map filter:", self.map.get());
    }
}
