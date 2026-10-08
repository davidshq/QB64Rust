//! The preprocessor: `$IF`, `$ELSEIF`, `$ELSE`, `$END IF`, `$LET` and `$ERROR` (design D8 of `m2-parser-breadth`),
//! evaluated in the parser in file order, as the old compiler's prepass does (`qb64pe.bas` 1836–1892 and
//! 2031–2069; the condition is `EvalPreIF`, 28352–28536; `$LET` is `SetPreLET`, 28337; versions are compared by
//! `CompareVersions`, 28778). The target is Windows, 64-bit, version 4.7.0. Measured with `qb64pe.exe`
//! (`verification\v16_m6_*`, `study\00` §5): the predefined names, `$LET WIN = 0` not overriding `WIN`, `NOT` and
//! two-word values being errors, `(A = 1)` and `DEFINED(A)` being silently false.
//!
//! The evaluator is a port, quirks included: it splits the condition at blanks, compares text (upper case), and
//! treats a value as a number only when it prints back the same (`VerifyNumber`).

use qb64rust_base::show_bytes;

/// The version the predefined name `VERSION` holds (the old compiler's `Version$`).
const VERSION: &[u8] = b"4.7.0-GLFW";

/// Names the old compiler sets from the whole program before its passes (`qb64pe.bas` 1713–1722,
/// `setPrecompFlags`: `OPTION _EXPLICIT`, `$ASSERTS`, `$CONSOLE`, `$DEBUG`, network functions), recompiling until
/// they are stable. Not known here yet: a condition naming one is "not supported yet", never evaluated with a
/// guessed value (found by upstream `precomp-flags/consoleonly`, whose output it changed).
const PRECOMPILER_FLAGS: [&str; 6] = [
    "_EXPLICIT_",
    "_EXPLICITARRAY_",
    "_ASSERTS_",
    "_CONSOLE_",
    "_DEBUG_",
    "_SOCKETS_",
];

/// The first precompiler flag a `$IF` condition names (upper case text), if any.
pub fn precompiler_flag(cond: &[u8]) -> Option<&'static str> {
    cond.split(|&b| !(b.is_ascii_alphanumeric() || b == b'_'))
        .find_map(|w| PRECOMPILER_FLAGS.iter().find(|f| f.as_bytes() == w).copied())
}

/// What a metacommand line is for the preprocessor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Directive {
    /// `$IF cond THEN` (the condition text, upper case).
    If(Vec<u8>),
    /// `$ELSEIF cond THEN` or `$ELSE IF cond THEN`.
    ElseIf(Vec<u8>),
    Else,
    /// `$END IF` or `$ENDIF`.
    EndIf,
    /// `$LET name = value` (the text after `$LET`, upper case).
    Let(Vec<u8>),
    /// `$ERROR text` (the text, upper case, as the old compiler shows it).
    Error(Vec<u8>),
    /// `$IF` or `$ELSE IF` without its final `THEN`: an error (the message).
    Malformed(&'static str),
    /// Any other metacommand.
    Other,
}

/// Classifies a `Metacommand` token's text (`$` to the line end) as the old compiler does: blanks trimmed, upper
/// case, then matched by prefix.
pub fn directive(text: &[u8]) -> Directive {
    let t = trim(&text.to_ascii_uppercase()).to_vec();
    if t == b"$END IF" || t == b"$ENDIF" {
        return Directive::EndIf;
    }
    if let Some(rest) = t.strip_prefix(b"$IF ") {
        return match condition(rest) {
            Some(c) => Directive::If(c),
            None => Directive::Malformed("`$IF` without `THEN`"),
        };
    }
    if t == b"$ELSE" {
        return Directive::Else;
    }
    if let Some(rest) = t.strip_prefix(b"$ELSE") {
        let rest = ltrim(rest);
        if let Some(rest) = rest.strip_prefix(b"IF ") {
            return match condition(rest) {
                Some(c) => Directive::ElseIf(c),
                None => Directive::Malformed("`$ELSEIF` without `THEN`"),
            };
        }
    }
    if let Some(rest) = t.strip_prefix(b"$LET ") {
        return Directive::Let(ltrim(rest).to_vec());
    }
    if let Some(rest) = t.strip_prefix(b"$ERROR ") {
        return Directive::Error(ltrim(rest).to_vec());
    }
    Directive::Other
}

