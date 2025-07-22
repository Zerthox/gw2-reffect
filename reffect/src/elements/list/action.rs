use super::ListIcon;
use crate::{
    clipboard::Clipboard,
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
    Duplicate(usize),
}

impl IconAction {
    pub const fn new() -> Self {
        Self::None
    }

    pub fn perform(self, children: &mut Vec<ListIcon>, size: [f32; 2]) {
        match self {
            Self::None => {}
            Self::Up(index) => {
                if index == 0 {
                    let first = children.remove(0);
                    children.push(first);
                } else {
                    children.swap(index, index - 1);
                }
            }
            Self::Down(index) => {
                if index == children.len() - 1 {
                    let last = children.pop().expect("icon action down with empty vec");
                    children.insert(0, last);
                } else {
                    children.swap(index, index + 1);
                };
            }
            Self::Delete(index) => {
                children.remove(index);
            }
            Self::Cut(index) => {
                Clipboard::set(children.remove(index).into_element(size));
            }
            Self::Paste(index) => {
                if let Some(Element {
                    common,
                    kind: ElementType::Icon(element),
                    ..
                }) = Clipboard::take()
                {
                    children.insert(index, ListIcon::from_element(common, element));
                } else {
                    panic!("icon action paste without icon element");
                }
            }
            Self::Duplicate(index) => {
                let child = children[index].clone();
                children.insert(index + 1, child);
            }
        }
    }
}

impl Default for IconAction {
    fn default() -> Self {
        Self::new()
    }
}
