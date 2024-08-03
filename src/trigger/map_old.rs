use super::{memo::Memo, MapTrigger};
use crate::context::MapCategory;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, EnumIter};

#[derive(Debug, Default, Clone, AsRefStr, EnumIter, Serialize, Deserialize)]
pub enum MapTriggerOld {
    #[default]
    Any,
    Category(MapCategory),
    Ids(Vec<u32>),
}

impl From<MapTriggerOld> for MapTrigger {
    fn from(old: MapTriggerOld) -> Self {
        match old {
            MapTriggerOld::Any => MapTrigger::default(),
            MapTriggerOld::Category(category) => MapTrigger {
                category: category.into(),
                ..MapTrigger::default()
            },
            MapTriggerOld::Ids(ids) => MapTrigger {
                ids,
                whitelist: true,
                ..MapTrigger::default()
            },
        }
    }
}

impl From<MapTriggerOld> for Memo<MapTrigger> {
    fn from(old: MapTriggerOld) -> Self {
        Memo::new(old.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde_migrate::Migrate;
    use enumflags2::BitFlags;

    #[test]
    fn migrate() {
        let json = r#""Any""#;
        let result = serde_json::from_str::<Migrate<MapTrigger, MapTriggerOld>>(&json);
        assert!(result.is_ok());
        let map = result.unwrap().inner;
        assert_eq!(map.category, BitFlags::empty());
        assert_eq!(map.ids.len(), 0);

        let json = r#"{ "Category": "Pve" }"#;
        let result = serde_json::from_str::<Migrate<MapTrigger, MapTriggerOld>>(&json);
        assert!(result.is_ok());
        let map = result.unwrap().inner;
        assert_eq!(map.category, BitFlags::from(MapCategory::PvE));

        let json = r#"{ "Ids": [1, 2, 3] }"#;
        let result = serde_json::from_str::<Migrate<MapTrigger, MapTriggerOld>>(&json);
        assert!(result.is_ok());
        let map = result.unwrap().inner;
        assert_eq!(&map.ids, &[1, 2, 3]);
    }
}
