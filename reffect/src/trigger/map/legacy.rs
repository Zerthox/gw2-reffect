use super::MapTrigger;
use crate::context::MapCategory;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter};

#[derive(Debug, Default, Clone, AsRefStr, EnumIter, Serialize, Deserialize)]
pub enum MapTriggerLegacy {
    #[default]
    Any,
    Category(MapCategory),
    Ids(Vec<u32>),
}

impl From<MapTriggerLegacy> for MapTrigger {
    fn from(legacy: MapTriggerLegacy) -> Self {
        match legacy {
            MapTriggerLegacy::Any => MapTrigger::default(),
            MapTriggerLegacy::Category(category) => MapTrigger {
                category: category.into(),
                ..MapTrigger::default()
            },
            MapTriggerLegacy::Ids(ids) => MapTrigger {
                ids,
                whitelist: true,
                ..MapTrigger::default()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde::Migrate;
    use enumflags2::BitFlags;

    #[test]
    fn migrate() {
        let json = r#""Any""#;
        let result = serde_json::from_str::<Migrate<MapTrigger, MapTriggerLegacy>>(&json)
            .expect("failed to deserialize");
        let map = result.inner;
        assert_eq!(map.category, BitFlags::empty());
        assert_eq!(map.ids.len(), 0);

        let json = r#"{ "Category": "Pve" }"#;
        let result = serde_json::from_str::<Migrate<MapTrigger, MapTriggerLegacy>>(&json)
            .expect("failed to deserialize");
        let map = result.inner;
        assert_eq!(map.category, BitFlags::from(MapCategory::PvE));

        let json = r#"{ "Ids": [1, 2, 3] }"#;
        let result = serde_json::from_str::<Migrate<MapTrigger, MapTriggerLegacy>>(&json)
            .expect("failed to deserialize");
        let map = result.inner;
        assert_eq!(&map.ids, &[1, 2, 3]);
    }
}
