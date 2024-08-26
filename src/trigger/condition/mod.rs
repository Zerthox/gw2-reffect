mod trigger;

pub use self::trigger::*;

use super::ProgressActive;
use crate::{
    context::{Context, EditState},
    elements::PartialProps,
    traits::RenderOptions,
};
use nexus::imgui::Ui;
use partial::{IntoPartial, Partial, PartialOps};
use serde::{Deserialize, Serialize};
use std::fmt;

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

    pub fn render_options(&mut self, ui: &Ui, state: &mut EditState, base: &T)
    where
        Partial<T>: PartialProps<T>,
    {
        self.trigger.render_options(ui, state);
        ui.spacing();
        self.properties.render_options(ui, base);
        ui.spacing();
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
