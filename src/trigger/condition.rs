use super::ProgressActive;
use crate::{
    context::{Context, EditState},
    render_util::{enum_combo, impl_static_variants},
    traits::RenderOptions,
    trigger::ProgressThreshold,
};
use nexus::imgui::{ComboBoxFlags, Ui};
use partial::{IntoPartial, Partial, PartialOps};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{AsRefStr, EnumIter, IntoStaticStr};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Condition<T>
where
    T: IntoPartial,
    T::Partial: Clone + fmt::Debug + Serialize + for<'d> Deserialize<'d>,
{
    pub trigger: ConditionTrigger,
    pub properties: Partial<T>,
}

impl<T> Condition<T>
where
    T: IntoPartial,
    T::Partial: Clone + fmt::Debug + Serialize + for<'d> Deserialize<'d>,
{
    pub fn process(&mut self, value: &mut T, ctx: &Context, active: &ProgressActive) {
        if self.trigger.is_active(ctx, active) {
            value.set(self.properties.clone());
        }
    }
}

impl<T> Default for Condition<T>
where
    T: IntoPartial,
    T::Partial: Clone + fmt::Debug + Serialize + for<'d> Deserialize<'d>,
{
    fn default() -> Self {
        Self {
            trigger: ConditionTrigger::default(),
            properties: T::Partial::empty(),
        }
    }
}

#[derive(Debug, Clone, AsRefStr, IntoStaticStr, EnumIter, Serialize, Deserialize)]
pub enum ConditionTrigger {
    #[strum(serialize = "Progress Threshold")]
    ProgressThreshold(ProgressThreshold),
}

impl_static_variants!(ConditionTrigger);

impl ConditionTrigger {
    pub fn is_active(&mut self, ctx: &Context, active: &ProgressActive) -> bool {
        match self {
            Self::ProgressThreshold(threshold) => threshold.is_met(active, ctx),
        }
    }
}

impl Default for ConditionTrigger {
    fn default() -> Self {
        Self::ProgressThreshold(ProgressThreshold::default())
    }
}

impl fmt::Display for ConditionTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ProgressThreshold(threshold) => threshold.fmt(f),
        }
    }
}

impl RenderOptions for ConditionTrigger {
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) {
        enum_combo(ui, "Condition", self, ComboBoxFlags::empty());

        match self {
            Self::ProgressThreshold(threshold) => threshold.render_options(ui, state),
        }
    }
}
