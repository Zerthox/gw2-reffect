use crate::{assets::MONSTER_ICON, element::IconSource};
use nexus::{
    imgui::TextureId,
    texture::{
        get_texture_or_create_from_memory, load_texture_from_file, load_texture_from_url,
        RawTextureReceiveCallback,
    },
    texture_receive,
};
use std::{
    collections::HashMap,
    path::Path,
    sync::{Mutex, MutexGuard, OnceLock},
};
use url::Url;

static TEXTURE_MANAGER: OnceLock<Mutex<TextureManager>> = OnceLock::new();

#[derive(Debug)]
pub struct TextureManager {
    pending: HashMap<String, IconSource>,
    loaded: HashMap<IconSource, TextureId>,
    default: TextureId,
    next_id: u32,
}

impl TextureManager {
    fn new() -> Self {
        let default = get_texture_or_create_from_memory("REFFECT_ICON_MONSTER", MONSTER_ICON)
            .expect("failed to load default icon")
            .id();

        let mut loaded = HashMap::new();
        loaded.insert(IconSource::Empty, default);

        Self {
            pending: HashMap::new(),
            loaded,
            default,
            next_id: 0,
        }
    }

    pub fn load() -> &'static Mutex<TextureManager> {
        TEXTURE_MANAGER.get_or_init(|| Mutex::new(Self::new()))
    }

    fn lock() -> MutexGuard<'static, TextureManager> {
        Self::load().lock().unwrap()
    }

    pub fn clear() {
        Self::lock().loaded.clear()
    }

    pub fn get_texture(source: &IconSource) -> TextureId {
        let textures = Self::lock();
        textures
            .loaded
            .get(source)
            .copied()
            .unwrap_or(textures.default)
    }

    pub fn add_source(source: &IconSource) {
        let mut textures = Self::lock();
        if !textures.loaded.contains_key(source) {
            match source {
                IconSource::Empty => {}
                IconSource::File(path) => {
                    let id = textures.add_pending(source.clone());
                    Self::load_from_file(&id, path);
                }
                IconSource::Url(url) => {
                    let id = textures.add_pending(source.clone());
                    Self::load_from_url(&id, url);
                }
            }
        }
    }

    fn load_from_file(id: &str, path: &Path) {
        load_texture_from_file(id, path, Some(Self::RECEIVE_TEXTURE));
    }

    fn load_from_url(id: &str, url: &str) {
        if let Ok(url) = Url::parse(url) {
            if !matches!(url.scheme(), "http" | "https") {
                return;
            }
            if let Some(host) = url.host_str() {
                let path = url.path();
                load_texture_from_url(
                    id,
                    format!("https://{host}"),
                    path,
                    Some(Self::RECEIVE_TEXTURE),
                );
            }
        }
    }

    const RECEIVE_TEXTURE: RawTextureReceiveCallback =
        texture_receive!(|id, texture| TextureManager::lock().add_loaded(id, texture.id()));

    fn next_id(&mut self) -> String {
        let id = format!("REFFECT_ICON_{}", self.next_id);
        self.next_id += 1;
        id
    }

    fn add_pending(&mut self, source: IconSource) -> String {
        let id = self.next_id();
        self.pending.insert(id.clone(), source);
        id
    }

    fn add_loaded(&mut self, pending_id: &str, texture_id: TextureId) {
        let source = self
            .pending
            .remove(pending_id)
            .expect("received load for non-pending texture");
        self.loaded.insert(source, texture_id);
    }
}
