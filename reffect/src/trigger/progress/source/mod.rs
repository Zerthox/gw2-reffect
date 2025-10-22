mod combatant;
mod legacy;

pub use self::combatant::*;

use super::ProgressActive;
use crate::{
    action::Action,
    context::{Buff, Category, Context, SkillInfo, Slot},
    enums::check_variant_array,
    error::Error,
    internal::{Interface, Internal},
    render::{Validation, enum_combo, helper, input_skill_id},
};
use const_default::ConstDefault;
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumCount, EnumIter, IntoStaticStr, VariantArray};

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    AsRefStr,
    IntoStaticStr,
    EnumIter,
    EnumCount,
    Serialize,
    Deserialize,
)]
#[serde(tag = "type")]
pub enum ProgressSource {
    /// Inherit from above.
    #[default]
    Inherit,

    /// Always active, no associated progress.
    #[serde(alias = "None")]
    Always,

    /// Buff ids, multiple matches are merged.
    #[serde(alias = "Single")]
    #[serde(alias = "Has")]
    #[serde(alias = "Any")]
    #[serde(alias = "AnyBuff")]
    #[serde(alias = "Effect")]
    #[strum(serialize = "Effect")]
    Buff {
        #[serde(default)]
        combatant: Combatant,

        #[serde(default)]
        ids: Vec<u32>,
    },

    /// Ability ids, first match is used.
    #[strum(serialize = "Ability Recharge")]
    Ability {
        #[serde(default)]
        ids: Vec<u32>,
    },

    /// Skillbar slot.
    #[strum(serialize = "Slot Recharge")]
    SkillbarSlot {
        #[serde(default)]
        slot: Slot,
    },

    /// Health.
    Health {
        #[serde(default)]
        combatant: Combatant,
    },

    /// Barrier.
    Barrier {
        #[serde(default)]
        combatant: Combatant,
    },

    // Defiance
    Defiance {
        #[serde(default)]
        combatant: Combatant,
    },

    /// Endurance.
    Endurance,

    /// Primary profession resource.
    #[strum(serialize = "Primary Resource")]
    PrimaryResource,

    /// Secondary profession resource.
    #[strum(serialize = "Secondary Resource")]
    SecondaryResource,
}

impl VariantArray for ProgressSource {
    const VARIANTS: &'static [Self] = &[
        Self::Inherit,
        Self::Always,
        Self::Buff {
            combatant: Combatant::DEFAULT,
            ids: Vec::new(),
        },
        Self::Ability { ids: Vec::new() },
        Self::SkillbarSlot {
            slot: Slot::DEFAULT,
        },
        Self::Health {
            combatant: Combatant::DEFAULT,
        },
        Self::Barrier {
            combatant: Combatant::DEFAULT,
        },
        Self::Defiance {
            combatant: Combatant::DEFAULT,
        },
        Self::Endurance,
        Self::PrimaryResource,
        Self::SecondaryResource,
    ];
}

const _: () = check_variant_array::<ProgressSource>();

impl ProgressSource {
    pub fn no_threshold(&self) -> bool {
        matches!(self, Self::Always)
    }

    pub fn progress(
        &self,
        ctx: &Context,
        parent: Option<&ProgressActive>,
    ) -> Option<ProgressActive> {
        match *self {
            Self::Inherit => parent.cloned(),
            Self::Always => Some(ProgressActive::dummy()),
            Self::Buff { combatant, ref ids } => {
                let buffs = combatant.buffs(ctx)?;
                let mut combined = Buff::empty();
                let mut found_id = 0;
                for id in ids {
                    if let Some(buff) = buffs.get(id).filter(|buff| buff.runout_time > ctx.now) {
                        combined.stacks += buff.stacks;
                        combined.apply_time = combined.apply_time.max(buff.apply_time);
                        combined.runout_time = combined.runout_time.max(buff.runout_time);
                        if found_id == 0 {
                            found_id = *id;
                        }
                    }
                }
                Some(ProgressActive::from_buff(found_id, &combined))
            }
            Self::Ability { ref ids } => {
                let skillbar = ctx.player.skillbar.as_ref().ok()?;
                ids.iter()
                    .copied()
                    .filter(|id| *id > 0)
                    .find_map(|id| skillbar.ability(id))
                    .map(ProgressActive::from_ability)
            }
            Self::SkillbarSlot { slot } => {
                let skillbar = ctx.player.skillbar.as_ref().ok()?;
                let ability = skillbar.slot(slot)?;
                Some(ProgressActive::from_ability(ability))
            }
            Self::Health { combatant } => {
                let resources = combatant.resources(ctx)?;
                resources.health.clone().try_into().ok()
            }
            Self::Barrier { combatant } => {
                let resources = combatant.resources(ctx)?;
                resources.barrier.clone().try_into().ok()
            }
            Self::Defiance { combatant } => {
                let resources = combatant.resources(ctx)?;
                Some(ProgressActive::Fixed {
                    current: resources.defiance?,
                    max: 100.0,
                })
            }
            Self::Endurance => {
                let resources = ctx.player.resources.as_ref().ok()?;
                resources.endurance.clone().try_into().ok()
            }
            Self::PrimaryResource => {
                let resources = ctx.player.resources.as_ref().ok()?;
                resources.primary.clone().try_into().ok()
            }
            Self::SecondaryResource => {
                let resources = ctx.player.resources.as_ref().ok()?;
                resources.secondary.clone().try_into().ok()
            }
        }
    }

