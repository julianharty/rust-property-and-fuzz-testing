use colorlib::{parse_hex_color, to_hex_color};
use proptest::prelude::*;
use testsupport::{model_parse_hex_color, model_to_hex, proptest_support::hex_case_strategy};

proptest! {
    #[test]
    fn parser_matches_model(case in hex_case_strategy()) {
        let actual = case.as_str().and_then(parse_hex_color);
        let expected = model_parse_hex_color(case.as_bytes());

        prop_assert_eq!(
            actual,
            expected,
            "kind={:?}, bytes={:?}, display={:?}",
            case.kind,
            case.as_bytes(),
            case.display()
        );
    }

    #[test]
    fn renderer_matches_model(r in any::<u8>(), g in any::<u8>(), b in any::<u8>()) {
        let rgb = (r, g, b);
        prop_assert_eq!(to_hex_color(rgb), model_to_hex(rgb));
    }

    #[test]
    fn parser_accepts_model_output(r in any::<u8>(), g in any::<u8>(), b in any::<u8>()) {
        let rgb = (r, g, b);
        let s = model_to_hex(rgb);
        prop_assert_eq!(parse_hex_color(&s), Some(rgb));
    }
}
