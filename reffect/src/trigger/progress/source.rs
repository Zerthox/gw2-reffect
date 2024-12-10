use super::ProgressActive;
use crate::{
    action::Action,
    context::{Context, EditState},
    internal::{Interface, Internal, Resource},
    render::RenderOptions,
    render_util::{enum_combo, helper, impl_static_variants, input_skill_id, Validation},
};
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use reffect_internal::Category;
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
        match self {
            Self::Inherit => parent.cloned(),
            Self::Always => Some(ProgressActive::Resource(Resource { current: 1, max: 1 })),
            Self::Buff(id) => ctx.own_buffs().map(|buffs| {
                buffs
                    .get(id)
                    .filter(|buff| buff.runout_time > ctx.now)
                    .map(Into::into)
                    .unwrap_or(ProgressActive::Buff {
                        stacks: 0,
                        apply: 0,
                        runout: 0,
                    })
            }),
            Self::AnyBuff(ids) => ctx.own_buffs().map(|buffs| {
                let mut stacks = 0;
                let mut apply = 0;
                let mut runout = 0;
                for id in ids {
                    if let Some(buff) = buffs.get(id).filter(|buff| buff.runout_time > ctx.now) {
                        stacks += buff.stacks;
                        apply = apply.max(buff.apply_time);
                        runout = runout.max(buff.runout_time);
                    }
                }
                ProgressActive::Buff {
                    stacks,
                    apply,
                    runout,
                }
            }),
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
        match self {
            Self::Inherit => parent.cloned().unwrap_or(ProgressActive::dummy()),
            Self::Always => ProgressActive::dummy(),
            Self::Buff(_) | Self::AnyBuff(_) => {
                let apply = ctx.now - passed;
                ProgressActive::Buff {
                    stacks: (progress * 25.0) as u32,
                    apply,
                    runout: apply + CYCLE,
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
        if let Ok(infos) = Internal::get_buff_infos() {
            if let Some(info) = infos.get(&id) {
                if info.category == Category::ScreenBorder {
                    Validation::Warn(format!("{} {id} is only valid for yourself", info.category))
                } else {
                    Validation::Confirm(format!("{} {id} is valid", info.category))
                }
            } else {
                Validation::Error(format!("Effect {id} is invalid or hidden"))
            }
        } else {
            Validation::Ok
        }
    }

    fn buff_helper(ui: &Ui) {
        helper(ui, || {
            ui.text("Can be found on the wiki");
            ui.text("Supports pasting chat links");
        });
    }
}

impl RenderOptions for ProgressSource {
    fn render_options(&mut self, ui: &Ui, _state: &mut EditState) {
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
                Self::buff_helper(ui);
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
                    Self::buff_helper(ui);
                }
                if ui.button("Add Effect") {
                    ids.push(0);
                }

                action.perform(ids);
            }
            _ => {}
        }
    }
}
