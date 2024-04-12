use super::Trigger;
use crate::{
    context::RenderContext,
    elements::RenderState,
    util::{enum_combo, input_u32},
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumDiscriminants, EnumIter};

#[derive(Debug, Default, Clone, EnumDiscriminants, Serialize, Deserialize)]
#[strum_discriminants(derive(AsRefStr, EnumIter))]
pub enum BuffTrigger {
    #[default]
    Always,

    Has(u32),

    Not(u32),

    Any(Vec<u32>),

    All(Vec<u32>),
}

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

    fn to_discrim(&self) -> BuffTriggerDiscriminants {
        match self {
            Self::Always => BuffTriggerDiscriminants::Always,
            Self::Any(_) => BuffTriggerDiscriminants::Any,
            Self::All(_) => BuffTriggerDiscriminants::All,
            Self::Not(_) => BuffTriggerDiscriminants::Not,
            Self::Has(_) => BuffTriggerDiscriminants::Has,
        }
    }

    fn from_discrim(discrim: BuffTriggerDiscriminants) -> Self {
        match discrim {
            BuffTriggerDiscriminants::Always => Self::Always,
            BuffTriggerDiscriminants::Any => Self::Any(Vec::new()),
            BuffTriggerDiscriminants::All => Self::All(Vec::new()),
            BuffTriggerDiscriminants::Not => Self::Not(0),
            BuffTriggerDiscriminants::Has => Self::Has(0),
        }
    }

    pub fn render_options(&mut self, ui: &Ui) {
        ui.group(|| {
            let mut discrim = self.to_discrim();
            if enum_combo(ui, "Trigger", &mut discrim) {
                *self = Self::from_discrim(discrim);
            }

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
