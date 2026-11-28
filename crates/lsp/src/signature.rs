//! `textDocument/signatureHelp`: the forms of the built-in or user procedure whose arguments the cursor is in, with
//! the argument under the cursor marked. The call is found in the text of the cursor's line before it, so that it
//! works while the line does not parse yet; a user procedure's header comes from the last parse of the program.
//!
//! Two shapes of call are found:
//! - a name and an open `(` (`LEFT$(a$, `, `CALL mine(1, `): a built-in function or a user procedure; the
//!   innermost such call wins, so `LEFT$(MID$(a$, ` is about `MID$`;
//! - a statement's first word and a blank after it (`CLS 1, `, `mine 1, `): a built-in or user SUB.
//!
//! A built-in's form is the hover's (`hover::form`). A form whose `specialformat` is more than slots, commas and
//! brackets (`LINE`, `CIRCLE`) is shown without a marked argument: commas do not count its arguments.

use crate::analysis::Analysis;
use crate::hover::{form_parts, help_page, name_end};
use crate::walk::{content_span, is_expr};
use lsp_types::{
    Documentation, MarkupContent, MarkupKind, ParameterInformation, ParameterLabel, SignatureHelp, SignatureInformation,
};
use qb64rust_base::show_bytes;
use qb64rust_builtins::Kind;
use qb64rust_syntax::ast::{DeclareLibraryBlock, ProcDef, ProcHeader};
use qb64rust_syntax::tree::Node;

/// The call the cursor is in, from the text of its line before it.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Call {
    /// The procedure's name as written, suffix included.
    pub name: String,
    /// Written with an open `(` after the name, not as a statement.
    pub parens: bool,
    /// How many commas stand between the call's start and the cursor at its own level: the argument's index.
    pub argument: u32,
}

/// An open `(` of the line: the name just before it, if any, and the commas after it so far.
struct Open {
    name: Option<String>,
    commas: u32,
}

/// The calls the text before the cursor stands in, the innermost first: each name with an open `(`, then the
/// statement's own first word when a blank follows it. Empty in a comment, after `REM`, and in a `DATA` statement.
pub(crate) fn calls(before: &str) -> Vec<Call> {
    let bytes = before.as_bytes();
    let mut stack: Vec<Open> = Vec::new();
    let mut top_commas = 0;
    let mut start = 0;
    let mut in_string = false;
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if in_string {
            in_string = c != b'"';
        } else {
            match c {
                b'"' => in_string = true,
                b'\'' => return Vec::new(),
                b':' => {
                    stack.clear();
                    top_commas = 0;
                    start = i + 1;
                }
                b'(' => stack.push(Open {
                    // `start` and `i` stand at an ASCII `:` or `(`, so both are character boundaries.
                    name: word_before(&before[start..i]),
                    commas: 0,
                }),
                b')' => {
                    stack.pop();
                }
                b',' => match stack.last_mut() {
                    Some(open) => open.commas += 1,
                    None => top_commas += 1,
                },
                _ => {}
            }
        }
        i += 1;
    }
    if in_string {
        // The cursor is inside a string: no argument is being chosen.
        return Vec::new();
    }
    let mut out: Vec<Call> = stack
        .iter()
        .rev()
        .filter_map(|o| {
            Some(Call {
                name: o.name.clone()?,
                parens: true,
                argument: o.commas,
            })
        })
        .collect();
    let statement = &before[start..];
    let rest = statement.trim_start().trim_start_matches(|c: char| c.is_ascii_digit());
    let rest = rest.trim_start();
    let first = &rest[..word_len(rest)];
    if first.eq_ignore_ascii_case("REM") || first.eq_ignore_ascii_case("DATA") {
        return Vec::new();
    }
    let (name, after) = if first.eq_ignore_ascii_case("CALL") {
        let more = rest[first.len()..].trim_start();
        (&more[..word_len(more)], &more[word_len(more)..])
    } else {
        (first, &rest[first.len()..])
    };
    // `x = ...` assigns: its first word calls nothing.
    if !name.is_empty() && after.starts_with([' ', '\t']) && !after.trim_start().starts_with('=') {
        out.push(Call {
            name: name.to_string(),
            parens: false,
            argument: top_commas,
        });
    }
    out
}

/// The length of the name at the start of `text`, suffix included: a letter or `_` first.
fn word_len(text: &str) -> usize {
    let b = text.as_bytes();
    if !b.first().is_some_and(|c| c.is_ascii_alphabetic() || *c == b'_') {
        return 0;
    }
    let end = name_end(b);
    end + b[end..].iter().take_while(|c| b"$%&!#~`".contains(c)).count()
}

