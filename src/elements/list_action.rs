use super::ListIcon;

#[derive(Debug, Clone)]
#[must_use]
pub enum ListAction {
    None,
    Up(usize),
    Down(usize),
    Delete(usize),
    Paste(usize, ListIcon),
}

impl ListAction {
    pub const fn new() -> Self {
        Self::None
    }

    pub fn perform(self, children: &mut Vec<ListIcon>) -> bool {
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
            Self::Paste(index, element) => {
                children.insert(index, element);
                true
            }
        }
    }
}

impl Default for ListAction {
    fn default() -> Self {
        Self::new()
    }
}
