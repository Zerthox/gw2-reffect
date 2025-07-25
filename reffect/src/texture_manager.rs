use crate::{addon::Addon, assets, elements::icon::IconSource};
use nexus::{
    imgui::TextureId,
    texture::{
        RawTextureReceiveCallback, Texture, get_texture, load_texture_from_file,
        load_texture_from_memory, load_texture_from_url,
    },
    texture_receive,
};
use std::{
    collections::HashMap,
    path::Path,
    sync::{Mutex, MutexGuard, OnceLock, mpsc},
    thread::{self, JoinHandle},
};
use url::Url;

static TEXTURE_MANAGER: OnceLock<Mutex<TextureManager>> = OnceLock::new();

#[derive(Debug)]
pub struct TextureManager {
    loader: Option<TextureLoader>,
    pending: HashMap<String, IconSource>,
    loaded: HashMap<IconSource, TextureId>,
    error: Option<TextureId>,
    weapon: Option<TextureId>,
    bundle: Option<TextureId>,
}

impl TextureManager {
    const ERROR_ID: &'static str = "REFFECT_ICON_ERROR";
    const WEAPON_ID: &'static str = "REFFECT_ICON_WEAPON";
    const BUNDLE_ID: &'static str = "REFFECT_ICON_BUNDLE";

    fn new() -> Self {
        Self {
            loader: TextureLoader::spawn(Self::loader_thread),
            pending: HashMap::new(),
            loaded: HashMap::new(),
            error: None,
            weapon: None,
            bundle: None,
        }
        .with_defaults()
    }

    fn with_defaults(mut self) -> Self {
        self.error = self.try_load_from_mem(Self::ERROR_ID, assets::TEMP_ICON);
        self.weapon = self.try_load_from_mem(Self::WEAPON_ID, assets::WEAPON_SWAP);
        self.bundle = self.try_load_from_mem(Self::BUNDLE_ID, assets::BUNDLE_DROP);

        let unknown = IconSource::UNKNOWN_ID;
        if let Some(texture) = self.try_load_from_mem(unknown, assets::MONSTER_ICON) {
            self.loaded.insert(IconSource::Unknown, texture);
        } else {
            self.pending.insert(unknown.into(), IconSource::Unknown);
        }
        self
    }

    fn loader_thread(source: IconSource) -> bool {
        let mut textures = Self::lock();

        // check for stop
        if textures.loader.is_none() {
            return true;
        }

        if !textures.exists(&source) {
            match &source {
                IconSource::Unknown | IconSource::Empty | IconSource::Automatic => {}
                IconSource::File(path) => {
                    let id = textures.add_pending(source.clone());
                    drop(textures); // drop to avoid recursive locking
                    Self::load_from_file(&id, path);
                }
                IconSource::Url(url) => {
                    let id = textures.add_pending(source.clone());
                    drop(textures); // drop to avoid recursive locking
                    Self::load_from_url(&id, url)
                        .unwrap_or_else(|| log::warn!("Failed to parse icon url \"{url}\""));
                }
            }
        }
        false
    }

    fn exists(&self, source: &IconSource) -> bool {
        self.loaded.contains_key(source) || self.pending.contains_key(&source.generate_id())
    }

    fn try_load_from_mem(
        &mut self,
        id: impl AsRef<str>,
        data: impl AsRef<[u8]>,
    ) -> Option<TextureId> {
        // check for the texture ourself to avoid recursive locking
        let id = id.as_ref();
        get_texture(id).map(|texture| texture.id()).or_else(|| {
            load_texture_from_memory(id, data, Some(Self::RECEIVE_TEXTURE));
            None
        })
    }

    pub fn load() -> &'static Mutex<TextureManager> {
        TEXTURE_MANAGER.get_or_init(|| Mutex::new(Self::new()))
    }

    fn lock() -> MutexGuard<'static, TextureManager> {
        Self::load().lock().unwrap()
    }

    pub fn unload() {
        let mut lock = Self::lock();
        if let Some(loader) = lock.loader.take() {
            drop(lock); // drop to avoid deadlock with loader thread
            loader.exit_and_wait();
        }
    }

    pub fn get_weapon_swap() -> Option<TextureId> {
        Self::lock().weapon
    }

    pub fn get_bundle_drop() -> Option<TextureId> {
        Self::lock().bundle
    }

    pub fn get_texture(source: &IconSource) -> Option<TextureId> {
        Self::lock().loaded.get(source).copied()
    }

    pub fn add_source(source: &IconSource) {
        if source.needs_load() {
            // send to loader thread
            if let Some(loader) = &Self::lock().loader {
                if loader.sender.send(source.clone()).is_err() {
                    log::error!("Texture loader receiver disconnected");
                }
            } else {
                log::error!("Texture loader sender disconnected");
            }
        }
    }

    fn load_from_file(id: &str, path: impl AsRef<Path>) {
        let path = path.as_ref();
        let path = if path.is_absolute() {
            path
        } else {
            &Addon::icons_dir().join(path)
        };
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

    fn add_pending(&mut self, source: IconSource) -> String {
        let id = source.generate_id();
        self.pending.insert(id.clone(), source);
        id
    }

    fn add_loaded(&mut self, pending_id: &str, texture_id: Option<TextureId>) {
        match pending_id {
            Self::ERROR_ID => {
                if texture_id.is_none() {
                    log::error!("Failed to error icon source");
                }
                self.error = texture_id;
            }
            Self::WEAPON_ID => {
                if texture_id.is_none() {
                    log::error!("Failed to weapon swap icon source");
                }
                self.weapon = texture_id;
            }
            Self::BUNDLE_ID => {
                if texture_id.is_none() {
                    log::error!("Failed to bundle drop icon source");
                }
                self.bundle = texture_id;
            }
            _ => {
                if let Some(source) = self.pending.remove(pending_id) {
                    if let Some(texture_id) = texture_id {
                        self.loaded.insert(source, texture_id);
                    } else {
                        log::warn!("Failed to load icon source {}", source.pretty_print());
                        if let Some(texture_id) = self.error {
                            self.loaded.insert(source, texture_id);
                        }
                    }
                } else {
                    log::warn!("Received load for non-pending texture \"{pending_id}\"");
                }
            }
        }
    }
}

#[derive(Debug)]
struct TextureLoader {
    sender: mpsc::Sender<IconSource>,
    handle: JoinHandle<()>,
}

impl TextureLoader {
    fn spawn(callback: impl Fn(IconSource) -> bool + Send + 'static) -> Option<Self> {
        let (sender, receiver) = mpsc::channel();
        thread::Builder::new()
            .name("reffect-texture-loader".into())
            .spawn(move || {
                log::debug!("Texture loader spawn");
                while let Ok(source) = receiver.recv() {
                    let stop = callback(source);
                    if stop {
                        break;
                    }
                }
                log::debug!("Texture loader exit");
            })
            .inspect_err(|err| log::error!("Failed to spawn texture loader: {err}"))
            .ok()
            .map(|handle| Self { sender, handle })
    }

    fn exit_and_wait(self) {
        let Self { sender, handle } = self;
        drop(sender);
        log::debug!("Waiting for texture loader");
        if handle.join().is_err() {
            log::error!("Failed to join texture loader");
        }
    }
}
