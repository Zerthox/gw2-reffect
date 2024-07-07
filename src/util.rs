use base64::{prelude::BASE64_STANDARD, Engine};
use std::{borrow::Cow, path::Path};

pub fn file_name(path: &Path) -> Cow<str> {
    path.file_name()
        .map(|file| file.to_string_lossy())
        .unwrap_or_default()
}

/// Decodes a skill id from a skill
pub fn decode_skill(chatcode: &str) -> Option<u32> {
    if let Some(code) = chatcode
        .strip_prefix("[&")
        .and_then(|text| text.strip_suffix(']'))
    {
        let bytes = BASE64_STANDARD.decode(code).ok()?;
        if let [0x06, b1, b2, b3, b4] = *bytes.as_slice() {
            return Some(u32::from_le_bytes([b1, b2, b3, b4]));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chatcode() {
        assert_eq!(decode_skill("[&BuQCAAA=]"), Some(740)); // might
        assert_eq!(decode_skill("[&BgAZAQA=]"), Some(71936)); // fire bullet
        assert_eq!(decode_skill("[&BgAZAQA="), None); // broken
        assert_eq!(decode_skill("[&AQEAAAA=]"), None); // coin
        assert_eq!(decode_skill("[&AgH1WQAA]"), None); // item
        assert_eq!(decode_skill("[&BDgAAAA=]"), None); // poi
        assert_eq!(decode_skill("[&B/IDAAA=]"), None); // trait
    }
}
