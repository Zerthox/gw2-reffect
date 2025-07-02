mod trigger;

pub use self::trigger::*;

use super::ProgressActive;
use crate::{context::Context, elements::PartialProps};
use nexus::imgui::Ui;
use partial::{IntoPartial, PartialOps};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Condition<T>
where
    T: IntoPartial,
{
    pub trigger: ConditionTrigger,
    pub properties: T::Partial,
}

impl<T> Condition<T>
where
    T: IntoPartial,
{
    pub fn process(&mut self, value: &mut T, ctx: &Context, active: &ProgressActive)
    where
        T::Partial: Clone,
    {
        if self.trigger.is_active(ctx, active) {
            value.set(self.properties.clone());
        }
    }

    pub fn render_options(&mut self, ui: &Ui, ctx: &Context, base: &T)
    where
        T::Partial: PartialProps<T>,
    {
        self.trigger.render_options(ui, ctx);
        ui.spacing();
        self.properties.render_options(ui, base);
        ui.spacing();
    }
}

impl<T> Default for Condition<T>
where
    T: IntoPartial,
{
    fn default() -> Self {
        Self {
            trigger: ConditionTrigger::default(),
            properties: T::Partial::empty(),
        }
    }
}
