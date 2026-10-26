//! Metacommands (`qb64pe.bas` 3424-3902): the line in capitals compared with each metacommand, its layout the
//! literal spelling; `$IF` and `$ELSEIF` with their condition in capitals around its operator; `$EMBED`, `$EXEICON`,
//! `$USELIBRARY` and `$VERSIONINFO` with their values as written.

use crate::fmt::Fmt;
use crate::lay::Lay;
use crate::walk::{LineLevel, Pass};
use qb64rust_syntax::tree::Node;

/// The metacommands written as one literal (`IF a3u$ = "$CONSOLE" THEN layout$ = SCase$("$Console")`).
const LITERALS: &[&str] = &[
    "$COLOR:0",
    "$COLOR:32",
    "$INCLUDEONCE",
    "$DEBUG",
    "$CHECKING:OFF",
    "$CHECKING:ON",
    "$ERRORLOCATION:ON",
    "$ERRORLOCATION:OFF",
    "$CONSOLE",
    "$CONSOLE:ONLY",
    "$ASSERTS",
    "$ASSERTS:CONSOLE",
    "$SCREENHIDE",
    "$SCREENSHOW",
    "$RESIZE:OFF",
    "$RESIZE:ON",
    "$RESIZE:STRETCH",
    "$RESIZE:SMOOTH",
    "$ELSE",
];

impl Fmt<'_> {
    /// The metacommand line, trimmed (`a3$`), and in capitals (`a3u$`).
    fn meta_text(&self, n: Node<'_>) -> (Vec<u8>, Vec<u8>) {
        let line = self.trimmed(n);
        let upper = line.to_ascii_uppercase();
        (line, upper)
    }

    /// What `$IF`, `$ELSEIF`, `$ELSE` and `$END IF` do to the level.
    pub(crate) fn meta_effect(&self, n: Node<'_>, pass: &mut Pass, level: &mut LineLevel) {
        let (_, u) = self.meta_text(n);
        meta_effect_of(&u, pass, level);
    }

    /// The layout of a metacommand, `None` for one whose handler sets none here.
    pub(crate) fn meta(&self, n: Node<'_>) -> Option<Lay> {
        let (a3, u) = self.meta_text(n);
        self.meta_line(&a3, &u)
    }

    /// The layout of a metacommand line `a3` (trimmed), `u` in capitals.
    pub(crate) fn meta_line(&self, a3: &[u8], u: &[u8]) -> Option<Lay> {
        let mut l = Lay::new();
        if u == b"$END IF" || u == b"$ENDIF" {
            l.text(&self.kw("$END IF"));
            return Some(l);
        }
        if let Some(rest) = u.strip_prefix(b"$IF ".as_slice()) {
            let cond = condition(rest, 4);
            l.text(&self.kw("$IF")).text(b" ");
            l.text(&if_condition(
                &cond,
                &["<=", "=<", ">=", "=>", "<>", "><", "=", ">", "<"],
            ));
            l.text(b" ").text(&self.kw("THEN"));
            return Some(l);
        }
        if let Some(w) = LITERALS.iter().find(|w| u == w.as_bytes()) {
            l.text(&self.kw(w));
            return Some(l);
        }
        if u.starts_with(b"$ELSE") {
            let rest = trim_start(&u[5..]);
            if let Some(cond) = rest.strip_prefix(b"IF ") {
                let cond = condition(trim_start(cond), 4);
                l.text(&self.kw("$ELSEIF")).text(b" ");
                l.text(&if_condition(&cond, &["="]));
                l.text(b" ").text(&self.kw("THEN"));
                return Some(l);
            }
        }
        if let Some(rest) = u.strip_prefix(b"$LET ") {
            let rest = trim_start(rest);
            let eq = rest.iter().position(|&c| c == b'=')?;
            l.text(&self.kw("$LET")).text(b" ");
            l.text(trim_end(&rest[..eq]))
                .text(b" = ")
                .text(trim_start(&rest[eq + 1..]));
            return Some(l);
        }
        if u.starts_with(b"$EXEICON") {
            let a = u.iter().position(|&c| c == b'\'')?;
            let b = a + 1 + u[a + 1..].iter().position(|&c| c == b'\'')?;
            l.text(&self.kw("$EXEICON:'"))
                .text(trim(&a3[a + 1..b]))
                .text(b"'")
                .text(&a3[b + 1..]);
            return Some(l);
        }
        if u.starts_with(b"$USELIBRARY") {
            let a = u.iter().position(|&c| c == b'\'')?;
            let b = a + 1 + u[a + 1..].iter().position(|&c| c == b'\'')?;
            l.text(&self.kw("$USELIBRARY:'"))
                .text(trim(&a3[a + 1..b]))
                .text(b"'")
                .text(&a3[b + 1..]);
            return Some(l);
        }
        if u.starts_with(b"$EMBED") {
            let quotes: Vec<usize> = u
                .iter()
                .enumerate()
                .filter(|(_, c)| **c == b'\'')
                .map(|(i, _)| i)
                .collect();
            if quotes.len() < 4 {
                return None;
            }
            let (file, handle) = (trim(&a3[quotes[0] + 1..quotes[1]]), trim(&a3[quotes[2] + 1..quotes[3]]));
            l.text(&self.kw("$EMBED:'"))
                .text(file)
                .text(b"','")
                .text(handle)
                .text(b"'");
            l.text(&a3[quotes[3] + 1..]);
            return Some(l);
        }
        if u.starts_with(b"$VERSIONINFO") {
            let colon = u.iter().position(|&c| c == b':')?;
            let eq = colon + 1 + u[colon + 1..].iter().position(|&c| c == b'=')?;
            let key = trim(&u[colon + 1..eq]);
            let value: Vec<u8> = trim(&a3[eq + 1..])
                .iter()
                .map(|&c| if c == b'"' { b'\'' } else { c })
                .collect();
            let named = |k: &str| -> Option<&'static str> {
                [
                    "CompanyName",
                    "FileDescription",
                    "FileVersion",
                    "InternalName",
                    "LegalCopyright",
                    "LegalTrademarks",
                    "OriginalFilename",
                    "ProductName",
                    "ProductVersion",
                    "Comments",
                    "Web",
                ]
                .into_iter()
                .find(|n| n.eq_ignore_ascii_case(k))
            };
            match key {
                b"FILEVERSION#" => l.text(&self.kw("$VERSIONINFO:FILEVERSION#=")),
                b"PRODUCTVERSION#" => l.text(&self.kw("$VERSIONINFO:PRODUCTVERSION#=")),
                k => {
                    let name = named(&crate::fmt::ascii_upper(k))?;
                    l.text(&self.kw("$VERSIONINFO:")).text(name.as_bytes()).text(b"=")
                }
            };
            l.text(&value);
            return Some(l);
        }
        if let Some(rest) = u.strip_prefix(b"$UNSTABLE:") {
            l.text(&self.kw("$UNSTABLE:"));
            match trim(rest) {
                b"MIDI" => l.text(&self.kw("MIDI")),
                b"HTTP" => l.text(&self.kw("HTTP")),
                b"TYPEFIELDS" => l.text(&self.kw("TYPEFIELDS")),
                _ => &mut l,
            };
            return Some(l);
        }
        None
    }
}