/// The condition between `$IF` and the final ` THEN`; `None` without it. `rest` starts with the blank after `IF`
/// (the old compiler checks `RIGHT$(…, 5) = " THEN"` on the whole line, so the blank before `THEN` may be the one
/// after `IF`: `$IF THEN` has an empty condition).
fn condition(rest: &[u8]) -> Option<Vec<u8>> {
    let whole = [b" ".as_slice(), rest].concat();
    if !whole.ends_with(b" THEN") {
        return None;
    }
    let inner = ltrim(&whole);
    Some(rtrim(&inner[..inner.len() - 4]).to_vec())
}

/// One `$IF` level (the old compiler's `ExecLevel` and `DefineElse`).
#[derive(Clone, Copy, Debug)]
struct Level {
    /// Code at this level is skipped.
    skip: bool,
    /// An `$ELSE` was seen.
    has_else: bool,
    /// A branch of this `$IF` was taken.
    taken: bool,
}

/// The preprocessor state: the `$LET` names and the open `$IF` levels. It flows through included files (a `$LET`
/// in an included file reaches the including one).
#[derive(Clone, Debug)]
pub struct PpState {
    /// Name and value, the predefined ones first (the old compiler's `UserDefine`); index 7 is `VERSION`.
    defines: Vec<(Vec<u8>, Vec<u8>)>,
    levels: Vec<Level>,
    /// The fewest open levels since [`Self::start_tracking`] (an included file may close a `$IF` of the
    /// including one).
    lowest: usize,
}

/// How many entries of `defines` are predefined (`UserDefineCountPresets` + 1).
const PRESETS: usize = 10;

impl Default for PpState {
    fn default() -> Self {
        let d = |n: &[u8], v: &[u8]| (n.to_vec(), v.to_vec());
        PpState {
            defines: vec![
                d(b"WINDOWS", b"-1"),
                d(b"WIN", b"-1"),
                d(b"LINUX", b"0"),
                d(b"MAC", b"0"),
                d(b"MACOSX", b"0"),
                d(b"32BIT", b"0"),
                d(b"64BIT", b"-1"),
                d(b"VERSION", VERSION),
                d(b"_QB64PE_", b"-1"),
                d(b"_ARM_", b"0"),
            ],
            levels: Vec::new(),
            lowest: 0,
        }
    }
}

impl PpState {
    /// Code at the current place is compiled (no enclosing `$IF` branch is skipped).
    pub fn active(&self) -> bool {
        self.levels.last().is_none_or(|l| !l.skip)
    }

    /// How many `$IF`s are open.
    pub fn depth(&self) -> usize {
        self.levels.len()
    }

    fn parent_skip(&self) -> bool {
        let n = self.levels.len();
        n >= 2 && self.levels[n - 2].skip
    }

    /// `$IF cond THEN`: opens a level.
    pub fn open_if(&mut self, cond: &[u8]) -> Result<(), String> {
        let parent_skip = !self.active();
        let result = self.eval(cond)?;
        let take = result && !parent_skip;
        self.levels.push(Level {
            skip: !take,
            has_else: false,
            taken: take,
        });
        Ok(())
    }

    /// A `$IF` line with an error (already reported): a level whose branch is taken, so that the code inside is
    /// still checked and its `$ELSE` branches are skipped.
    pub fn open_broken(&mut self) {
        let take = self.active();
        self.levels.push(Level {
            skip: !take,
            has_else: false,
            taken: true,
        });
    }

    /// `$ELSEIF cond THEN`.
    pub fn else_if(&mut self, cond: &[u8]) -> Result<(), String> {
        let parent_skip = self.parent_skip();
        let Some(level) = self.levels.last().copied() else {
            return Err("`$ELSEIF` without `$IF`".into());
        };
        if level.has_else {
            return Err("`$ELSEIF` cannot follow `$ELSE`".into());
        }
        let take = if level.taken {
            false
        } else {
            self.eval(cond)? && !parent_skip
        };
        let top = self.levels.last_mut().unwrap();
        top.skip = !take;
        top.taken |= take;
        Ok(())
    }

    /// `$ELSE`.
    pub fn else_(&mut self) -> Result<(), String> {
        let parent_skip = self.parent_skip();
        let Some(top) = self.levels.last_mut() else {
            return Err("`$ELSE` without `$IF`".into());
        };
        if top.has_else {
            return Err("this `$IF` already has an `$ELSE`".into());
        }
        top.has_else = true;
        top.skip = top.taken || parent_skip;
        Ok(())
    }

