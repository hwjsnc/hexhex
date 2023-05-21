use crate::decode::buf::{decode_ascii_to_buf_exact, decode_to_buf_exact};
use crate::decode::common::{has_0x_prefix, has_0x_prefix_ascii};
use crate::decode::FromHexError;

use std::vec;

/// Decode a hex string.
///
/// Accepts lowercase, uppercase, and mixedcase hex digits a-f.
/// Strips leading `0x` prefix if present.
pub fn decode(hex: &str) -> Result<std::vec::Vec<u8>, FromHexError> {
    let expected_len = if has_0x_prefix(hex) {
        (hex.len() - 2) / 2
    } else {
        hex.len() / 2
    };
    let mut vec = vec![0u8; expected_len];
    decode_to_buf_exact(hex, &mut vec)?;
    Ok(vec)
}

/// Decode a hex bytestring.
///
/// Accepts lowercase, uppercase, and mixedcase hex digits a-f.
/// Strips leading `0x` prefix if present.
pub fn decode_ascii(hex: &[u8]) -> Result<std::vec::Vec<u8>, FromHexError> {
    let expected_len = if has_0x_prefix_ascii(hex) {
        (hex.len() - 2) / 2
    } else {
        hex.len() / 2
    };
    let mut vec = vec![0u8; expected_len];
    decode_ascii_to_buf_exact(hex, &mut vec)?;
    Ok(vec)
}
