use crate::texture::{TextureKey, TextureManager, TextureSource};
use const_default::ConstDefault;
use serde::{Deserialize, Serialize};

pub trait AsTextureSource {
    fn as_texture_source(&self) -> Option<TextureSource>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LoadedTexture<T>
where
    T: AsTextureSource,
{
    pub source: T,

    #[serde(skip)]
    pub key: Option<TextureKey>,
}

impl<T> LoadedTexture<T>
where
    T: AsTextureSource,
{
    pub const fn unloaded(source: T) -> Self {
        Self { source, key: None }
    }

    pub fn load(&mut self) {
        self.key = self
            .source
            .as_texture_source()
            .and_then(TextureManager::add_source);
    }
}

impl<T> ConstDefault for LoadedTexture<T>
where
    T: AsTextureSource + ConstDefault,
{
    const DEFAULT: Self = Self::unloaded(T::DEFAULT);
}

impl<T> Default for LoadedTexture<T>
where
    T: AsTextureSource + Default,
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
