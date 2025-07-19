use super::Addon;
use crate::context::Context;
use nexus::imgui::{TreeNodeFlags, Ui};

impl Addon {
    pub fn render_options(&mut self, ui: &Ui) {
        let ctx = &mut *Context::lock();
        ctx.edit.update_allowed(&ctx.ui);

        if let Some(_token) = ui.tab_bar("options") {
            if let Some(_token) = ui.tab_item("Editor") {
                self.render_editor(ui, ctx);
            }

            if let Some(_token) = ui.tab_item("Settings") {
                ui.checkbox(
                    "Allow edit mode in combat",
                    &mut ctx.edit.settings.during_combat,
                );
                ui.checkbox(
                    "Show all pack elements during edit mode",
                    &mut ctx.edit.settings.show_all,
                );

                self.settings.render_options(ui);

                ui.spacing();
                if ui.collapsing_header(
                    "Stacks Text",
                    TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
                ) {
                    let _id = ui.push_id("stackstext");
                    self.settings.icon.stack_text.render_options(ui);
                }

                ui.spacing();
                if ui.collapsing_header(
                    "Duration Text",
                    TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
                ) {
                    let _id = ui.push_id("duratext");
                    self.settings.icon.duration_text.render_options(ui);
                }

                ui.spacing();
                if ui.collapsing_header(
                    "Duration Bar",
                    TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
                ) {
                    let _id = ui.push_id("durabar");
                    self.settings.icon.duration_bar.render_options(ui);
                }

                ui.checkbox("Debug window", &mut self.debug);

                #[cfg(feature = "profile")]
                {
                    use reffect_core::profiling;

                    let mut enabled = profiling::enabled();
                    if ui.checkbox("Profiling", &mut enabled) {
                        profiling::toggle(enabled);
                    }
                }
            }
        }
    }
}