    /// `$END IF`.
    pub fn end_if(&mut self) -> Result<(), String> {
        match self.levels.pop() {
            Some(_) => {
                self.lowest = self.lowest.min(self.levels.len());
                Ok(())
            }
            None => Err("`$END IF` without `$IF`".into()),
        }
    }

    /// Starts tracking the fewest open levels from here on (an included file is about to be parsed); returns the
    /// tracking state to hand to [`Self::stop_tracking`].
    pub fn start_tracking(&mut self) -> usize {
        std::mem::replace(&mut self.lowest, self.levels.len())
    }

    /// Ends the tracking begun by [`Self::start_tracking`] (which returned `saved`): the fewest open levels since
    /// then. The outer tracking goes on, and sees that low point too.
    pub fn stop_tracking(&mut self, saved: usize) -> usize {
        let low = self.lowest;
        self.lowest = saved.min(low);
        low
    }

    /// `$LET name = value` (the text after `$LET`), with the prepass's checks (`qb64pe.bas` 2037–2068): a valid
    /// name; the value may be quoted, may start with `-`, and holds only `.`, digits, `:`–`@` and upper-case
    /// letters; its blanks are dropped. A predefined name is not overridden (a second entry is added).
    pub fn let_(&mut self, text: &[u8]) -> Result<(), String> {
        let Some(eq) = text.iter().position(|&b| b == b'=') else {
            return Err("expected `$LET name = value`".into());
        };
        let name = rtrim(&text[..eq]).to_vec();
        if !valid_flag_name(&name) {
            return Err(format!("invalid `$LET` name `{}`", show_bytes(&name)));
        }
        let mut r = ltrim(&text[eq + 1..]);
        if let Some(rest) = r.strip_prefix(b"\"") {
            r = ltrim(rest);
        }
        if let Some(rest) = r.strip_suffix(b"\"") {
            r = rtrim(rest);
        }
        let mut value = Vec::new();
        if let Some(rest) = r.strip_prefix(b"-") {
            value.push(b'-');
            r = ltrim(rest);
        }
        for &b in r {
            match b {
                b' ' => {}
                b'.' => value.push(b'.'),
                b if !(48..=90).contains(&b) => {
                    return Err(format!("invalid `$LET` value `{}`", show_bytes(r)));
                }
                b => value.push(b),
            }
        }
        match self.defines[PRESETS..].iter_mut().find(|(n, _)| *n == name) {
            Some(entry) => entry.1 = value,
            None => self.defines.push((name, value)),
        }
        Ok(())
    }

    /// The truth of a name: a number other than 0, or the first defined entry of that name whose value is not `0`
    /// or empty.
    fn truth(&self, word: &[u8]) -> bool {
        if is_number(word) {
            return parse_val(word) != 0.0;
        }
        self.defines.iter().any(|(n, v)| {
            n == word && {
                let v = trim(v);
                v != b"0" && !v.is_empty()
            }
        })
    }

    /// `EvalPreIF`: the condition's truth, or the old compiler's error.
    pub fn eval(&self, text: &[u8]) -> Result<bool, String> {
        let mut temp = text.to_vec();
        // Comparisons first, leftmost first, each replaced by ` -1 ` or ` 0 `.
        while let Some(first) = temp.iter().position(|b| b"=<>".contains(b)) {
            let first_sym = temp[first];
            let mut second = None;
            for (i, &c) in temp.iter().enumerate().skip(first + 1) {
                match c {
                    b' ' => {}
                    b'=' | b'<' | b'>' => {
                        if c == first_sym {
                            return Err(format!("duplicate operator `{}` in `$IF`", c as char));
                        }
                        second = Some(i);
                    }
                    _ => break,
                }
            }
            let l_full = rtrim(&temp[..first]).to_vec();
            let r_full = ltrim(&temp[second.unwrap_or(first) + 1..]).to_vec();
            let mut symbol = vec![first_sym];
            if let Some(s) = second {
                symbol.push(temp[s]);
            }
            // The word left of the operator, and what is left of it.
            let space = l_full.iter().rposition(|&b| b == b' ').map_or(0, |i| i + 1);
            let leftside = rtrim(&temp[..space]).to_vec();
            let l = trim(&l_full[space..]).to_vec();
            let space = r_full.iter().position(|&b| b == b' ').unwrap_or(r_full.len());
            let rightside = ltrim(r_full.get(space + 1..).unwrap_or_default()).to_vec();
            let r = trim(&r_full[..space]).to_vec();
            let symbol: &[u8] = match symbol.as_slice() {
                b"=<" => b"<=",
                b"=>" => b">=",
                b"><" => b"<>",
                s => s,
            };
            let result = self.compare(&l, symbol, &r);
            temp = [leftside.as_slice(), if result { b" -1 " } else { b" 0 " }, &rightside].concat();
        }
        // Then `AND`, `OR`, `XOR`, left to right, each operand a single word.
        loop {
            let ops: [&[u8]; 3] = [b" AND ", b" OR ", b" XOR "];
            let found = ops
                .iter()
                .filter_map(|op| find(&temp, op).map(|at| (at, *op)))
                .min_by_key(|&(at, _)| at);
            let Some((at, op)) = found else {
                break;
            };
            let leftside = rtrim(&temp[..at]).to_vec();
            let t = &temp[at + op.len()..];
            let (m, rightside) = match t.iter().position(|&b| b == b' ') {
                Some(sp) => (trim(&t[..sp]).to_vec(), ltrim(&t[sp..]).to_vec()),
                None => (ltrim(t).to_vec(), Vec::new()),
            };
            let (a, b) = (self.truth(trim(&leftside)), self.truth(&m));
            let result = match op {
                b" AND " => a && b,
                b" OR " => a || b,
                _ => a != b,
            };
            temp = [if result { b" -1 ".as_slice() } else { b" 0 " }, &rightside].concat();
        }
        let temp = trim(&temp);
        if is_number(temp) {
            return Ok(parse_val(temp) != 0.0);
        }
        if temp.contains(&b' ') {
            return Err("cannot evaluate this `$IF` condition (one word or comparison expected per operand)".into());
        }
        Ok(self.truth(temp))
    }

