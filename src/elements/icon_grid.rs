use super::{Direction, Element, Icon, Node, Render, RenderState};
use crate::{
    context::RenderContext,
    trigger::Trigger,
    util::{enum_combo, input_float_with_format},
};
use nexus::imgui::{InputTextFlags, Ui};
use serde::{Deserialize, Serialize};

// TODO: wrapping options
// TODO: sorting options

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconGrid {
    pub direction: Direction,
    pub size: [f32; 2],
    pub pad: f32,
    pub icons: Vec<Icon>,
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
            .filter(|icon| icon.buff.is_active_or_edit(ctx, state))
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

        // TODO: icons
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
