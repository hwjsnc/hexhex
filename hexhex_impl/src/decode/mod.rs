/// Represents an error encountered while decoding a hex string
#[derive(Debug, Clone, Copy)]
pub struct FromHexError {
    position: usize,
    kind: FromHexErrorKind,
}

#[derive(Debug, Clone, Copy)]
pub enum FromHexErrorKind {
    UnexpectedCharacter(char),
    UnexpectedByte(u8),
    Eof,
    OutputBufferTooShort,
}

mod common;

//pub mod generic;
pub mod buf;
pub mod streaming;

#[cfg(feature = "std")]
pub mod vec;
//#[cfg(feature = "std")]
//pub mod write;