    pub fn progress_edit(&self, ctx: &Context, parent: Option<&ProgressActive>) -> ProgressActive {
        const CYCLE: u32 = 5000;

        let passed = ctx.now % CYCLE;
        let progress = passed as f32 / CYCLE as f32;
        match *self {
            Self::Inherit => parent.cloned().unwrap_or(ProgressActive::dummy()),
            Self::Always => ProgressActive::dummy(),
            Self::Buff { ref ids, .. } => {
                let id = ids.first().copied().unwrap_or(0);
                ProgressActive::edit_buff(id, progress, ctx.now)
            }
            Self::Ability { ref ids } => {
                let id = ids.first().copied().unwrap_or(0);
                ProgressActive::edit_ability(id.into(), progress, ctx.now)
            }
            Self::SkillbarSlot { slot } => {
                let skill = ctx
                    .player
                    .skillbar
                    .as_ref()
                    .ok()
                    .and_then(|skillbar| skillbar.slot(slot))
                    .map(|ability| ability.id)
                    .unwrap_or_default();
                ProgressActive::edit_ability(skill, progress, ctx.now)
            }
            Self::Health { .. } => ProgressActive::edit_resource(progress, 15_000.0),
            Self::Barrier { .. } => ProgressActive::edit_resource(0.5 * progress, 15_000.0),
            Self::Defiance { .. } | Self::Endurance => {
                ProgressActive::edit_resource(progress, 100.0)
            }
            Self::PrimaryResource | Self::SecondaryResource => {
                ProgressActive::edit_resource(progress, 30.0)
            }
        }
    }

    pub fn into_ids(self) -> Vec<u32> {
        match self {
            Self::Buff { ids, .. } | Self::Ability { ids } => ids,
            _ => Vec::new(),
        }
    }

    pub fn combatant(&self) -> Option<&Combatant> {
        match self {
            Self::Buff { combatant, .. }
            | Self::Health { combatant }
            | Self::Barrier { combatant }
            | Self::Defiance { combatant } => Some(combatant),
            _ => None,
        }
    }

    pub fn combatant_mut(&mut self) -> Option<&mut Combatant> {
        match self {
            Self::Buff { combatant, .. }
            | Self::Health { combatant }
            | Self::Barrier { combatant }
            | Self::Defiance { combatant } => Some(combatant),
            _ => None,
        }
    }

    fn buff_validate(id: u32) -> Validation<impl AsRef<str>> {
        match Internal::get_skill_info(id) {
            Ok(SkillInfo::Buff { category, .. }) => match category {
                Category::Boon | Category::Condition | Category::Effect => {
                    Validation::Confirm(format!("{category} {id} is valid"))
                }
                Category::ScreenBorder => {
                    Validation::Warn(format!("Screen border {id} is only valid for yourself"))
                }
                Category::SquadHighlight => {
                    Validation::Warn(format!("Squad highlight {id} is only valid for your squad"))
                }
            },
            Ok(SkillInfo::Ability { .. }) => Validation::Error(format!("Id {id} is an ability")),
            Err(Error::Skill) => Validation::Error(format!("Id {id} is invalid or hidden")),
            Err(_) => Validation::Ok,
        }
    }

    fn ability_validate(id: u32) -> Validation<impl AsRef<str>> {
        match Internal::get_skill_info(id) {
            Ok(SkillInfo::Ability { .. }) => Validation::Confirm(format!("Ability {id} is valid")),
            Ok(SkillInfo::Buff { .. }) => Validation::Error(format!("Id {id} is an effect")),
            Err(Error::Skill) => Validation::Error(format!("Id {id} is invalid or hidden")),
            Err(_) => Validation::Ok,
        }
    }

    fn id_helper(ui: &Ui) {
        helper(ui, || {
            ui.text("Can be found on the wiki");
            ui.text("Supports pasting chat links");
        });
    }

    pub fn render_options(&mut self, ui: &Ui) -> bool {
        let mut changed = false;

        if let Some(prev) = enum_combo(ui, "Trigger", self, ComboBoxFlags::HEIGHT_LARGE) {
            changed = true;
            match self {
                Self::Buff { ids, .. } | Self::Ability { ids } => *ids = prev.into_ids(),
                _ => {}
            }
        }
        helper(ui, || {
            ui.text("Source of information");
            ui.text("Effect merges all matches");
            ui.text("Ability uses first match");
            ui.text("For group no effect on visibility, only passed down for inherit");
        });

        if let Some(combatant) = self.combatant_mut() {
            changed |= combatant.render_options(ui);
        }

        match self {
            Self::Buff { ids, .. } => {
                let mut action = Action::new();
                for (i, id) in ids.iter_mut().enumerate() {
                    let _id = ui.push_id(i as i32);

                    action.input_with_buttons(ui, i, || {
                        Self::buff_validate(*id).for_item(ui, || {
                            changed |= input_skill_id(ui, "##id", id, InputTextFlags::empty());
                        });
                    });

                    ui.same_line();
                    ui.text(format!("Effect Id {}", i + 1));
                    Self::id_helper(ui);
                }
                if ui.button("Add Effect") {
                    ids.push(0);
                }

                changed |= action.perform(ids);
            }
            Self::Ability { ids } => {
                let mut action = Action::new();
                for (i, id) in ids.iter_mut().enumerate() {
                    let _id = ui.push_id(i as i32);

                    action.input_with_buttons(ui, i, || {
                        Self::ability_validate(*id).for_item(ui, || {
                            changed |= input_skill_id(ui, "##id", id, InputTextFlags::empty());
                        });
                    });

                    ui.same_line();
                    ui.text(format!("Ability Id {}", i + 1));
                    Self::id_helper(ui);
                }
                if ui.button("Add Ability") {
                    ids.push(0);
                }

                changed |= action.perform(ids);
            }
            Self::SkillbarSlot { slot } => {
                changed |= enum_combo(ui, "Slot", slot, ComboBoxFlags::HEIGHT_LARGEST).is_some();
            }
            _ => {}
        }

        changed
    }
}
