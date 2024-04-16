use super::Trigger;
use crate::{
    context::RenderContext,
    elements::RenderState,
    render_util::{enum_combo, impl_static_variants, input_u32},
    traits::RenderOptions,
};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoStaticStr};

// TODO: allow displaying with 0 stacks?

#[derive(Debug, Default, Clone, AsRefStr, IntoStaticStr, EnumIter, Serialize, Deserialize)]
pub enum BuffTrigger {
    #[default]
    Always,

    #[strum(serialize = "Has")]
    Has(u32),

    #[strum(serialize = "Missing")]
    Not(u32),

    #[strum(serialize = "Any of")]
    Any(Vec<u32>),

    #[strum(serialize = "All of")]
    All(Vec<u32>),
}

impl_static_variants!(BuffTrigger);

impl Trigger for BuffTrigger {
    fn is_active(&self, ctx: &RenderContext) -> bool {
        match self {
            Self::Always => true,
            Self::Any(ids) => ids.iter().any(|id| ctx.has_buff(*id)),
            Self::All(ids) => ids.iter().any(|id| ctx.has_buff(*id)),
            Self::Not(id) => !ctx.has_buff(*id),
            Self::Has(id) => ctx.has_buff(*id),
        }
    }
}

impl BuffTrigger {
    fn get_stacks(&self, ctx: &RenderContext) -> Option<i32> {
        match self {
            Self::Always => Some(0),
            Self::Any(ids) => {
                let mut iter = ids.iter().filter_map(|id| ctx.stacks_of(*id));
                iter.next().map(|first| first + iter.sum::<i32>())
            }
            Self::All(ids) => {
                let mut sum = 0;
                for id in ids {
                    if let Some(stacks) = ctx.stacks_of(*id) {
                        sum += stacks;
                    } else {
                        return None;
                    }
                }
                Some(sum)
            }
            Self::Not(id) => (!ctx.has_buff(*id)).then_some(0),
            Self::Has(id) => ctx.stacks_of(*id),
        }
    }

    pub fn get_stacks_or_edit(&self, ctx: &RenderContext, state: &RenderState) -> Option<i32> {
        if state.edit {
            Some(1)
        } else {
            self.get_stacks(ctx)
        }
    }
}

impl RenderOptions for BuffTrigger {
    fn render_options(&mut self, ui: &Ui) {
        ui.group(|| {
            // TODO: propagate ids when selecting another trigger?
            enum_combo(ui, "Buff", self, ComboBoxFlags::empty());

            match self {
                BuffTrigger::Always => {}
                BuffTrigger::Any(ids) | BuffTrigger::All(ids) => {
                    // TODO: as single text input?
                    for (i, id) in ids.iter_mut().enumerate() {
                        input_u32(ui, format!("Id {}", i + 1), id);
                    }
                    if ui.button("+") {
                        ids.push(0);
                    }
                    ui.same_line();
                    if ui.button("-") {
                        ids.pop();
                    }
                }
                BuffTrigger::Not(id) | BuffTrigger::Has(id) => {
                    input_u32(ui, "Id", id);
                }
            }
        })
    }
}
