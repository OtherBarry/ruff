use std::borrow::Cow;

use ruff_text_size::{Ranged, TextLen, TextRange, TextSize};

use crate::{BytesLiteral, StringLiteral};

pub struct ConcatenatedStr<'a> {
    pub(super) parts: &'a [StringLiteral],
    // Cached value of the concatenated string
    // value: OnceCell<Cow<'a, str>>,
}

impl Ranged for ConcatenatedStr<'_> {
    fn range(&self) -> TextRange {
        assert!(!self.parts.is_empty());
        TextRange::new(
            self.parts.first().unwrap().start(),
            self.parts.last().unwrap().end(),
        )
    }
}

impl<'a> ConcatenatedStr<'a> {
    /// Returns `true` if the concatenated string has a length of zero, and `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.parts.iter().all(|string| string.is_empty())
    }

    /// Returns the number of parts in the string.
    pub fn count(&self) -> usize {
        self.parts.len()
    }

    /// Returns the total length of the string.
    pub fn len(&self) -> usize {
        self.parts.iter().map(|string| string.len()).sum()
    }

    pub fn text_len(&self) -> TextSize {
        self.parts.iter().map(|string| string.text_len()).sum()
    }

    pub fn as_str(&self) -> Cow<'_, str> {
        match self.parts {
            [] => Cow::Borrowed(""),
            [string] => Cow::Borrowed(string.as_str()),
            strings => Cow::Owned(strings.iter().map(StringLiteral::as_str).collect()),
        }
    }
}

impl PartialEq<str> for ConcatenatedStr<'_> {
    fn eq(&self, other: &str) -> bool {
        if self.len() != other.len() {
            return false;
        }
        let mut self_chars = self.parts.iter().flat_map(|string| string.chars());
        let mut other_chars = other.chars();
        loop {
            match (self_chars.next(), other_chars.next()) {
                (Some(self_char), Some(other_char)) => {
                    if self_char != other_char {
                        return false;
                    }
                }
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}

pub struct ConcatenatedBytes<'a> {
    pub(super) parts: &'a [BytesLiteral],
}

impl PartialEq<[u8]> for ConcatenatedBytes<'_> {
    fn eq(&self, other: &[u8]) -> bool {
        let mut self_bytes = self.parts.iter().flat_map(BytesLiteral::as_slice);
        let mut other_bytes = other.iter();
        loop {
            match (self_bytes.next(), other_bytes.next()) {
                (Some(self_byte), Some(other_byte)) => {
                    if self_byte != other_byte {
                        return false;
                    }
                }
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}

impl Ranged for ConcatenatedBytes<'_> {
    fn range(&self) -> TextRange {
        assert!(!self.parts.is_empty());
        TextRange::new(
            self.parts.first().unwrap().start(),
            self.parts.last().unwrap().end(),
        )
    }
}

// impl ConcatenatedStr<'a> {
//     // self == "value"
//     fn eq() -> bool {}
//     // Total `text_len` of all values
//     fn len() -> usize {}
//     // `ruff_python_stdlib::identifiers::is_identifier`
//     fn is_identifier() -> bool {}
//     // The argument is a `Pattern` :(
//     fn contains() -> bool {}
//     // Concatenated source
//     // We can cache it in this struct?
//     fn as_str() -> &str {}
// }
