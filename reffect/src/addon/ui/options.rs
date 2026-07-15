use super::Addon;
use crate::{
    context::Context,
    metadata::{BUILD_TIME, COMMIT, RUSTC},
};
use nexus::imgui::{TreeNodeFlags, Ui};

impl Addon {
    pub fn render_options(&mut self, ui: &Ui) {
        let ctx = &mut *Context::lock();
        ctx.update_edit_mode();

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
                    use crate::profiling;

                    let mut enabled = profiling::enabled();
                    if ui.checkbox("Profiling", &mut enabled) {
                        profiling::toggle(enabled);
                    }
                }
            }

            if let Some(_token) = ui.tab_item("?") {
                ui.text("Reffect");
                ui.same_line();
                copyable_text(
                    ui,
                    format!(
                        "{} ({COMMIT} {})",
                        Self::VERSION,
                        BUILD_TIME.format("%Y-%m-%d %H:%M UTC")
                    ),
                );

                ui.text("Built with");
                ui.same_line();
                copyable_text(ui, RUSTC);

                ui.text("Disclosed source licensed under GNU Lesser General Public License v3");

                ui.spacing();
            }
        }
    }
}

fn copyable_text(ui: &Ui, text: impl AsRef<str>) {
    let text = text.as_ref();
    ui.text(text);
    if ui.is_item_hovered() {
        ui.tooltip_text("Click to copy");
    }
    if ui.is_item_clicked() {
        ui.set_clipboard_text(text);
    }
}
