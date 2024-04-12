use super::{Anchor, Element, Node};
use crate::{
    component_wise::ComponentWise,
    context::RenderContext,
    state::{render_or_children, OptionsState, RenderState},
    util::{enum_combo, position_input},
};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Pack {
    pub enabled: bool, // TODO: store enabled separately in addon settings?
    pub name: String,  // TODO: group things common between pack & element in struct?
    pub layer: i32,
    pub anchor: Anchor,
    pub pos: [f32; 2],
    pub elements: Vec<Element>,

    #[serde(skip)]
    pub file: PathBuf,

    #[serde(skip)]
    pub edit: bool,

    #[serde(skip)]
    pub guid: Uuid,
}

impl Pack {
    pub fn load_from_file(path: impl Into<PathBuf>) -> Option<Self> {
        let path = path.into();
        let file = File::open(&path).ok()?;
        let reader = BufReader::new(file);
        match serde_json::from_reader::<_, Self>(reader) {
            Ok(mut pack) => {
                pack.file = path;
                pack.load();
                Some(pack)
            }
            Err(err) => {
                log::warn!("Failed to parse pack \"{}\": {}", path.display(), err);
                None
            }
        }
    }

    pub fn save_to_file(&self) -> Option<()> {
        let file = File::create(&self.file).ok()?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, self).ok()
    }

    pub fn pos(&self, ui: &Ui) -> [f32; 2] {
        self.anchor.calc_pos(ui).add(self.pos)
    }

    pub fn load(&mut self) {
        for element in &mut self.elements {
            element.load();
        }
    }

    pub fn render(&mut self, ui: &Ui, ctx: &RenderContext) {
        let ctx = ctx.with_edit(self.edit);
        if self.enabled && ctx.should_show() {
            let pos = self.pos(ui);
            let mut state = RenderState::with_pos(pos);

            for element in &mut self.elements {
                element.render(ui, &ctx, &mut state);
            }

            if ctx.edit {
                const SIZE: f32 = 3.0;
                const COLOR: [f32; 4] = [1.0, 0.0, 0.0, 0.8];

                let offset = [SIZE, SIZE];
                let start = pos.sub(offset);
                let end = pos.add(offset);
                ui.get_window_draw_list()
                    .add_rect(start, end, COLOR)
                    .filled(true)
                    .build();

                ui.set_cursor_screen_pos(pos.add([SIZE + 1.0, 0.0]));
                ui.text_colored(COLOR, &self.name);
            }
        }
    }

    pub fn render_select_tree(&mut self, ui: &Ui, state: &mut OptionsState) {
        state.render_select_tree(ui, self.guid, &self.name, "Pack", &mut self.elements)
    }

    pub fn try_render_options(&mut self, ui: &Ui, state: &OptionsState) -> bool {
        render_or_children!(self, ui, state)
    }

    pub fn render_options(&mut self, ui: &Ui) {
        ui.checkbox("Enabled", &mut self.enabled);

        ui.align_text_to_frame_padding();
        ui.text("Name");
        ui.same_line();
        ui.input_text("##name", &mut self.name).build();

        ui.text("Layer");
        ui.same_line();
        ui.text_disabled("coming soon"); // TODO: layer input, then sort packs

        enum_combo(ui, "Anchor", &mut self.anchor);

        let [x, y] = &mut self.pos;
        position_input(ui, x, y);
    }
}

impl Node for Pack {
    fn children(&mut self) -> &mut [Element] {
        &mut self.elements
    }
}

impl Default for Pack {
    fn default() -> Self {
        Self {
            name: "Unnamed".into(),
            enabled: false,
            layer: 0,
            anchor: Anchor::TopLeft,
            pos: [0.0, 0.0],
            elements: Vec::new(),
            file: PathBuf::new(),
            edit: false,
            guid: Uuid::new_v4(),
        }
    }
}
