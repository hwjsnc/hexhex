extern crate std;

use std::format;
use std::string::ToString;

use super::*;

#[cfg(feature = "std")]
use std::vec;

#[test]
fn hex_default_empty() {
    let data = [];
    assert_eq!(Hex::new(data).to_string(), "");
}

#[test]
fn hex_default_1() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    assert_eq!(Hex::new(data).to_string(), "0102030405060708090a0b0c0d0e0f");
}

#[test]
fn hex_default_2() {
    let data = [
        16 * 1,
        16 * 2,
        16 * 3,
        16 * 4,
        16 * 5,
        16 * 6,
        16 * 7,
        16 * 8,
        16 * 9,
        16 * 10,
        16 * 11,
        16 * 12,
        16 * 13,
        16 * 14,
        16 * 15,
    ];
    assert_eq!(Hex::new(data).to_string(), "102030405060708090a0b0c0d0e0f0");
}

#[test]
fn hex_default_single_byte() {
    for x in 0u8..=u8::MAX {
        assert_eq!(Hex::new([x]).to_string(), format!("{x:02x}"));
    }
}

#[test]
fn hex_with_prefix_empty() {
    // This is perhaps counterintuitive, but required for roundtrip of empty data
    assert_eq!(Hex::new([]).with_prefix(true).to_string(), "0x");
}

#[test]
fn hex_with_options() {
    let data = [0x01, 0x0a, 0x4b, 0xb5, 0x00, 0xff, 0xb2, 0x04, 0x42];
    let hex = Hex::new(&data);
    assert_eq!(hex.to_string(), "010a4bb500ffb20442");
    assert_eq!(hex.with_case(Case::Lower).to_string(), "010a4bb500ffb20442");
    assert_eq!(hex.with_case(Case::Upper).to_string(), "010A4BB500FFB20442");
    assert_eq!(hex.with_prefix(false).to_string(), "010a4bb500ffb20442");
    assert_eq!(hex.with_prefix(true).to_string(), "0x010a4bb500ffb20442");
    assert_eq!(
        hex.with_case(Case::Upper).with_prefix(true).to_string(),
        "0x010A4BB500FFB20442"
    );
    assert_eq!(
        hex.with_options(DisplayOptions {
            with_prefix: true,
            case: Case::Upper
        })
        .to_string(),
        "0x010A4BB500FFB20442"
    );
    assert_eq!(
        hex.with_options(DisplayOptions {
            with_prefix: false,
            case: Case::Lower,
        })
        .to_string(),
        "010a4bb500ffb20442"
    );
    assert_eq!(
        hex.with_options(DisplayOptions {
            with_prefix: true,
            case: Case::Upper,
        })
        .with_prefix(false)
        .to_string(),
        "010A4BB500FFB20442"
    );
}

#[test]
fn decode_ascii_to_buf_empty() {
    let mut buf = [0xFFu8; 4];
    assert_eq!(decode_ascii_to_buf(b"", &mut buf).unwrap(), 0);
    assert_eq!(buf, [0xFFu8; 4]);
}

#[test]
fn decode_ascii_to_buf_empty_with_prefix() {
    let mut buf = [0xFFu8; 4];
    assert_eq!(decode_ascii_to_buf(b"0x", &mut buf).unwrap(), 0);
    assert_eq!(buf, [0xFFu8; 4]);
}

#[test]
fn decode_ascii_to_buf_odd() {
    let mut buf = [0xFFu8; 4];
    assert!(decode_ascii_to_buf(b"0x01020", &mut buf).is_err());
}

#[test]
fn decode_ascii_to_buf_1() {
    let mut buf = [0xFFu8; 4];
    assert_eq!(decode_ascii_to_buf(b"ab0110", &mut buf).unwrap(), 3);
    assert_eq!(buf, [0xab, 0x01, 0x10, 0xFF]);
}

#[test]
fn decode_ascii_to_buf_2() {
    let mut buf = [0xffu8; 4];
    assert_eq!(decode_ascii_to_buf(b"0xAb0110", &mut buf).unwrap(), 3);
    assert_eq!(buf, [0xab, 0x01, 0x10, 0xff]);
}

