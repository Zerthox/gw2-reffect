use super::TextFragment;
use crate::trigger::ProgressValue;
use const_default::ConstDefault;
use itertools::Itertools;
use std::{iter::FusedIterator, str::Chars};

impl<'s> TextFragment<'s> {
    /// Prefix for variable fragments.
    pub const PREFIX: char = '%';

    /// Parses text fragments from the source text.
    pub fn parse(source: &'s str) -> impl Iterator<Item = Self> {
        SimpleTextFragmentIter {
            iter: source.chars(),
        }
    }

    /// Parses a single text fragment.
    pub fn parse_next(iter: &mut Chars<'s>) -> Option<Self> {
        let start = iter.as_str();
        if Self::peek(iter)? == Self::PREFIX {
            iter.next();
            if start.len() == 1 {
                Some(Self::Literal(start))
            } else {
                let start = iter.as_str();
                Self::try_parse_prefixed(iter).or_else(|| Self::parse_literal(iter, start))
            }
        } else {
            Self::parse_literal(iter, start)
        }
    }

    /// Attempts to parse literal text.
    ///
    /// The returned literal will range from the given start to the begin of the next non-literal fragment.
    fn parse_literal(iter: &mut Chars<'s>, start: &'s str) -> Option<Self> {
        while iter
            .peeking_take_while(|el| *el != Self::PREFIX)
            .next()
            .is_some()
        {}
        let end = iter.as_str();
        let len = start.len() - end.len();
        if end.len() == 1 {
            // only ending prefix left
            iter.next();
            Some(Self::Literal(start))
        } else if len > 0 {
            Some(Self::Literal(&start[..len]))
        } else {
            None
        }
    }

    /// Attempts to parse a variable fragment without the prefix.
    fn try_parse_prefixed(iter: &mut Chars<'s>) -> Option<Self> {
        let next = Self::peek(iter)?;
        let pretty = next.is_ascii_uppercase();
        match next {
            'n' => {
                iter.next();
                Some(Self::Name)
            }
            'i' | 's' | 'I' => {
                // backwards compat for %s stacks
                iter.next();
                Some(Self::Intensity { pretty })
            }
            'c' | 'r' | 'C' => {
                // backwards compat for %r remaining
                iter.next();
                let value = Self::parse_value(iter);
                Some(Self::Current { pretty, value })
            }
            'f' | 'F' => {
                iter.next();
                let value = Self::parse_value(iter);
                Some(Self::Full { pretty, value })
            }

            'p' | 'P' => {
                iter.next();
                let value = Self::parse_value(iter);
                Some(Self::Percent { pretty, value })
            }
            Self::PREFIX => {
                iter.next();
                None
            }
            _ => None,
        }
    }

    /// Attempts to parse a progress value.
    fn parse_value(iter: &mut Chars<'s>) -> ProgressValue {
        match Self::peek(iter) {
            Some('1') => {
                iter.next();
                ProgressValue::Primary
            }
            Some('2') => {
                iter.next();
                ProgressValue::Secondary
            }
            _ => ProgressValue::DEFAULT,
        }
    }

    /// Peeks the next [`char`] without consuming it.
    fn peek(iter: &Chars<'s>) -> Option<char> {
        iter.clone().next()
    }
}

#[derive(Debug, Clone)]
pub struct SimpleTextFragmentIter<'s> {
    iter: Chars<'s>,
}

impl<'s> Iterator for SimpleTextFragmentIter<'s> {
    type Item = TextFragment<'s>;

    fn next(&mut self) -> Option<Self::Item> {
        TextFragment::parse_next(&mut self.iter)
    }
}

impl FusedIterator for SimpleTextFragmentIter<'_> {}

#[cfg(test)]
mod tests {
    use super::*;
    use const_default::ConstDefault;

    #[test]
    fn parse() {
        let fragments = TextFragment::parse("static %% text %c %P2 %").collect::<Vec<_>>();
        assert_eq!(
            fragments,
            [
                TextFragment::Literal("static "),
                TextFragment::Literal("% text "),
                TextFragment::Current {
                    pretty: false,
                    value: ProgressValue::DEFAULT
                },
                TextFragment::Literal(" "),
                TextFragment::Percent {
                    pretty: true,
                    value: ProgressValue::Secondary
                },
                TextFragment::Literal(" %")
            ]
        );
    }
}
