mod element;

use crate::render_util::{button_disabled, close_button};
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
            Self::Up(index) => children.swap(index, index - 1), // TODO: index wrap around instead of disable?
            Self::Down(index) => children.swap(index, index + 1),
            Self::Delete(index) => {
                children.remove(index);
            }
        }
    }

    pub fn render_buttons(&mut self, ui: &Ui, index: usize, len: usize) {
        let enabled = index > 0;
        button_disabled(ui, enabled, || {
            if ui.arrow_button("up", Direction::Up) && enabled {
                *self = Action::Up(index);
            }
        });

        let enabled = index < len - 1;
        button_disabled(ui, enabled, || {
            ui.same_line();
            if ui.arrow_button("down", Direction::Down) && enabled {
                *self = Action::Down(index);
            }
        });

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
