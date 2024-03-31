#[derive(Debug, Clone)]
pub struct RenderState {
    pub pos: [f32; 2],
}

impl RenderState {
    pub const fn with_pos(pos: [f32; 2]) -> Self {
        Self { pos }
    }

    pub fn add_offset(&mut self, offset: [f32; 2]) {
        let [x, y] = &mut self.pos;
        let [offset_x, offset_y] = offset;
        *x += offset_x;
        *y += offset_y;
    }

    pub fn with_offset(&mut self, offset: [f32; 2], body: impl FnOnce(&mut RenderState)) {
        let saved = self.pos;
        self.add_offset(offset);
        body(self);
        self.pos = saved;
    }
}
