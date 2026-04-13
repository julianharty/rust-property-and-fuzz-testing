pub fn parse_hex_color(s: &str) -> Option<(u8, u8, u8)> {
    if s.len() != 7 || !s.starts_with('#') {
        return None;
    }
    let r = u8::from_str_radix(&s[1..3], 16).ok()?;
    let g = u8::from_str_radix(&s[3..5], 16).ok()?;
    let b = u8::from_str_radix(&s[5..7], 16).ok()?;
    Some((r, g, b))
}

pub fn to_hex_color((r, g, b): (u8, u8, u8)) -> String {
    format!("#{r:02X}{g:02X}{b:02X}")
}

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
