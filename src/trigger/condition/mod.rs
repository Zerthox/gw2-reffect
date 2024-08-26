mod trigger;

pub use self::trigger::*;

use super::ProgressActive;
use crate::context::Context;
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