    /// One comparison `l symbol r` (`symbol` is `=`, `<>`, `<`, `>`, `<=` or `>=`).
    fn compare(&self, l: &[u8], symbol: &[u8], r: &[u8]) -> bool {
        let others = || self.defines.iter().enumerate().filter(|&(i, _)| i != 7).map(|(_, d)| d);
        if symbol == b"<>" && self.defines.iter().any(|(n, v)| n == l && v != r) {
            return true;
        }
        let version = (l == b"VERSION").then(|| compare_versions(VERSION, r));
        if symbol.contains(&b'=') {
            if version == Some(std::cmp::Ordering::Equal) {
                return true;
            }
            let mut found = false;
            for (n, v) in others() {
                if n == l {
                    found = true;
                    if v == r {
                        return true;
                    }
                }
            }
            if (!found && r == b"UNDEFINED") || (found && r == b"DEFINED") {
                return true;
            }
        }
        for (want, ord) in [(b'>', std::cmp::Ordering::Greater), (b'<', std::cmp::Ordering::Less)] {
            if !symbol.contains(&want) {
                continue;
            }
            if version == Some(ord) {
                return true;
            }
            for (n, v) in others() {
                if n != l {
                    continue;
                }
                let o = if is_number(r) && is_number(v) {
                    parse_val(v).partial_cmp(&parse_val(r))
                } else {
                    Some(v.as_slice().cmp(r))
                };
                if o == Some(ord) {
                    return true;
                }
            }
        }
        false
    }
}

/// `CompareVersions`: dot-separated numbers compared one by one; a `-UNKNOWN` suffix is ignored.
fn compare_versions(a: &[u8], b: &[u8]) -> std::cmp::Ordering {
    use std::cmp::Ordering::*;
    let strip = |s: &[u8]| s.strip_suffix(b"-UNKNOWN").unwrap_or(s).to_vec();
    let (mut t, mut t1) = (strip(a), strip(b));
    loop {
        let take = |s: &mut Vec<u8>| -> i64 {
            match s.iter().position(|&c| c == b'.') {
                Some(i) => {
                    let v = int_val(&s[..i]);
                    s.drain(..=i);
                    v
                }
                None => {
                    let v = int_val(s);
                    s.clear();
                    v
                }
            }
        };
        let (v, v1) = (take(&mut t), take(&mut t1));
        match v.cmp(&v1) {
            Equal => {}
            o => return o,
        }
        match (t.is_empty(), t1.is_empty()) {
            (true, true) => return Equal,
            (true, false) => return Less,
            (false, true) => return Greater,
            (false, false) => {}
        }
    }
}

