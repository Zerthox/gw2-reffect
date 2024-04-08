use super::{Direction, Icon, Render, RenderContext, RenderState};
use crate::component_wise::ComponentWise;
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconGrid {
    pub name: String,
    pub direction: Direction,
    pub size: [f32; 2],
    pub padding: f32,
    pub offset: [f32; 2],
    pub icons: Vec<Icon>,
}

impl Render for IconGrid {
    fn load(&mut self) {
        for icon in &mut self.icons {
            icon.load();
        }
    }

    fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &mut RenderState) {
        state.with_offset(self.offset, |state| {
            let icons = self
                .icons
                .iter_mut()
                .filter(|icon| icon.is_active(ctx))
                .collect::<Vec<_>>();
            let icon_count = icons.len();

            let start_pos = state.pos;
            for (i, icon) in icons.into_iter().enumerate() {
                let offset = self
                    .direction
                    .offset_for(self.size, self.padding, i, icon_count);
                let pos = start_pos.add(offset);
                icon.render(ui, ctx, pos, self.size);
            }
        })
    }
}

impl Default for IconGrid {
    fn default() -> Self {
        Self {
            name: "Unnamed".into(),
            direction: Direction::Right,
            padding: 5.0,
            size: [32.0, 32.0],
            offset: [0.0, 0.0],
            icons: Vec::new(),
        }
    }
}
