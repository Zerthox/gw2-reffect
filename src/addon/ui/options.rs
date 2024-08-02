use super::Addon;
use crate::{
    render_util::{
        enum_combo, font_select, helper, input_color_alpha, input_float_with_format, input_percent,
        input_pos, input_seconds, input_u32,
    },
    settings::icon::{DurationBarSettings, DurationTextSettings, StackTextSettings},
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

                ui.spacing();
                if ui.collapsing_header(
                    "Stacks Text",
                    TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
                ) {
                    let StackTextSettings {
                        scale,
                        offset,
                        color,
                        decoration,
                    } = &mut self.context.icon_settings.stack_text;

                    let _id = ui.push_id("stackstext");
                    input_percent("Scale", scale);
                    input_pos(offset);
                    enum_combo(ui, "Decoration", decoration, ComboBoxFlags::empty());
                    input_color_alpha(ui, "Color", color);
                }

                ui.spacing();
                if ui.collapsing_header(
                    "Duration Text",
                    TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
                ) {
                    let DurationTextSettings {
                        min_remain,
                        scale,
                        color,
                        decoration,
                    } = &mut self.context.icon_settings.duration_text;

                    let _id = ui.push_id("duratext");
                    input_seconds("Min remaining", min_remain);
                    helper(ui, || ui.text("Minimum time remaining in seconds"));
                    input_percent("Scale", scale);
                    enum_combo(ui, "Decoration", decoration, ComboBoxFlags::empty());
                    input_color_alpha(ui, "Color", color);
                }

                ui.spacing();
                if ui.collapsing_header(
                    "Duration Bar",
                    TreeNodeFlags::SPAN_AVAIL_WIDTH | TreeNodeFlags::DEFAULT_OPEN,
                ) {
                    let DurationBarSettings { height, color } =
                        &mut self.context.icon_settings.duration_bar;

                    let _id = ui.push_id("durabar");
                    input_float_with_format(
                        "Height",
                        height,
                        1.0,
                        10.0,
                        "%.2f",
                        InputTextFlags::empty(),
                    );
                    input_color_alpha(ui, "Color", color);
                }

                ui.spacing();
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
