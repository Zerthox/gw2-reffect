use super::ProgressActive;
use crate::{
    action::Action,
    context::{Context, EditState},
    internal::Resource,
    render_util::{enum_combo, helper, impl_static_variants, input_skill_id},
    traits::RenderOptions,
};
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoStaticStr};

#[derive(Debug, Default, Clone, AsRefStr, IntoStaticStr, EnumIter, Serialize, Deserialize)]
pub enum ProgressSource {
    /// Always active, no associated progress.
    #[default]
    #[serde(alias = "Always")]
    None,

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
    pub fn always(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn progress(&self, ctx: &Context) -> Option<ProgressActive> {
        match self {
            Self::None => Some(Resource { current: 1, max: 1 }.into()),
            Self::Buff(id) => ctx.buff(*id).map(Into::into),
            Self::AnyBuff(ids) => {
                let mut stacks = 0;
                let mut apply = 0;
                let mut runout = 0;
                for id in ids {
                    if let Some(buff) = ctx.buff(*id) {
                        stacks += buff.stacks;
                        apply = apply.max(buff.apply_time);
                        runout = runout.max(buff.runout_time);
                    }
                }
                (stacks > 0).then_some(ProgressActive::Buff {
                    stacks,
                    apply,
                    runout,
                })
            }
            Self::Health => ctx.resources().map(|res| res.health.clone().into()),
            Self::Barrier => ctx.resources().map(|res| res.barrier.clone().into()),
            Self::PrimaryResource => ctx.resources().map(|res| res.primary.clone().into()),
            Self::SecondaryResource => ctx.resources().map(|res| res.secondary.clone().into()),
        }
    }

    pub fn progress_edit(&self, ctx: &Context) -> ProgressActive {
        match self {
            Self::None => ProgressActive::Resource(Resource { current: 1, max: 1 }),
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

    fn helper(ui: &Ui) {
        helper(ui, || {
            ui.text("Can be found on the wiki");
            ui.text("Type the id or paste the chat link");
        });
    }
}

impl RenderOptions for ProgressSource {
    fn render_options(&mut self, ui: &Ui, _state: &mut EditState) {
        ui.group(|| {
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
                    Self::helper(ui);
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
                            Self::helper(ui);
                        }
                    }
                    if ui.button("Add Effect") {
                        ids.push(0);
                    }

                    action.perform(ids);
                }
                _ => {}
            }
        })
    }
}
