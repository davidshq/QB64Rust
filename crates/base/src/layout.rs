//! The layout settings of `-y` (spec `compiler/format`): QB64pe's `-f:` layout settings, read as `qb64pe` reads
//! them (`qb64pe.bas`, the `-f` options and `ParseBooleanSetting&`/`ParseLongSetting&`), with QB64pe's built-in
//! defaults. Here in `base` so that the language server can hold them without the formatter.

/// The letter case of keywords: QB64pe's `IDEAutoLayoutKwStyle` (`_EQUAL`, `_GREATER`, `_LESS`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum KeywordStyle {
    /// `Print`, `End If`, `_RGB32`: the spellings of QB64pe's tables (`SCase$`, `SCase2$`).
    #[default]
    Camel,
    /// `PRINT`.
    Upper,
    /// `print`.
    Lower,
}

/// The layout settings. `Default` is QB64pe's built-in default (`IDE_AutoFormat`, `IDE_AutoIndent` and
/// `IDE_IndentSUBs` true, `IDE_IndentSize` 4, both keyword settings false).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Settings {
    /// `AutoIndent`: indent by block depth; off keeps each line's own indent.
    pub auto_indent: bool,
    /// `AutoIndentSize`: spaces per level, 1 to 64.
    pub indent_size: u32,
    /// `IndentSubs`: the body of a SUB or FUNCTION is one level in.
    pub indent_subs: bool,
    /// `AutoLayout`: one space between elements; off keeps each line's own spacing.
    pub auto_layout: bool,
    pub keywords: KeywordStyle,
    // `KeywordCapitals` and `KeywordLowercase` as given (`tmpKwCap`, `tmpKwLow`): the style follows from both.
    capitals: Option<bool>,
    lowercase: Option<bool>,
    // The style before the first keyword setting, which the two leave when neither is true.
    keywords_before: Option<KeywordStyle>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            auto_indent: true,
            indent_size: 4,
            indent_subs: true,
            auto_layout: true,
            keywords: KeywordStyle::Camel,
            capitals: None,
            lowercase: None,
            keywords_before: None,
        }
    }
}

impl Settings {
    /// Reads one `-f:<name>=<value>` setting. `Ok(false)` when `name` is not a layout setting (a build setting
    /// is the caller's); `Err` names a bad value. Names are not case sensitive, as with `qb64pe`.
    pub fn set(&mut self, name: &str, value: &str) -> Result<bool, String> {
        match name.to_ascii_lowercase().as_str() {
            "autoindent" => self.auto_indent = boolean(name, value)?,
            "autoindentsize" => {
                let size = val(value);
                if !(1.0..=64.0).contains(&size) {
                    return Err(format!("setting `{name}` must be in the range 1 to 64, not `{value}`"));
                }
                // A LONG takes VAL's double rounded half to even, as QB64pe assigns it.
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let size = size.round_ties_even() as u32;
                self.indent_size = size;
            }
            "indentsubs" => self.indent_subs = boolean(name, value)?,
            "autolayout" => self.auto_layout = boolean(name, value)?,
            "keywordcapitals" => {
                self.capitals = Some(boolean(name, value)?);
                self.keyword_style();
            }
            "keywordlowercase" => {
                self.lowercase = Some(boolean(name, value)?);
                self.keyword_style();
            }
            _ => return Ok(false),
        }
        Ok(true)
    }

    /// The style from the two keyword settings, as `qb64pe` decides it after reading all its options (worked out
    /// again from both at each one, so that a setting given twice counts by its last value): both the same (a missing
    /// one counts as neither) is CamelCase, else the one that is true; neither true leaves the style as it was before
    /// them.
    fn keyword_style(&mut self) {
        let before = *self.keywords_before.get_or_insert(self.keywords);
        self.keywords = if self.capitals == self.lowercase {
            KeywordStyle::Camel
        } else if self.capitals == Some(true) {
            KeywordStyle::Upper
        } else if self.lowercase == Some(true) {
            KeywordStyle::Lower
        } else {
            before
        };
    }
}

/// `ParseBooleanSetting&`: `true`, `on`, `yes`, `1`, `-1` and `false`, `off`, `no`, `0`, in any case.
fn boolean(name: &str, value: &str) -> Result<bool, String> {
    match value.to_ascii_lowercase().as_str() {
        "true" | "on" | "yes" | "1" | "-1" => Ok(true),
        "false" | "off" | "no" | "0" => Ok(false),
        _ => Err(format!("setting `{name}` needs `true` or `false`, not `{value}`")),
    }
}

