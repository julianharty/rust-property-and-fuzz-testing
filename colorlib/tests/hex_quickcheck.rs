use colorlib::parse_hex_color;
use quickcheck::{QuickCheck, TestResult};
use testsupport::{model_parse_hex_color, HexCase};

#[test]
fn quickcheck_parser_matches_model() {
    QuickCheck::new()
        .tests(10_000)
        .quickcheck(prop as fn(HexCase) -> TestResult);

    fn prop(case: HexCase) -> TestResult {
        let actual = case.as_str().and_then(parse_hex_color);
        let expected = model_parse_hex_color(case.as_bytes());
        TestResult::from_bool(actual == expected)
    }
}
