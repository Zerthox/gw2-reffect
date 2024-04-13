use super::{Direction, Element, IconNamed, Node, Render, RenderState};
use crate::{
    context::RenderContext,
    trigger::Trigger,
    util::{enum_combo, input_float_with_format},
};
use nexus::imgui::{CollapsingHeader, InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

// TODO: wrapping options
// TODO: sorting options

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconGrid {
    pub direction: Direction,
    pub size: [f32; 2],
    pub pad: f32,
    pub icons: Vec<IconNamed>,
}

impl Node for IconGrid {
    fn load(&mut self) {
        for icon in &mut self.icons {
            icon.load();
        }
    }

    fn children(&mut self) -> &mut [Element] {
        &mut []
    }
}

impl Render for IconGrid {
    fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &RenderState) {
        let icons = self
            .icons
            .iter_mut()
            .filter(|icon| icon.inner.buff.is_active_or_edit(ctx, state))
            .collect::<Vec<_>>();
        let icon_count = icons.len();

        for (i, icon) in icons.into_iter().enumerate() {
            let offset = self
                .direction
                .offset_for(self.size, self.pad, i, icon_count);
            icon.render(ui, ctx, &state.with_offset(offset), self.size);
        }
    }

    fn render_options(&mut self, ui: &Ui) {
        enum_combo(ui, "Direction", &mut self.direction);

        let [x, y] = &mut self.size;
        input_float_with_format("Size x", x, 1.0, 10.0, "%.2f", InputTextFlags::empty());
        input_float_with_format("Size y", y, 1.0, 10.0, "%.2f", InputTextFlags::empty());

        input_float_with_format("Padding", y, 1.0, 10.0, "%.2f", InputTextFlags::empty());

        let mut remove = None;
        for (i, icon) in self.icons.iter_mut().enumerate() {
            let _id = ui.push_id(i as i32);
            let mut open = true;
            if CollapsingHeader::new(&icon.name).build_with_close_button(ui, &mut open) {
                icon.render_options(ui);
            }
            if !open {
                remove = Some(i);
            }
        }
        if ui.button("Add Icon") {
            self.icons.push(IconNamed::default());
        }
        if let Some(index) = remove {
            self.icons.remove(index);
        }
    }
}

impl Default for IconGrid {
    fn default() -> Self {
        Self {
            direction: Direction::Right,
            pad: 3.0,
            size: [32.0, 32.0],
            icons: Vec::new(),
        }
    }
}
