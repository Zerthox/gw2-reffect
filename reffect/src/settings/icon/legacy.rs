use super::{DurationTextSettings, StackTextSettings, TextSettings};
use crate::elements::text::TextDecoration;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DurationTextSettingsLegacy {
    pub scale: f32,
    pub decoration: TextDecoration,
    pub color: [f32; 4],
}

impl From<DurationTextSettingsLegacy> for TextSettings {
    fn from(legacy: DurationTextSettingsLegacy) -> Self {
        Self {
            scale: legacy.scale,
            decoration: legacy.decoration,
            color: legacy.color,
            ..DurationTextSettings::new().text
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StackTextSettingsLegacy {
    pub scale: f32,
    pub decoration: TextDecoration,
    pub color: [f32; 4],
}

impl From<StackTextSettingsLegacy> for TextSettings {
    fn from(legacy: StackTextSettingsLegacy) -> Self {
        Self {
            scale: legacy.scale,
            decoration: legacy.decoration,
            color: legacy.color,
            ..StackTextSettings::new().text
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_text() {
        let result = serde_json::from_str::<StackTextSettings>(
            r#"{
                "scale": 0.0,
                "offset": [0.0, 0.0],
                "color": [0.0, 0.0, 0.0, 0.0],
                "decoration": "None"
            }"#,
        )
        .unwrap();

        assert_eq!(
            result.text,
            TextSettings {
                scale: 0.0,
                color: [0.0, 0.0, 0.0, 0.0],
                decoration: TextDecoration::None,
                ..StackTextSettings::new().text
            }
        );
    }

    #[test]
    fn duration_text() {
        let result = serde_json::from_str::<DurationTextSettings>(
            r#"{
                "scale": 0.0,
                "color": [0.0, 0.0, 0.0, 0.0],
                "decoration": "None"
            }"#,
        )
        .unwrap();

        assert_eq!(
            result.text,
            TextSettings {
                scale: 0.0,
                color: [0.0, 0.0, 0.0, 0.0],
                decoration: TextDecoration::None,
                ..DurationTextSettings::new().text
            }
        );
    }
}
