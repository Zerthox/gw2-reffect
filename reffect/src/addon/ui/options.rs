use super::Addon;
use crate::render::{helper, input_u32};
use nexus::imgui::{TreeNodeFlags, Ui};

impl Addon {
    pub fn render_options(&mut self, ui: &Ui) {
        self.context.edit.update_allowed(&self.context.ui);

        if let Some(_token) = ui.tab_bar("options") {
            if let Some(_token) = ui.tab_item("Editor") {
                self.render_editor(ui);
            }

            if let Some(_token) = ui.tab_item("Settings") {
                ui.checkbox(
                    "Allow edit mode in combat",
                    &mut self.context.edit.during_combat,
                );
                ui.checkbox(
                    "Show all pack elements during edit mode",
                    &mut self.context.edit.show_all,
                );

                self.context.settings.render_options(ui);

                ui.spacing();
                if ui.collapsing_header(
                    "Stacks Text",
                    TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
                ) {
                    let _id = ui.push_id("stackstext");
                    self.context.settings.icon.stack_text.render_options(ui);
                }

                ui.spacing();
                if ui.collapsing_header(
                    "Duration Text",
                    TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
                ) {
                    let _id = ui.push_id("duratext");
                    self.context.settings.icon.duration_text.render_options(ui);
                }

                ui.spacing();
                if ui.collapsing_header(
                    "Duration Bar",
                    TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
                ) {
                    let _id = ui.push_id("durabar");
                    self.context.settings.icon.duration_bar.render_options(ui);
                }

                ui.spacing();
                if ui.collapsing_header(
                    "Advanced",
                    TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
                ) {
                    if input_u32(
                        ui,
                        "Combat updates",
                        &mut self.context.state_interval.frequency,
                        10,
                        100,
                    ) {
                        self.context
                            .state_interval
                            .refresh_next_update(self.context.now);
                    }
                    helper(ui, || {
                        ui.text("Interval between updates for combat information in milliseconds")
                    });

                    if input_u32(
                        ui,
                        "Player updates",
                        &mut self.context.player_interval.frequency,
                        10,
                        100,
                    ) {
                        self.context
                            .player_interval
                            .refresh_next_update(self.context.now);
                    }
                    helper(ui, || {
                        ui.text(
                            "Interval between updates for profession, specialization, map etc. in milliseconds",
                        )
                    });

                    if ui.button("Reset update intervals") {
                        self.context.reset_intervals();
                    }

                    ui.checkbox("Debug window", &mut self.debug);
                }
            }
        }
    }
}
