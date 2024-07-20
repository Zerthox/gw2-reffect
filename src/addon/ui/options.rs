use super::Addon;
use crate::{
    elements::TextDecoration,
    render_util::{
        enum_combo, font_select, helper, input_float_with_format, input_u32, style_disabled,
    },
};
use nexus::imgui::{ComboBoxFlags, InputTextFlags, TreeNodeFlags, Ui};

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

                font_select(ui, "Font", &mut self.context.font);

                if ui.collapsing_header(
                    "Stacks Text (soon)",
                    TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
                ) {
                    let _style = style_disabled(ui); // TODO: stacks settings

                    let _id = ui.push_id("stackstext");
                    enum_combo(
                        ui,
                        "Decoration",
                        &mut TextDecoration::Shadow,
                        ComboBoxFlags::empty(),
                    );

                    input_float_with_format(
                        "Size",
                        &mut 100.0,
                        1.0,
                        10.0,
                        "%.2f",
                        InputTextFlags::READ_ONLY,
                    );

                    input_float_with_format(
                        "Position x",
                        &mut 0.0,
                        10.0,
                        100.0,
                        "%.2f",
                        InputTextFlags::READ_ONLY,
                    );
                    input_float_with_format(
                        "Position y",
                        &mut 0.0,
                        10.0,
                        100.0,
                        "%.2f",
                        InputTextFlags::READ_ONLY,
                    );
                }

                if ui.collapsing_header(
                    "Duration Text (soon)",
                    TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
                ) {
                    let _style = style_disabled(ui); // TODO: duration text settings

                    let _id = ui.push_id("duratext");
                    enum_combo(
                        ui,
                        "Decoration",
                        &mut TextDecoration::Outline,
                        ComboBoxFlags::empty(),
                    );

                    input_float_with_format(
                        "Size",
                        &mut 100.0,
                        1.0,
                        10.0,
                        "%.2f",
                        InputTextFlags::READ_ONLY,
                    );

                    input_float_with_format(
                        "Position x",
                        &mut 0.0,
                        10.0,
                        100.0,
                        "%.2f",
                        InputTextFlags::READ_ONLY,
                    );
                    input_float_with_format(
                        "Position y",
                        &mut 0.0,
                        10.0,
                        100.0,
                        "%.2f",
                        InputTextFlags::READ_ONLY,
                    );
                }

                // TODO: duration bar settings

                if ui.collapsing_header("Advanced", TreeNodeFlags::SPAN_AVAIL_WIDTH) {
                    if input_u32(
                        ui,
                        "Own character interval",
                        &mut self.context.own_interval.frequency,
                        10,
                        100,
                    ) {
                        self.context
                            .own_interval
                            .refresh_next_update(self.context.now);
                    }
                    helper(ui, || {
                        ui.text(
                            "Interval between updates for own buffs & resources in milliseconds",
                        )
                    });

                    if input_u32(
                        ui,
                        "Player interval",
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
