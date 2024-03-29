#[derive(Debug, Clone)]
pub struct State {
    pub pos: [f32; 2],
}

impl State {
    pub const fn new() -> Self {
        Self { pos: [0.0, 0.0] }
    }

    pub fn add_offset(&mut self, offset: [f32; 2]) {
        let [x, y] = &mut self.pos;
        let [offset_x, offset_y] = offset;
        *x += offset_x;
        *y += offset_y;
    }

    pub fn with_offset(&mut self, offset: [f32; 2], body: impl FnOnce(&mut State)) {
        let saved = self.pos;
        self.add_offset(offset);
        body(self);
        self.pos = saved;
    }
}
