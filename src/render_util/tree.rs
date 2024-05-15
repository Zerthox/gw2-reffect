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
    let open = ui.is_item_clicked() && !ui.is_item_toggled_open();
    (token, open)
}

pub fn collapsing_header_same_line_end(ui: &Ui, size_x: f32) {
    let [_, min_y] = ui.item_rect_min();
    let [max_x, _] = ui.item_rect_max();
    let pos_x = max_x - size_x;
    let pos_y = min_y;
    ui.set_cursor_screen_pos([pos_x, pos_y]);
}
