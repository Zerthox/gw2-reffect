mod dirs;
mod fonts;
mod packs;
mod ui;

use crate::{
    context::Context,
    elements::Pack,
    internal::{Interface, Internal},
    links::Links,
    settings::{AddonSettings, GeneralSettings},
    texture::TextureManager,
    worker::StoppableWorker,
};
use nexus::gui::{RenderType, register_render, render};
use std::sync::{Mutex, MutexGuard, OnceLock};

static ADDON: OnceLock<Mutex<Addon>> = OnceLock::new();

#[derive(Debug)]
pub struct Addon {
    debug: bool,
    create_error: bool,
    links: Links,
    packs: Vec<Pack>,
    settings: GeneralSettings,
    worker: Option<StoppableWorker>,
}

impl Addon {
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");

    const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

    pub fn new() -> Self {
        Self {
            debug: false,
            create_error: false,
            links: Links::load(),
            packs: Vec::new(),
            settings: GeneralSettings::new(),
            worker: None,
        }
    }

    pub fn lock() -> MutexGuard<'static, Addon> {
        ADDON
            .get_or_init(|| Mutex::new(Addon::new()))
            .lock()
            .unwrap()
    }

    pub fn release(&mut self) {
        *self = Self::new();
    }

    pub fn load() {
        log::info!("Reffect v{} load", Self::VERSION);
        TextureManager::load();

        register_render(
            RenderType::Render,
            render!(|ui| {
                Addon::render_init(ui);
                Addon::lock().render(ui);
            }),
        )
        .revert_on_unload();

        register_render(
            RenderType::OptionsRender,
            render!(|ui| Addon::lock().render_options(ui)),
        )
        .revert_on_unload();

        Internal::init();

        Self::create_dirs();

        let mut addon = Self::lock();
        let mut ctx = Context::lock();
        if let Some(settings) = AddonSettings::try_load() {
            settings.apply(&mut addon.settings, &mut ctx);
        }
        addon.worker = Context::create_worker(addon.links.clone());
        addon.load_packs(&mut ctx);
    }

    pub fn unload() {
        log::info!("Reffect v{} unload", Self::VERSION);

        let mut addon = Self::lock();
        AddonSettings::new(&addon.settings, &Context::lock()).save();
        let pack_worker = addon.settings.save_on_unload.then(|| addon.save_packs());

        Internal::deinit();

        TextureManager::unload();

        if let Some(worker) = addon.worker.take() {
            worker.exit_and_wait();
        }
        if let Some(thread) = pack_worker {
            let _ = thread.join();
        }

        addon.release();
        Context::unload();
    }
}
