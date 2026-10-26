//! The final text of a line: a port of QB64pe's `apply_layout_indent$` (`source\utilities\format.bas`), step by
//! step, with the line of `format.bas` each step follows.
//!
//! The input is a line's layout in QB64pe's own form (design D2): the indent level as that many leading spaces,
//! then the elements joined by [`SP`] ("one space here") or [`SP2`] ("no space here"), with the keywords already
//! in the chosen case.

/// QB64pe's `sp` (`_CHR_CR`, `source\global\constants.bas`): one space.
pub const SP: u8 = b'\r';
/// QB64pe's `sp2` (`_CHR_LF`): no space.
pub const SP2: u8 = b'\n';

/// The settings `apply_layout_indent$` reads: `IDEAutoIndent`, `IDEAutoIndentSize` and `IDEAutoLayout` as they
/// stand at the line (`'$FORMAT:OFF` turns both off).
#[derive(Clone, Copy, Debug)]
pub struct Apply {
    pub auto_indent: bool,
    pub indent_size: u32,
    pub auto_layout: bool,
}

/// The line as `-y` writes it, or `None` where `apply_layout_indent$` returns nothing and the line is written as
/// read (`original`): an empty layout, or one that comes to no text.
pub fn apply_layout_indent(layout: &[u8], original: &[u8], a: Apply) -> Option<Vec<u8>> {
    // `IF LEN(layout2$) THEN` (line 6).
    if layout.is_empty() {
        return None;
    }
    // The indent level, from the leading spaces (lines 9-20): a layout of spaces only leaves nothing.
    let (indent, rest) = match layout.iter().position(|&c| c != b' ') {
        Some(i) => (i, &layout[i..]),
        None => (layout.len(), &layout[..0]),
    };
    // `sp` to a space and `sp2` to nothing, outside strings (lines 22-35). A quote toggles `ignoresp` (two
    // quotes end it), so the separators inside a string stay.
    let mut laid = Vec::with_capacity(rest.len());
    let mut in_string = false;
    for &c in rest {
        if c == b'"' {
            in_string = !in_string;
        }
        if !in_string && c == SP {
            laid.push(b' ');
        } else if in_string || c != SP2 {
            laid.push(c);
        }
    }
    // The indent (lines 37-51): with `AutoIndent` off, the original line's own (its leading spaces; a line of
    // spaces only counts one less than its length).
    let indent = if a.auto_indent {
        indent * a.indent_size as usize
    } else {
        original
            .iter()
            .position(|&c| c != b' ')
            .unwrap_or(original.len().saturating_sub(1))
    };
    // With `AutoLayout` off, the original line after its leading spaces (lines 53-63); a line of spaces only keeps
    // its last one.
    let olay = laid;
    let mut text = if a.auto_layout {
        olay.clone()
    } else {
        match original.iter().position(|&c| c != b' ') {
            Some(i) => original[i..].to_vec(),
            None => original.last().map(|_| vec![b' ']).unwrap_or_default(),
        }
    };
    if text != olay {
        // QB64pe's walk keeps in step only while the layout differs from the user's text by spacing, letter case,
        // `?` and a closing quote. Where the layout changed more (`1 + +2` laid out `1 + 2`), the walk goes out of
        // step and can change the program (`PRINT " three"` becoming `PRINT " thre""`): then the case is carried
        // only up to the first difference it cannot explain, and the rest is kept as written (D-039).
        let user = text.clone();
        carry_over(&mut text, &olay, false);
        let key = program_key(&text);
        if key != program_key(&olay) && key != program_key(&user) {
            text = user;
            carry_over(&mut text, &olay, true);
        }
    }
    // `IF LEN(layout2$) THEN apply_layout_indent$ = indent$ + layout2$` (lines 99-101).
    if text.is_empty() {
        return None;
    }
    let mut out = vec![b' '; indent];
    out.extend_from_slice(&text);
    Some(out)
}

