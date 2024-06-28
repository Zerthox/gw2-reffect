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
}

impl_static_variants!(BuffTriggerId);

impl BuffTriggerId {
    pub fn always(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn count_stacks(&self, ctx: &Context) -> u32 {
        match self {
            Self::None => 1, // 1 dummy stack
            Self::Single(id) => ctx.stacks_of(*id).unwrap_or(0),
            Self::Any(ids) => ids.iter().filter_map(|id| ctx.stacks_of(*id)).sum(), // sum of all stacks
        }
    }

    pub fn times(&self, ctx: &Context) -> (u32, u32) {
        match self {
            Self::None => (u32::MAX, u32::MAX),
            Self::Single(id) => ctx.times(*id).unwrap_or((0, 0)),
            Self::Any(ids) => ids.iter().find_map(|id| ctx.times(*id)).unwrap_or((0, 0)), // times of first match
        }
    }

    pub fn into_ids(self) -> Vec<u32> {
        match self {
            Self::None => Vec::new(),
            Self::Single(id) => vec![id],
            Self::Any(ids) => ids,
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
                    Self::Any(ids) => *ids = prev.into_ids(),
                }
            }

            match self {
                Self::None => {}
                Self::Single(id) => {
                    input_u32(ui, "Effect Id", id, 0, 0);
                }
                Self::Any(ids) => {
                    let mut action = Action::new();
                    for (i, id) in ids.iter_mut().enumerate() {
                        let _id = ui.push_id(i as i32);
                        action.input_with_buttons(ui, i, || input_u32(ui, "##id", id, 0, 0));

                        ui.same_line();
                        ui.text(format!("Effect Id {}", i + 1));
                    }
                    if ui.button("Add Effect") {
                        ids.push(0);
                    }

                    action.perform(ids);
                }
            }
        })
    }
}