#[test]
fn decode_ascii_to_buf_3() {
    let mut buf = [0u8; 4];
    assert_eq!(decode_ascii_to_buf(b"0xAb01105c", &mut buf).unwrap(), 4);
    assert_eq!(buf, [0xab, 0x01, 0x10, 0x5c]);
}

#[test]
fn decode_ascii_to_buf_too_long() {
    let mut buf = [0u8; 5];
    assert!(decode_ascii_to_buf(b"010203040506", &mut buf).is_err());
}

#[test]
fn decode_ascii_to_buf_double_prefix() {
    let mut buf = [0u8; 5];
    assert!(decode_ascii_to_buf(b"0x0x1234", &mut buf).is_err());
}

#[test]
fn decode_to_buf_empty() {
    let mut buf = [0xFFu8; 4];
    assert_eq!(decode_ascii_to_buf(b"", &mut buf).unwrap(), 0);
    assert_eq!(buf, [0xFFu8; 4]);
}

#[test]
fn decode_to_buf_empty_with_prefix() {
    let mut buf = [0xFFu8; 4];
    assert_eq!(decode_ascii_to_buf(b"0x", &mut buf).unwrap(), 0);
    assert_eq!(buf, [0xFFu8; 4]);
}

#[test]
fn decode_to_buf_odd() {
    let mut buf = [0xFFu8; 4];
    assert!(decode_to_buf("0x01020", &mut buf).is_err());
}

#[test]
fn decode_to_buf_1() {
    let mut buf = [0xFFu8; 4];
    assert_eq!(decode_to_buf("ab0110", &mut buf).unwrap(), 3);
    assert_eq!(buf, [0xab, 0x01, 0x10, 0xFF]);
}

#[test]
fn decode_to_buf_2() {
    let mut buf = [0xffu8; 4];
    assert_eq!(decode_to_buf("0xAb0110", &mut buf).unwrap(), 3);
    assert_eq!(buf, [0xab, 0x01, 0x10, 0xff]);
}

#[test]
fn decode_to_buf_3() {
    let mut buf = [0u8; 4];
    assert_eq!(decode_to_buf("0xAb01105c", &mut buf).unwrap(), 4);
    assert_eq!(buf, [0xab, 0x01, 0x10, 0x5c]);
}

#[test]
fn decode_to_buf_too_long() {
    let mut buf = [0u8; 5];
    assert!(decode_to_buf("010203040506", &mut buf).is_err());
}

#[test]
fn decode_to_buf_double_prefix() {
    let mut buf = [0u8; 5];
    assert!(decode_to_buf("0x0x1234", &mut buf).is_err());
}

#[test]
fn decode_ascii_to_buf_exact_1() {
    let mut buf = [0u8; 5];
    assert!(decode_ascii_to_buf_exact(b"0102030405", &mut buf).is_ok());
    assert_eq!(buf, [0x01, 0x02, 0x03, 0x04, 0x05]);
}

#[test]
fn decode_ascii_to_buf_exact_2() {
    let mut buf = [0u8; 3];
    assert!(decode_ascii_to_buf_exact(b"0xc0ffee", &mut buf).is_ok());
    assert_eq!(buf, [0xc0, 0xff, 0xee]);
}

#[test]
fn decode_ascii_to_buf_exact_empty() {
    let mut buf = [0u8; 0];
    assert!(decode_ascii_to_buf_exact(b"", &mut buf).is_ok());
}

#[test]
fn decode_ascii_to_buf_exact_empty_with_prefix() {
    let mut buf = [0u8; 0];
    assert!(decode_ascii_to_buf_exact(b"0x", &mut buf).is_ok());
}

#[test]
fn decode_ascii_to_buf_exact_too_short() {
    let mut buf = [0u8; 4];
    assert!(decode_ascii_to_buf_exact(b"0xc0ffee", &mut buf).is_err());
}

