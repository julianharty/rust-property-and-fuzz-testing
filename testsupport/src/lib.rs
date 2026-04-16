// testsupport/src/lib.rs

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HexCaseKind {
    ValidHex,
    BadPrefix,
    BadAsciiHex,
    Utf8BoundaryBreaker,
    WrongLength,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HexCase {
    pub kind: HexCaseKind,
    pub bytes: Vec<u8>,
}

impl HexCase {
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn as_str(&self) -> Option<&str> {
        std::str::from_utf8(&self.bytes).ok()
    }

    pub fn display(&self) -> String {
        String::from_utf8_lossy(&self.bytes).into_owned()
    }

    pub fn interesting_cases() -> Vec<Self> {
        vec![
            Self {
                kind: HexCaseKind::ValidHex,
                bytes: b"#000000".to_vec(),
            },
            Self {
                kind: HexCaseKind::ValidHex,
                bytes: b"#FFFFFF".to_vec(),
            },
            Self {
                kind: HexCaseKind::BadAsciiHex,
                bytes: b"#12GG99".to_vec(),
            },
            Self {
                kind: HexCaseKind::Utf8BoundaryBreaker,
                bytes: "#0¡® ".as_bytes().to_vec(),
            },
            Self {
                kind: HexCaseKind::WrongLength,
                bytes: b"#123".to_vec(),
            },
            Self {
                kind: HexCaseKind::BadPrefix,
                bytes: b"!123456".to_vec(),
            },
        ]
    }
}

pub fn model_parse_hex_color(input: &[u8]) -> Option<(u8, u8, u8)> {
    if input.len() != 7 || input.first().copied() != Some(b'#') {
        return None;
    }

    fn hex_val(b: u8) -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        }
    }

    Some((
        (hex_val(input[1])? << 4) | hex_val(input[2])?,
        (hex_val(input[3])? << 4) | hex_val(input[4])?,
        (hex_val(input[5])? << 4) | hex_val(input[6])?,
    ))
}

pub fn model_to_hex(rgb: (u8, u8, u8)) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb.0, rgb.1, rgb.2)
}

#[cfg(feature = "with-proptest")]
pub mod proptest_support {
    use super::{HexCase, HexCaseKind};
    use proptest::prelude::*;

    pub fn hex_case_strategy() -> impl Strategy<Value = HexCase> {
        let hex = prop::sample::select("0123456789ABCDEFabcdef".bytes().collect::<Vec<_>>());
        let bad = prop::sample::select("GgZz/:-_ ".bytes().collect::<Vec<_>>());
        let ascii = prop::sample::select("0123456789ABCDEFabcdefXYZ ".bytes().collect::<Vec<_>>());
        let two_byte_utf8 = prop::sample::select(
            (0x00A1u32..=0x00FF)
                .filter_map(char::from_u32)
                .map(|c| c.to_string().into_bytes())
                .collect::<Vec<_>>(),
        );

        prop_oneof![
            // Exactly "#RRGGBB" with valid ASCII hex.
            (
                hex.clone(),
                hex.clone(),
                hex.clone(),
                hex.clone(),
                hex.clone(),
                hex.clone()
            )
                .prop_map(|t| HexCase {
                    kind: HexCaseKind::ValidHex,
                    bytes: vec![b'#', t.0, t.1, t.2, t.3, t.4, t.5],
                }),
            // Right shape, wrong ASCII content in at least one hex position.
            (
                bad,
                hex.clone(),
                hex.clone(),
                hex.clone(),
                hex.clone(),
                hex.clone()
            )
                .prop_map(|t| HexCase {
                    kind: HexCaseKind::BadAsciiHex,
                    bytes: vec![b'#', t.0, t.1, t.2, t.3, t.4, t.5],
                }),
            // Valid byte length and prefix, but unsafe for naive UTF-8 slicing.
            // Shape: '#' + 1 ASCII byte + 2-byte char + 2-byte char + 1 ASCII byte = 7 bytes
            (ascii.clone(), two_byte_utf8.clone(), two_byte_utf8, ascii).prop_map(
                |(a, b, c, d)| {
                    let mut bytes = vec![b'#', a];
                    bytes.extend(b);
                    bytes.extend(c);
                    bytes.push(d);
                    HexCase {
                        kind: HexCaseKind::Utf8BoundaryBreaker,
                        bytes,
                    }
                }
            ),
            // Wrong prefix but otherwise plausible length.
            (
                hex.clone(),
                hex.clone(),
                hex.clone(),
                hex.clone(),
                hex.clone(),
                hex
            )
                .prop_map(|t| HexCase {
                    kind: HexCaseKind::BadPrefix,
                    bytes: vec![b'!', t.0, t.1, t.2, t.3, t.4, t.5],
                }),
            // Arbitrary wrong lengths.
            prop::collection::vec(any::<u8>(), 0..=12)
                .prop_filter("exclude exact 7-byte #xxxxxx cases for WrongLength", |v| {
                    v.len() != 7
                })
                .prop_map(|bytes| HexCase {
                    kind: HexCaseKind::WrongLength,
                    bytes,
                }),
        ]
    }
}

