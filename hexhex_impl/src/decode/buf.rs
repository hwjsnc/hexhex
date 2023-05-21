use fallible_iterator::FallibleIterator;

use super::{FromHexError, FromHexErrorKind};
use crate::decode::common::{has_0x_prefix, has_0x_prefix_ascii};
use crate::decode::streaming::HexDecodeAsciiIterator;

#[derive(Debug, PartialEq, Eq)]
enum OutputLength {
    MayBeShorterThanDst,
    MustEqualDst,
}

fn decode_ascii_to_buf_internal<I: FallibleIterator<Item = (usize, u8), Error = FromHexError>>(
    mut iter: HexDecodeAsciiIterator<I>,
    dst: &mut [u8],
    output_length: OutputLength,
) -> Result<usize, FromHexError> {
    let mut off_dst = 0;
    loop {
        match iter.next() {
            Ok(Some(byte)) => match dst.get_mut(off_dst) {
                Some(x) => {
                    *x = byte;
                    off_dst += 1;
                }
                None => {
                    return Err(FromHexError {
                        position: iter.last_position(),
                        kind: FromHexErrorKind::Eof,
                    });
                }
            },
            Ok(None) => {
                if output_length == OutputLength::MustEqualDst && off_dst < dst.len() {
                    return Err(FromHexError {
                        position: iter.last_position(),
                        kind: FromHexErrorKind::OutputBufferTooShort,
                    });
                } else {
                    return Ok(off_dst);
                }
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

fn decode_to_buf_internal(
    hex: &str,
    dst: &mut [u8],
    output_length: OutputLength,
) -> Result<usize, FromHexError> {
    // iterate over chars
    // convert each char to ASCII u8 (< 128), if possible; otherwise fail
    let chars_as_bytes_with_position = hex
        .char_indices()
        .map(|(position, c)| match u8::try_from(c) {
            Ok(byte) if byte < 128 => Ok((position, byte)),
            _ => Err(FromHexError {
                position,
                kind: FromHexErrorKind::UnexpectedCharacter(c),
            }),
        })
        .skip(if has_0x_prefix(hex) { 2 } else { 0 });
    // use the ASCII decode function, writing to dst
    let result = decode_ascii_to_buf_internal(
        HexDecodeAsciiIterator::new(fallible_iterator::convert(chars_as_bytes_with_position)),
        dst,
        output_length,
    );
    // if there is an error, make sure we refer to the char, not its first byte
    match result {
        Ok(len) => Ok(len),
        Err(FromHexError {
            position,
            kind: FromHexErrorKind::UnexpectedByte(byte),
        }) => Err(FromHexError {
            position,
            kind: FromHexErrorKind::UnexpectedCharacter(byte as char), // we constructed this ASCII byte from char, so this cast is correct
        }),
        Err(err) => Err(err),
    }
}

/// Decode the given ASCII hex string and write the corresponding bytes to dst.
/// Returns the number of bytes written to dst on success.
///
/// Accepts lower case, upper case, and mixed case hex characters a-f.
/// Strips leading "0x" if present.
///
/// Does not allocate or panic.
///
/// # Example
///
/// ```
/// use hexhex_impl::*;
/// let input = b"0x1234"; // or b"0x1234"
/// let mut output = [0u8; 4];
/// assert_eq!(decode_ascii_to_buf(input, &mut output).unwrap(), 2);
/// assert_eq!(&output[..2], &[0x12, 0x34]);
/// ```
pub fn decode_ascii_to_buf(hex: &[u8], dst: &mut [u8]) -> Result<usize, FromHexError> {
    decode_ascii_to_buf_internal(
        HexDecodeAsciiIterator::new(fallible_iterator::convert(
            hex.iter()
                .copied()
                .enumerate()
                .skip(if has_0x_prefix_ascii(hex) { 2 } else { 0 })
                .map(Ok),
        )),
        dst,
        OutputLength::MayBeShorterThanDst,
    )
}

/// Like [`decode_ascii_to_buf`], but returns an error if not all of dst has been overwritten.
///
/// # Examples
///
/// ```
/// use hexhex_impl::*;
/// let input = b"0xc0ffee";
/// let mut output = [0u8; 3];
/// assert!(decode_ascii_to_buf_exact(input, &mut output).is_ok());
/// assert_eq!(output, [0xc0, 0xff, 0xee]);
/// ```
///
/// ```
/// use hexhex_impl::*;
/// let input = b"0xc0ffee";
/// let mut output = [0u8; 4];
/// assert!(decode_ascii_to_buf_exact(input, &mut output).is_err());
/// ```
pub fn decode_ascii_to_buf_exact(hex: &[u8], dst: &mut [u8]) -> Result<(), FromHexError> {
    decode_ascii_to_buf_internal(
        HexDecodeAsciiIterator::new(fallible_iterator::convert(
            hex.iter()
                .copied()
                .enumerate()
                .skip(if has_0x_prefix_ascii(hex) { 2 } else { 0 })
                .map(Ok),
        )),
        dst,
        OutputLength::MustEqualDst,
    )
    .map(|_| ())
}

/// Decode the given hex string and write the corresponding bytes to dst.
/// Returns the number of bytes written to dst on success.
///
/// Accepts lower case, upper case, and mixed case hex characters a-f.
/// Strips `0x` prefix if present.
///
/// Does not allocate or panic.
///
/// # Example
///
/// ```
/// use hexhex_impl::*;
/// let input = "1234"; // or "0x1234"
/// let mut output = [0u8; 4];
/// assert_eq!(decode_to_buf(input, &mut output).unwrap(), 2);
/// assert_eq!(&output[..2], &[0x12, 0x34]);
/// ```
pub fn decode_to_buf(hex: &str, dst: &mut [u8]) -> Result<usize, FromHexError> {
    decode_to_buf_internal(hex, dst, OutputLength::MayBeShorterThanDst)
}

/// Like [`decode_to_buf`], but returns an error if not all of dst has been overwritten.
///
/// # Examples
///
/// ```
/// use hexhex_impl::*;
/// let input = "0xc0ffee";
/// let mut output = [0u8; 3];
/// assert!(decode_to_buf_exact(input, &mut output).is_ok());
/// assert_eq!(output, [0xc0, 0xff, 0xee]);
/// ```
///
/// ```
/// use hexhex_impl::*;
/// let input = "0xc0ffee";
/// let mut output = [0u8; 4];
/// assert!(decode_to_buf_exact(input, &mut output).is_err());
/// ```
pub fn decode_to_buf_exact(hex: &str, dst: &mut [u8]) -> Result<(), FromHexError> {
    decode_to_buf_internal(hex, dst, OutputLength::MustEqualDst).map(|_| ())
}
