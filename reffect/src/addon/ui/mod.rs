mod debug;
mod editor;
mod options;

use super::Addon;
use crate::{
    context::{Context, Update},
    elements::RenderCtx,
    tree::FilterUpdater,
};
use nexus::imgui::Ui;

impl Addon {
    pub fn render(&mut self, ui: &Ui) {
        let mut ctx = Context::lock();

        ctx.prepare_render(&self.links);
        if ctx.has_update(Update::Map) {
            FilterUpdater::update(&ctx, &mut self.packs);
        }

        self.render_displays(ui, &ctx);

        if self.debug {
            self.render_debug(ui, &ctx);
        }

        self.render_popups(ui);
        ctx.reset();
    }

    pub fn render_displays(&mut self, ui: &Ui, ctx: &Context) {
        if ctx.ui.should_show() || ctx.edit.is_editing() {
            let _font = self.settings.font.push();
            let ctx = RenderCtx::create(ui, ctx, &self.settings);
            for pack in &mut self.packs {
                pack.render(ui, &ctx);
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
