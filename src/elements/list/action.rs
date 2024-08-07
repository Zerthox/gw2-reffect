use super::ListIcon;
use crate::{
    context::EditState,
    elements::{Element, ElementType},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub enum IconAction {
    None,
    Up(usize),
    Down(usize),
    Delete(usize),
    Cut(usize),
    Paste(usize),
}

impl IconAction {
    pub const fn new() -> Self {
        Self::None
    }

    pub fn perform(
        self,
        children: &mut Vec<ListIcon>,
        size: [f32; 2],
        state: &mut EditState,
    ) -> bool {
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
                    let last = children.pop().expect("icon action down with empty vec");
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
            Self::Cut(index) => {
                state.set_clipboard(children.remove(index).into_element(size));
                true
            }
            Self::Paste(index) => {
                if let Some(Element {
                    common,
                    kind: ElementType::Icon(element),
                    ..
                }) = state.take_clipboard()
                {
                    children.insert(index, ListIcon::from_element(common, element));
                } else {
                    panic!("icon action paste without icon element");
                }
                true
            }
        }
    }
}

impl Default for IconAction {
    fn default() -> Self {
        Self::new()
    }
}
