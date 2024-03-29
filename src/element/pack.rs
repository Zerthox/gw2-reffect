use super::{util::ComponentWise, Anchor, Context, Element, Render, State};
use crate::trigger::{PackTrigger, Trigger};
use nexus::imgui::Ui;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Pack {
    pub enabled: bool,
    pub name: String,
    pub trigger: PackTrigger,
    pub anchor: Anchor,
    pub pos: [f32; 2],
    pub elements: Vec<Element>,

    #[serde(skip)]
    pub edit: bool,

    #[serde(skip)]
    pub file: PathBuf,
}

impl Pack {
    pub fn load_from_file(path: impl Into<PathBuf>) -> Option<Self> {
        let path = path.into();
        let file = File::open(&path).ok()?;
        let reader = BufReader::new(file);
        let mut pack: Self = serde_json::from_reader(reader).ok()?;
        pack.file = path;
        pack.load();
        Some(pack)
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

    pub fn render(&mut self, ui: &Ui, ctx: &Context) {
        if self.edit || (self.enabled && self.trigger.is_active(ctx)) {
            let ctx = ctx.with_edit(self.edit);
            let pos = self.pos(ui);
            let mut state = State::with_pos(pos);

            for element in &mut self.elements {
                element.render(ui, &ctx, &mut state);
            }

            if self.edit {
                const SIZE: f32 = 3.0;
                const COLOR: [f32; 4] = [1.0, 0.0, 0.0, 0.8];

                let offset = [SIZE, SIZE];
                let start = pos.sub(offset);
                let end = pos.add(offset);
                ui.get_window_draw_list()
                    .add_rect(start, end, COLOR)
                    .filled(true)
                    .build();

                ui.set_cursor_screen_pos(pos.add([SIZE, 0.0]));
                ui.text_colored(COLOR, &self.name);
            }
        }
    }
}

impl Default for Pack {
    fn default() -> Self {
        Self {
            enabled: false,
            name: "Unnamed".into(),
            trigger: PackTrigger::default(),
            anchor: Anchor::TopLeft,
            pos: [0.0, 0.0],
            elements: Vec::new(),
            edit: false,
            file: PathBuf::new(),
        }
    }
}
