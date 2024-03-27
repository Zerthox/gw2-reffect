use crate::{assets::MONSTER_ICON, element::IconSource};
use nexus::{imgui::TextureId, texture::get_texture_or_create_from_memory};
use std::{
    collections::HashMap,
    sync::{Mutex, MutexGuard, OnceLock},
};

pub static TEXTURE_MANAGER: TextureManager = TextureManager::new();

#[derive(Debug)]
pub struct TextureManager {
    textures: OnceLock<Mutex<Textures>>,
}

impl TextureManager {
    pub const fn new() -> Self {
        Self {
            textures: OnceLock::new(),
        }
    }

    fn lock(&self) -> MutexGuard<Textures> {
        self.textures
            .get_or_init(|| Mutex::new(Textures::new()))
            .lock()
            .unwrap()
    }

    pub fn get_texture(&self, source: &IconSource) -> TextureId {
        let textures = self.lock();
        textures
            .loaded
            .get(source)
            .cloned()
            .unwrap_or(textures.default)
    }

    pub fn add_texture(&self, source: &IconSource) {
        let mut textures = self.lock();
        if !textures.loaded.contains_key(source) {
            if let Some(id) = source.force_load_texture() {
                textures.loaded.insert(source.clone(), id);
            }
        }
    }
}

#[derive(Debug)]
struct Textures {
    loaded: HashMap<IconSource, TextureId>,
    default: TextureId,
}

impl Textures {
    fn new() -> Self {
        Self {
            loaded: HashMap::new(),
            default: get_texture_or_create_from_memory("REFFECT_ICON_MONSTER", MONSTER_ICON)
                .expect("failed to load default icon")
                .id(),
        }
    }
}
