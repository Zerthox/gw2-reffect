use crate::{
    action::Action,
    context::Context,
    render_util::{enum_combo, impl_static_variants, input_u32},
    traits::RenderOptions,
};
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoStaticStr};

// FIXME: serde does not support flatten aliases, see https://github.com/serde-rs/serde/pull/2387
#[derive(Debug, Default, Clone, AsRefStr, IntoStaticStr, EnumIter, Serialize, Deserialize)]
pub enum BuffTriggerId {
    /// Always active, no associated buff(s).
    #[default]
    #[serde(alias = "Always")]
    None,

    /// Single buff id.
    #[serde(alias = "Has")]
    Single(u32),

    /// Any of the buff ids, stacks are summed.
    #[strum(serialize = "Any of")]
    Any(Vec<u32>),

    /// All of the buff ids, stacks are summed.
    ///
    /// Same as any but all buffs need to be present.
    #[strum(serialize = "All of")]
    All(Vec<u32>),
}

impl_static_variants!(BuffTriggerId);

impl BuffTriggerId {
    pub fn always(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn count_stacks(&self, ctx: &Context) -> i32 {
        match self {
            Self::None => 1, // return 1 stack
            Self::Single(id) => ctx.stacks_of(*id).unwrap_or(0),
            Self::Any(ids) => ids.iter().filter_map(|id| ctx.stacks_of(*id)).sum(),
            Self::All(ids) => {
                let mut sum = 0;
                for id in ids {
                    if let Some(stacks) = ctx.stacks_of(*id) {
                        sum += stacks;
                    } else {
                        return 0; // missing one of the buffs means no stacks
                    }
                }
                sum
            }
        }
    }

    pub fn into_ids(self) -> Vec<u32> {
        match self {
            Self::None => Vec::new(),
            Self::Single(id) => vec![id],
            Self::Any(ids) | Self::All(ids) => ids,
        }
    }
}

impl RenderOptions for BuffTriggerId {
    fn render_options(&mut self, ui: &Ui) {
        ui.group(|| {
            if let Some(prev) = enum_combo(ui, "Effect", self, ComboBoxFlags::empty()) {
                match self {
                    Self::None => {}
                    Self::Single(id) => {
                        if let Some(first) = prev.into_ids().first() {
                            *id = *first;
                        }
                    }
                    Self::Any(ids) | Self::All(ids) => *ids = prev.into_ids(),
                }
            }

            match self {
                Self::None => {}
                Self::Single(id) => {
                    input_u32(ui, "Effect id", id, 0, 0);
                }
                Self::Any(ids) | Self::All(ids) => {
                    let mut action = Action::new();
                    for (i, id) in ids.iter_mut().enumerate() {
                        let _id = ui.push_id(i as i32);
                        action.set_next_input_size(ui);
                        input_u32(ui, "##id", id, 0, 0);

                        ui.same_line();
                        action.render_buttons(ui, i);

                        ui.same_line();
                        ui.text(format!("Effect id {}", i + 1));
                    }
                    if ui.button("Add") {
                        ids.push(0);
                    }

                    action.perform(ids);
                }
            }
        })
    }
}
