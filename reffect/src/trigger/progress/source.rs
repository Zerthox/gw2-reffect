use super::ProgressActive;
use crate::{
    action::Action,
    context::Context,
    internal::{Interface, Internal},
    render::RenderOptions,
    render_util::{enum_combo, helper, impl_static_variants, input_skill_id, Validation},
};
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use reffect_internal::{Buff, Category, Error, SkillInfo, Slot};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoStaticStr};

#[derive(Debug, Default, Clone, AsRefStr, IntoStaticStr, EnumIter, Serialize, Deserialize)]
pub enum ProgressSource {
    /// Inherit from above.
    #[default]
    Inherit,

    /// Always active, no associated progress.
    #[serde(alias = "None")]
    Always,

    /// Single buff id.
    #[serde(alias = "Single")]
    #[serde(alias = "Has")]
    #[strum(serialize = "Single Effect")]
    Buff(u32),

    /// Any of the buff ids, stacks are summed.
    #[serde(alias = "Any")]
    #[strum(serialize = "Multiple Effects")]
    AnyBuff(Vec<u32>),

    #[strum(serialize = "Ability Recharge")]
    Ability(u32),

    #[strum(serialize = "Ability Ammo Recharge")]
    AbilityAmmo(u32),

    #[strum(serialize = "Slot Recharge")]
    SkillbarSlot(Slot),

    #[strum(serialize = "Slot Ammo Recharge")]
    SkillbarSlotAmmo(Slot),

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
            Self::Buff(id) => ctx.own_buffs().map(|buffs| {
                buffs
                    .get(&id)
                    .filter(|buff| buff.runout_time > ctx.now)
                    .map(|buff| ProgressActive::from_buff(id, &buff))
                    .unwrap_or_else(|| ProgressActive::empy_timed(id))
            }),
            Self::AnyBuff(ref ids) => ctx.own_buffs().map(|buffs| {
                let mut combined = Buff::empty();
                for id in ids {
                    if let Some(buff) = buffs.get(&id).filter(|buff| buff.runout_time > ctx.now) {
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
                Some(ProgressActive::from_ability(skillbar, ability))
            }
            Self::SkillbarSlotAmmo(slot) => {
                let skillbar = ctx.own_skillbar()?;
                let ability = skillbar.slot(slot)?;
                Some(ProgressActive::from_ability_ammo(skillbar, ability))
            }
            Self::Ability(id) => {
                let skillbar = ctx.own_skillbar()?;
                let ability = skillbar.ability(id)?;
                Some(ProgressActive::from_ability(skillbar, ability))
            }
            Self::AbilityAmmo(id) => {
                let skillbar = ctx.own_skillbar()?;
                let ability = skillbar.ability(id)?;
                Some(ProgressActive::from_ability_ammo(skillbar, ability))
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
            Self::Buff(id) | Self::Ability(id) | Self::AbilityAmmo(id) => {
                let apply = ctx.now - passed;
                ProgressActive::Timed {
                    id,
                    intensity: (progress * 25.0) as u32,
                    duration: apply,
                    end: apply + CYCLE,
                    rate: 1.0,
                }
            }
            Self::SkillbarSlot(slot) | Self::SkillbarSlotAmmo(slot) => {
                let apply = ctx.now - passed;
                ProgressActive::Timed {
                    id: ctx
                        .state
                        .own_skillbar
                        .as_ref()
                        .ok()
                        .and_then(|skillbar| skillbar.slot(slot))
                        .map(|ability| ability.id)
                        .unwrap_or(0),
                    intensity: (progress * 25.0) as u32,
                    duration: apply,
                    end: apply + CYCLE,
                    rate: 1.0,
                }
            }
            Self::AnyBuff(ref ids) => {
                let apply = ctx.now - passed;
                ProgressActive::Timed {
                    id: ids.first().copied().unwrap_or(0),
                    intensity: (progress * 25.0) as u32,
                    duration: apply,
                    end: apply + CYCLE,
                    rate: 1.0,
                }
            }
            Self::Health => ProgressActive::from_percent(progress, 15_000),
            Self::Barrier => ProgressActive::from_percent(0.5 * progress, 15_000),
            Self::Endurance => ProgressActive::from_percent(progress, 100),
            Self::PrimaryResource | Self::SecondaryResource => {
                ProgressActive::from_percent(progress, 30)
            }
        }
    }

    pub fn into_ids(self) -> Vec<u32> {
        match self {
            Self::Buff(id) => vec![id],
            Self::AnyBuff(ids) => ids,
            _ => Vec::new(),
        }
    }

    fn buff_validate(id: u32) -> Validation<impl AsRef<str>> {
        match Internal::get_skill_info(id) {
            Ok(SkillInfo::Buff {
                category,
                stacking: _,
            }) => {
                if category == Category::ScreenBorder {
                    Validation::Warn(format!("{category} {id} is only valid for yourself"))
                } else {
                    Validation::Confirm(format!("{category} {id} is valid"))
                }
            }
            Ok(SkillInfo::Ability) => Validation::Error(format!("Effect {id} is an ability")),
            Err(Error::SkillNotFound) => {
                Validation::Error(format!("Effect {id} is invalid or hidden"))
            }
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
                Self::Buff(id) => {
                    if let Some(first) = prev.into_ids().first() {
                        *id = *first;
                    }
                }
                Self::AnyBuff(ids) => *ids = prev.into_ids(),
                _ => {}
            }
        }
        helper(ui, || ui.text("Source of information"));

        match self {
            Self::Buff(id) => {
                Self::buff_validate(*id).for_item(ui, || {
                    input_skill_id(ui, "Effect Id", id, InputTextFlags::empty());
                });
                Self::id_helper(ui);
            }
            Self::AnyBuff(ids) => {
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
            Self::Ability(id) | Self::AbilityAmmo(id) => {
                input_skill_id(ui, "Ability Id", id, InputTextFlags::empty());
                Self::id_helper(ui);
            }
            Self::SkillbarSlot(slot) | Self::SkillbarSlotAmmo(slot) => {
                enum_combo(ui, "Slot", slot, ComboBoxFlags::HEIGHT_LARGEST);
            }
            _ => {}
        }
    }
}