#[cfg(feature = "with-quickcheck")]
impl quickcheck::Arbitrary for HexCase {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let choice = u8::arbitrary(g) % 5;

        match choice {
            0 => {
                let mut bytes = vec![b'#'];
                for _ in 0..6 {
                    let v = u8::arbitrary(g) % 16;
                    bytes.push(b"0123456789ABCDEF"[v as usize]);
                }
                HexCase {
                    kind: HexCaseKind::ValidHex,
                    bytes,
                }
            }
            1 => HexCase {
                kind: HexCaseKind::BadAsciiHex,
                bytes: b"#12GG99".to_vec(),
            },
            2 => HexCase {
                kind: HexCaseKind::Utf8BoundaryBreaker,
                bytes: "#0¡® ".as_bytes().to_vec(),
            },
            3 => HexCase {
                kind: HexCaseKind::BadPrefix,
                bytes: b"!123456".to_vec(),
            },
            _ => {
                let len = usize::arbitrary(g) % 13;
                let mut bytes = Vec::with_capacity(len);
                for _ in 0..len {
                    bytes.push(u8::arbitrary(g));
                }
                if bytes.len() == 7 {
                    bytes.pop();
                }
                HexCase {
                    kind: HexCaseKind::WrongLength,
                    bytes,
                }
            }
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(std::iter::empty())
    }
}

#[cfg(feature = "with-arbitrary")]
impl<'a> arbitrary::Arbitrary<'a> for HexCase {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let choice = u.int_in_range::<u8>(0..=4)?;

        Ok(match choice {
            0 => {
                let mut bytes = vec![b'#'];
                for _ in 0..6 {
                    let v = u.int_in_range::<u8>(0..=15)?;
                    bytes.push(b"0123456789ABCDEF"[v as usize]);
                }
                HexCase {
                    kind: HexCaseKind::ValidHex,
                    bytes,
                }
            }
            1 => HexCase {
                kind: HexCaseKind::BadAsciiHex,
                bytes: b"#12GG99".to_vec(),
            },
            2 => HexCase {
                kind: HexCaseKind::Utf8BoundaryBreaker,
                bytes: "#0¡® ".as_bytes().to_vec(),
            },
            3 => HexCase {
                kind: HexCaseKind::BadPrefix,
                bytes: b"!123456".to_vec(),
            },
            _ => {
                let len = u.int_in_range::<usize>(0..=12)?;
                let mut bytes = u.bytes(len)?.to_vec();
                if bytes.len() == 7 {
                    bytes.pop();
                }
                HexCase {
                    kind: HexCaseKind::WrongLength,
                    bytes,
                }
            }
        })
    }
}
