use std::{mem, ops};

use crate::texture::{TextureKey, TextureManager, TextureSource};
use const_default::ConstDefault;
use serde::{Deserialize, Serialize};

pub trait AsTextureSource {
    fn as_texture_source(&self) -> Option<TextureSource>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct LoadedTexture<T>
where
    T: AsTextureSource,
{
    source: T,

    #[serde(skip)]
    key: Option<TextureKey>,
}

impl<T> LoadedTexture<T>
where
    T: AsTextureSource,
{
    /// Creates a new not loaded texture.
    pub const fn unloaded(source: T) -> Self {
        Self { source, key: None }
    }

    /// Returns the texture source.
    pub const fn source(&self) -> &T {
        &self.source
    }

    /// Returns a guard for the texture source.
    pub fn source_mut(&mut self) -> SourceGuard<'_, T> {
        SourceGuard(self)
    }

    /// Changes the texture source and reloads it.
    pub fn load_source(&mut self, action: impl FnOnce(&mut T)) {
        action(&mut self.source);
        self.load();
    }

    /// Returns the texture key.
    pub const fn key(&self) -> Option<TextureKey> {
        self.key
    }

    /// Loads the texture.
    pub fn load(&mut self) {
        self.key = self
            .source
            .as_texture_source()
            .and_then(TextureManager::add_source);
    }
}

impl<T> ConstDefault for LoadedTexture<T>
where
    T: ConstDefault + AsTextureSource,
{
    const DEFAULT: Self = Self::unloaded(T::DEFAULT);
}

impl<T> Default for LoadedTexture<T>
where
    T: Default + AsTextureSource,
{
    fn default() -> Self {
        Self::unloaded(T::default())
    }
}

impl<T> From<T> for LoadedTexture<T>
where
    T: AsTextureSource,
{
    fn from(source: T) -> Self {
        Self::unloaded(source)
    }
}

/// Guard protecting access to a texture source.
#[must_use]
pub struct SourceGuard<'a, T>(&'a mut LoadedTexture<T>)
where
    T: AsTextureSource;

impl<'a, T> SourceGuard<'a, T>
where
    T: AsTextureSource,
{
    /// Reloads the texture.
    /// Same as dropping.
    pub fn reload(self) {}

    /// Resets the texture to unloaded state.
    pub const fn reset(self) {
        self.0.key = None;
        mem::forget(self);
    }

    /// Skips reloading and keeps the old texture key.
    pub const fn unchanged(self) {
        mem::forget(self);
    }
}

impl<T> ops::Deref for SourceGuard<'_, T>
where
    T: AsTextureSource,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0.source
    }
}

impl<T> ops::DerefMut for SourceGuard<'_, T>
where
    T: AsTextureSource,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.source
    }
}

impl<T> Drop for SourceGuard<'_, T>
where
    T: AsTextureSource,
{
    fn drop(&mut self) {
        self.0.load();
    }
}
