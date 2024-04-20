use nexus::imgui::{TreeNode, TreeNodeFlags, TreeNodeToken, Ui};

pub fn tree_select_empty<'ui>(
    ui: &'ui Ui,
    id: impl AsRef<str>,
    selected: bool,
    leaf: bool,
) -> (Option<TreeNodeToken<'ui>>, bool) {
    let token = TreeNode::new(id)
        .label::<&str, _>("") // FIXME: unused type param in imgui-rs
        .flags(TreeNodeFlags::SPAN_AVAIL_WIDTH)
        .open_on_arrow(true)
        .selected(selected)
        .leaf(leaf)
        .tree_push_on_open(!leaf)
        .push(ui);
    (token, ui.is_item_clicked() && !ui.is_item_toggled_open())
}
