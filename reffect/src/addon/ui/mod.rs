mod debug;
mod editor;
mod options;

use super::Addon;
use crate::{context::Context, elements::RenderCtx, render::Io, tree::Updater};
use nexus::{
    font::{font_receive, get_font},
    imgui::Ui,
};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

impl Addon {
    pub fn render_init(ui: &Ui) {
        static INIT: AtomicBool = AtomicBool::new(true);

        if INIT.load(Ordering::Relaxed) {
            INIT.store(false, Ordering::Relaxed);
            log::debug!("Render init");

            let io = Io::get(ui);

            // subscribe to default font to get notified when atlas rebuilds
            // this is also called immediately
            get_font(
                "FONT_DEFAULT",
                font_receive!(|_, _| {
                    let io = unsafe { Io::force() }; // called in renderer thread
                    Addon::lock().reload_fonts(io)
                }),
            )
            .revert_on_unload();

            let size = io.default_font().map(|font| font.size()).unwrap_or(16.0);
            thread::spawn(move || Self::load_font_files(size));
        }
    }

    pub fn render(&mut self, ui: &Ui) {
        let mut ctx = Context::lock();

        ctx.prepare_render(&self.links);
        if ctx.has_any_update_or_edit() {
            Updater::update(&ctx, &mut self.packs);
        }

        self.render_displays(ui, &ctx);

        if self.debug {
            self.render_debug(ui, &ctx);
        }

        self.render_popups(ui);
        ctx.reset_after_render();
    }

    pub fn render_displays(&mut self, ui: &Ui, ctx: &Context) {
        if ctx.ui.should_show() || ctx.edit.is_editing() {
            let _font = self.settings.font.push(ui);
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
