#![no_main]

use arbitrary::Unstructured;
use libfuzzer_sys::fuzz_target;
use testsupport::HexInput;

fuzz_target!(|data: &[u8]| {
    let _ = colorlib::parse_hex_color(std::str::from_utf8(data).unwrap_or(""));

    if let Ok(u) = Unstructured::new(data).arbitrary::<HexInput>() {
        let s = u.to_ascii_hex_string();
        let _ = colorlib::parse_hex_color(&s);
    }
});
