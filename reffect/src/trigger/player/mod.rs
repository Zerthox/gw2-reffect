mod combat;
mod traits;

pub use self::{combat::*, traits::*};

use super::{check_bitflags_optional, Trigger};
use crate::{
    context::{Context, EditState, Mount, Profession, Specialization},
    render::RenderOptions,
    render_util::enum_combo_bitflags,
    serde_bitflags,
};
use enumflags2::BitFlags;
use nexus::imgui::{ComboBoxFlags, Ui};
use reffect_internal::Weapon;
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

    #[serde(with = "serde_bitflags")]
    pub weapons: BitFlags<Weapon>,

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

    pub fn specs_active(&self, ctx: &Context) -> bool {
        check_bitflags_optional(self.specs, ctx.player.spec.ok())
    }

    pub fn weapons_active(&self, ctx: &Context) -> bool {
        check_bitflags_optional(self.weapons, ctx.player.weapons())
    }

    pub fn mounts_active(&self, ctx: &Context) -> bool {
        check_bitflags_optional(self.mounts, ctx.player.mount.ok())
    }
}

impl Trigger for PlayerTrigger {
    fn is_active(&mut self, ctx: &Context) -> bool {
        self.traits.is_active(ctx)
            && self.combat.is_active(ctx)
            && self.specs_active(ctx)
            && self.weapons_active(ctx)
            && self.mounts_active(ctx)
    }
}

impl RenderOptions for PlayerTrigger {
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        self.combat.render_options(ui, state);

        enum_combo_bitflags(
            ui,
            "Specialization",
            &mut self.specs,
            ComboBoxFlags::HEIGHT_LARGE,
        );

        enum_combo_bitflags(
            ui,
            "Weapons",
            &mut self.weapons,
            ComboBoxFlags::HEIGHT_LARGE,
        );

        self.traits.render_options(ui, state);

        enum_combo_bitflags(ui, "Mount", &mut self.mounts, ComboBoxFlags::HEIGHT_LARGE);
    }
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
