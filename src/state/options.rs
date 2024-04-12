use crate::{elements::Element, util::tree_select};
use nexus::imgui::Ui;
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct OptionsState {
    pub selected: Uuid,
}

impl OptionsState {
    pub fn render_select_tree(
        &mut self,
        ui: &Ui,
        guid: Uuid,
        name: &str,
        kind: &str,
        children: &mut [Element],
    ) {
        let id = guid.simple().to_string();
        let label = format!("{kind}: {name}##{id}");
        if tree_select(
            ui,
            id,
            label,
            self.is_active(guid),
            children.is_empty(),
            || {
                for child in children {
                    child.render_select_tree(ui, self);
                }
            },
        ) {
            self.selected = guid;
        }
    }

    pub fn is_active(&self, guid: Uuid) -> bool {
        guid == self.selected
    }
}

macro_rules! render_or_children {
    ( $self:ident, $ui:expr, $state:expr ) => {
        if $state.is_active($self.guid) {
            $self.render_options($ui);
            true
        } else {
            $self
                .children()
                .iter_mut()
                .any(|child| child.try_render_options($ui, $state))
        }
    };
}

pub(crate) use render_or_children;