/// The name that ends `text`, blanks after it allowed (`LEFT$ (`), when one does.
fn word_before(text: &str) -> Option<String> {
    let text = text.trim_end();
    let start = text
        .char_indices()
        .rev()
        .take_while(|(_, c)| c.is_ascii_alphanumeric() || "_.$%&!#~`".contains(*c))
        .last()
        .map(|(i, _)| i)?;
    let word = &text[start..];
    (word_len(word) == word.len()).then(|| word.to_string())
}

/// The signature help for the text `before` the cursor on its line, with the user procedures of the parse `a`.
pub fn signature_help(a: Option<&Analysis>, before: &str) -> Option<SignatureHelp> {
    for call in calls(before) {
        let mut signatures = user_signatures(a, &call);
        if signatures.is_empty() {
            signatures = builtin_signatures(&call);
        }
        if signatures.is_empty() {
            continue;
        }
        // The first form with enough arguments for the cursor's; else the first.
        let active = signatures
            .iter()
            .position(|s| s.parameters.as_ref().is_some_and(|p| p.len() > call.argument as usize))
            .unwrap_or(0);
        return Some(SignatureHelp {
            signatures,
            active_signature: Some(qb64rust_base::to_u32(active)),
            active_parameter: Some(call.argument),
        });
    }
    None
}

/// The built-in's forms for the call: functions for a call with `(`, SUBs for a statement.
fn builtin_signatures(call: &Call) -> Vec<SignatureInformation> {
    let word = call.name.as_bytes();
    let kind = if call.parens { Kind::Function } else { Kind::Sub };
    let documentation = help_page(word).filter(|p| p.about).map(|p| {
        Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: p.sentence.to_string(),
        })
    });
    let mut out: Vec<SignatureInformation> = Vec::new();
    for b in qb64rust_builtins::find_any(&word[..name_end(word)]).filter(|b| b.kind == kind) {
        let (label, ranges) = form_parts(b);
        if out.iter().any(|s| s.label == label) {
            continue;
        }
        out.push(SignatureInformation {
            label,
            documentation: documentation.clone(),
            parameters: Some(offsets(&ranges)),
            active_parameter: None,
        });
    }
    out
}

/// The headers of the user procedures named as the call names them, from every file of the parse: SUBs and
/// FUNCTIONs, and those of `DECLARE LIBRARY` blocks. A statement calls only a SUB.
fn user_signatures(a: Option<&Analysis>, call: &Call) -> Vec<SignatureInformation> {
    let Some(a) = a else {
        return Vec::new();
    };
    let want = call.name.as_bytes();
    let want = &want[..name_end(want)];
    let mut headers = Vec::new();
    for tree in &a.parsed.trees {
        collect_headers(tree.root(), &mut headers);
    }
    let mut out: Vec<SignatureInformation> = Vec::new();
    for h in headers {
        let (Some(name), Some(keyword)) = (h.name(), h.keyword()) else {
            continue;
        };
        let written = a.map.text(name.span);
        if !written[..name_end(written)].eq_ignore_ascii_case(want) {
            continue;
        }
        if !call.parens && !a.map.text(keyword.span).eq_ignore_ascii_case(b"SUB") {
            continue;
        }
        let sig = header_signature(a, h);
        if !out.iter().any(|s| s.label == sig.label) {
            out.push(sig);
        }
    }
    out
}

fn collect_headers<'a>(node: Node<'a>, out: &mut Vec<ProcHeader<'a>>) {
    for n in node.child_nodes() {
        if let Some(p) = ProcDef::cast(n) {
            out.extend(p.header());
        } else if let Some(d) = DeclareLibraryBlock::cast(n) {
            out.extend(d.procs());
        } else if !is_expr(n) {
            collect_headers(n, out);
        }
    }
}

/// A header's text as the label (`SUB greet (who$, times AS INTEGER)`), each parameter's text marked.
fn header_signature(a: &Analysis, h: ProcHeader) -> SignatureInformation {
    let span = content_span(h.node());
    let mut label = String::new();
    let mut ranges = Vec::new();
    let mut at = span.start;
    for p in h.params() {
        let ps = content_span(p.node());
        label.push_str(&show_bytes(
            &a.map.file(span.file).bytes[at as usize..ps.start as usize],
        ));
        // The marks count UTF-16 units: a header may hold a byte above 127 (an `ALIAS "…"`), shown as one character.
        let start = label.encode_utf16().count();
        label.push_str(&show_bytes(a.map.text(ps)));
        ranges.push((start, label.encode_utf16().count()));
        at = ps.end;
    }
    label.push_str(&show_bytes(
        &a.map.file(span.file).bytes[at as usize..span.end as usize],
    ));
    SignatureInformation {
        label,
        documentation: None,
        parameters: Some(offsets(&ranges)),
        active_parameter: None,
    }
}

