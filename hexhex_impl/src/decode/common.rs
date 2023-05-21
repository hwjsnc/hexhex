use super::{FromHexError, FromHexErrorKind};

impl core::fmt::Display for FromHexErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            FromHexErrorKind::UnexpectedCharacter(c) => write!(f, "unexpected character {c}"),
            FromHexErrorKind::UnexpectedByte(b) => write!(f, "unexpected byte 0x{b:02x}"),
            FromHexErrorKind::Eof => write!(f, "unexpected end of input"),
            FromHexErrorKind::OutputBufferTooShort => write!(f, "output buffer is too short"),
        }
    }
}

impl core::fmt::Display for FromHexError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "error converting from hex: {} at position {}",
            self.kind, self.position
        )
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FromHexError {}

/*
pub fn strip_0x(s: &str) -> Option<&str> {
    let prefix = "0x";
    if s.starts_with(prefix) {
        Some(&s[prefix.len()..])
    } else {
        None
    }
}

pub fn strip_0x_ascii(buf: &[u8]) -> Option<&[u8]> {
    let prefix = b"0x";
    if buf.starts_with(prefix) {
        Some(&buf[prefix.len()..])
    } else {
        None
    }
}
*/

pub fn has_0x_prefix(s: &str) -> bool {
    s.starts_with("0x")
}

pub fn has_0x_prefix_ascii(buf: &[u8]) -> bool {
    buf.starts_with(b"0x")
}
