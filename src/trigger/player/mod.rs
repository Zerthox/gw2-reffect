mod combat;
mod traits;

pub use self::{combat::*, traits::*};

use super::Trigger;
use crate::{
    context::{Context, EditState, Mount, Profession, Specialization},
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

    #[serde(skip_serializing)]
    #[serde(with = "serde_bitflags")]
    profs: BitFlags<Profession>, // TODO: remove after grace period

    #[serde(with = "serde_bitflags")]
    pub specs: BitFlags<Specialization>,

    #[serde(flatten)]
    pub traits: TraitTrigger,

    #[serde(with = "serde_bitflags")]
    pub mounts: BitFlags<Mount>,
}

impl PlayerTrigger {
    pub fn load(&mut self) {
        // translate old profs to specs if specs empty
        // TODO: remove after grace period
        if self.specs.is_empty() {
            for prof in self.profs.iter() {
                self.specs.insert(prof.specializations());
            }
        }
    }
}

impl Trigger for PlayerTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        self.traits.is_active(ctx)
            && self.combat.is_active(ctx)
            && check_bitflags_optional(self.specs, ctx.player.spec.ok())
            && check_bitflags_optional(self.mounts, ctx.player.mount.ok())
    }
}

impl RenderOptions for PlayerTrigger {
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        self.combat.render_options(ui, state);

        ui.spacing();

        enum_combo_bitflags(
            ui,
            "Specialization",
            &mut self.specs,
            ComboBoxFlags::HEIGHT_LARGE,
        );
        self.traits.render_options(ui, state);

        ui.spacing();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prof_transition() {
        let mut trigger = PlayerTrigger {
            profs: Profession::Guardian | Profession::Necromancer,
            ..Default::default()
        };
        trigger.load();
        assert_eq!(
            trigger.specs,
            Specialization::Guardian
                | Specialization::Dragonhunter
                | Specialization::Firebrand
                | Specialization::Willbender
                | Specialization::Necromancer
                | Specialization::Reaper
                | Specialization::Scourge
                | Specialization::Harbinger
        );

        let mut trigger = PlayerTrigger {
            profs: Profession::Guardian | Profession::Necromancer,
            specs: Specialization::Dragonhunter | Specialization::Reaper,
            ..Default::default()
        };
        trigger.load();
        assert_eq!(
            trigger.specs,
            Specialization::Dragonhunter | Specialization::Reaper
        );
    }
}
