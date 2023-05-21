# hexhex 🪄 hexadecimal conversion

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

## Decoding

Allocating, only with `std`:

```
use hexhex::decode;
assert_eq!(&decode("c0ffee").unwrap(), &[0xc0, 0xff, 0xee]);
```

## Macro

```
use hexhex::hex_literal;
let bytes: &[u8; 3] = hex_literal!("0xc0ffee");
assert_eq!(bytes, &[0xc0, 0xff, 0xee]);
```

## License

TBD

## Contributing

[Bug reports](https://github.com/hwjsnc/hexhex/issues) are very much appreciated.
Feel free to request features, but no promises.
I do not intend to accept third-party patches at this time.
