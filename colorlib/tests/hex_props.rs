use colorlib::parse_hex_color;
use proptest::prelude::*;
use testsupport::{model_parse_hex_color, proptest_support::hex_case_strategy};

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
}
