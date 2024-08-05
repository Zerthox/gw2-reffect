use super::ProgressActive;
use crate::{
    context::{Context, EditState},
    render_util::{enum_combo, impl_static_variants},
    traits::RenderOptions,
    trigger::ProgressThreshold,
};
use fields::Fields;
use nexus::imgui::{ComboBoxFlags, Ui};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::{AsRefStr, EnumIter, IntoStaticStr};

// TODO: multiple conditions with 1 threshold? use props struct with all optional fields instead of enum?

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition<T>
where
    T: Fields,
    T::Field: Clone + fmt::Debug + Serialize + for<'d> Deserialize<'d>,
{
    pub trigger: ConditionTrigger,
    pub property: <T as Fields>::Field,
}

impl<T> Condition<T>
where
    T: Fields,
    T::Field: Clone + fmt::Debug + Serialize + for<'d> Deserialize<'d>,
{
    pub fn process(&mut self, value: &mut T, ctx: &Context, active: &ProgressActive) {
        if self.trigger.is_active(ctx, active) {
            value.set(self.property.clone());
        }
    }
}
impl<T> Default for Condition<T>
where
    T: Fields,
    T::Field: Default + Clone + fmt::Debug + Serialize + for<'d> Deserialize<'d>,
{
    fn default() -> Self {
        Self {
            trigger: ConditionTrigger::default(),
            property: T::Field::default(),
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

impl RenderOptions for ConditionTrigger {
    fn render_options(&mut self, ui: &Ui, state: &mut EditState) -> () {
        enum_combo(ui, "Condition", self, ComboBoxFlags::empty());

        match self {
            Self::ProgressThreshold(threshold) => threshold.render_options(ui, state),
        }
    }
}