#[test]
fn decode_ascii_to_buf_exact_too_long() {
    let mut buf = [0u8; 4];
    assert!(decode_ascii_to_buf_exact(b"0x0a0b0c0d0e0f", &mut buf).is_err());
}

#[test]
fn decode_ascii_to_buf_exact_odd_1() {
    let mut buf = [0u8; 2];
    assert!(decode_ascii_to_buf_exact(b"0xabc", &mut buf).is_err());
}

#[test]
fn decode_ascii_to_buf_exact_odd_2() {
    let mut buf = [0u8; 2];
    assert!(decode_ascii_to_buf_exact(b"0xabcde", &mut buf).is_err());
}

#[test]
fn decode_ascii_to_buf_exact_double_prefix() {
    let mut buf = [0u8; 2];
    assert!(decode_ascii_to_buf(b"0x0x1234", &mut buf).is_err());
}

#[test]
fn decode_to_buf_exact_1() {
    let mut buf = [0u8; 5];
    assert!(decode_to_buf_exact("0102030405", &mut buf).is_ok());
    assert_eq!(buf, [0x01, 0x02, 0x03, 0x04, 0x05]);
}

#[test]
fn decode_to_buf_exact_2() {
    let mut buf = [0u8; 3];
    assert!(decode_to_buf_exact("0xc0ffee", &mut buf).is_ok());
    assert_eq!(buf, [0xc0, 0xff, 0xee]);
}

#[test]
fn decode_to_buf_exact_empty() {
    let mut buf = [0u8; 0];
    assert!(decode_to_buf_exact("", &mut buf).is_ok());
}

#[test]
fn decode_to_buf_exact_empty_with_prefix() {
    let mut buf = [0u8; 0];
    assert!(decode_to_buf_exact("0x", &mut buf).is_ok());
}

#[test]
fn decode_to_buf_exact_too_short() {
    let mut buf = [0u8; 4];
    assert!(decode_to_buf_exact("0xc0ffee", &mut buf).is_err());
}

#[test]
fn decode_to_buf_exact_too_long() {
    let mut buf = [0u8; 4];
    assert!(decode_to_buf_exact("0x0a0b0c0d0e0f", &mut buf).is_err());
}

#[test]
fn decode_to_buf_exact_odd_1() {
    let mut buf = [0u8; 2];
    assert!(decode_to_buf_exact("0xabc", &mut buf).is_err());
}

#[test]
fn decode_to_buf_exact_odd_2() {
    let mut buf = [0u8; 2];
    assert!(decode_to_buf_exact("0xabcde", &mut buf).is_err());
}

#[test]
fn decode_to_buf_exact_double_prefix() {
    let mut buf = [0u8; 2];
    assert!(decode_to_buf("0x0x1234", &mut buf).is_err());
}

#[cfg(feature = "std")]
#[test]
fn decode_ascii_empty() {
    assert_eq!(decode_ascii(b"").unwrap(), vec![]);
}

#[cfg(feature = "std")]
#[test]
fn decode_ascii_empty_with_prefix() {
    assert_eq!(decode_ascii(b"0x").unwrap(), vec![]);
}

#[cfg(feature = "std")]
#[test]
fn decode_ascii_1() {
    assert_eq!(decode_ascii(b"0xc0ffee").unwrap(), vec![0xc0, 0xff, 0xee]);
}

#[cfg(feature = "std")]
#[test]
fn decode_ascii_2() {
    assert_eq!(
        decode_ascii(b"a92e1000f1").unwrap(),
        vec![0xa9, 0x2e, 0x10, 0x00, 0xf1]
    );
}

#[cfg(feature = "std")]
#[test]
fn decode_ascii_odd() {
    assert!(decode_ascii(b"123").is_err());
}

#[cfg(feature = "std")]
#[test]
fn decode_ascii_double_prefix() {
    assert!(decode_ascii(b"0x0x1234").is_err());
}

#[cfg(feature = "std")]
#[test]
fn decode_empty() {
    assert_eq!(decode("").unwrap(), vec![]);
}