/// `VAL` of a decimal number, as `ParseLongSetting&` reads a value: leading blanks, a sign, digits with a point and
/// an exponent; it stops at the first character that does not fit, and gives 0 when nothing does.
fn val(text: &str) -> f64 {
    let t = text.trim_start_matches([' ', '\t']).as_bytes();
    let mut end = 0;
    if t.first().is_some_and(|c| *c == b'+' || *c == b'-') {
        end = 1;
    }
    let digits = |from: usize| from + t[from..].iter().take_while(|c| c.is_ascii_digit()).count();
    end = digits(end);
    if t.get(end) == Some(&b'.') {
        end = digits(end + 1);
    }
    if t.get(end)
        .is_some_and(|c| matches!(c.to_ascii_uppercase(), b'E' | b'D'))
    {
        let mut e = end + 1;
        if t.get(e).is_some_and(|c| *c == b'+' || *c == b'-') {
            e += 1;
        }
        if t.get(e).is_some_and(u8::is_ascii_digit) {
            end = digits(e);
        }
    }
    // ASCII digits, signs, a point and an exponent letter: each byte is its character.
    let number: String = t[..end]
        .iter()
        .map(|&b| if b == b'd' || b == b'D' { 'e' } else { char::from(b) })
        .collect();
    number.parse().unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with(pairs: &[(&str, &str)]) -> Result<Settings, String> {
        let mut s = Settings::default();
        for (n, v) in pairs {
            assert!(s.set(n, v)?, "{n} is a layout setting");
        }
        Ok(s)
    }

    #[test]
    fn defaults_are_qb64pes_built_in_ones() {
        let s = Settings::default();
        assert!(s.auto_indent && s.indent_subs && s.auto_layout);
        assert_eq!(s.indent_size, 4);
        assert_eq!(s.keywords, KeywordStyle::Camel);
    }

    #[test]
    fn each_boolean_setting() {
        let s = with(&[("AutoIndent", "false"), ("IndentSubs", "0"), ("AutoLayout", "OFF")]).unwrap();
        assert!(!s.auto_indent && !s.indent_subs && !s.auto_layout);
        let s = with(&[("autoindent", "yes"), ("indentsubs", "-1"), ("autolayout", "On")]).unwrap();
        assert!(s.auto_indent && s.indent_subs && s.auto_layout);
    }

    #[test]
    fn a_bad_boolean_is_an_error() {
        let e = with(&[("AutoLayout", "maybe")]).unwrap_err();
        assert!(e.contains("AutoLayout") && e.contains("maybe"), "{e}");
    }

    #[test]
    fn indent_size_range() {
        assert_eq!(with(&[("AutoIndentSize", "2")]).unwrap().indent_size, 2);
        assert_eq!(with(&[("AutoIndentSize", "64")]).unwrap().indent_size, 64);
        assert_eq!(with(&[("AutoIndentSize", " 3x")]).unwrap().indent_size, 3);
        for bad in ["65", "0", "", "x", "-1"] {
            let e = with(&[("AutoIndentSize", bad)]).unwrap_err();
            assert!(e.contains("range 1 to 64"), "{bad}: {e}");
        }
    }

    #[test]
    fn keyword_settings() {
        assert_eq!(with(&[("KeywordCapitals", "1")]).unwrap().keywords, KeywordStyle::Upper);
        assert_eq!(
            with(&[("KeywordLowercase", "true")]).unwrap().keywords,
            KeywordStyle::Lower
        );
        let both = [("KeywordCapitals", "true"), ("KeywordLowercase", "true")];
        assert_eq!(with(&both).unwrap().keywords, KeywordStyle::Camel);
        let neither = [("KeywordCapitals", "0"), ("KeywordLowercase", "0")];
        assert_eq!(with(&neither).unwrap().keywords, KeywordStyle::Camel);
        // One false and the other not given changes nothing.
        assert_eq!(with(&[("KeywordCapitals", "0")]).unwrap().keywords, KeywordStyle::Camel);
        let lower_then_off = [("KeywordLowercase", "1"), ("KeywordCapitals", "0")];
        assert_eq!(with(&lower_then_off).unwrap().keywords, KeywordStyle::Lower);
        // A setting given twice counts by its last value (measured with `qb64pe -y`: `print 1` stays `Print 1`).
        let twice = [("KeywordCapitals", "1"), ("KeywordCapitals", "0")];
        assert_eq!(with(&twice).unwrap().keywords, KeywordStyle::Camel);
        let twice = [("KeywordLowercase", "1"), ("KeywordLowercase", "0")];
        assert_eq!(with(&twice).unwrap().keywords, KeywordStyle::Camel);
        let three = [
            ("KeywordCapitals", "1"),
            ("KeywordLowercase", "0"),
            ("KeywordCapitals", "0"),
        ];
        assert_eq!(with(&three).unwrap().keywords, KeywordStyle::Camel);
    }

    #[test]
    fn other_names_are_not_layout_settings() {
        let mut s = Settings::default();
        assert_eq!(s.set("OptimizeCppProgram", "true"), Ok(false));
        assert_eq!(s, Settings::default());
    }
}
