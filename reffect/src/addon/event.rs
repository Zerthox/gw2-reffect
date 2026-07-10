use super::Addon;
use crate::{
    context::Context,
    elements::Pack,
    file::TempFile,
    internal::{Interface, Internal},
    render::Io,
    settings::AddonSettings,
    texture::TextureManager,
    tree::Updater,
};
use nexus::{
    font::{font_receive, get_font},
    gui::{RenderType, register_render, render},
};
use rfd::FileDialog;
use std::{fs, thread};

const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

impl Addon {
    pub fn load() {
        log::info!("Reffect v{} load", Self::VERSION);
        TextureManager::load();

        register_render(RenderType::Render, render!(|ui| Addon::lock().render(ui)))
            .revert_on_unload();

        register_render(
            RenderType::OptionsRender,
            render!(|ui| Addon::lock().render_options(ui)),
        )
        .revert_on_unload();

        // subscribe to default font to get notified when atlas rebuilds
        get_font(
            "FONT_DEFAULT",
            font_receive!(|_, _| {
                let io = unsafe { Io::force() }; // called in renderer thread
                Addon::lock().load_fonts(io)
            }),
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

    pub fn load_packs(&mut self, ctx: &mut Context) {
        let dir = Self::packs_dir();
        log::info!("Loading packs from \"{}\"", dir.display());

        Self::create_dirs();
        match fs::read_dir(&dir) {
            Ok(iter) => {
                for entry in iter.filter_map(|entry| entry.ok()) {
                    let path = entry.path();
                    let ext = path.extension().and_then(|ext| ext.to_str());
                    if let Some("json" | "yml" | "yaml") = ext {
                        if let Some(pack) = Pack::load_from_file(path) {
                            self.add_pack(pack);
                        }
                    } else if TempFile::is_temp(&path) {
                        log::warn!("Leftover temp pack file \"{}\"", path.display());
                    }
                }
                log::info!("Loaded {} packs", self.packs.len());

                Updater::load(ctx, &mut self.packs);
            }
            Err(err) => log::error!("Failed to read packs directory: {err}"),
        }
    }

    pub fn add_pack(&mut self, pack: Pack) {
        let index = self
            .packs
            .partition_point(|entry| entry.layer <= pack.layer);
        self.packs.insert(index, pack);
    }

    pub fn delete_pack(&mut self, index: usize) {
        let pack = &self.packs[index];
        match fs::remove_file(&pack.file) {
            Ok(_) => {
                let pack = self.packs.remove(index);
                log::info!(
                    "Deleted pack \"{}\" file \"{}\"",
                    pack.name(),
                    pack.file.display()
                );
            }
            Err(err) => log::error!(
                "Failed to delete pack \"{}\" file \"{}\": {err}",
                pack.name(),
                pack.file.display()
            ),
        }
    }

    pub fn save_packs(&self) -> thread::JoinHandle<()> {
        log::info!("Saving packs");
        Self::create_dirs();
        let files = self
            .packs
            .iter()
            .filter_map(|pack| pack.save_temp())
            .collect::<Vec<_>>();
        thread::spawn(move || {
            for file in files {
                let target = file.target_path();
                if let Err(err) = file.persist() {
                    log::error!(
                        "Failed to persist temp pack file \"{}\": {err}",
                        target.display()
                    );
                }
            }

            log::debug!("Persisted temp pack files");
        })
    }

    pub fn open_create_dialog(&self) {
        thread::spawn(|| {
            Self::create_dirs();
            let packs = Self::packs_dir();
            if let Some(file) = FileDialog::new()
                .set_title("Save Pack")
                .set_directory(&packs)
                .add_filter("JSON", &["json"])
                .save_file()
            {
                log::debug!("Request to create {}", file.display());
                if let Some(dir) = file.parent() {
                    if dir == packs {
                        if let Some(pack) = Pack::create(file) {
                            Self::lock().add_pack(pack);
                        }
                    } else {
                        Self::lock().create_error = true;
                        log::warn!("Unable to create pack in \"{}\"", dir.display());
                    }
                }
            }
        });
    }

    pub fn open_docs(&self, file: &'static str) {
        let url = format!("{REPOSITORY}/tree/main/docs/{file}.md");
        if let Err(err) = open::that_detached(url) {
            log::error!("Failed to open docs URL: {err}");
        }
    }
}
