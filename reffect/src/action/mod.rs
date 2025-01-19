mod dynamic;
mod element;

use crate::render::{button_size_with_spacing, close_button};
use nexus::imgui::{Direction, Ui};

pub use self::{dynamic::*, element::*};

// TODO: action clear entire vec?

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

    pub fn perform<T>(self, children: &mut Vec<T>) -> bool {
        match self {
            Self::None => false,
            Self::Up(index) => {
                if index == 0 {
                    let first = children.remove(0);
                    children.push(first);
                } else {
                    children.swap(index, index - 1);
                }
                true
            }
            Self::Down(index) => {
                if index == children.len() - 1 {
                    let last = children.pop().expect("action down with empty vec");
                    children.insert(0, last);
                } else {
                    children.swap(index, index + 1);
                };
                true
            }
            Self::Delete(index) => {
                children.remove(index);
                true
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

    pub fn input_with_buttons<T>(&mut self, ui: &Ui, index: usize, input: impl FnOnce() -> T) -> T {
        Self::set_next_input_size(ui);
        let result = input();
        ui.same_line();
        self.render_buttons(ui, index);
        result
    }

    pub fn set_next_input_size(ui: &Ui) {
        let width = ui.calc_item_width() - 3.0 * button_size_with_spacing(ui);
        ui.set_next_item_width(width);
    }
}

impl Default for Action {
    fn default() -> Self {
        Self::new()
    }
}
