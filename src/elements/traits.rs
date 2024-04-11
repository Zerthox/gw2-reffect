use super::Element;
use crate::{context::RenderContext, state::RenderState};
use nexus::imgui::Ui;

pub trait Node {
    fn load(&mut self) {
        for child in self.children() {
            child.load();
        }
    }

    fn children(&mut self) -> &mut [Element];
}

pub trait Render {
    fn render(&mut self, ui: &Ui, ctx: &RenderContext, state: &mut RenderState);

    fn render_options(&mut self, ui: &Ui);
}