/// Ranges of a label in UTF-16 units as the label offsets of its parameters. A built-in's label is ASCII, so its
/// byte ranges are such units.
fn offsets(ranges: &[(usize, usize)]) -> Vec<ParameterInformation> {
    ranges
        .iter()
        .map(|&(s, e)| ParameterInformation {
            label: ParameterLabel::LabelOffsets([qb64rust_base::to_u32(s), qb64rust_base::to_u32(e)]),
            documentation: None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call(name: &str, parens: bool, argument: u32) -> Call {
        Call {
            name: name.to_string(),
            parens,
            argument,
        }
    }

    #[test]
    fn calls_in_the_text_before_the_cursor() {
        assert_eq!(calls("x$ = LEFT$(a$, "), vec![call("LEFT$", true, 1)]);
        assert_eq!(calls("x$ = LEFT$ (a$"), vec![call("LEFT$", true, 0)]);
        assert_eq!(
            calls("x$ = LEFT$(MID$(a$, 2), "),
            vec![call("LEFT$", true, 1)],
            "MID$ is closed"
        );
        assert_eq!(
            calls("PRINT LEFT$(MID$(a$, "),
            vec![call("MID$", true, 1), call("LEFT$", true, 0), call("PRINT", false, 0)]
        );
        assert_eq!(calls("CLS 1, "), vec![call("CLS", false, 1)]);
        assert_eq!(calls("10 CLS "), vec![call("CLS", false, 0)]);
        assert_eq!(calls("a = 1: CALL mine(1, (2), "), vec![call("mine", true, 2)]);
        assert_eq!(calls("CALL mine "), vec![call("mine", false, 0)]);
        assert_eq!(calls("CLS"), vec![], "still in the word");
        assert_eq!(calls("PRINT \"a, (b"), vec![], "in a string");
        assert_eq!(
            calls("PRINT \"a, (b\"; LEN("),
            vec![call("LEN", true, 0), call("PRINT", false, 0)]
        );
        assert_eq!(calls("x = 1 ' LEFT$("), vec![]);
        assert_eq!(calls("REM LEFT$("), vec![]);
        assert_eq!(calls("DATA 1, 2"), vec![]);
        assert_eq!(calls("x = (1 + "), vec![], "a group, not a call");
        assert_eq!(calls("mine (1 + "), vec![call("mine", true, 0), call("mine", false, 0)]);
    }

    #[test]
    fn built_in_forms_with_the_argument_marked() {
        let help = signature_help(None, "x$ = LEFT$(a$, ").unwrap();
        assert_eq!(help.active_parameter, Some(1));
        let s = &help.signatures[0];
        assert_eq!(s.label, "Left$(STRING, LONG) AS STRING");
        let labels: Vec<_> = s
            .parameters
            .iter()
            .flatten()
            .map(|p| match p.label {
                ParameterLabel::LabelOffsets([a, b]) => &s.label[a as usize..b as usize],
                ParameterLabel::Simple(_) => unreachable!(),
            })
            .collect();
        assert_eq!(labels, ["STRING", "LONG"]);
        assert!(s.documentation.is_some(), "the reference's first sentence");

        let instr = signature_help(None, "p = INSTR(").unwrap();
        let s = &instr.signatures[0];
        let first = match s.parameters.as_ref().unwrap()[0].label {
            ParameterLabel::LabelOffsets([a, b]) => &s.label[a as usize..b as usize],
            ParameterLabel::Simple(_) => unreachable!(),
        };
        assert_eq!(first, "LONG", "a slot of the special format `[?],?,?`");

        let cls = signature_help(None, "CLS 1, ").unwrap();
        assert!(cls.signatures[0].label.starts_with("Cls "), "{:?}", cls.signatures);
        assert_eq!(cls.signatures[0].parameters.as_ref().map(Vec::len), Some(3));
        assert_eq!(cls.active_parameter, Some(1));
        assert_eq!(signature_help(None, "x = NOTABUILTIN("), None);
        assert_eq!(signature_help(None, "LEFT$ "), None, "a function is not a statement");
    }

    #[test]
    fn a_special_format_marks_no_argument() {
        let line = signature_help(None, "LINE (0, 0)-(10, 10), ").unwrap();
        assert!(line.signatures.iter().all(|s| s.parameters.as_deref() == Some(&[])));
    }

    #[test]
    fn names() {
        assert_eq!(word_before("x = LEFT$ "), Some("LEFT$".to_string()));
        assert_eq!(word_before("x = "), None);
        assert_eq!(word_before("x = 2"), None);
        assert_eq!(word_before("t.field"), Some("t.field".to_string()));
        assert_eq!(word_before("PRINT \"é\"; LEN"), Some("LEN".to_string()));
    }
}
