use super::{MapTrigger, PlayerTrigger, map::legacy::MapTriggerLegacy};
use crate::{
    context::{Context, Updateable},
    serde::migrate,
};
use const_default::ConstDefault;
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

/// Visibility filter.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct FilterTrigger {
    /// Whether the element is enabled.
    pub enabled: bool,

    /// Player configuration.
    pub player: PlayerTrigger,

    /// Map configuration.
    #[serde(deserialize_with = "migrate::<_, _, MapTriggerLegacy>")]
    pub map: MapTrigger,

    #[serde(skip)]
    dirty: bool,
}

impl ConstDefault for FilterTrigger {
    const DEFAULT: Self = Self {
        enabled: true,
        player: PlayerTrigger::DEFAULT,
        map: MapTrigger::DEFAULT,
        dirty: false,
    };
}

impl Default for FilterTrigger {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl FilterTrigger {
    /// Returns a filter for non-competitive maps.
    pub fn non_competitive() -> Self {
        Self {
            map: MapTrigger::non_competitive(),
            ..Self::DEFAULT
        }
    }

    /// Loads the filter.
    pub fn load(&mut self) {
        self.player.load();
    }

    /// Whether the filter is active.
    pub fn is_active(&self, ctx: &Context) -> bool {
        self.enabled && self.player.is_active(ctx) && self.map.is_active()
    }

    /// Updates the filter if needed and returns update information.
    pub fn update(&mut self, ctx: &Context, force: bool) -> ChildUpdates {
        let before = self.allow_child_updates();
        self.update_if_force_or_need(ctx, force);
        if self.dirty {
            self.dirty = false;
            ChildUpdates {
                allow: true,
                force: true,
            }
        } else {
            let after = self.allow_child_updates();
            ChildUpdates {
                allow: after,
                force: after != before,
            }
        }
    }

    /// Whether child updates are allowed.
    fn allow_child_updates(&self) -> bool {
        self.enabled
            && self.player.build.is_active()
            && self.player.gear.is_active()
            && self.map.is_active()
    }

    /// Marks the filter as dirty, forcing the next round of updates.
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn render_enabled(&mut self, ui: &Ui, _ctx: &Context) {
        if ui.checkbox("Enabled", &mut self.enabled) {
            self.dirty |= self.enabled;
        }
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

/// Child update information.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChildUpdates {
    /// Whether child updates are allowed or skipped.
    pub allow: bool,

    /// Whether child updates need to be forced.
    pub force: bool,
}
