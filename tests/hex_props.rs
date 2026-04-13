use chatgpt_rust_property_and_fuzz_testing::{parse_hex_color, to_hex_color};

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;


    fn boundary_breaker_string() -> impl Strategy<Value = String> {
     // 2-byte UTF-8 chars from Latin-1 Supplement excluding control-ish gaps
        let two_byte = prop::sample::select(
            (0x00A1u32..=0x00FF)
                .filter_map(char::from_u32)
                .collect::<Vec<_>>(),
        );
        let ascii = prop::sample::select(
            "0123456789ABCDEFabcdefXYZ ".chars().collect::<Vec<_>>(),
        );

        // 1 + 1 + 2 + 2 + 1 = 7 bytes total, always starts with '#'
        (ascii.clone(), two_byte.clone(), two_byte, ascii).prop_map(|(a, b, c, d)| {
            format!("#{a}{b}{c}{d}")
        })
    }

    fn hexish_strings() -> impl Strategy<Value = String> {
        let hex = prop::sample::select("0123456789ABCDEFabcdef".chars().collect::<Vec<_>>());
        let bad_ascii = prop::sample::select("GgZz/:-_ ".chars().collect::<Vec<_>>());

        prop_oneof![
        // fully valid shape and chars
            (hex.clone(), hex.clone(), hex.clone(), hex.clone(), hex.clone(), hex.clone())
                .prop_map(|t| format!("#{}{}{}{}{}{}", t.0,t.1,t.2,t.3,t.4,t.5)),
            // valid shape, invalid ASCII content
            (bad_ascii.clone(), hex.clone(), hex.clone(), hex.clone(), hex.clone(), hex.clone())
                .prop_map(|t| format!("#{}{}{}{}{}{}", t.0,t.1,t.2,t.3,t.4,t.5)),
            // valid shape, dangerous UTF-8 boundaries
            boundary_breaker_string(),
        ]
    }

    proptest! {
        #[test]
        fn parse_never_panics_on_any_string(s in any::<String>()) {
            println!("{}", s);
            let _ = parse_hex_color(&s);
        }

        #[test]
        fn hex_roundtrip(r in any::<u8>(), g in any::<u8>(), b in any::<u8>()) {
            let s = to_hex_color((r, g, b));
            println!("{}", s);
            prop_assert_eq!(parse_hex_color(&s), Some((r, g, b)));
        }

        #[test]
        fn parse_never_panics_on_strings_that_pass_shape_checks(
            s in boundary_breaker_string()) {
            let _ = parse_hex_color(&s);
        }

        #[test]
        fn parse_never_panics_on_hexish_strings(s in hexish_strings()) {
            let _ = parse_hex_color(&s);
        }

    }
}
