use crate::render::{Validation, enum_combo, helper};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, Ui};
use reffect_core::context::{BuffMap, CombatantResources, Context};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoStaticStr, VariantArray};

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
    Display,
    AsRefStr,
    IntoStaticStr,
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

    pub fn validate_buff(&self) -> Validation<impl AsRef<str> + 'static> {
        match self {
            Self::Player | Self::Target => Validation::Ok,
            Self::GroupMember1 | Self::GroupMember2 | Self::GroupMember3 | Self::GroupMember4 => {
                Validation::Warn("Group member only supports boon & condition effects")
            }
            Self::Pet => Validation::Error("Pet does not support effects"),
        }
    }

    pub fn validate_health_barrier(&self) -> Validation<impl AsRef<str> + 'static> {
        match self {
            Self::Player | Self::Pet => Validation::Ok,
            Self::Target => Validation::Warn("Target only supports normalized health/barrier"),
            Self::GroupMember1 | Self::GroupMember2 | Self::GroupMember3 | Self::GroupMember4 => {
                Validation::Warn("Group member only supports normalized health/barrier")
            }
        }
    }

    pub fn validate_defiance(&self) -> Validation<impl AsRef<str> + 'static> {
        match self {
            Self::Player | Self::Target => Validation::Ok,
            Self::Pet => Validation::Error("Pet does not support defiance"),
            Self::GroupMember1 | Self::GroupMember2 | Self::GroupMember3 | Self::GroupMember4 => {
                Validation::Error("Group member does not support defiance")
            }
        }
    }

    pub fn render_options(&mut self, ui: &Ui, validation: Validation<impl AsRef<str>>) -> bool {
        let mut changed = false;

        changed |= validation
            .for_item(ui, || {
                enum_combo(ui, "Combatant", self, ComboBoxFlags::empty())
            })
            .is_some();
        helper(ui, || {
            ui.text("Combatant to use");
            ui.text("Player: controlled character");
            ui.text("Pet: Ranger pet or Mechanist mech");
        });

        changed
    }
}