/// What a metacommand line in capitals does to the level: `$IF` opens a level, `$ELSEIF` and `$ELSE` stand one
/// level out, `$END IF` closes it.
pub(crate) fn meta_effect_of(u: &[u8], pass: &mut Pass, level: &mut LineLevel) {
    if u == b"$END IF" || u == b"$ENDIF" {
        pass.level -= 1;
    } else if u.starts_with(b"$IF ") {
        pass.level += 1;
    } else if u == b"$ELSE" || (u.starts_with(b"$ELSE") && trim_start(&u[5..]).starts_with(b"IF ")) {
        level.lhs -= 1;
    }
}

/// Whether a line in capitals is one the precompiler reads also inside a branch that is not taken (`qb64pe.bas`
/// 1833-1890): `$IF … THEN`, `$ELSE`, `$ELSEIF … THEN` and `$END IF`.
pub(crate) fn is_precompiler_line(u: &[u8]) -> bool {
    (u.starts_with(b"$IF ") && u.ends_with(b" THEN"))
        || u == b"$ELSE"
        || (u.starts_with(b"$ELSE") && trim_start(&u[5..]).starts_with(b"IF ") && u.ends_with(b" THEN"))
        || u == b"$END IF"
        || u == b"$ENDIF"
}

/// The condition of `$IF cond THEN`: the text without the `THEN` of `len` bytes at its end and the blanks.
fn condition(rest: &[u8], then_len: usize) -> Vec<u8> {
    let t = trim_start(rest);
    trim_end(&t[..t.len().saturating_sub(then_len)]).to_vec()
}

/// `l op r` with one blank around the first operator of `ops` found, in that order (`=<` written `<=`, as QB64pe
/// normalises it), or the condition as it is.
fn if_condition(cond: &[u8], ops: &[&str]) -> Vec<u8> {
    for op in ops {
        if let Some(at) = cond.windows(op.len()).position(|w| w == op.as_bytes()) {
            let shown: &[u8] = match *op {
                "=<" => b"<=",
                "=>" => b">=",
                "><" => b"<>",
                o => o.as_bytes(),
            };
            let mut out = trim_end(&cond[..at]).to_vec();
            out.push(b' ');
            out.extend_from_slice(shown);
            out.push(b' ');
            out.extend_from_slice(trim_start(&cond[at + op.len()..]));
            return out;
        }
    }
    cond.to_vec()
}

fn trim_start(b: &[u8]) -> &[u8] {
    let i = b.iter().position(|&c| c != b' ').unwrap_or(b.len());
    &b[i..]
}

fn trim_end(b: &[u8]) -> &[u8] {
    let i = b.iter().rposition(|&c| c != b' ').map_or(0, |e| e + 1);
    &b[..i]
}

fn trim(b: &[u8]) -> &[u8] {
    trim_end(trim_start(b))
}
