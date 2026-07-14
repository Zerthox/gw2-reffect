use super::{MapTrigger, PlayerTrigger, map::legacy::MapTriggerLegacy};
use crate::{
    context::{Context, Updateable},
    serde::migrate,
};
use const_default::ConstDefault;
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

/// Visibility filter.
#[derive(Debug, Default, ConstDefault, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct FilterTrigger {
    /// Player configuration.
    pub player: PlayerTrigger,

    /// Map configuration.
    #[serde(deserialize_with = "migrate::<_, _, MapTriggerLegacy>")]
    pub map: MapTrigger,
}

impl FilterTrigger {
    pub fn load(&mut self) {
        self.player.load();
    }

    pub fn is_active(&self, ctx: &Context) -> bool {
        self.player.is_active(ctx) && self.map.is_active()
    }

    /// Updates the filter if needed and returns update information.
    pub fn update(&mut self, ctx: &Context, force: bool) -> ChildUpdates {
        let before = self.allow_child_updates();
        self.update_if_force_or_need(ctx, force);
        let after = self.allow_child_updates();
        ChildUpdates {
            allow: after,
            force: after != before,
        }
    }

    pub fn allow_child_updates(&self) -> bool {
        self.player.build.is_active() && self.player.gear.is_active() && self.map.is_active()
    }

    pub fn render_options(&mut self, ui: &Ui, ctx: &Context) {
        self.player.render_options(ui, ctx);

        ui.spacing();
        self.map.render_options(ui, ctx);
    }

    pub fn render_debug(&mut self, ui: &Ui, _ctx: &Context) {
        ui.text(format!("Gear filter: {}", self.player.gear.is_active()));
        ui.text(format!("Build filter: {}", self.player.build.is_active()));
        ui.text(format!("Map filter: {}", self.map.is_active()));
    }
}

impl Updateable for FilterTrigger {
    fn needs_update(&self, ctx: &Context) -> bool {
        self.map.needs_update(ctx) || self.player.needs_update(ctx)
    }

    fn force_update(&mut self, ctx: &Context) {
        self.player.force_update(ctx);
        self.map.force_update(ctx);
    }

    fn update_if_need(&mut self, ctx: &Context) {
        self.player.update_if_need(ctx);
        self.map.update_if_need(ctx);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChildUpdates {
    pub allow: bool,
    pub force: bool,
}
