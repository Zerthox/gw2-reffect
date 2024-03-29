use super::{util::add_pos, Anchor, Context, Element, Render, State};
use crate::trigger::{PackTrigger, Trigger};
use nexus::imgui::{ImColor32, Ui};
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
    pub author: String,
    pub trigger: PackTrigger,
    pub anchor: Anchor,
    pub pos: [f32; 2],
    pub elements: Vec<Element>,

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
        add_pos(self.anchor.pos(ui), self.pos)
    }
}

impl Render for Pack {
    fn load(&mut self) {
        for element in &mut self.elements {
            element.load();
        }
    }

    fn render(&mut self, ui: &Ui, ctx: &Context, state: &mut State) {
        if self.enabled && self.trigger.is_active(ctx) {
            state.pos = self.pos(ui);
            let [x, y] = state.pos;

            for element in &mut self.elements {
                element.render(ui, ctx, state);
            }
            if ctx.edit {
                const SIZE: f32 = 3.0;
                let start = [x - SIZE, y - SIZE];
                let end = [x + SIZE, y + SIZE];
                ui.get_window_draw_list()
                    .add_rect(start, end, ImColor32::from_rgb(255, 0, 0))
                    .filled(true)
                    .build()
            }
        }
    }
}

impl Default for Pack {
    fn default() -> Self {
        Self {
            enabled: false,
            name: "Unnamed".into(),
            author: "Unknown".into(),
            trigger: PackTrigger::default(),
            anchor: Anchor::TopLeft,
            pos: [0.0, 0.0],
            elements: Vec::new(),
            file: PathBuf::new(),
        }
    }
}
