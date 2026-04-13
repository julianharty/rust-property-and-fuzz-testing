#![no_main]
use libfuzzer_sys::fuzz_target;
use chatgpt_rust_property_and_fuzz_testing::{parse_hex_color};

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        println!("{}", s);
        let _ = parse_hex_color(s); // should never panic
    }
});
