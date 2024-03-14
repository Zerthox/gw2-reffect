mod icon;

pub use self::icon::*;

use arcdps_imgui::Ui;

pub trait Render {
    fn render(&mut self, ui: Ui);
}

#[derive(Debug, Clone)]
pub enum Element {
    Icon(Icon),
}

impl Render for Element {
    fn render(&mut self, ui: Ui) {
        match self {
            Self::Icon(icon) => icon.render(ui),
        }
    }
}
