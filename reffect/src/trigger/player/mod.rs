mod build;
mod combat;
mod gear;
mod item;
mod traits;

pub use self::{build::*, combat::*, gear::*, item::*, traits::*};

use super::TriggerMode;
use crate::{
    context::{Context, Mount, Updateable},
    render::enum_combo_bitflags,
    serde::bitflags,
};
use const_default::ConstDefault;
use enumflags2::BitFlags;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default)]
pub struct PlayerTrigger {
    /// Combat configuration.
    pub combat: CombatTrigger,

    /// Gear configuration.
    #[serde(flatten)]
    pub gear: GearTrigger,

    /// Build configuration.
    #[serde(flatten)]
    pub build: BuildTrigger,

    /// Current mount.
    #[serde(with = "bitflags")]
    #[cfg_attr(feature = "schema", schemars(with = "bitflags::Schema<Mount>"))]
    pub mounts: BitFlags<Mount>,
}

impl ConstDefault for PlayerTrigger {
    const DEFAULT: Self = Self {
        combat: CombatTrigger::DEFAULT,
        gear: GearTrigger::DEFAULT,
        build: BuildTrigger::DEFAULT,
        mounts: BitFlags::EMPTY,
    };
}

impl Default for PlayerTrigger {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl PlayerTrigger {
    pub fn load(&mut self) {
        self.build.load();
    }

    pub fn is_active(&self, ctx: &Context) -> bool {
        self.build.is_active()
            && self.gear.is_active()
            && self.combat.is_active(ctx)
            && self.mounts_active(ctx)
    }

    pub fn mounts_active(&self, ctx: &Context) -> bool {
        TriggerMode::Any.check_flags_optional(self.mounts, ctx.player.mount.ok())
    }
}

impl Updateable for PlayerTrigger {
    fn needs_update(&self, ctx: &Context) -> bool {
        self.gear.needs_update(ctx) || self.build.needs_update(ctx)
    }

    fn force_update(&mut self, ctx: &Context) {
        self.build.force_update(ctx);
        self.gear.force_update(ctx);
    }

    fn update_if_need(&mut self, ctx: &Context) {
        self.build.update_if_need(ctx);
        self.gear.update_if_need(ctx);
    }
}

impl PlayerTrigger {
    pub fn render_options(&mut self, ui: &Ui, ctx: &Context) {
        self.combat.render_options(ui);

        ui.spacing();
        self.gear.render_options(ui, ctx);

        ui.spacing();
        self.build.render_options(ui, ctx);

        ui.spacing();
        enum_combo_bitflags(ui, "Mount", &mut self.mounts, ComboBoxFlags::HEIGHT_LARGE);
    }
}
