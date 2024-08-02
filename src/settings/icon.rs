use crate::{
    colors::{self, with_alpha},
    elements::TextDecoration,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconSettings {
    pub stack_text: StackTextSettings,
    pub duration_text: DurationTextSettings,
    pub duration_bar: DurationBarSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct StackTextSettings {
    pub scale: f32,
    pub offset: [f32; 2],
    pub color: [f32; 4],
    pub decoration: TextDecoration,
}

impl Default for StackTextSettings {
    fn default() -> Self {
        Self {
            scale: 0.5,
            offset: [1.0, 1.0],
            color: with_alpha(colors::WHITE, 0.8),
            decoration: TextDecoration::Shadow,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DurationTextSettings {
    pub min_remain: u32,
    pub scale: f32,
    pub color: [f32; 4],
    pub decoration: TextDecoration,
}

impl Default for DurationTextSettings {
    fn default() -> Self {
        Self {
            min_remain: 5000,
            scale: 0.5,
            color: colors::WHITE,
            decoration: TextDecoration::Outline,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DurationBarSettings {
    pub height: f32,
    pub color: [f32; 4],
}

impl Default for DurationBarSettings {
    fn default() -> Self {
        Self {
            height: 2.0,
            color: colors::GREEN,
        }
    }
}
