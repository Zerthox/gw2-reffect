use super::{MapTrigger, PlayerTrigger, Trigger, map::legacy::MapTriggerLegacy};
use crate::{context::Context, serde::migrate};
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

    pub fn update(&mut self, ctx: &Context) {
        self.player.traits.update(ctx);
        self.map.update(ctx);
    }

    pub fn render_options(&mut self, ui: &Ui, ctx: &Context) {
        self.player.render_options(ui, ctx);

        ui.spacing();
        self.map.render_options(ui, ctx);
    }

    pub fn render_debug(&mut self, ui: &Ui, ctx: &Context) {
        ui.text(format!(
            "Trait filter: {}",
            self.player.traits.is_active(ctx)
        ));
        ui.text(format!("Map filter: {}", self.map.is_active(ctx)));
    }
}

impl Trigger for FilterTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        self.player.is_active(ctx) && self.map.is_active(ctx)
    }
}
