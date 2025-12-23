mod build;
mod combat;
mod gear;
mod traits;

pub use self::{build::*, combat::*, gear::*, traits::*};

use super::{Trigger, TriggerMode};
use crate::{
    context::{Context, Mount},
    render::enum_combo_bitflags,
    serde::bitflags,
    trigger::MemoizedTrigger,
};
use const_default::ConstDefault;
use enumflags2::BitFlags;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerTrigger {
    pub combat: CombatTrigger,

    #[serde(flatten)]
    pub gear: GearTrigger,

    #[serde(flatten)]
    pub build: BuildTrigger,

    #[serde(with = "bitflags")]
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

    pub fn update(&mut self, ctx: &Context) {
        self.gear.update_full(ctx);
        self.build.update(ctx);
    }

    pub fn mounts_active(&self, ctx: &Context) -> bool {
        TriggerMode::Any.check_flags_optional(self.mounts, ctx.player.mount.ok())
    }
}

impl Trigger for PlayerTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        self.build.is_active(ctx)
            && self.combat.is_active(ctx)
            && self.gear.is_active(ctx)
            && self.build.is_active(ctx)
            && self.mounts_active(ctx)
    }
}

impl PlayerTrigger {
    pub fn render_options(&mut self, ui: &Ui, ctx: &Context) {
        self.combat.render_options(ui);
        self.gear.render_options(ui, ctx);
        self.build.render_options(ui, ctx);

        enum_combo_bitflags(ui, "Mount", &mut self.mounts, ComboBoxFlags::HEIGHT_LARGE);
    }
}
