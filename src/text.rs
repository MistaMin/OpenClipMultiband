//! Parameter value text (from OpenDynamics), the same for hosts (automation lanes, tooltips, typed
//! values) and the editor: whole dB, ceilings in tenths, at most three digits per number, kilo
//! units from 1000, never "-0".

/// A non-negative number in at most three digits.
fn fit(v: f32) -> String {
    match v {
        v if v.round() >= 100.0 => format!("{v:.0}"),
        v if (v * 10.0).round() >= 100.0 => format!("{v:.1}"),
        v => format!("{v:.2}"),
    }
}

fn signed_whole(v: f32) -> String {
    let r = v.round();
    if r == 0.0 { "0".into() } else { format!("{r:.0}") }
}

/// The number at the start of `s` and the rest, lower-cased and trimmed.
fn split(s: &str) -> Option<(f32, String)> {
    let s = s.trim();
    let end = s.find(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-' || c == '+')).unwrap_or(s.len());
    let n: f32 = s[..end].trim().parse().ok()?;
    n.is_finite().then(|| (n, s[end..].trim().to_lowercase()))
}

pub fn db(v: f32) -> String {
    format!("{} dB", signed_whole(v))
}

/// Ceilings keep a tenth of a dB (a mastering ceiling is -0.3 or -1.0).
pub fn db_tenth(v: f32) -> String {
    let t = (v * 10.0).round() / 10.0;
    if t == 0.0 { "0.0 dB".into() } else { format!("{t:.1} dB") }
}

pub fn parse_db(s: &str) -> Option<f32> {
    let (n, rest) = split(s)?;
    matches!(rest.as_str(), "" | "db").then_some(n)
}

pub fn hz(v: f32) -> String {
    if v >= 1000.0 { format!("{} kHz", fit(v / 1000.0)) } else { format!("{} Hz", fit(v)) }
}

pub fn parse_hz(s: &str) -> Option<f32> {
    let (n, rest) = split(s)?;
    match rest.as_str() {
        "" | "hz" => Some(n),
        "khz" | "k" => Some(n * 1000.0),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prints_and_reads_back() {
        assert_eq!(db(-18.0), "-18 dB");
        assert_eq!(db(-0.2), "0 dB");
        assert_eq!(db_tenth(-1.04), "-1.0 dB");
        assert_eq!(db_tenth(-0.01), "0.0 dB");
        assert_eq!(hz(22_000.0), "22.0 kHz");
        assert_eq!(hz(141.4), "141 Hz");
        assert_eq!(parse_db("-30 dB"), Some(-30.0));
        assert_eq!(parse_db("-0.3"), Some(-0.3));
        assert_eq!(parse_hz("2.25 kHz"), Some(2250.0));
        assert_eq!(parse_hz("750"), Some(750.0));
        assert_eq!(parse_db("loud"), None);
    }
}