/// Lines 65-97: walks the user's text (`layout2$`) beside the laid-out one (`olay$`), skipping spaces on either
/// side, and carries over the letter-case changes, with the two corrections of the old IDE: a `?` where the
/// layout has `PRINT`, and a closing quote the user left out. With `stop_out_of_step`, the walk ends at the first
/// difference none of these explains, where QB64pe goes on as if one character stood for the other (D-039).
fn carry_over(text: &mut Vec<u8>, olay: &[u8], stop_out_of_step: bool) {
    // MID$ of a 1-based position, empty past the end.
    let at = |s: &[u8], i: usize| -> Option<u8> { i.checked_sub(1).and_then(|i| s.get(i).copied()) };
    let (mut lcnt, mut ocnt) = (0usize, 0usize);
    let (mut lch, mut och): (Option<u8>, Option<u8>) = (None, None);
    let (mut llch, mut loch): (Option<u8>, Option<u8>) = (None, None);
    // `WHILE lcnt <= LEN(layout2$)`.
    while lcnt <= text.len() {
        lcnt += 1;
        ocnt += 1;
        // `recheckdiff:`.
        loop {
            // The last character that is not a space, of each side.
            if lch.is_some_and(|c| c != b' ') {
                llch = lch;
            }
            if och.is_some_and(|c| c != b' ') {
                loch = och;
            }
            lch = at(text, lcnt);
            och = at(olay, ocnt);
            if lch == och {
                break; // no difference: `_CONTINUE`
            }
            if lch == Some(b' ') {
                lcnt += 1;
                if lcnt > text.len() && ocnt > olay.len() {
                    return;
                }
                continue;
            }
            if och == Some(b' ') {
                ocnt += 1;
                if ocnt > olay.len() && lcnt > text.len() {
                    return;
                }
                continue;
            }
            if lch == Some(b'?')
                && olay
                    .get(ocnt - 1..)
                    .is_some_and(|s| s.len() >= 5 && s[..5].eq_ignore_ascii_case(b"PRINT"))
            {
                // `? = PRINT special case`: lower case, then a space unless one follows.
                let mut ps = b"print".to_vec();
                if at(text, lcnt + 1).is_some_and(|c| c != b' ') {
                    ps.push(b' ');
                }
                text.splice(lcnt - 1..lcnt, ps);
                continue;
            }
            if och == Some(b'"') && llch == loch {
                // The closing quote added: `LEFT$(layout2$, lcnt - 1) + CHR$(34) + RIGHT$(layout2$, LEN(layout2$) -
                // lcnt)` puts it in place of the character at `lcnt`, after the text when `lcnt` is past its end.
                let end = lcnt.min(text.len());
                text.splice((lcnt - 1).min(end)..end, *b"\"");
                continue;
            }
            // `KW case diff`: two letters that differ only by case take the layout's.
            if let (Some(l), Some(o)) = (lch, och)
                && l.is_ascii_alphabetic()
                && o.is_ascii_alphabetic()
                && l.abs_diff(o) == 32
            {
                text[lcnt - 1] = o;
            } else if stop_out_of_step {
                return;
            }
            break;
        }
    }
}

