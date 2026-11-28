//! `textDocument/hover` (design D3 of `m5b-language-features`): a symbol's text from the semantic index, else a
//! built-in's form from the built-in table, else nothing. After a built-in's form, and alone for a keyword, the
//! first sentence of its page of the project's own language reference (`help_table.rs`). No QB64pe wiki text: the
//! wiki has no license.

use crate::help_table::{PAGES, WORDS};
use crate::index::Indexed;
use lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind};
use qb64rust_base::{FileId, Span};
use qb64rust_builtins::{Builtin, Kind};
use std::fmt::Write as _;

/// The hover at `offset` of the program's file `file`.
pub fn hover(ix: &Indexed, file: FileId, offset: u32) -> Option<Hover> {
    let (text, span) = match ix.index.entry_near(file, offset) {
        Some((id, span)) => (ix.index.entries[id].detail.clone(), span),
        None => {
            let (span, word) = word_at(ix, file, offset)?;
            (builtin_text(&word).or_else(|| keyword_text(&word))?, span)
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

/// The word at `offset` (or just before it) of the file `file`: a name with its suffix, as the lexer reads one.
pub(crate) fn word_at(ix: &Indexed, file: FileId, offset: u32) -> Option<(Span, Vec<u8>)> {
    let main = ix.analysis.tree(file)?;
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
    if let Some(page) = help_page(word).filter(|p| p.about) {
        write!(out, "\n\n{}", page.sentence).unwrap();
    }
    Some(out)
}

/// The hover of a keyword that is not a built-in (`FOR`, `CASE`): the first sentence of its page, when the
/// sentence is about the word; `None` for a word covered beside another (`TO` on the page of `FOR`) or with no page.
fn keyword_text(word: &[u8]) -> Option<String> {
    help_page(word).filter(|p| p.about).map(|p| p.sentence.to_string())
}

/// A word's page of the language reference (`help_table.rs`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HelpPage {
    /// The page's name, its file name without `.md`: `_clamp`, `select-case`.
    pub name: &'static str,
    /// The first sentence of its summary, as Markdown.
    pub sentence: &'static str,
    /// Whether the sentence is about the word, not only beside it.
    pub about: bool,
}

/// The page of `word`, in any case and with its suffix (`left$`, `_Clamp`, `CASE`).
pub(crate) fn help_page(word: &[u8]) -> Option<HelpPage> {
    let upper = word.to_ascii_uppercase();
    let i = WORDS.binary_search_by(|(w, _, _)| w.as_bytes().cmp(&upper)).ok()?;
    let (_, page, about) = WORDS[i];
    let (name, sentence) = PAGES[usize::from(page)];
    Some(HelpPage { name, sentence, about })
}

/// One built-in's form: `Left$(STRING, LONG) AS STRING`, `Cls [LONG][,[_UNSIGNED LONG][,LONG]]`. With a
/// `specialformat`, each `?` is the next argument's type and each `{word}` a word written as it is; without one,
/// the arguments in order, an optional one in brackets.
pub fn form(b: &Builtin) -> String {
    form_parts(b).0
}

/// [`form`] and where each argument stands in it, as byte ranges (the form is ASCII): one per argument slot, an
/// optional one with its brackets. For a `specialformat` that is a plain list, each slot's type; for any other, no
/// ranges, since commas do not count its arguments (`LINE (?,?)-(?,?)`).
pub(crate) fn form_parts(b: &Builtin) -> (String, Vec<(usize, usize)>) {
    let name = format!("{}{}", b.name, b.musthave.unwrap_or(""));
    let mut ranges = Vec::new();
    let args = match b.specialformat {
        Some(f) => {
            // A format of slots, commas and brackets only (`[?],?,?` of `INSTR`) is a plain list: commas count
            // its arguments, so each slot is marked.
            let list = f.chars().all(|c| "?,[]".contains(c));
            let mut types = b.arg_types.iter();
            let mut out = String::new();
            for c in f.chars() {
                match c {
                    '?' => {
                        let start = out.len();
                        out.push_str(types.next().copied().unwrap_or("?"));
                        if list {
                            ranges.push((start, out.len()));
                        }
                    }
                    '{' | '}' => {}
                    ',' => out.push_str(", "),
                    _ => out.push(c),
                }
            }
            out
        }
        None => {
            let optional = b.optional.unwrap_or(&[]);
            let mut out = String::new();
            for (i, t) in b.arg_types.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                let start = out.len();
                if optional.get(i) == Some(&true) {
                    write!(out, "[{t}]").unwrap();
                } else {
                    out.push_str(t);
                }
                ranges.push((start, out.len()));
            }
            out
        }
    };
    let (text, args_at) = match (b.kind, b.ret) {
        (Kind::Function, Some(ret)) if args.is_empty() => (format!("{name} AS {ret}"), 0),
        (Kind::Function, Some(ret)) => (format!("{name}({args}) AS {ret}"), name.len() + 1),
        _ if args.is_empty() => (name, 0),
        _ => (format!("{name} {args}"), name.len() + 1),
    };
    let ranges = ranges.into_iter().map(|(s, e)| (s + args_at, e + args_at)).collect();
    (text, ranges)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forms_of_built_ins() {
        let text = |w: &str| builtin_text(w.as_bytes());
        assert!(
            text("left$")
                .unwrap()
                .starts_with("```qb64\nLeft$(STRING, LONG) AS STRING\n```\nBuilt-in.\n\n`LEFT$")
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

    /// A built-in with a page gains its first sentence; a keyword gives its page's sentence; a word covered beside
    /// another, or with no page, gives none.
    #[test]
    fn sentences_of_the_reference() {
        let clamp = builtin_text(b"_CLAMP").unwrap();
        assert!(
            clamp.contains("_CLAMP(") && clamp.contains("holds a value between two bounds"),
            "{clamp}"
        );
        let max = builtin_text(b"_max").unwrap();
        assert!(max.contains("`_MAX` the greater"), "{max}");
        let for_ = keyword_text(b"FOR").unwrap();
        assert!(for_.starts_with("`FOR` repeats statements"), "{for_}");
        assert!(keyword_text(b"case").unwrap().starts_with("`SELECT CASE`"));
        assert_eq!(keyword_text(b"TO"), None);
        assert_eq!(keyword_text(b"notakeyword"), None);
        assert_eq!(help_page(b"TO").map(|p| p.name), Some("for"));
    }

    /// The table is in order of word, for the binary search, and every page index is in range.
    #[test]
    fn the_table_is_sorted() {
        assert!(WORDS.windows(2).all(|w| w[0].0 < w[1].0));
        assert!(WORDS.iter().all(|(_, p, _)| usize::from(*p) < PAGES.len()));
        assert!(PAGES.windows(2).all(|w| w[0].0 < w[1].0));
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
