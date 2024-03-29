#![forbid(unsafe_code)]

use hexhex_impl::decode;
use proc_macro::{Literal, TokenStream, TokenTree};

macro_rules! return_compile_error {
    ($($fmt:tt)*) => {{
        return compile_error(&format!($($fmt)*));
    }}
}

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
                return_compile_error!("unexpected argument {lit}");
            };

            let bytes = match decode(content) {
                Ok(x) => x,
                Err(e) => return_compile_error!("{e}"),
            };

            if let Some(tree) = iter.next() {
                return_compile_error!("unexpected argument {tree}");
            }
            [TokenTree::from(Literal::byte_string(&bytes))]
                .into_iter()
                .collect()
        }
        None => return_compile_error!("expected a string or bytestring"),
        _ => return_compile_error!("unexpected argument"),
    }
}

/// Return a token tree that causes a compile error with a nice error message
fn compile_error(msg: &str) -> TokenStream {
    // https://stackoverflow.com/a/75506477
    use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span};
    [
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Ident(Ident::new("std", Span::mixed_site())),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
        TokenTree::Ident(Ident::new("compile_error", Span::mixed_site())),
        TokenTree::Punct(Punct::new('!', Spacing::Alone)),
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            [TokenTree::Literal(Literal::string(msg))]
                .into_iter()
                .collect(),
        )),
    ]
    .into_iter()
    .collect()
}