/// A line's text as the program reads it, for D-039: spaces and tabs outside strings left out and letters outside
/// strings in capitals; a string's text as it is.
fn program_key(text: &[u8]) -> Vec<u8> {
    let mut key = Vec::with_capacity(text.len());
    let mut in_string = false;
    for &c in text {
        if c == b'"' {
            in_string = !in_string;
        }
        if in_string {
            key.push(c);
        } else if c != b' ' && c != b'\t' {
            key.push(c.to_ascii_uppercase());
        }
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    const ON: Apply = Apply {
        auto_indent: true,
        indent_size: 4,
        auto_layout: true,
    };

    /// A layout written with `|` for `sp` and `~` for `sp2`.
    fn lay(s: &str) -> Vec<u8> {
        s.bytes()
            .map(|c| match c {
                b'|' => SP,
                b'~' => SP2,
                c => c,
            })
            .collect()
    }

    #[expect(clippy::disallowed_methods, reason = "the test's texts are ASCII")]
    fn apply(layout: &str, original: &str, a: Apply) -> Option<String> {
        apply_layout_indent(&lay(layout), original.as_bytes(), a).map(|v| String::from_utf8(v).unwrap())
    }

    #[test]
    fn separators_outside_strings() {
        // Lines 22-35: `sp` is a space, `sp2` nothing.
        assert_eq!(apply("Print|x~;", "print x;", ON).as_deref(), Some("Print x;"));
    }

    #[test]
    fn separators_inside_strings_stay() {
        let got = apply("Print|\"a|b~c\"~;", "", ON).unwrap();
        assert_eq!(got.as_bytes(), b"Print \"a\rb\nc\";");
    }

    #[test]
    fn indent_from_leading_spaces() {
        // Lines 9-20 and 51: two levels at four spaces.
        assert_eq!(apply("  Print|1", "print 1", ON).as_deref(), Some("        Print 1"));
        let three = Apply { indent_size: 3, ..ON };
        assert_eq!(apply(" Next", "next", three).as_deref(), Some("   Next"));
    }

    #[test]
    fn empty_layout_is_the_line_as_read() {
        // Line 6.
        assert_eq!(apply("", "  print   1", ON), None);
        // A layout of spaces only comes to nothing (lines 11-12, 99).
        assert_eq!(apply("   ", "x", ON), None);
    }

    #[test]
    fn auto_indent_off_keeps_the_old_indent() {
        // Lines 37-48.
        let a = Apply {
            auto_indent: false,
            ..ON
        };
        assert_eq!(apply("  Print|1", "   print 1", a).as_deref(), Some("   Print 1"));
        assert_eq!(apply("Print|1", "print 1", a).as_deref(), Some("Print 1"));
    }

    #[test]
    fn auto_layout_off_keeps_spacing_and_carries_case() {
        // Lines 53-97: the user's spacing, the layout's letter case.
        let a = Apply {
            auto_layout: false,
            ..ON
        };
        assert_eq!(
            apply("If|x|=|1|Then", "if x   =   1 then", a).as_deref(),
            Some("If x   =   1 Then")
        );
        assert_eq!(
            apply(" Print|myVar", "    PRINT   MYVAR", a).as_deref(),
            Some("    Print   myVar")
        );
    }

    #[test]
    fn question_mark_becomes_print() {
        // Lines 82-87, with the layout off: `?` before a name gets `print` and a space.
        let a = Apply {
            auto_layout: false,
            ..ON
        };
        assert_eq!(apply("Print|x", "?x", a).as_deref(), Some("Print x"));
        assert_eq!(apply("Print|x", "? x", a).as_deref(), Some("Print x"));
    }

    #[test]
    fn closing_quote_is_added() {
        // Lines 88-91: the layout has the closing quote the user left out.
        let a = Apply {
            auto_layout: false,
            ..ON
        };
        assert_eq!(
            apply("Print|\"abc\"", "print \"abc", a).as_deref(),
            Some("Print \"abc\"")
        );
    }

    #[test]
    fn a_walk_out_of_step_does_not_change_the_program() {
        // D-039: the layout folded `+ +2` to `+ 2`, so QB64pe's walk slips by one and writes `PRINT " thre"": END
        // SELECT` (`tests\format\settings\autolayout=0\corpus\slice\s17_signs.bas`, line 26); the case is carried up
        // to the slip and the rest kept as written.
        let a = Apply {
            auto_layout: false,
            ..ON
        };
        assert_eq!(
            apply(
                "Select|Case|x~:|Case|1|+|2~:|Print|\" three\"~:|End|Select",
                "SELECT CASE x: CASE 1 + +2: PRINT \" three\": END SELECT",
                a
            )
            .as_deref(),
            Some("Select Case x: Case 1 + +2: PRINT \" three\": END SELECT")
        );
        // QB64pe's own walk, as ported, gives the recorded line.
        let mut text = b"SELECT CASE x: CASE 1 + +2: PRINT \" three\": END SELECT".to_vec();
        carry_over(
            &mut text,
            b"Select Case x: Case 1 + 2: Print \" three\": End Select",
            false,
        );
        assert_eq!(text, b"Select Case x: Case 1 + +2: PRINT \" thre\"\": END SELECT");
        // Where the walk slips but keeps the program (a tab where the layout has a space), QB64pe's result stands.
        assert_eq!(apply("Print|x~;|y", "print\tx; y", a).as_deref(), Some("Print\tx; y"));
    }
}
