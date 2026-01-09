use super::GeneralSettings;
use crate::context::{Context, EditSettings};
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextSettings {
    #[serde(flatten)]
    pub general: GeneralSettings,

    pub edit: EditSettings,

    #[serde(skip_serializing)]
    edit_during_combat: Option<bool>,

    #[serde(skip_serializing)]
    edit_show_all: Option<bool>,
}

impl ContextSettings {
    pub fn new(settings: &GeneralSettings, ctx: &Context) -> Self {
        Self {
            general: settings.clone(),
            edit: ctx.edit.settings.clone(),
            edit_during_combat: None,
            edit_show_all: None,
        }
    }

    pub fn migrate(&mut self, _version: &Version) -> bool {
        false
    }

    pub fn apply(self, settings: &mut GeneralSettings, ctx: &mut Context) {
        let Self {
            general,
            mut edit,
            edit_during_combat,
            edit_show_all,
        } = self;

        // migrate old settings
        if let Some(value) = edit_during_combat {
            edit.during_combat = value;
        }
        if let Some(value) = edit_show_all {
            edit.show_all = value;
        }

        *settings = general;
        ctx.edit.settings = edit;
    }
}
