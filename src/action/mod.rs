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
            Self::Up(index) => children.swap(index, index - 1),
            Self::Down(index) => children.swap(index, index + 1),
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
}

impl Default for Action {
    fn default() -> Self {
        Self::new()
    }
}
