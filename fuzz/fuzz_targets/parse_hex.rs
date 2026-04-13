#![no_main]
use libfuzzer_sys::fuzz_target;
use your_crate::parse_hex_color;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = parse_hex_color(s); // should never panic
    }
});
