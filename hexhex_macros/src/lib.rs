#![forbid(unsafe_code)]

use hexhex_impl::decode;
use proc_macro::{Literal, TokenStream, TokenTree};

/// Convert a hex literal (string or byte string) to bytes at compile time.
///
/// # Examples
///
/// ```
/// use hexhex_macros::hex_literal;
/// assert_eq!(hex_literal!("0123"), &[0x1, 0x23]);
/// ```
///
/// Malformed inputs are rejected at compile time
///
/// ```compile_fail
/// use hexhex_macros::hex_literal;
/// let _ = hex_literal!("012"); // odd number of hex digits is not allowed
/// ```
#[proc_macro]
pub fn hex_literal(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    match iter.next() {
        Some(TokenTree::Literal(lit)) => {
            let s = lit.to_string();
            let content = if s.starts_with('"') && s.ends_with('"') {
                // must be a string
                &s[1..s.len() - 1]
            } else if s.starts_with("b\"") && s.ends_with('"') {
                &s[2..s.len() - 1]
            } else {
                panic!("unexpected argument {}", lit);
            };

            let bytes = match decode(content) {
                Ok(x) => x,
                Err(e) => panic!("{e}"),
            };

            if let Some(tree) = iter.next() {
                panic!("unexpected argument {}", tree);
            }
            [TokenTree::from(Literal::byte_string(&bytes))]
                .into_iter()
                .collect()
        }
        None => panic!("expected a string or bytestring"),
        _ => panic!("unexpected argument"),
    }
}
