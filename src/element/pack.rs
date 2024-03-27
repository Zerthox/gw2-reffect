use super::{util::with_offset, Anchor, Context, Element, Render};
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
}

impl Render for Pack {
    fn load(&mut self) {
        for element in &mut self.elements {
            element.load();
        }
    }

    fn render(&mut self, ui: &Ui, ctx: &Context) {
        if self.trigger.is_active(ctx) {
            self.anchor.set_cursor(ui);
            with_offset(ui, self.pos, || {
                for element in &mut self.elements {
                    element.render(ui, ctx)
                }
            })
        }
    }
}

impl Default for Pack {
    fn default() -> Self {
        Self {
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
