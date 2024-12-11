use super::Addon;
use crate::{
    elements::Pack,
    internal::{Interface, Internal},
    settings::Settings,
    texture_manager::TextureManager,
    util::file_name,
};
use nexus::{
    font::{font_receive, get_font},
    gui::{register_render, RenderType},
};
use rfd::FileDialog;
use std::{fs, thread};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

impl Addon {
    pub fn load() {
        log::info!("Reffect v{VERSION} load");
        TextureManager::load();

        register_render(
            RenderType::Render,
            nexus::gui::render!(|ui| Addon::lock().render(ui)),
        )
        .revert_on_unload();

        register_render(
            RenderType::OptionsRender,
            nexus::gui::render!(|ui| Addon::lock().render_options(ui)),
        )
        .revert_on_unload();

        // subscribe to default font to get notified when atlas rebuilds
        get_font("FONT_DEFAULT", font_receive!(|_, _| Addon::reload_fonts())).revert_on_unload();

        Internal::init();

        Self::create_dirs();

        let mut plugin = Self::lock();
        if let Some(settings) = Settings::try_load() {
            plugin.context.load(settings.context);
        }
        plugin.load_packs();
    }

    pub fn unload() {
        log::info!("Reffect v{VERSION} unload");

        {
            let plugin = Self::lock();
            Settings::new(&plugin.context).save();
            if plugin.context.save_on_unload {
                plugin.save_packs();
            }
        }

        TextureManager::unload();
    }

    pub fn create_dirs() {
        let _ = fs::create_dir_all(Self::packs_dir());
        let _ = fs::create_dir(Self::icons_dir());
    }

    pub fn load_packs(&mut self) {
        let dir = Self::packs_dir();
        log::info!("Loading packs from \"{}\"", dir.display());

        Self::create_dirs();
        match fs::read_dir(&dir) {
            Ok(iter) => {
                let files = iter.filter_map(|entry| entry.ok()).filter(|entry| {
                    matches!(
                        entry.path().extension().and_then(|ext| ext.to_str()),
                        Some("json" | "yml" | "yaml")
                    )
                });

                for file in files {
                    if let Some(pack) = Pack::load_from_file(file.path()) {
                        self.add_pack(pack);
                    }
                }
                log::info!("Loaded {} packs", self.packs.len());
            }

            Err(err) => log::error!("Failed to read pack directory: {err}"),
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
                    pack.common.name,
                    file_name(&pack.file)
                );
            }
            Err(err) => log::error!(
                "Failed to delete pack \"{}\" file \"{}\": {err}",
                pack.common.name,
                file_name(&pack.file)
            ),
        }
    }

    pub fn save_packs(&self) {
        log::info!("Saving packs");
        Self::create_dirs();
        for pack in &self.packs {
            pack.save_to_file();
        }
    }

    pub fn open_create_dialog(&self) {
        // just spawn a thread to not have to deal with futures
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

    pub fn reload_fonts() {
        log::debug!("Reloading fonts");
        let mut addon = Self::lock();
        addon.context.font.reload();
        for pack in &mut addon.packs {
            pack.reload_fonts();
        }
    }

    pub fn open_addon_folder(&self) {
        if let Err(err) = open::that_detached(Self::addon_dir()) {
            log::error!("Failed to open addon folder: {err}");
        }
    }

    pub fn open_packs_folder(&self) {
        if let Err(err) = open::that_detached(Self::packs_dir()) {
            log::error!("Failed to open packs folder: {err}");
        }
    }

    pub fn open_doc(&self, file: &'static str) {
        let url = format!("{REPOSITORY}/tree/main/docs/{file}.md");
        if let Err(err) = open::that_detached(url) {
            log::error!("Failed to open docs URL: {err}");
        }
    }
}
