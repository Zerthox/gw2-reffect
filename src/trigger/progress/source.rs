use super::ProgressActive;
use crate::{
    action::Action,
    context::{Context, EditState},
    internal::Resource,
    render::RenderOptions,
    render_util::{enum_combo, helper, impl_static_variants, input_skill_id},
};
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
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
                    .buff(*id, ctx.now)
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
                    if let Some(buff) = buffs.buff(*id, ctx.now) {
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
            Self::Health => ctx.resources()?.health.clone().try_into().ok(),
            Self::Barrier => ctx.resources()?.barrier.clone().try_into().ok(),
            Self::PrimaryResource => ctx.resources()?.primary.clone().try_into().ok(),
            Self::SecondaryResource => ctx.resources()?.secondary.clone().try_into().ok(),
        }
    }

    pub fn progress_edit(&self, ctx: &Context, parent: Option<&ProgressActive>) -> ProgressActive {
        match self {
            Self::Inherit => parent.cloned().unwrap_or(ProgressActive::dummy()),
            Self::Always => ProgressActive::dummy(),
            Self::Buff(_) | Self::AnyBuff(_) => {
                let apply = ctx.now - (ctx.now % 5000);
                ProgressActive::Buff {
                    stacks: 1,
                    apply,
                    runout: apply + 5000,
                }
            }
            Self::Health | Self::Barrier | Self::PrimaryResource | Self::SecondaryResource => {
                let current = (ctx.now % 5000) / 100;
                ProgressActive::Resource(Resource { current, max: 50 })
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

    fn buff_helper(ui: &Ui) {
        helper(ui, || {
            ui.text("Can be found on the wiki");
            ui.text("Supports pasting chat links");
        });
    }
}

impl RenderOptions for ProgressSource {
    fn render_options(&mut self, ui: &Ui, _state: &mut EditState) {
        if let Some(prev) = enum_combo(ui, "Trigger", self, ComboBoxFlags::empty()) {
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
                input_skill_id(ui, "Effect Id", id, InputTextFlags::empty());
                Self::buff_helper(ui);
            }
            Self::AnyBuff(ids) => {
                let mut action = Action::new();
                for (i, id) in ids.iter_mut().enumerate() {
                    let _id = ui.push_id(i as i32);
                    action.input_with_buttons(ui, i, || {
                        input_skill_id(ui, "##id", id, InputTextFlags::empty())
                    });

                    ui.same_line();
                    ui.text(format!("Effect Id {}", i + 1));
                    if i == 0 {
                        Self::buff_helper(ui);
                    }
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
