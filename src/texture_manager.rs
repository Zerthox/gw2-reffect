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
    sync::{mpsc, Mutex, MutexGuard, OnceLock},
    thread,
};
use url::Url;

static TEXTURE_MANAGER: OnceLock<Mutex<TextureManager>> = OnceLock::new();

#[derive(Debug)]
pub struct TextureManager {
    sender: mpsc::Sender<IconSource>,
    pending: HashMap<String, IconSource>,
    loaded: HashMap<IconSource, TextureId>,
    default: Option<TextureId>,
}

impl TextureManager {
    fn new(sender: mpsc::Sender<IconSource>) -> Self {
        Self {
            sender,
            pending: HashMap::new(),
            loaded: HashMap::new(),
            default: None,
        }
    }

    fn init() -> Self {
        let (sender, receiver) = mpsc::channel::<IconSource>();

        thread::Builder::new()
            .name("reffect-texture-loader".into())
            .spawn(move || {
                while let Ok(source) = receiver.recv() {
                    let mut textures = Self::lock();
                    if !textures.loaded.contains_key(&source) {
                        match &source {
                            IconSource::Empty => {}
                            IconSource::File(path) => {
                                let id = textures.add_pending(source.clone());
                                drop(textures); // drop to avoid recursive locking
                                Self::load_from_file(&id, path);
                            }
                            IconSource::Url(url) => {
                                let id = textures.add_pending(source.clone());
                                drop(textures); // drop to avoid recursive locking
                                Self::load_from_url(&id, url).unwrap_or_else(|| {
                                    log::warn!("Failed to parse icon url \"{url}\"")
                                });
                            }
                        }
                    }
                }
            })
            .expect("failed to spawn texture loading thread");

        Self::new(sender)
    }

    fn with_default(mut self) -> Self {
        // check for the texture ourselves to avoid recursive locking
        let id = IconSource::DEFAULT_ID;
        if let Some(texture) = get_texture(id) {
            let texture_id = texture.id();
            self.default = Some(texture_id);
        } else {
            self.pending.insert(id.into(), IconSource::Empty);
            load_texture_from_memory(id, MONSTER_ICON, Some(Self::RECEIVE_TEXTURE));
        };

        self
    }

    pub fn load() -> &'static Mutex<TextureManager> {
        TEXTURE_MANAGER.get_or_init(|| Mutex::new(Self::init().with_default()))
    }

    fn lock() -> MutexGuard<'static, TextureManager> {
        Self::load().lock().unwrap()
    }

    pub fn get_texture(source: &IconSource) -> Option<TextureId> {
        let textures = Self::lock();
        textures.loaded.get(source).copied().or(textures.default)
    }

    pub fn add_source(source: &IconSource) {
        if source.needs_load() {
            // send to loader thread
            if Self::lock().sender.send(source.clone()).is_err() {
                log::warn!("texture loading thread receiver disconnected");
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

    fn add_pending(&mut self, source: IconSource) -> String {
        let id = source.generate_id();
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
                self.default = Some(texture_id);
            } else {
                self.loaded.insert(source, texture_id);
            }
        } else {
            log::warn!("Failed to load icon source {}", source.pretty_print());
        }
    }
}