#[cfg(feature = "std")]
#[test]
fn decode_empty_with_prefix() {
    assert_eq!(decode("0x").unwrap(), vec![]);
}

#[cfg(feature = "std")]
#[test]
fn decode_1() {
    assert_eq!(decode("0xc0ffee").unwrap(), vec![0xc0, 0xff, 0xee]);
}

#[cfg(feature = "std")]
#[test]
fn decode_2() {
    assert_eq!(
        decode("a92e1000f1").unwrap(),
        vec![0xa9, 0x2e, 0x10, 0x00, 0xf1]
    );
}

#[cfg(feature = "std")]
#[test]
fn decode_odd() {
    assert!(decode("123").is_err());
}

#[cfg(feature = "std")]
#[test]
fn decode_double_prefix() {
    assert!(decode("0x0x1234").is_err());
}

#[cfg(all(feature = "proptest", feature = "std"))]
mod property_tests {
    use super::*;

    const CASES: u32 = 1_000_000;

    proptest::proptest! {
        #![proptest_config(proptest::prelude::ProptestConfig::with_cases(CASES))]
        #[test]
        fn no_crash_str(s in ".*") {
            // this buf is definitely large enough to hold the result if the input is well-formed
            let mut buf1 = vec![0u8; s.len()];
            // this buffer is definitely not large enough to hold the result (unless the input is empty)
            let mut buf2 = vec![0u8; (s.chars().count() / 2).saturating_sub(2)];

            // one of these two will exactly fit the output, assuming the input is well-formed.
            let mut buf3 = vec![0u8; s.chars().count() / 2];
            let mut buf4 = vec![0u8; (s.chars().count() / 2).saturating_sub(1)];

            // anyway, here we go.
            _ = decode_to_buf(&s, &mut buf1);
            _ = decode_to_buf(&s, &mut buf2);
            _ = decode_to_buf(&s, &mut buf3);
            _ = decode_to_buf(&s, &mut buf4);

            // and again
            _ = decode_to_buf_exact(&s, &mut buf1);
            _ = decode_to_buf_exact(&s, &mut buf2);
            _ = decode_to_buf_exact(&s, &mut buf3);
            _ = decode_to_buf_exact(&s, &mut buf4);

            _ = decode(&s);
        }
    }

    proptest::proptest! {
        #![proptest_config(proptest::prelude::ProptestConfig::with_cases(CASES))]
        #[test]
        fn no_crash_bytes(bytes: std::vec::Vec<u8>) {
            let mut buf1 = vec![0u8; bytes.len()];
            let mut buf2 = vec![0u8; (bytes.len() / 2).saturating_sub(2)];
            let mut buf3 = vec![0u8; bytes.len() / 2];
            let mut buf4 = vec![0u8; (bytes.len() / 2).saturating_sub(1)];

            _ = decode_ascii_to_buf(&bytes, &mut buf1);
            _ = decode_ascii_to_buf(&bytes, &mut buf2);
            _ = decode_ascii_to_buf(&bytes, &mut buf3);
            _ = decode_ascii_to_buf(&bytes, &mut buf4);

            _ = decode_ascii_to_buf_exact(&bytes, &mut buf1);
            _ = decode_ascii_to_buf_exact(&bytes, &mut buf2);
            _ = decode_ascii_to_buf_exact(&bytes, &mut buf3);
            _ = decode_ascii_to_buf_exact(&bytes, &mut buf4);

            _ = decode_ascii(&bytes);
        }
    }

    proptest::proptest! {
        #![proptest_config(proptest::prelude::ProptestConfig::with_cases(CASES))]
        #[test]
        fn roundtrip_parse_to_string(original in "(0x)?([[:xdigit:]]{2})*") {
            let has_prefix = original.starts_with("0x");
            let bytes = decode(&original).unwrap();
            let roundtripped = Hex::new(bytes).with_prefix(has_prefix).to_string();
            assert_eq!(roundtripped.to_lowercase(), original.to_lowercase());
        }
    }

