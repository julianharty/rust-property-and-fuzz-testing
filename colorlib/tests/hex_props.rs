use colorlib::{parse_hex_color, to_hex_color};
use proptest::prelude::*;
use testsupport::hexish_strings;

proptest! {
    #[test]
    fn valid_roundtrip(r in any::<u8>(), g in any::<u8>(), b in any::<u8>()) {
        let s = to_hex_color((r, g, b));
        prop_assert_eq!(parse_hex_color(&s), Some((r, g, b)));
    }

    #[test]
    fn parse_never_panics_on_targeted_strings(s in hexish_strings()) {
        println!("{}", s);
        let _ = parse_hex_color(&s);
    }

    #[test]
    fn never_panics_any_input(s in any::<String>()) {
        let _ = parse_hex_color(&s);
    }
}
