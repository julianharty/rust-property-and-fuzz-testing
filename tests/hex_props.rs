use chatgpt_rust_property_and_fuzz_testing::{parse_hex_color, to_hex_color};

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn hex_roundtrip(r in any::<u8>(), g in any::<u8>(), b in any::<u8>()) {
            let s = to_hex_color((r, g, b));
            println!("{}", s);
            prop_assert_eq!(parse_hex_color(&s), Some((r, g, b)));
        }
    }
}
