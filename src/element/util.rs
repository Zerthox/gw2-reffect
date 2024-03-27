use nexus::imgui::Ui;

pub fn add_pos(a: [f32; 2], b: [f32; 2]) -> [f32; 2] {
    let [ax, ay] = a;
    let [bx, by] = b;
    [ax + bx, ay + by]
}

pub fn with_offset(ui: &Ui, offset: [f32; 2], body: impl FnOnce()) {
    let cursor = ui.cursor_screen_pos();
    ui.set_cursor_screen_pos(add_pos(cursor, offset));
    body();
    ui.set_cursor_screen_pos(cursor);
}
