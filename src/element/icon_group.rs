use super::{util::add_pos, Context, Icon, Render, State};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconGroup {
    pub name: String,
    pub direction: Direction,
    pub size: [f32; 2],
    pub padding: f32,
    pub offset: [f32; 2],
    pub icons: Vec<Icon>,
}

impl Render for IconGroup {
    fn load(&mut self) {
        for icon in &mut self.icons {
            icon.load();
        }
    }

    fn render(&mut self, ui: &Ui, ctx: &Context, state: &mut State) {
        state.with_offset(self.offset, |state| {
            let icons = self
                .icons
                .iter_mut()
                .filter(|icon| icon.is_active(ctx))
                .collect::<Vec<_>>();
            let icon_count = icons.len();

            let start_pos = add_pos(state.pos, self.direction.initial_offset(self.size));
            for (i, icon) in icons.into_iter().enumerate() {
                let offset = self
                    .direction
                    .offset_for(self.size, self.padding, i, icon_count);
                let pos = add_pos(start_pos, offset);
                icon.render(ui, pos, self.size);
            }
        })
    }
}

impl Default for IconGroup {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
    Horizontal,
    Vertical,
}

impl Direction {
    pub fn initial_offset(&self, size: [f32; 2]) -> [f32; 2] {
        let [width, height] = size;
        match self {
            Self::Right | Self::Left | Self::Horizontal => [0.0, -0.5 * height], // center horizontally
            Self::Up | Self::Down | Self::Vertical => [-0.5 * width, 0.0], // center vertically
        }
    }

    pub fn offset_for(&self, size: [f32; 2], pad: f32, element: usize, total: usize) -> [f32; 2] {
        let [width, height] = size;
        let i = element as f32;
        let offset_x = i * (width + pad);
        let offset_y = i * (height + pad);
        let half = 0.5 * total as f32;
        let half_pad = 0.5 * total.saturating_sub(1) as f32 * pad;
        match self {
            Self::Right => [offset_x, 0.0],
            Self::Left => [-offset_x, 0.0],
            Self::Up => [0.0, -offset_y],
            Self::Down => [0.0, offset_x],
            Self::Horizontal => [offset_x - half * width - half_pad, 0.0],
            Self::Vertical => [0.0, offset_y - half * height - half_pad],
        }
    }
}
