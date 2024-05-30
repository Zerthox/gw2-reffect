mod element;

use crate::render_util::close_button;
use nexus::imgui::{Direction, Ui};

pub use self::element::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub enum Action {
    None,
    Up(usize),
    Down(usize),
    Delete(usize),
}

impl Action {
    pub const fn new() -> Self {
        Self::None
    }

    pub fn perform<T>(self, children: &mut Vec<T>) {
        match self {
            Self::None => {}
            Self::Up(index) => {
                let other = if index > 0 {
                    index - 1
                } else {
                    children.len() - 1
                };
                children.swap(index, other)
            }
            Self::Down(index) => {
                let other = if index < children.len() - 1 {
                    index + 1
                } else {
                    0
                };
                children.swap(index, other)
            }
            Self::Delete(index) => {
                children.remove(index);
            }
        }
    }

    pub fn render_buttons(&mut self, ui: &Ui, index: usize) {
        if ui.arrow_button("up", Direction::Up) {
            *self = Action::Up(index);
        }

        ui.same_line();
        if ui.arrow_button("down", Direction::Down) {
            *self = Action::Down(index);
        }

        ui.same_line();
        if close_button(ui, "##del") {
            *self = Action::Delete(index);
        }
    }

    pub fn set_next_input_size(&self, ui: &Ui) {
        let button_size = ui.frame_height();
        let [spacing, _] = ui.clone_style().item_inner_spacing;
        let width = ui.calc_item_width() - 3.0 * (button_size + spacing);
        ui.set_next_item_width(width);
    }
}

impl Default for Action {
    fn default() -> Self {
        Self::new()
    }
}