/// BASIC's `VAL` on the leading number of `s` (blanks skipped, `0` when there is none).
fn parse_val(s: &[u8]) -> f64 {
    let s: Vec<u8> = s.iter().copied().filter(|&b| b != b' ').collect();
    let mut end = 0;
    if matches!(s.first(), Some(b'-' | b'+')) {
        end = 1;
    }
    while end < s.len() && (s[end].is_ascii_digit() || s[end] == b'.') {
        end += 1;
    }
    // ASCII only: a sign, digits and dots.
    let text: String = s[..end].iter().map(|&b| char::from(b)).collect();
    text.parse().unwrap_or(0.0)
}

/// `VAL` of one part of a version into a `LONG`: its leading digits (a part has no dot), `0` when there are none.
fn int_val(s: &[u8]) -> i64 {
    let s = ltrim(s);
    let (neg, digits) = match s.first() {
        Some(b'-') => (true, &s[1..]),
        Some(b'+') => (false, &s[1..]),
        _ => (false, s),
    };
    let v = digits
        .iter()
        .take_while(|b| b.is_ascii_digit())
        .fold(0i64, |v, &b| v.saturating_mul(10).saturating_add(i64::from(b - b'0')));
    if neg { -v } else { v }
}

/// `VerifyNumber`: the text prints back the same as a number. Approximated as a plain decimal without leading or
/// trailing zeros (`5`, `-1`, `0.5`), which is what the inputs use.
fn is_number(s: &[u8]) -> bool {
    let s = trim(s);
    let s = s.strip_prefix(b"-").unwrap_or(s);
    let (int, frac) = match s.iter().position(|&b| b == b'.') {
        Some(i) => (&s[..i], Some(&s[i + 1..])),
        None => (s, None),
    };
    let int_ok = !int.is_empty() && int.iter().all(u8::is_ascii_digit) && (int == b"0" || int[0] != b'0');
    let frac_ok = frac.is_none_or(|f| !f.is_empty() && f.iter().all(u8::is_ascii_digit) && f.last() != Some(&b'0'));
    int_ok && frac_ok
}

