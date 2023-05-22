/*!

# `hexhex` 🪄 hexadecimal conversion

Features:

- Display bytes as hex with no (heap) allocations
- Convert bytes to hex `String`
- Convert hex `&str` or `&[u8]` to a new byte vector
- Convert hex `&str` or `&[u8]` to bytes in a preallocated buffer
- Macro for all your compile-time hex to bytes conversion needs
- `#![no_std]` support for a subset of the above
- No runtime panics (except for internal bugs)

## Encoding

```
use hexhex::Hex;
let bytes = [0xc0, 0xff, 0xee];
println!("{}", Hex::new(&bytes)); // no allocations, prints "c0ffee"
```

```
use hexhex::{Hex, Case};
let bytes = [0xc0, 0xff, 0xee];
println!("{}", Hex::new(&bytes).with_prefix(true).with_case(Case::Upper)); // no allocations, prints "0xC0FFEE"
```

## Encode to String

`Hex` implements the [`core::fmt::Display`] trait, so conversion to string is as easy as:

```
use hexhex::Hex;
let bytes = [0xc0, 0xff, 0xee];
let hex = Hex::new(&bytes).to_string();
assert_eq!(hex, "c0ffee");
```

One perhaps surprising conversion is that of an empty byte slice when prefixes are enabled:

```
use hexhex::Hex;
assert_eq!(Hex::new(b"").with_prefix(true).to_string(), "0x");
```

## Decoding (no allocations)

```
use hexhex::decode_to_buf_exact;
let hex = "0xc0fFEe";
let mut buf = [0u8; 3];
assert!(decode_to_buf_exact(hex, &mut buf).is_ok());
assert_eq!(buf, [0xc0, 0xff, 0xee]);
```

There are some other variants, check the list of functions to see them all.
The `ascii` variants take byte strings (`&[u8]`) which need not be valid ASCII or UTF-8 (however, only valid ASCII can be valid hex strings).

## Decoding (std)

```
#[cfg(feature = "std")]
{
use hexhex::decode;
assert_eq!(&decode("c0ffee").unwrap(), &[0xc0, 0xff, 0xee]);
}
```

```
#[cfg(feature = "std")]
{
use hexhex::decode_ascii;
assert_eq!(&decode_ascii(b"c0ffee").unwrap(), &[0xc0, 0xff, 0xee]);
}
```

## Macro

```
use hexhex::hex_literal;
let bytes: &[u8; 3] = hex_literal!("0xc0ffee");
assert_eq!(bytes, &[0xc0, 0xff, 0xee]);
```

Malformed hex strings will cause a compile-time error:

```compile_fail
use hexhex::hex_literal;
let bytes = hex_literal!("c0f"); // odd number of hex digits is invalid
```

The macro is a proc-macro, not a declarative macro; it can be used in a `match` arm:

```
use hexhex::hex_literal;
let x = &[0x12, 0x34];
match x {
    hex_literal!("1234") => {},
    _ => panic!(),
}
```

## Feature flags

- `std` (enabled by default): Enables functionality that makes use of `std`. With this flag disabled, the crate is `#![no_std]` compatible.

*/

#![forbid(unsafe_code)]

#[cfg(feature = "std")]
pub use hexhex_impl::{decode, decode_ascii};
pub use hexhex_impl::{
    decode_ascii_to_buf, decode_ascii_to_buf_exact, decode_to_buf, decode_to_buf_exact, Case,
    FromHexError, Hex,
};
pub use hexhex_macros::*;
