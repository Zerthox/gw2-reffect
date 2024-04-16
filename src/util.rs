use std::{borrow::Cow, path::Path};

pub fn file_name(path: &Path) -> Cow<str> {
    path.file_name()
        .map(|file| file.to_string_lossy())
        .unwrap_or_default()
}
