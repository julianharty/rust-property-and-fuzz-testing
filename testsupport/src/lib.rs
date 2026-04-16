use arbitrary::Arbitrary;
use proptest::prelude::*;

#[derive(Debug, Clone, Arbitrary)]
pub struct HexInput {
    pub body: [u8; 6],
}

impl HexInput {
    pub fn to_ascii_hex_string(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}",
            self.body[0], self.body[1], self.body[2]
        )
    }
}

pub fn boundary_breaker_string() -> impl Strategy<Value = String> {
    let two_byte = prop::sample::select(
        (0x00A1u32..=0x00FF)
            .filter_map(char::from_u32)
            .collect::<Vec<_>>(),
    );
    let ascii = prop::sample::select("0123456789ABCDEFabcdefXYZ ".chars().collect::<Vec<_>>());

    (ascii.clone(), two_byte.clone(), two_byte, ascii)
        .prop_map(|(a, b, c, d)| format!("#{a}{b}{c}{d}"))
}

pub fn hexish_strings() -> impl Strategy<Value = String> {
    let hex = prop::sample::select("0123456789ABCDEFabcdef".chars().collect::<Vec<_>>());
    let bad = prop::sample::select("GgZz/:-_ ".chars().collect::<Vec<_>>());

    prop_oneof![
        (
            hex.clone(),
            hex.clone(),
            hex.clone(),
            hex.clone(),
            hex.clone(),
            hex.clone()
        )
            .prop_map(|t| format!("#{}{}{}{}{}{}", t.0, t.1, t.2, t.3, t.4, t.5)),
        (
            bad,
            hex.clone(),
            hex.clone(),
            hex.clone(),
            hex.clone(),
            hex.clone()
        )
            .prop_map(|t| format!("#{}{}{}{}{}{}", t.0, t.1, t.2, t.3, t.4, t.5)),
        boundary_breaker_string(),
    ]
}
