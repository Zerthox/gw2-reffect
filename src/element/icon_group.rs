use super::{
    util::{add_pos, with_offset},
    Context, Icon, Render,
};
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
    fn load(&mut self) {}

    fn render(&mut self, ui: &Ui, ctx: &Context) {
        with_offset(ui, self.offset, || {
            let initial_pos = ui.cursor_screen_pos();
            let icons = self
                .icons
                .iter_mut()
                .filter(|icon| icon.is_active(ctx))
                .collect::<Vec<_>>();
            let icon_count = icons.len();

            for (i, icon) in icons.into_iter().enumerate() {
                let offset = self.direction.offset_for(self.size, i, icon_count);
                ui.set_cursor_screen_pos(add_pos(initial_pos, offset));
                icon.render(ui, self.size);
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
    Vertical,
    Horizontal,
}

impl Direction {
    pub fn offset_for(&self, size: [f32; 2], element: usize, total: usize) -> [f32; 2] {
        let [width, height] = size;
        let i = element as f32;
        let half = 0.5 * total as f32;
        match self {
            Self::Right => [i * width, -0.5 * height],
            Self::Left => [-i * width, -0.5 * height],
            Self::Up => [-0.5 * width, -i * height],
            Self::Down => [-0.5 * width, i * height],
            Self::Vertical => [-half * width + 0.5 * i * width, 0.5 * height],
            Self::Horizontal => [0.5 * width, -half * height + 0.5 * i * height],
        }
    }
}
