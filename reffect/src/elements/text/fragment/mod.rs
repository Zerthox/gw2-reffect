mod parse;

use crate::{
    context::Context,
    fmt::Unit,
    settings::FormatSettings,
    trigger::{ProgressActive, ProgressValue},
};
use std::fmt::{self, Display};

/// Text fragments.
#[derive(Debug, Clone, PartialEq)]
pub enum TextFragment<'s> {
    /// Literal text.
    Literal(&'s str),

    /// Element name.
    Name,

    /// Progress intensity.
    Intensity { pretty: bool },

    /// Current (remaining) progress.
    Current { pretty: bool, value: ProgressValue },

    /// Full (max) progress.
    Full { pretty: bool, value: ProgressValue },

    /// Progress percent.
    Percent { pretty: bool, value: ProgressValue },
}

impl<'s> TextFragment<'s> {
    /// Returns a type that can display the fragment's processed text.
    pub fn display(
        &self,
        active: &ProgressActive,
        ctx: &Context,
        settings: &FormatSettings,
        name: &str,
    ) -> impl fmt::Display {
        fmt::from_fn(|formatter| match *self {
            Self::Literal(text) => formatter.write_str(text),
            Self::Name => formatter.write_str(name),
            Self::Intensity { pretty } => {
                if pretty {
                    active.intensity().fmt(formatter)
                } else {
                    Unit::format(active.intensity()).fmt(formatter)
                }
            }
            Self::Current { pretty, value } => active
                .current_text(value, ctx.now, pretty, settings)
                .fmt(formatter),
            Self::Full { pretty, value } => active.max_text(value, pretty, settings).fmt(formatter),
            Self::Percent { pretty, value } => {
                let progress = 100.0 * active.progress_or_default(value, ctx.now);
                let precision = if pretty { 1 } else { 0 };
                write!(formatter, "{progress:.precision$}")
            }
        })
    }
}
