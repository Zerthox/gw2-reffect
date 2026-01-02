use crate::{
    addon::Addon,
    assets,
    texture::{TextureEntry, TextureKey, TextureLoader, TextureSource},
};
use nexus::{
    imgui,
    texture::{
        RawTextureReceiveCallback, Texture, get_texture, load_texture_from_file,
        load_texture_from_memory, load_texture_from_url,
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
    loader: Option<TextureLoader>,

    textures: Vec<TextureEntry>,
    by_nexus_id: HashMap<String, TextureKey>,

    error: Option<imgui::TextureId>,
    unknown: Option<imgui::TextureId>,
    weapon: Option<imgui::TextureId>,
    bundle: Option<imgui::TextureId>,
}

impl TextureManager {
    const ERROR_ID: &'static str = "REFFECT_ICON_ERROR";
    const UNKNOWN_ID: &'static str = "REFFECT_ICON_UNKNOWN";
    const WEAPON_ID: &'static str = "REFFECT_ICON_WEAPON";
    const BUNDLE_ID: &'static str = "REFFECT_ICON_BUNDLE";

    fn empty() -> Self {
        Self {
            loader: None,
            textures: Vec::new(),
            by_nexus_id: HashMap::new(),
            error: None,
            unknown: None,
            weapon: None,
            bundle: None,
        }
    }

    fn create() -> Self {
        Self {
            loader: TextureLoader::spawn(),
            textures: Vec::new(),
            by_nexus_id: HashMap::new(),
            error: Self::try_load_from_mem(Self::ERROR_ID, assets::TEMP_ICON),
            unknown: Self::try_load_from_mem(Self::UNKNOWN_ID, assets::MONSTER_ICON),
            weapon: Self::try_load_from_mem(Self::WEAPON_ID, assets::WEAPON_SWAP),
            bundle: Self::try_load_from_mem(Self::BUNDLE_ID, assets::BUNDLE_DROP),
        }
    }

    pub fn load() -> &'static Mutex<TextureManager> {
        TEXTURE_MANAGER.get_or_init(|| Mutex::new(Self::create()))
    }

    pub fn unload() {
        let mut manager = Self::lock();
        if let Some(worker) = manager.loader.take() {
            drop(manager); // allow loader to access if needed
            log::debug!("Waiting for texture loader");
            worker.exit_and_wait();
        }

        let mut manager = Self::lock();
        log::debug!(
            "Releasing {} referenced textures",
            manager.by_nexus_id.len()
        );
        for nexus_id in manager.by_nexus_id.keys() {
            Self::release_texture(nexus_id);
        }
        *manager = Self::empty();
    }

    fn lock() -> MutexGuard<'static, TextureManager> {
        Self::load().lock().unwrap()
    }

    pub fn is_active() -> bool {
        Self::lock().loader.is_some()
    }

    pub fn get_unknown() -> Option<imgui::TextureId> {
        Self::lock().unknown
    }

    pub fn get_weapon_swap() -> Option<imgui::TextureId> {
        Self::lock().weapon
    }

    pub fn get_bundle_drop() -> Option<imgui::TextureId> {
        Self::lock().bundle
    }

    pub fn get_texture(key: TextureKey) -> Option<imgui::TextureId> {
        Self::lock()
            .textures
            .get(key.0)
            .and_then(|entry| entry.texture())
    }

    pub fn add_source(source: TextureSource) -> Option<TextureKey> {
        Self::lock().try_add_pending(source)
    }

    fn try_add_pending(&mut self, source: TextureSource) -> Option<TextureKey> {
        if let Some(loader) = &self.loader {
            let nexus_id = source.generate_nexus_id();
            if let Some(key) = self.by_nexus_id.get(&nexus_id).copied() {
                Some(key)
            } else {
                let key = TextureKey(self.textures.len());
                self.textures.push(TextureEntry::pending(source.clone()));
                self.by_nexus_id.insert(nexus_id, key);
                loader.send(source);
                Some(key)
            }
        } else {
            log::error!("No texture loader present");
            None
        }
    }

    pub fn load_source(source: TextureSource) {
        let id = &source.generate_nexus_id();
        match source {
            TextureSource::File(path) => Self::load_from_file(id, path),
            TextureSource::Url(url) => Self::load_from_url(id, &url)
                .unwrap_or_else(|| log::warn!("Failed to parse icon url \"{url}\"")),
        }
    }

    fn try_load_from_mem(id: impl AsRef<str>, data: impl AsRef<[u8]>) -> Option<imgui::TextureId> {
        // check for the texture ourself to avoid recursive locking
        let id = id.as_ref();
        get_texture(id).map(|texture| texture.id()).or_else(|| {
            load_texture_from_memory(id, data, Some(Self::RECEIVE_TEXTURE));
            None
        })
    }

    fn load_from_file(nexus_id: impl AsRef<str>, path: impl AsRef<Path>) {
        let path = path.as_ref();
        let path = if path.is_absolute() {
            path
        } else {
            &Addon::icons_dir().join(path)
        };
        load_texture_from_file(nexus_id, path, Some(Self::RECEIVE_TEXTURE));
    }

    #[must_use]
    fn load_from_url(nexus_id: impl AsRef<str>, url: &str) -> Option<()> {
        let url = Url::parse(url).ok()?;
        if !matches!(url.scheme(), "http" | "https") {
            return None;
        }
        let host = url.host_str()?;
        let path = url.path();
        load_texture_from_url(
            nexus_id,
            format!("https://{host}"),
            path,
            Some(Self::RECEIVE_TEXTURE),
        );
        Some(())
    }

    fn release_texture(_nexus_id: impl AsRef<str>) {}

    fn receive_texture(nexus_id: &str, texture: Option<&Texture>) {
        Self::lock().add_loaded(nexus_id, texture.map(|texture| texture.id()));
    }

    const RECEIVE_TEXTURE: RawTextureReceiveCallback =
        texture_receive!(TextureManager::receive_texture);

    fn add_loaded(&mut self, nexus_id: &str, texture_id: Option<imgui::TextureId>) {
        match nexus_id {
            Self::ERROR_ID => {
                if texture_id.is_none() {
                    log::error!("Failed to load error texture");
                }
                self.error = texture_id;
            }
            Self::UNKNOWN_ID => {
                if texture_id.is_none() {
                    log::error!("Failed to load unknown texture");
                }
                self.unknown = texture_id;
            }
            Self::WEAPON_ID => {
                if texture_id.is_none() {
                    log::error!("Failed to load weapon swap texture");
                }
                self.weapon = texture_id;
            }
            Self::BUNDLE_ID => {
                if texture_id.is_none() {
                    log::error!("Failed to load bundle drop texture");
                }
                self.bundle = texture_id;
            }
            _ => {
                if let Some(entry) = self
                    .by_nexus_id
                    .get(nexus_id)
                    .and_then(|key| self.textures.get_mut(key.0))
                {
                    if let Some(texture_id) = texture_id {
                        entry.load(texture_id);
                    } else {
                        entry.fail(self.error);
                    }
                } else {
                    log::warn!("Received load for unknown texture \"{nexus_id}\"");
                }
            }
        }
    }
}
