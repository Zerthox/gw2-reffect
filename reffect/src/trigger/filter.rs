use super::{
    map::legacy::MapTriggerLegacy, memo::MemoizedTrigger, MapTrigger, PlayerTrigger, Trigger,
};
use crate::{
    context::Context,
    elements::RenderState,
    render::{RenderDebug, RenderOptions},
    render_util::debug_optional,
    serde::migrate,
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FilterTrigger {
    pub player: PlayerTrigger, // player not memoized due to combat & mount

    #[serde(deserialize_with = "migrate::<_, _, MapTriggerLegacy>")]
    pub map: MapTrigger,
}

impl FilterTrigger {
    pub fn load(&mut self) {
        self.player.load();
    }
}

impl Trigger for FilterTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        self.player.is_active(ctx) && self.map.is_active(ctx)
    }

    fn is_active_or_edit(&mut self, ctx: &Context, state: &RenderState) -> bool {
        if state.is_edit(ctx) {
            self.map.update(ctx);
            true
        } else {
            !ctx.edit.is_editing() && self.is_active(ctx)
        }
    }
}

impl RenderOptions for FilterTrigger {
    fn render_options(&mut self, ui: &Ui, ctx: &Context) {
        self.player.render_options(ui, ctx);

        ui.spacing();
        let changed = self.map.render_options(ui, ctx);
        if changed {
            // map trigger changed, ensure fresh state next access
            self.map.clear();
        }
    }
}

impl RenderDebug for FilterTrigger {
    fn render_debug(&mut self, ui: &Ui, _ctx: &Context) {
        debug_optional(ui, "Trait filter", self.player.traits.get());
        debug_optional(ui, "Map filter", self.map.get());
    }
}
