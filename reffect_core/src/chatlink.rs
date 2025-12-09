use base64::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum LinkType {
    Coin = 1, // disabled
    Item = 2,
    Npc = 3, // disabled
    Map = 4,
    Pvp = 5,
    Skill = 6,
    Trait = 7,
    User = 8, // disabled
    Recipe = 9,
    Wardrobe = 10,
    Outfit = 11,
    Wvw = 12,
    Build = 13,
    Achievement = 14, // disabled
}

/// Decodes a chat link to bytes.
pub fn decode_bytes(code: &str) -> Option<Vec<u8>> {
    code.strip_prefix("[&")
        .and_then(|text| text.strip_suffix(']'))
        .and_then(|text| BASE64_STANDARD.decode(text).ok())
}

/// Decodes an id as unsigned 24-bit int from the code.
pub fn decode_simple_id(link_type: LinkType, code: &str) -> Option<u32> {
    let bytes = decode_bytes(code)?;
    if let [code_type, id0, id1, id2, _unused] = *bytes
        && code_type == link_type as u8
    {
        return Some(u32::from_le_bytes([id0, id1, id2, 0]));
    }
    None
}

/// Decodes an item id from a chat link.
pub fn decode_item(code: &str) -> Option<u32> {
    const TYPE: u8 = LinkType::Item as _;
    let bytes = decode_bytes(code)?;
    if let [TYPE, _count, id0, id1, id2, ..] = *bytes {
        return Some(u32::from_le_bytes([id0, id1, id2, 0]));
    }
    None
}

/// Decodes a skill id from a chat link.
pub fn decode_skill(code: &str) -> Option<u32> {
    decode_simple_id(LinkType::Skill, code)
}

/// Decodes a trait id from a chat link.
pub fn decode_trait(code: &str) -> Option<u32> {
    decode_simple_id(LinkType::Trait, code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn items() {
        assert_eq!(decode_item("[&AgEAWgAA]"), Some(23040)); // basic savlage kit
        assert_eq!(decode_item("[&AgH1WQAA]"), Some(23029)); // copper harvesting sickle

        assert_eq!(decode_item("[&BgAZAQA="), None); // broken
        assert_eq!(decode_item("[&BuQCAAA=]"), None); // skill
        assert_eq!(decode_item("[&B/IDAAA=]"), None); // trait
        assert_eq!(decode_item("[&AQEAAAA=]"), None); // coin
        assert_eq!(decode_item("[&BDgAAAA=]"), None); // poi
    }

    #[test]
    fn skills() {
        assert_eq!(decode_skill("[&BuQCAAA=]"), Some(740)); // might
        assert_eq!(decode_skill("[&BgAZAQA=]"), Some(71936)); // fire bullet

        assert_eq!(decode_skill("[&BgAZAQA="), None); // broken
        assert_eq!(decode_skill("[&B/IDAAA=]"), None); // trait
        assert_eq!(decode_skill("[&AQEAAAA=]"), None); // coin
        assert_eq!(decode_skill("[&AgH1WQAA]"), None); // item
        assert_eq!(decode_skill("[&BDgAAAA=]"), None); // poi
    }

    #[test]
    fn traits() {
        assert_eq!(decode_trait("[&B/IDAAA=]"), Some(1010)); // opening strike
        assert_eq!(decode_trait("[&BwMJAAA=]"), Some(2307)); // eternal champion

        assert_eq!(decode_trait("[&B/IDAAA="), None); // broken
        assert_eq!(decode_trait("[&BuQCAAA=]"), None); // skill
        assert_eq!(decode_trait("[&AQEAAAA=]"), None); // coin
        assert_eq!(decode_trait("[&AgH1WQAA]"), None); // item
        assert_eq!(decode_trait("[&BDgAAAA=]"), None); // poi
    }
}
