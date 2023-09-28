use fallible_iterator::FallibleIterator;

use super::{FromHexError, FromHexErrorKind};

/// Fallible iterator that produces u8 from a hex string (or byte string) one at a time.
/// Accepts lowercase, uppercase, and mixedcase hex digits a-f.
/// Does not accept leading '0x' prefix.
///
/// The intended use is to take in either:
/// - a `str.char_indices()` iterator, or
/// - an `byte_slice.iter().copied().enumerate()` iterator
/// and decode it one byte at a time via the `FallibleIterator` interface.
///
/// You should probably not use this.
pub struct HexDecodeIterator<I> {
    iterator: I,
}

/// Like HexDecodeIterator, but for (ASCII) `u8` instead of `char`s.
///
/// The underlying iterator need not be valid ASCII or UTF-8 (but only valid ASCII strings can be valid hex strings).
///
/// You should probably not use this.
pub struct HexDecodeAsciiIterator<I> {
    iterator: I,
    last_position: usize,
}

impl<I: FallibleIterator<Item = (usize, char), Error = FromHexError>> HexDecodeIterator<I> {
    /// Construct a new HexDecodeIterator from a `(usize, char)` iterator.
    ///
    /// The usize should be the (byte) position of the corresponding char.
    pub fn new(iterator: I) -> Self {
        Self { iterator }
    }
}

impl<I: FallibleIterator<Item = (usize, u8), Error = FromHexError>> HexDecodeAsciiIterator<I> {
    /// Construct a new HexDecodeAsciiIterator from a `(usize, byte)` iterator.
    ///
    /// The usize should be the (byte) position of the corresponding ASCII u8.
    pub fn new(iterator: I) -> Self {
        Self {
            iterator,
            last_position: 0,
        }
    }

    pub(crate) fn last_position(&self) -> usize {
        self.last_position
    }
}

impl<I> fallible_iterator::FallibleIterator for HexDecodeIterator<I>
where
    I: FallibleIterator<Item = (usize, char), Error = FromHexError>,
{
    type Item = u8;
    type Error = FromHexError;

    fn next(&mut self) -> Result<Option<u8>, FromHexError> {
        // read first char
        let (position, c) = match self.iterator.next() {
            Ok(Some(tup)) => tup,
            Ok(None) => return Ok(None),
            Err(e) => return Err(e),
        };
        let Some(v1) = c.to_digit(16) else {
            return Err(FromHexError {
                position,
                kind: FromHexErrorKind::UnexpectedCharacter(c),
            });
        };
        // read second char
        let (position, c) = match self.iterator.next() {
            Ok(Some(tup)) => tup,
            Ok(None) => {
                return Err(FromHexError {
                    position,
                    kind: FromHexErrorKind::Eof,
                })
            }
            Err(e) => return Err(e),
        };
        let Some(v2) = c.to_digit(16) else {
            return Err(FromHexError {
                position,
                kind: FromHexErrorKind::UnexpectedCharacter(c),
            });
        };
        // Got two hex digits, done
        Ok(Some((v1 * 16 + v2) as u8))
    }
}

impl<I> fallible_iterator::FallibleIterator for HexDecodeAsciiIterator<I>
where
    I: FallibleIterator<Item = (usize, u8), Error = FromHexError>,
{
    type Item = u8;
    type Error = FromHexError;

    fn next(&mut self) -> Result<Option<u8>, FromHexError> {
        // read first char
        let (position, c) = match self.iterator.next() {
            Ok(Some(tup)) => tup,
            Ok(None) => return Ok(None),
            Err(e) => return Err(e),
        };
        let Some(v1) = (c as char).to_digit(16) else {
            return Err(FromHexError {
                position,
                kind: FromHexErrorKind::UnexpectedByte(c),
            });
        };
        // read second char
        let (position, c) = match self.iterator.next() {
            Ok(Some(tup)) => tup,
            Ok(None) => {
                return Err(FromHexError {
                    position,
                    kind: FromHexErrorKind::Eof,
                })
            }
            Err(e) => return Err(e),
        };
        let Some(v2) = (c as char).to_digit(16) else {
            return Err(FromHexError {
                position,
                kind: FromHexErrorKind::UnexpectedByte(c),
            });
        };
        self.last_position = position;
        // Got two hex digits, done
        Ok(Some((v1 * 16 + v2) as u8))
    }
}
