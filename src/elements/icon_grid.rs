use super::{Direction, IconNamed, Layout, RenderState};
use crate::{
    action::Action,
    colors,
    context::Context,
    render_util::{
        collapsing_header_same_line_end, delete_confirm_modal, enum_combo, input_float_with_format,
        input_size,
    },
    traits::{Render, RenderOptions, TreeLeaf},
};
use nexus::imgui::{
    self as ig, CollapsingHeader, ComboBoxFlags, InputTextFlags, StyleColor, TreeNodeFlags, Ui,
};
use serde::{Deserialize, Serialize};

// TODO: wrapping, sorting options

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconGrid {
    pub layout: Layout,
    pub direction: Direction,
    pub size: [f32; 2],
    pub pad: f32,
    pub icons: Vec<IconNamed>,
}

// technically there is children, but they are not full elements
impl TreeLeaf for IconGrid {}

impl Render for IconGrid {
    fn render(&mut self, ui: &Ui, ctx: &Context, state: &RenderState) {
        let render_icon = |icon: &mut IconNamed, i, len| {
            let offset = self.direction.offset_for(self.size, self.pad, i, len);
            icon.render(ui, ctx, &state.with_offset(offset), self.size);
        };

        match self.layout {
            Layout::Dynamic => {
                let mut filtered = Vec::new();
                for icon in &mut self.icons {
                    if icon.is_visible(ctx, state) {
                        filtered.push(icon);
                    }
                }
                let len = filtered.len();

                for (i, icon) in filtered.into_iter().enumerate() {
                    render_icon(icon, i, len);
                }
            }
            Layout::Static => {
                let len = self.icons.len();
                for (i, icon) in self.icons.iter_mut().enumerate() {
                    if icon.is_visible(ctx, state) {
                        render_icon(icon, i, len);
                    }
                }
            }
        };
    }
}

impl RenderOptions for IconGrid {
    fn render_options(&mut self, ui: &Ui) {
        enum_combo(ui, "Layout", &mut self.layout, ComboBoxFlags::empty());

        enum_combo(ui, "Direction", &mut self.direction, ComboBoxFlags::empty());

        let [x, y] = &mut self.size;
        input_size(x, y);

        input_float_with_format(
            "Spacing",
            &mut self.pad,
            1.0,
            10.0,
            "%.2f",
            InputTextFlags::empty(),
        );

        ui.spacing();
        ui.text_disabled("Icons");

        let mut action = Action::new();
        for (i, icon) in self.icons.iter_mut().enumerate() {
            let _id = ui.push_id(i as i32);

            let mut remains = true;
            let open = CollapsingHeader::new(format!("{}###icon{i}", icon.name))
                .flags(TreeNodeFlags::ALLOW_ITEM_OVERLAP)
                .begin_with_close_button(ui, &mut remains);

            let title = format!("Confirm Delete##gridicon{i}");
            if !remains {
                ui.open_popup(&title);
            }
            if delete_confirm_modal(ui, &title, || {
                ui.text(format!("Delete Icon {}?", icon.name))
            }) {
                action = Action::Delete(i);
            }

            let size_x = ui.frame_height();
            let [spacing_x, _] = ui.clone_style().item_spacing;
            let button_color = ui.push_style_color(StyleColor::Button, colors::TRANSPARENT);
            collapsing_header_same_line_end(ui, 3.0 * size_x + 2.0 * spacing_x);

            if ui.arrow_button("up", ig::Direction::Up) {
                action = Action::Up(i);
            }

            ui.same_line();
            if ui.arrow_button("down", ig::Direction::Down) {
                action = Action::Down(i);
            }

            button_color.end();

            if open {
                // TODO: apply option to all context menu option
                icon.render_options(ui);
                ui.spacing();
            }
        }
        if ui.button("Add Icon") {
            self.icons.push(IconNamed::default());
        }

        action.perform(&mut self.icons);
    }
}

impl Default for IconGrid {
    fn default() -> Self {
        Self {
            layout: Layout::Dynamic,
            direction: Direction::Right,
            size: [32.0, 32.0],
            pad: 2.0,
            icons: Vec::new(),
        }
    }
}
