use super::{ProgressActive, Skill};
use crate::{
    action::Action,
    context::Context,
    internal::{Buff, Category, Error, Interface, Internal, SkillInfo, Slot},
    render::{enum_combo, helper, impl_static_variants, input_skill_id, RenderOptions, Validation},
};
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};
use serde_with::{formats::PreferMany, serde_as, OneOrMany};
use strum::{AsRefStr, EnumIter, IntoStaticStr};

#[serde_as]
#[derive(
    Debug, Default, Clone, PartialEq, AsRefStr, IntoStaticStr, EnumIter, Serialize, Deserialize,
)]
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
    Buff(#[serde_as(as = "OneOrMany<_, PreferMany>")] Vec<u32>),

    /// Ability ids, first match is used.
    #[strum(serialize = "Ability Recharge")]
    Ability(Vec<u32>),

    /// Skillbar slot.
    #[strum(serialize = "Slot Recharge")]
    SkillbarSlot(Slot),

    /// Health.
    Health,

    /// Barrier.
    Barrier,

    /// Endurance.
    Endurance,

    /// Primary profession resource.
    #[strum(serialize = "Primary Resource")]
    PrimaryResource,

    /// Secondary profession resource.
    #[strum(serialize = "Secondary Resource")]
    SecondaryResource,
}

impl_static_variants!(ProgressSource);

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
            Self::Buff(ref ids) => ctx.own_buffs().map(|buffs| {
                let mut combined = Buff::empty();
                for id in ids {
                    if let Some(buff) = buffs.get(id).filter(|buff| buff.runout_time > ctx.now) {
                        combined.stacks += buff.stacks;
                        combined.apply_time = combined.apply_time.max(buff.apply_time);
                        combined.runout_time = combined.runout_time.max(buff.runout_time);
                    }
                }
                ProgressActive::from_buff(ids.first().copied().unwrap_or(0), &combined)
            }),
            Self::SkillbarSlot(slot) => {
                let skillbar = ctx.own_skillbar()?;
                let ability = skillbar.slot(slot)?;
                let skill = Skill::from_slot(skillbar, slot);
                Some(ProgressActive::from_ability(skill, ability))
            }
            Self::Ability(ref ids) => {
                let skillbar = ctx.own_skillbar()?;
                ids.iter()
                    .copied()
                    .filter(|id| *id > 0)
                    .find_map(|id| skillbar.ability(id))
                    .map(|ability| ProgressActive::from_ability(ability.id.into(), ability))
            }
            Self::Health => ctx.own_resources()?.health.clone().try_into().ok(),
            Self::Barrier => ctx.own_resources()?.barrier.clone().try_into().ok(),
            Self::Endurance => ctx.own_resources()?.endurance.clone().try_into().ok(),
            Self::PrimaryResource => ctx.own_resources()?.primary.clone().try_into().ok(),
            Self::SecondaryResource => ctx.own_resources()?.secondary.clone().try_into().ok(),
        }
    }

    pub fn progress_edit(&self, ctx: &Context, parent: Option<&ProgressActive>) -> ProgressActive {
        const CYCLE: u32 = 5000;

        let passed = ctx.now % CYCLE;
        let progress = passed as f32 / CYCLE as f32;
        match *self {
            Self::Inherit => parent.cloned().unwrap_or(ProgressActive::dummy()),
            Self::Always => ProgressActive::dummy(),
            Self::Buff(ref ids) => {
                let id = ids.first().copied().unwrap_or(0);
                ProgressActive::edit_buff(id, progress, ctx.now)
            }
            Self::Ability(ref ids) => {
                let id = ids.first().copied().unwrap_or(0);
                ProgressActive::edit_ability(id.into(), progress, ctx.now)
            }
            Self::SkillbarSlot(slot) => {
                let skill = ctx
                    .own_skillbar()
                    .map(|skillbar| Skill::from_slot(skillbar, slot))
                    .unwrap_or_default();
                ProgressActive::edit_ability(skill, progress, ctx.now)
            }
            Self::Health => ProgressActive::edit_resource(progress, 15_000),
            Self::Barrier => ProgressActive::edit_resource(0.5 * progress, 15_000),
            Self::Endurance => ProgressActive::edit_resource(progress, 100),
            Self::PrimaryResource | Self::SecondaryResource => {
                ProgressActive::edit_resource(progress, 30)
            }
        }
    }

    pub fn into_ids(self) -> Vec<u32> {
        match self {
            Self::Buff(ids) => ids,
            Self::Ability(ids) => ids,
            _ => Vec::new(),
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
            Err(Error::SkillNotFound) => Validation::Error(format!("Id {id} is invalid or hidden")),
            Err(_) => Validation::Ok,
        }
    }

    fn ability_validate(id: u32) -> Validation<impl AsRef<str>> {
        match Internal::get_skill_info(id) {
            Ok(SkillInfo::Ability { .. }) => Validation::Confirm(format!("Ability {id} is valid")),
            Ok(SkillInfo::Buff { .. }) => Validation::Error(format!("Id {id} is an effect")),
            Err(Error::SkillNotFound) => Validation::Error(format!("Id {id} is invalid or hidden")),
            Err(_) => Validation::Ok,
        }
    }

    fn id_helper(ui: &Ui) {
        helper(ui, || {
            ui.text("Can be found on the wiki");
            ui.text("Supports pasting chat links");
        });
    }
}

impl RenderOptions for ProgressSource {
    fn render_options(&mut self, ui: &Ui, _ctx: &Context) {
        if let Some(prev) = enum_combo(ui, "Trigger", self, ComboBoxFlags::HEIGHT_LARGE) {
            match self {
                Self::Buff(ids) => *ids = prev.into_ids(),
                Self::Ability(ids) => *ids = prev.into_ids(),
                _ => {}
            }
        }
        helper(ui, || {
            ui.text("Source of information");
            ui.text("Effect merges all matches");
            ui.text("Ability uses first match");
            ui.text("For group no effect on visibility, only passed down for inherit");
        });

        match self {
            Self::Buff(ids) => {
                let mut action = Action::new();
                for (i, id) in ids.iter_mut().enumerate() {
                    let _id = ui.push_id(i as i32);

                    action.input_with_buttons(ui, i, || {
                        Self::buff_validate(*id).for_item(ui, || {
                            input_skill_id(ui, "##id", id, InputTextFlags::empty());
                        });
                    });

                    ui.same_line();
                    ui.text(format!("Effect Id {}", i + 1));
                    Self::id_helper(ui);
                }
                if ui.button("Add Effect") {
                    ids.push(0);
                }

                action.perform(ids);
            }
            Self::Ability(ids) => {
                let mut action = Action::new();
                for (i, id) in ids.iter_mut().enumerate() {
                    let _id = ui.push_id(i as i32);

                    action.input_with_buttons(ui, i, || {
                        Self::ability_validate(*id).for_item(ui, || {
                            input_skill_id(ui, "##id", id, InputTextFlags::empty());
                        });
                    });

                    ui.same_line();
                    ui.text(format!("Ability Id {}", i + 1));
                    Self::id_helper(ui);
                }
                if ui.button("Add Ability") {
                    ids.push(0);
                }

                action.perform(ids);
            }
            Self::SkillbarSlot(slot) => {
                enum_combo(ui, "Slot", slot, ComboBoxFlags::HEIGHT_LARGEST);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrate() {
        let json = r#"{ "Buff": 123 }"#;
        let result = serde_json::from_str::<ProgressSource>(&json).expect("failed to deserialize");
        assert_eq!(result, ProgressSource::Buff(vec![123]));
    }
}
