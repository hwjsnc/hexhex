#![no_std]

#[cfg(feature = "std")]
extern crate std;

mod decode;
mod encode;

#[cfg(test)]
mod tests;

pub use decode::buf::{
    decode_ascii_to_buf, decode_ascii_to_buf_exact, decode_to_buf, decode_to_buf_exact,
};
pub use decode::streaming::{HexDecodeAsciiIterator, HexDecodeIterator};
pub use decode::FromHexError;

#[cfg(feature = "std")]
pub use decode::vec::{decode, decode_ascii};

pub use encode::{Case, DisplayOptions, Hex};
