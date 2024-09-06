use std::{borrow::Cow, path::Path};

/// Associated short name.
pub trait ShortName {
    /// Returns the short name.
    fn short_name(&self) -> &'static str;
}

#[macro_export]
macro_rules! non_zero_u32 {
    ( $val:literal ) => {{
        use ::std::num::NonZero;
        const VAL: NonZero<u32> = match NonZero::new($val) {
            Some(val) => val,
            None => panic!(concat!(stringify!($val), " is zero")),
        };
        VAL
    }};
}

pub use non_zero_u32;

pub fn file_name(path: &Path) -> Cow<str> {
    path.file_name()
        .map(|file| file.to_string_lossy())
        .unwrap_or_default()
}