    proptest::proptest! {
        #![proptest_config(proptest::prelude::ProptestConfig::with_cases(CASES))]
        #[test]
        fn roundtrip_parse_ascii_to_string(original in "(0x)?([[:xdigit:]]{2})*") {
            let has_prefix = original.starts_with("0x");
            let bytes = decode_ascii(original.as_bytes()).unwrap();
            let roundtripped = Hex::new(bytes).with_prefix(has_prefix).to_string();
            assert_eq!(roundtripped.to_lowercase(), original.to_lowercase());
        }
    }

    proptest::proptest! {
        #![proptest_config(proptest::prelude::ProptestConfig::with_cases(CASES))]
        #[test]
        fn reject_odd(s in "(0x)?([[:xdigit:]]{2})*[[:xdigit:]]") {
            let len = if s.starts_with("0x") {
                s.len() / 2 - 1
            } else {
                s.len() / 2
            };
            let mut buf = vec![0u8; len];
            assert!(decode_ascii_to_buf(s.as_bytes(), &mut buf).is_err());
            assert!(decode_ascii_to_buf_exact(s.as_bytes(), &mut buf).is_err());
            assert!(decode_ascii(s.as_bytes()).is_err());

            assert!(decode_to_buf(&s, &mut buf).is_err());
            assert!(decode_to_buf_exact(&s, &mut buf).is_err());
            assert!(decode(&s).is_err());
        }
    }

    proptest::proptest! {
        #![proptest_config(proptest::prelude::ProptestConfig::with_cases(CASES))]
        #[test]
        fn reject_bad_char(s in "[^0].*[[:^xdigit:]].*|0x.*[[:^xdigit:]].*") {
            let len = if s.starts_with("0x") {
                s.len() / 2 - 1
            } else {
                s.len() / 2
            };
            let mut buf = vec![0u8; len];
            assert!(decode_ascii_to_buf(s.as_bytes(), &mut buf).is_err());
            assert!(decode_ascii_to_buf_exact(s.as_bytes(), &mut buf).is_err());
            assert!(decode_ascii(s.as_bytes()).is_err());

            assert!(decode_to_buf(&s, &mut buf).is_err());
            assert!(decode_to_buf_exact(&s, &mut buf).is_err());
            assert!(decode(&s).is_err());
        }
    }

    proptest::proptest! {
        #![proptest_config(proptest::prelude::ProptestConfig::with_cases(CASES))]
        #[test]
        fn roundtrip_bytes_to_string_and_back(original: std::vec::Vec<u8>) {
            // via string
            {
                let s = Hex::new(&original).to_string();
                let roundtripped = decode(&s).unwrap();
                assert_eq!(roundtripped, original);
            }

            {
                let s = Hex::new(&original).with_prefix(true).to_string();
                let roundtripped = decode(&s).unwrap();
                assert_eq!(roundtripped, original);
            }

            {
                let s = Hex::new(&original).with_case(Case::Upper).to_string();
                let roundtripped = decode(&s).unwrap();
                assert_eq!(roundtripped, original);
            }

            {
                let s = Hex::new(&original).with_prefix(true).with_case(Case::Upper).to_string();
                let roundtripped = decode(&s).unwrap();
                assert_eq!(roundtripped, original);
            }

            // via bytes
            {
                let s = Hex::new(&original).to_string();
                let roundtripped = decode_ascii(s.as_bytes()).unwrap();
                assert_eq!(roundtripped, original);
            }

            {
                let s = Hex::new(&original).with_prefix(true).to_string();
                let roundtripped = decode_ascii(s.as_bytes()).unwrap();
                assert_eq!(roundtripped, original);
            }

            {
                let s = Hex::new(&original).with_case(Case::Upper).to_string();
                let roundtripped = decode_ascii(s.as_bytes()).unwrap();
                assert_eq!(roundtripped, original);
            }

            {
                let s = Hex::new(&original).with_prefix(true).with_case(Case::Upper).to_string();
                let roundtripped = decode_ascii(s.as_bytes()).unwrap();
                assert_eq!(roundtripped, original);
            }
        }
    }
}
