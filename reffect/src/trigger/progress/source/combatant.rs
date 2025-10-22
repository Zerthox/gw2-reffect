use crate::render::{enum_combo, helper};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, Ui};
use reffect_core::context::{BuffMap, CombatantResources, Context};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, VariantArray};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    EnumIter,
    VariantArray,
    AsRefStr,
    Serialize,
    Deserialize,
)]
pub enum Combatant {
    /// Player controlled character.
    Player,

    /// Pet of player controlled character.
    Pet,

    /// Current target.
    Target,

    /// Group member.
    #[strum(serialize = "Group Member 1")]
    GroupMember1,

    /// Group member.
    #[strum(serialize = "Group Member 2")]
    GroupMember2,

    /// Group member.
    #[strum(serialize = "Group Member 3")]
    GroupMember3,

    /// Group member.
    #[strum(serialize = "Group Member 4")]
    GroupMember4,
}

impl Default for Combatant {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl ConstDefault for Combatant {
    const DEFAULT: Self = Self::Player;
}

impl Combatant {
    pub fn buffs<'ctx>(&self, ctx: &'ctx Context) -> Option<&'ctx BuffMap> {
        match self {
            Self::Player => Some(&ctx.player.buff_info.as_ref().ok()?.buffs),
            Self::Pet => None,
            Self::Target => ctx.target.buffs.as_ref().ok(),
            Self::GroupMember1 => ctx.group.as_ref().ok()?.members[0].buffs.as_ref().ok(),
            Self::GroupMember2 => ctx.group.as_ref().ok()?.members[1].buffs.as_ref().ok(),
            Self::GroupMember3 => ctx.group.as_ref().ok()?.members[2].buffs.as_ref().ok(),
            Self::GroupMember4 => ctx.group.as_ref().ok()?.members[3].buffs.as_ref().ok(),
        }
    }

    pub fn resources<'ctx>(&self, ctx: &'ctx Context) -> Option<&'ctx CombatantResources> {
        match self {
            Self::Player => Some(&ctx.player.resources.as_ref().ok()?.combatant),
            Self::Pet => None,
            Self::Target => ctx.target.resources.as_ref().ok(),
            Self::GroupMember1 => ctx.group.as_ref().ok()?.members[0].resources.as_ref().ok(),
            Self::GroupMember2 => ctx.group.as_ref().ok()?.members[1].resources.as_ref().ok(),
            Self::GroupMember3 => ctx.group.as_ref().ok()?.members[2].resources.as_ref().ok(),
            Self::GroupMember4 => ctx.group.as_ref().ok()?.members[3].resources.as_ref().ok(),
        }
    }

    pub fn render_options(&mut self, ui: &Ui) -> bool {
        let mut changed = false;

        changed |= enum_combo(ui, "Combatant", self, ComboBoxFlags::empty()).is_some();
        helper(ui, || {
            ui.text("Combatant character to use");
            ui.text("Player: controlled character");
            ui.text("Pet: Ranger pet or Mechanist mech");
        });

        changed
    }
}
