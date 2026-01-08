use super::TextureSource;
use nexus::imgui;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TextureKey(pub(super) usize);

#[derive(Debug, Clone)]
pub(super) enum TextureEntry {
    Pending { source: TextureSource },
    Resolved { texture: imgui::TextureId },
}

impl TextureEntry {
    pub const fn pending(source: TextureSource) -> Self {
        Self::Pending { source }
    }

    pub const fn is_pending(&self) -> bool {
        matches!(self, Self::Pending { .. })
    }

    pub const fn texture(&self) -> Option<imgui::TextureId> {
        match *self {
            Self::Pending { .. } => None,
            Self::Resolved { texture } => Some(texture),
        }
    }

    pub fn load(&mut self, texture: imgui::TextureId) {
        if self.is_pending() {
            *self = Self::Resolved { texture };
        } else {
            log::warn!("Attempt to populate already loaded texture");
        }
    }

    pub fn fail(&mut self, error_texture: Option<imgui::TextureId>) {
        match self {
            TextureEntry::Pending { source } => {
                log::warn!("Failed to load texture {}", source.pretty_print());
                if let Some(texture) = error_texture {
                    *self = Self::Resolved { texture };
                }
            }
            TextureEntry::Resolved { .. } => log::warn!("Received fail for already loaded texture"),
        }
    }
}
