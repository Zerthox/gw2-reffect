use crate::{
    action::Action,
    colors,
    context::Context,
    internal::Resource,
    render_util::{enum_combo, helper, impl_static_variants, input_buff_id},
    traits::RenderOptions,
};
use nexus::imgui::{ComboBoxFlags, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter, IntoStaticStr};

use super::ProgressActive;

// FIXME: serde does not support flatten aliases, see https://github.com/serde-rs/serde/pull/2387
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

    /// Primary resource.
    #[strum(serialize = "Primary Resource")]
    PrimaryResource,

    /// Secondary resource.
    #[strum(serialize = "Secondary Resource")]
    SecondaryResource,
}

impl_static_variants!(ProgressSource);

impl ProgressSource {
    pub fn always(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn progress(&self, ctx: &Context) -> ProgressActive {
        match self {
            ProgressSource::None => ProgressActive::Resource(Resource { current: 1, max: 1 }),
            ProgressSource::Buff(id) => {
                let stacks = ctx.stacks_of(*id).unwrap_or(0);
                let (apply, runout) = ctx.time_range(*id).unwrap_or((0, 0));
                ProgressActive::Buff {
                    stacks,
                    apply,
                    runout,
                }
            }
            ProgressSource::AnyBuff(ids) => {
                let stacks = ids.iter().filter_map(|id| ctx.stacks_of(*id)).sum(); // sum of all stacks
                let (apply, runout) = ids
                    .iter()
                    .find_map(|id| ctx.time_range(*id))
                    .unwrap_or((0, 0)); // times of first match, TODO: max of apply and max of runout
                ProgressActive::Buff {
                    stacks,
                    apply,
                    runout,
                }
            }
            ProgressSource::PrimaryResource => {
                ProgressActive::Resource(ctx.resources.primary.clone())
            }
            ProgressSource::SecondaryResource => {
                ProgressActive::Resource(ctx.resources.secondary.clone())
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
    fn render_options(&mut self, ui: &Ui) {
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
            helper(ui, || {
                ui.text("Source of information");
                ui.text_colored(colors::RED, "Some resources are not yet implemented");
            });

            match self {
                Self::Buff(id) => {
                    input_buff_id(ui, "Effect Id", id, InputTextFlags::empty());
                    Self::helper(ui);
                }
                Self::AnyBuff(ids) => {
                    let mut action = Action::new();
                    for (i, id) in ids.iter_mut().enumerate() {
                        let _id = ui.push_id(i as i32);
                        action.input_with_buttons(ui, i, || {
                            input_buff_id(ui, "##id", id, InputTextFlags::empty())
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
