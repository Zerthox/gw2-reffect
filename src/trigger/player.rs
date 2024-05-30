use super::{CombatTrigger, Trigger};
use crate::{
    context::{Context, Mount, Profession, Specialization},
    render_util::enum_combo_bitflags,
    serde_bitflags,
    traits::RenderOptions,
};
use enumflags2::{BitFlag, BitFlags};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct PlayerTrigger {
    pub combat: CombatTrigger,

    #[serde(with = "serde_bitflags")]
    pub profs: BitFlags<Profession>,

    #[serde(with = "serde_bitflags")]
    pub specs: BitFlags<Specialization>,

    #[serde(with = "serde_bitflags")]
    pub mounts: BitFlags<Mount>,
}

impl Trigger for PlayerTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        self.combat.is_active(ctx)
            && check_bitflags_optional(self.profs, ctx.player.prof.ok())
            && check_bitflags_optional(self.specs, ctx.player.spec.ok())
            && check_bitflags_optional(self.mounts, ctx.player.mount.ok())
    }
}

impl RenderOptions for PlayerTrigger {
    fn render_options(&mut self, ui: &Ui) {
        self.combat.render_options(ui);

        ui.spacing();
        enum_combo_bitflags(
            ui,
            "Profession",
            &mut self.profs,
            ComboBoxFlags::HEIGHT_LARGE,
        );

        enum_combo_bitflags(
            ui,
            "Specialization",
            &mut self.specs,
            ComboBoxFlags::HEIGHT_LARGE,
        );

        enum_combo_bitflags(ui, "Mount", &mut self.mounts, ComboBoxFlags::HEIGHT_LARGE);
    }
}

fn check_bitflags<T>(flags: BitFlags<T>, value: T) -> bool
where
    T: Copy + BitFlag,
{
    flags.is_empty() || flags.contains(value)
}

fn check_bitflags_optional<T>(flags: BitFlags<T>, value: Option<T>) -> bool
where
    T: Copy + BitFlag,
{
    value
        .map(|value| check_bitflags(flags, value))
        .unwrap_or(true)
}
