use super::Addon;
use crate::element::Element;
use nexus::{
    gui::{register_render, RenderType},
    paths::get_addon_dir,
};
use std::fs;

const ADDON_NAME: &str = "reffect";

impl Addon {
    pub fn load() {
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

        Self::lock().load_elements();
    }

    pub fn load_elements(&mut self) {
        let addon_dir = get_addon_dir(ADDON_NAME).expect("invalid addon directory");

        let files = fs::read_dir(addon_dir)
            .expect("failed to read addon directory")
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                matches!(
                    entry.path().extension().and_then(|ext| ext.to_str()),
                    Some("json" | "yml" | "yaml")
                )
            });

        for file in files {
            if let Some(element) = Element::from_file(&file.path()) {
                self.elements.push(element);
            }
        }
    }
}
