/// Parses a color in `#RRGGBB` form.
///
/// ```
/// use colorlib::parse_hex_color;
///
/// assert_eq!(parse_hex_color("#FF0000"), Some((255, 0, 0)));
/// assert_eq!(parse_hex_color("red"), None);
/// ```
pub fn parse_hex_color(s: &str) -> Option<(u8, u8, u8)> {
    let bytes = s.as_bytes();

    if bytes.len() != 7 || bytes[0] != b'#' {
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

    fn parse_pair(hi: u8, lo: u8) -> Option<u8> {
        Some((hex_val(hi)? << 4) | hex_val(lo)?)
    }

    Some((
        parse_pair(bytes[1], bytes[2])?,
        parse_pair(bytes[3], bytes[4])?,
        parse_pair(bytes[5], bytes[6])?,
    ))
}

pub fn to_hex_color((r, g, b): (u8, u8, u8)) -> String {
    format!("#{r:02X}{g:02X}{b:02X}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_black() {
        assert_eq!(parse_hex_color("#000000"), Some((0, 0, 0)));
    }

    #[test]
    fn rejects_bad_prefix() {
        assert_eq!(parse_hex_color("!123456"), None);
    }

    #[test]
    fn rejects_wrong_length() {
        assert_eq!(parse_hex_color("#123"), None);
    }

    #[test]
    fn renderer_formats_uppercase() {
        assert_eq!(to_hex_color((10, 11, 12)), "#0A0B0C");
    }
}