/// `validname` for a `$LET` name: letters, digits, `_` and `.`; at least one letter, no digit before the first
/// letter, no trailing `_`, at most 40 characters, not a single leading `_`.
fn valid_flag_name(n: &[u8]) -> bool {
    if n.is_empty() || n.len() > 40 {
        return false;
    }
    if n.len() >= 2 && n[0] == b'_' && n[1] != b'_' {
        return false;
    }
    if !n.iter().all(|&b| b.is_ascii_alphanumeric() || b == b'_' || b == b'.') || n.last() == Some(&b'_') {
        return false;
    }
    match n.iter().position(u8::is_ascii_alphabetic) {
        Some(i) => !n[..i].iter().any(u8::is_ascii_digit),
        None => false,
    }
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

fn ltrim(s: &[u8]) -> &[u8] {
    let start = s.iter().position(|&b| b != b' ').unwrap_or(s.len());
    &s[start..]
}

fn rtrim(s: &[u8]) -> &[u8] {
    let end = s.iter().rposition(|&b| b != b' ').map_or(0, |i| i + 1);
    &s[..end]
}

fn trim(s: &[u8]) -> &[u8] {
    ltrim(rtrim(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(cond: &str) -> Result<bool, String> {
        PpState::default().eval(cond.as_bytes())
    }

    #[test]
    fn directives() {
        assert_eq!(directive(b"$if win then"), Directive::If(b"WIN".to_vec()));
        assert_eq!(directive(b"  $IF  A = 1   THEN  "), Directive::If(b"A = 1".to_vec()));
        assert_eq!(directive(b"$IF THEN"), Directive::If(Vec::new()));
        assert_eq!(directive(b"$IF WIN"), Directive::Malformed("`$IF` without `THEN`"));
        assert_eq!(directive(b"$ELSEIF MAC THEN"), Directive::ElseIf(b"MAC".to_vec()));
        assert_eq!(directive(b"$ELSE IF MAC THEN"), Directive::ElseIf(b"MAC".to_vec()));
        assert_eq!(directive(b"$Else"), Directive::Else);
        assert_eq!(directive(b"$END IF"), Directive::EndIf);
        assert_eq!(directive(b"$endif"), Directive::EndIf);
        assert_eq!(directive(b"$LET x = 1"), Directive::Let(b"X = 1".to_vec()));
        assert_eq!(directive(b"$ERROR too old"), Directive::Error(b"TOO OLD".to_vec()));
        assert_eq!(directive(b"$CONSOLE:ONLY"), Directive::Other);
    }

    #[test]
    fn predefined_names() {
        for t in ["WIN", "WINDOWS", "64BIT", "_QB64PE_", "VERSION"] {
            assert_eq!(eval(t), Ok(true), "{t}");
        }
        for f in ["LINUX", "MAC", "MACOSX", "32BIT", "_ARM_", "NOPE"] {
            assert_eq!(eval(f), Ok(false), "{f}");
        }
    }

    #[test]
    fn comparisons_and_operators() {
        assert_eq!(eval("WIN = -1"), Ok(true));
        assert_eq!(eval("WIN <> 0"), Ok(true));
        assert_eq!(eval("LINUX = 0"), Ok(true));
        assert_eq!(eval("VERSION >= 4.7.0"), Ok(true));
        assert_eq!(eval("VERSION => 4.7"), Ok(true));
        assert_eq!(eval("VERSION < 4.6.9"), Ok(false));
        assert_eq!(eval("VERSION > 4.6.9"), Ok(true));
        assert_eq!(eval("WIN AND 64BIT"), Ok(true));
        assert_eq!(eval("WIN AND LINUX"), Ok(false));
        assert_eq!(eval("LINUX OR MAC OR WIN"), Ok(true));
        assert_eq!(eval("WIN XOR 64BIT"), Ok(false));
        assert_eq!(eval("LINUX = 0 AND WIN = -1"), Ok(true));
        assert_eq!(eval("FOO = UNDEFINED"), Ok(true));
        assert_eq!(eval("WIN = DEFINED"), Ok(true));
        assert_eq!(eval("(WIN = -1)"), Ok(false));
        assert_eq!(eval("DEFINED(WIN)"), Ok(false));
        assert!(eval("NOT LINUX").is_err());
        assert!(eval("WIN == -1").is_err());
    }

    #[test]
    fn let_names() {
        let mut s = PpState::default();
        s.let_(b"A = 2").unwrap();
        assert_eq!(s.eval(b"A = 2"), Ok(true));
        assert_eq!(s.eval(b"A"), Ok(true));
        assert_eq!(s.eval(b"A > 1"), Ok(true));
        s.let_(b"A = 0").unwrap();
        assert_eq!(s.eval(b"A"), Ok(false));
        // A predefined name is not overridden: the bare-name test stops at the first true entry.
        s.let_(b"WIN = 0").unwrap();
        assert_eq!(s.eval(b"WIN"), Ok(true));
        s.let_(b"S = \"HELLO\"").unwrap();
        assert_eq!(s.eval(b"S = HELLO"), Ok(true));
        assert!(s.let_(b"E").is_err());
        assert!(s.let_(b"1A = 1").is_err());
        assert!(s.let_(b"B = X!").is_err());
    }

    #[test]
    fn precompiler_flags() {
        assert_eq!(precompiler_flag(b"_CONSOLE_ = 2"), Some("_CONSOLE_"));
        assert_eq!(precompiler_flag(b"WIN AND _DEBUG_"), Some("_DEBUG_"));
        assert_eq!(precompiler_flag(b"_CONSOLE_X"), None);
        assert_eq!(precompiler_flag(b"WIN"), None);
    }

    #[test]
    fn levels() {
        let mut s = PpState::default();
        s.open_if(b"LINUX").unwrap();
        assert!(!s.active());
        s.else_if(b"WIN").unwrap();
        assert!(s.active());
        s.else_if(b"64BIT").unwrap();
        assert!(!s.active(), "only the first true branch is taken");
        s.else_().unwrap();
        assert!(!s.active());
        assert!(s.else_().is_err());
        s.end_if().unwrap();
        assert!(s.active());
        assert!(s.end_if().is_err());
        // A true `$IF` inside a skipped branch stays skipped.
        s.open_if(b"LINUX").unwrap();
        s.open_if(b"WIN").unwrap();
        assert!(!s.active());
        s.end_if().unwrap();
        s.else_().unwrap();
        assert!(s.active());
    }

    #[test]
    fn lowest_depth_while_tracking() {
        let mut s = PpState::default();
        s.open_if(b"WIN").unwrap();
        s.open_if(b"WIN").unwrap();
        // An included file closes one level and opens another: the depth is unchanged, the low point is not.
        let saved = s.start_tracking();
        s.end_if().unwrap();
        s.open_if(b"WIN").unwrap();
        // A file it includes in turn closes one more.
        let inner = s.start_tracking();
        s.end_if().unwrap();
        assert_eq!(s.stop_tracking(inner), 1);
        assert_eq!(
            s.stop_tracking(saved),
            1,
            "the inner low point counts for the outer tracking"
        );
        assert_eq!(s.depth(), 1);
    }
}
