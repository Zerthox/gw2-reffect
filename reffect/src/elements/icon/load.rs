use super::IconSource;
use crate::{
    action::DynAction,
    context::SkillId,
    elements::{RenderCtx, icon::IconEditResult},
    internal::{Interface, Internal},
    texture::{AsTextureSource, LoadedTexture, TextureManager, TextureSource},
};
use nexus::imgui::{self, Ui};
use windows::core::Interface as _;

pub type LoadedIcon = LoadedTexture<IconSource>;

impl AsTextureSource for IconSource {
    fn as_texture_source(&self) -> Option<TextureSource> {
        match self {
            Self::Unknown | Self::Empty | Self::Automatic => None,
            Self::Url(url) => Some(TextureSource::Url(url.clone())),
            Self::File(file) => Some(TextureSource::File(file.clone())),
        }
    }
}

impl LoadedIcon {
    pub fn get_texture(&self, ui: &Ui, skill: SkillId) -> Option<imgui::TextureId> {
        match self.source() {
            IconSource::Empty => None,
            IconSource::Unknown => TextureManager::get_unknown(),
            IconSource::Automatic => match skill {
                SkillId::Unknown => TextureManager::get_unknown(),
                SkillId::WeaponSwap | SkillId::PetSwap => TextureManager::get_weapon_swap(),
                SkillId::BundleDrop => TextureManager::get_bundle_drop(),
                SkillId::Id(id) => match Internal::get_skill_icon(ui, id) {
                    Some(tex) => Some(tex.as_raw().into()),
                    None => TextureManager::get_unknown(),
                },
            },
            IconSource::File(_) | IconSource::Url(_) => {
                self.key().and_then(TextureManager::get_texture)
            }
        }
    }

    pub fn render_select(&mut self, ui: &Ui, ctx: &RenderCtx) -> DynAction<IconSource> {
        let mut source = self.source_mut();
        let IconEditResult { reload, action } = source.render_select(ui, ctx);
        if reload {
            source.reload();
        } else {
            source.unchanged();
        }
        action
    }
}
