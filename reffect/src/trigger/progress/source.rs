use super::ProgressActive;
use crate::{
    action::Action,
    context::{Buff, Category, Context, SkillInfo, Slot},
    enums::check_variant_array,
    error::Error,
    internal::{Interface, Internal},
    render::{Validation, enum_combo, helper, input_skill_id},
};
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};
use serde_with::{OneOrMany, formats::PreferMany, serde_as};
use strum::{AsRefStr, EnumCount, EnumIter, IntoStaticStr, VariantArray};

#[serde_as]
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

    // Defiance
    Defiance,

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
        Self::Buff(Vec::new()),
        Self::Ability(Vec::new()),
        Self::SkillbarSlot(Slot::DEFAULT),
        Self::Health,
        Self::Barrier,
        Self::Defiance,
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
            Self::Buff(ref ids) => {
                let buff_info = ctx.player.buff_info.as_ref().ok()?;
                let mut combined = Buff::empty();
                for id in ids {
                    if let Some(buff) = buff_info
                        .buffs
                        .get(id)
                        .filter(|buff| buff.runout_time > ctx.now)
                    {
                        combined.stacks += buff.stacks;
                        combined.apply_time = combined.apply_time.max(buff.apply_time);
                        combined.runout_time = combined.runout_time.max(buff.runout_time);
                    }
                }
                Some(ProgressActive::from_buff(
                    ids.first().copied().unwrap_or(0),
                    &combined,
                ))
            }
            Self::SkillbarSlot(slot) => {
                let skillbar = ctx.player.skillbar.as_ref().ok()?;
                let ability = skillbar.slot(slot)?;
                Some(ProgressActive::from_ability(ability))
            }
            Self::Ability(ref ids) => {
                let skillbar = ctx.player.skillbar.as_ref().ok()?;
                ids.iter()
                    .copied()
                    .filter(|id| *id > 0)
                    .find_map(|id| skillbar.ability(id))
                    .map(|ability| ProgressActive::from_ability(ability))
            }
            Self::Health => {
                let resources = ctx.player.resources.as_ref().ok()?;
                resources.health.clone().try_into().ok()
            }
            Self::Barrier => {
                let resources = ctx.player.resources.as_ref().ok()?;
                resources.barrier.clone().try_into().ok()
            }
            Self::Defiance => {
                let resources = ctx.player.resources.as_ref().ok()?;
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
                    .player
                    .skillbar
                    .as_ref()
                    .ok()
                    .and_then(|skillbar| skillbar.slot(slot))
                    .map(|ability| ability.id)
                    .unwrap_or_default();
                ProgressActive::edit_ability(skill, progress, ctx.now)
            }
            Self::Health => ProgressActive::edit_resource(progress, 15_000.0),
            Self::Barrier => ProgressActive::edit_resource(0.5 * progress, 15_000.0),
            Self::Defiance | Self::Endurance => ProgressActive::edit_resource(progress, 100.0),
            Self::PrimaryResource | Self::SecondaryResource => {
                ProgressActive::edit_resource(progress, 30.0)
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

    pub fn render_options(&mut self, ui: &Ui) -> bool {
        let mut changed = false;

        if let Some(prev) = enum_combo(ui, "Trigger", self, ComboBoxFlags::HEIGHT_LARGE) {
            changed = true;
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
            Self::Ability(ids) => {
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
            Self::SkillbarSlot(slot) => {
                changed |= enum_combo(ui, "Slot", slot, ComboBoxFlags::HEIGHT_LARGEST).is_some();
            }
            _ => {}
        }

        changed
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
