#![no_main]

use arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::fuzz_target;
use testsupport::{HexCase, model_parse_hex_color};

fuzz_target!(|data: &[u8]| {
    // Raw-byte path
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = colorlib::parse_hex_color(s);
    }

    // Canonical structured path
    if let Ok(case) = HexCase::arbitrary(&mut Unstructured::new(data)) {
        let actual = case.as_str().and_then(colorlib::parse_hex_color);
        let expected = model_parse_hex_color(case.as_bytes());
        assert_eq!(actual, expected, "case={:?}", case);
    }
});
