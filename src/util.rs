use std::{borrow::Cow, path::Path};

pub fn file_name(path: &Path) -> Cow<str> {
    path.file_name()
        .map(|file| file.to_string_lossy())
        .unwrap_or_default()
}

pub fn as_secs_hundreds(millis: u32) -> (u32, u32) {
    let secs = millis / 1000;
    let millis = millis % 1000;
    (secs, millis / 100)
}
