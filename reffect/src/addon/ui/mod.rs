mod debug;
mod editor;
mod options;

use super::Addon;
use crate::{context::ContextUpdate, tree::FilterUpdater};
use nexus::imgui::Ui;

impl Addon {
    pub fn render(&mut self, ui: &Ui) {
        self.context.update(); // TODO: perform update in separate thread?

        if self.context.has_update(ContextUpdate::Map) {
            FilterUpdater::update(&self.context, &mut self.packs);
        }

        self.render_displays(ui);

        if self.debug {
            self.render_debug(ui);
        }

        self.render_popups(ui);

        self.context.edit.reset_allowed();
    }

    pub fn render_displays(&mut self, ui: &Ui) {
        // TODO: profiling?
        if self.context.ui.should_show() || self.context.edit.is_editing() {
            let _font = self.context.settings.font.push();
            for pack in &mut self.packs {
                pack.render(ui, &self.context);
            }
        }
    }

    fn render_popups(&mut self, ui: &Ui) {
        const CREATE_ERROR_TITLE: &str = "Pack Creation Error##reffect";

        if self.create_error {
            self.create_error = false;
            ui.open_popup(CREATE_ERROR_TITLE)
        }

        ui.popup_modal(CREATE_ERROR_TITLE)
            .always_auto_resize(true)
            .save_settings(false)
            .build(ui, || {
                ui.text("Can not create outside of packs folder");
                if ui.button("Ok") {
                    ui.close_current_popup();
                }
                ui.set_item_default_focus();
            });
    }
}
