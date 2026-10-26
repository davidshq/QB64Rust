//! `textDocument/hover` (design D3 of `m5b-language-features`): a symbol's text from the semantic index, else a
//! built-in's form from the built-in table, else nothing. No help text: the QB64pe wiki has no licence.

use crate::index::Indexed;
use lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind};
use qb64rust_base::Span;
use qb64rust_builtins::{Builtin, Kind};
use std::fmt::Write as _;

/// The hover at `offset` of the program's main file.
pub fn hover(ix: &Indexed, offset: u32) -> Option<Hover> {
    let main = ix.analysis.parsed.main().file;
    let (text, span) = match ix.index.entry_near(main, offset) {
        Some((id, span)) => (ix.index.entries[id].detail.clone(), span),
        None => {
            let (span, word) = word_at(ix, offset)?;
            (builtin_text(&word)?, span)
        }
    };
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: text,
        }),
        range: Some(ix.analysis.range(span)),
    })
}

/// The word at `offset` (or just before it) of the main file: a name with its suffix, as the lexer reads one.
pub(crate) fn word_at(ix: &Indexed, offset: u32) -> Option<(Span, Vec<u8>)> {
    let main = ix.analysis.parsed.main();
    let bytes = &ix.analysis.map.file(main.file).bytes;
    let covers = |s: Span, at: u32| s.start <= at && at < s.end;
    let tokens = main.root().tokens();
    let tok = tokens
        .iter()
        .find(|t| covers(t.span, offset))
        .filter(|t| is_word(&bytes[t.span.start as usize..t.span.end as usize]))
        .or_else(|| {
            let before = offset.checked_sub(1)?;
            tokens.iter().find(|t| covers(t.span, before))
        })?;
    let text = &bytes[tok.span.start as usize..tok.span.end as usize];
    is_word(text).then(|| (tok.span, text.to_vec()))
}

/// A name, possibly with a type suffix: a letter or `_` first, then letters, digits, `_` and `.`, then suffix
/// characters.
fn is_word(text: &[u8]) -> bool {
    let end = name_end(text);
    end > 0
        && (text[0].is_ascii_alphabetic() || text[0] == b'_')
        && text[end..].iter().all(|b| b"$%&!#~`0123456789".contains(b))
}

/// The length of a name without its suffix.
pub(crate) fn name_end(text: &[u8]) -> usize {
    text.iter()
        .position(|b| !(b.is_ascii_alphanumeric() || *b == b'_' || *b == b'.'))
        .unwrap_or(text.len())
}

/// The forms of the built-in named `word` (suffix ignored), one per entry of the table, as Markdown; `None` when the
/// table has none.
pub fn builtin_text(word: &[u8]) -> Option<String> {
    let name = &word[..name_end(word)];
    let forms: Vec<String> = qb64rust_builtins::find_any(name).map(form).collect();
    if forms.is_empty() {
        return None;
    }
    let mut out = String::from("```qb64\n");
    for f in &forms {
        writeln!(out, "{f}").unwrap();
    }
    out.push_str("```\nBuilt-in.");
    Some(out)
}

/// One built-in's form: `Left$(STRING, LONG) AS STRING`, `Cls [LONG][,[_UNSIGNED LONG][,LONG]]`. With a
/// `specialformat`, each `?` is the next argument's type and each `{word}` a word written as it is; without one,
/// the arguments in order, an optional one in brackets.
pub fn form(b: &Builtin) -> String {
    let name = format!("{}{}", b.name, b.musthave.unwrap_or(""));
    let args = match b.specialformat {
        Some(f) => {
            let mut types = b.arg_types.iter();
            let mut out = String::new();
            for c in f.chars() {
                match c {
                    '?' => out.push_str(types.next().copied().unwrap_or("?")),
                    '{' | '}' => {}
                    ',' => out.push_str(", "),
                    _ => out.push(c),
                }
            }
            out
        }
        None => {
            let optional = b.optional.unwrap_or(&[]);
            let list: Vec<String> = b
                .arg_types
                .iter()
                .enumerate()
                .map(|(i, t)| {
                    if optional.get(i) == Some(&true) {
                        format!("[{t}]")
                    } else {
                        (*t).to_string()
                    }
                })
                .collect();
            list.join(", ")
        }
    };
    match (b.kind, b.ret) {
        (Kind::Function, Some(ret)) if args.is_empty() => format!("{name} AS {ret}"),
        (Kind::Function, Some(ret)) => format!("{name}({args}) AS {ret}"),
        _ if args.is_empty() => name,
        _ => format!("{name} {args}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forms_of_built_ins() {
        let text = |w: &str| builtin_text(w.as_bytes());
        assert_eq!(
            text("left$").as_deref(),
            Some("```qb64\nLeft$(STRING, LONG) AS STRING\n```\nBuilt-in.")
        );
        assert!(
            text("_RGB32")
                .unwrap()
                .contains("_RGB32(LONG, LONG, LONG, LONG) AS _UNSIGNED LONG")
        );
        assert!(text("instr").unwrap().contains("InStr([LONG], STRING, STRING) AS LONG"));
        assert!(text("CLS").unwrap().contains("Cls [LONG][, [_UNSIGNED LONG][, LONG]]"));
        assert_eq!(text("FOR"), None);
        assert_eq!(text("notabuiltin"), None);
    }

    #[test]
    fn words() {
        assert!(is_word(b"count%"));
        assert!(is_word(b"a.b"));
        assert!(is_word(b"x`8"));
        assert!(!is_word(b"'comment"));
        assert!(!is_word(b"12"));
        assert!(!is_word(b"REM a comment"));
    }
}
