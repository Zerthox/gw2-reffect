use super::Trigger;
use crate::context::Context;
use serde::{Deserialize, Serialize};
use std::ops;

/// Memoization for a [`Trigger`].
#[derive(Debug, Default, Clone)]
pub struct Memo<T>
where
    T: Trigger,
{
    trigger: T,
    cache: Option<bool>,
}

impl<T> Memo<T>
where
    T: Trigger,
{
    pub fn new(trigger: T) -> Self {
        Self {
            trigger,
            cache: None,
        }
    }

    /// Checks if the memo is empty.
    pub fn is_empty(&self) -> bool {
        self.cache.is_none()
    }

    /// Retrieves the memoized state, if initialized.
    pub fn get(&self) -> Option<bool> {
        self.cache
    }

    /// Clears the memoized state.
    pub fn clear(&mut self) {
        self.cache = None;
    }

    /// Updates the memoized state from the [`Context`].
    pub fn update(&mut self, ctx: &Context) -> bool {
        *self.cache.insert(self.trigger.is_active(ctx))
    }

    /// Retrieves the memoized state or creates it from the [`Context`].
    pub fn get_or_update(&mut self, ctx: &Context) -> bool {
        if let Some(active) = self.cache {
            active
        } else {
            self.update(ctx)
        }
    }
}

impl<T> ops::Deref for Memo<T>
where
    T: Trigger,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.trigger
    }
}

impl<T> ops::DerefMut for Memo<T>
where
    T: Trigger,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.trigger
    }
}

impl<T> Trigger for Memo<T>
where
    T: Trigger,
{
    fn is_active(&mut self, ctx: &Context) -> bool {
        self.get_or_update(ctx)
    }
}

impl<T> Serialize for Memo<T>
where
    T: Trigger + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.trigger.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Memo<T>
where
    T: Trigger + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let trigger = T::deserialize(deserializer)?;
        Ok(Self::new(trigger))
    }
}
