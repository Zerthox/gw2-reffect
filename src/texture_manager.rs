use crate::{assets::MONSTER_ICON, element::IconSource};
use nexus::{
    imgui::TextureId,
    texture::{
        get_texture, load_texture_from_file, load_texture_from_memory, load_texture_from_url,
        RawTextureReceiveCallback, Texture,
    },
    texture_receive,
};
use std::{
    collections::HashMap,
    path::Path,
    sync::{Mutex, MutexGuard, OnceLock},
};
use url::Url;
use uuid::Uuid;

static TEXTURE_MANAGER: OnceLock<Mutex<TextureManager>> = OnceLock::new();

#[derive(Debug)]
pub struct TextureManager {
    pending: HashMap<String, IconSource>,
    loaded: HashMap<IconSource, TextureId>,
    default: Option<TextureId>,
}

impl TextureManager {
    fn empty() -> Self {
        Self {
            pending: HashMap::new(),
            loaded: HashMap::new(),
            default: None,
        }
    }

    fn with_default(mut self) -> Self {
        const ID: &str = "REFFECT_MONSTER_ICON";
        const SOURCE: IconSource = IconSource::Empty;

        if let Some(texture) = get_texture(ID) {
            let texture_id = texture.id();
            self.default = Some(texture_id);
            log::debug!("Already loaded default icon: id {}", texture_id.id());
        } else {
            log::debug!("Requesting default icon load");
            self.pending.insert(ID.into(), SOURCE);
            load_texture_from_memory(ID, MONSTER_ICON, Some(Self::RECEIVE_TEXTURE));
        };

        self
    }

    pub fn load() -> &'static Mutex<TextureManager> {
        TEXTURE_MANAGER.get_or_init(|| Mutex::new(Self::empty().with_default()))
    }

    fn lock() -> MutexGuard<'static, TextureManager> {
        Self::load().lock().unwrap()
    }

    pub fn clear() {
        Self::lock().loaded.clear()
    }

    pub fn get_texture(source: &IconSource) -> Option<TextureId> {
        let textures = Self::lock();
        textures.loaded.get(source).copied().or(textures.default)
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
                    Self::load_from_url(&id, url)
                        .unwrap_or_else(|| log::warn!("Failed to parse icon url {url}"));
                }
            }
        }
    }

    fn load_from_file(id: &str, path: &Path) {
        load_texture_from_file(id, path, Some(Self::RECEIVE_TEXTURE));
    }

    #[must_use]
    fn load_from_url(id: &str, url: &str) -> Option<()> {
        let url = Url::parse(url).ok()?;
        if !matches!(url.scheme(), "http" | "https") {
            return None;
        }
        let host = url.host_str()?;
        let path = url.path();
        load_texture_from_url(
            id,
            format!("https://{host}"),
            path,
            Some(Self::RECEIVE_TEXTURE),
        );
        Some(())
    }

    fn receive_texture(id: &str, texture: Option<&Texture>) {
        TextureManager::lock().add_loaded(id, texture.map(|texture| texture.id()));
    }

    const RECEIVE_TEXTURE: RawTextureReceiveCallback =
        texture_receive!(TextureManager::receive_texture);

    fn next_id(&mut self) -> String {
        format!("REFFECT_ICON_{}", Uuid::new_v4().as_simple())
    }

    fn add_pending(&mut self, source: IconSource) -> String {
        let id = self.next_id();
        self.pending.insert(id.clone(), source);
        id
    }

    fn add_loaded(&mut self, pending_id: &str, texture_id: Option<TextureId>) {
        let source = self
            .pending
            .remove(pending_id)
            .expect("received load for non-pending texture");
        if let Some(texture_id) = texture_id {
            if let IconSource::Empty = source {
                log::debug!("Loaded default icon: id {}", texture_id.id());
                self.default = Some(texture_id);
            } else {
                log::debug!("Loaded icon source {source:?}: id {}", texture_id.id());
                self.loaded.insert(source, texture_id);
            }
        } else {
            log::warn!("Failed to load icon source {source:?}");
        }
    }
}
