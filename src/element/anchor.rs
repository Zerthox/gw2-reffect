use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};

/// Screen anchor point.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Anchor {
    /// Anchored to top left of screen.
    TopLeft,

    /// Anchored to top right of screen.
    TopRight,

    /// Anchored to bottom left of screen.
    BottomLeft,

    /// Anchored to bottom right of screen.
    BottomRight,

    /// Anchored to center of screen.
    Center,
}

impl Anchor {
    /// Calculates the screen position.
    pub fn pos(&self, ui: &Ui) -> [f32; 2] {
        let [screen_x, screen_y] = ui.io().display_size;
        match self {
            Self::TopLeft => [0.0, 0.0],
            Self::TopRight => [screen_x, 0.0],
            Self::BottomLeft => [0.0, screen_y],
            Self::BottomRight => [screen_x, screen_y],
            Self::Center => [0.5 * screen_x, 0.5 * screen_y],
        }
    }

    /// Sets the cursor to the anchor position.
    pub fn set_cursor(&self, ui: &Ui) {
        let pos = self.pos(ui);
        ui.set_cursor_screen_pos(pos);
    }

    /// Renders a select for the anchor.
    pub fn render_select(&mut self, ui: &Ui) {
        ui.group(|| {
            ui.radio_button("Top left", self, Self::TopLeft);
            ui.radio_button("Top right", self, Self::TopRight);
            ui.radio_button("Bottom left", self, Self::BottomLeft);
            ui.radio_button("Bottom right", self, Self::BottomRight);
            ui.radio_button("Center", self, Self::Center);
        });
    }
}
