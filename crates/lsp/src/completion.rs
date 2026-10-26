//! `textDocument/completion` (design D6, D7 of `m5b-language-features`): the names the program sees at the cursor,
//! from the semantic index (the last one when none is ready for the current text), the built-ins as the table
//! spells them, and the reserved words; after `x.` for a variable of a user type, that type's members alone.

use crate::hover::{builtin_text, form, name_end};
use crate::index::{EntryKind, Indexed};
use lsp_types::{CompletionItem, CompletionItemKind, Documentation, MarkupContent, MarkupKind};
use qb64rust_builtins::{BUILTINS, Kind};
use std::collections::HashSet;

/// The items at `offset` of the program's main file (in the index's text), where `before` is the current text of
/// the cursor's line up to the cursor.
pub fn items(ix: Option<&Indexed>, offset: Option<u32>, before: &str) -> Vec<CompletionItem> {
    if let Some(base) = dotted_base(before) {
        return ix.map_or_else(Vec::new, |ix| members(ix, offset, base));
    }
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    if let Some(ix) = ix {
        let main = ix.analysis.parsed.main().file;
        let body = offset.and_then(|o| ix.index.body_at(main, o));
        for (i, e) in ix.index.entries.iter().enumerate() {
            // A member only after its variable's dot; a label only in its own body.
            let shown = match e.kind {
                EntryKind::Member => false,
                EntryKind::Label => e.scope == body,
                _ => ix.index.visible_in(i, body),
            };
            if shown && seen.insert(e.name.to_ascii_uppercase()) {
                out.push(entry_item(ix, i));
            }
        }
    }
    for b in BUILTINS {
        let name = format!("{}{}", b.name, b.musthave.unwrap_or(""));
        if seen.insert(name.to_ascii_uppercase()) {
            let kind = match b.kind {
                Kind::Function => CompletionItemKind::FUNCTION,
                Kind::Sub => CompletionItemKind::KEYWORD,
            };
            out.push(CompletionItem {
                label: name,
                kind: Some(kind),
                detail: Some(form(b)),
                documentation: builtin_text(b.name.as_bytes()).map(markdown),
                ..CompletionItem::default()
            });
        }
    }
    for k in qb64rust_syntax::keywords() {
        if seen.insert((*k).to_string()) {
            out.push(CompletionItem {
                label: (*k).to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..CompletionItem::default()
            });
        }
    }
    out
}

/// The name before a dot that ends `before` with a partial name after it (`p.` or `p.x`), if any.
fn dotted_base(before: &str) -> Option<&str> {
    let b = before.as_bytes();
    let is_name = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    let mut i = b.len();
    while i > 0 && is_name(b[i - 1]) {
        i -= 1;
    }
    if i == 0 || b[i - 1] != b'.' {
        return None;
    }
    let dot = i - 1;
    let mut start = dot;
    while start > 0 && is_name(b[start - 1]) {
        start -= 1;
    }
    (start < dot && b[start].is_ascii_alphabetic()).then(|| &before[start..dot])
}

/// The members of the user type of the variable `base` seen at `offset` (design D7); none when `base` is no such
/// variable.
fn members(ix: &Indexed, offset: Option<u32>, base: &str) -> Vec<CompletionItem> {
    let main = ix.analysis.parsed.main().file;
    let body = offset.and_then(|o| ix.index.body_at(main, o));
    let bare = |name: &str| name[..name_end(name.as_bytes())].to_ascii_uppercase();
    let base = base.to_ascii_uppercase();
    let found = ix.index.entries.iter().enumerate().find(|&(i, e)| {
        e.kind == EntryKind::Variable && e.user_type.is_some() && bare(&e.name) == base && ix.index.visible_in(i, body)
    });
    let Some((_, var)) = found else {
        return Vec::new();
    };
    let ty = var.user_type;
    ix.index
        .entries
        .iter()
        .enumerate()
        .filter(|(_, e)| e.kind == EntryKind::Member && e.user_type == ty)
        .map(|(i, _)| entry_item(ix, i))
        .collect()
}

/// The item of a symbol: its name as defined, its kind, its type, and its hover text.
fn entry_item(ix: &Indexed, i: usize) -> CompletionItem {
    let e = &ix.index.entries[i];
    let kind = match e.kind {
        EntryKind::Variable => CompletionItemKind::VARIABLE,
        EntryKind::Constant => CompletionItemKind::CONSTANT,
        EntryKind::Procedure => CompletionItemKind::FUNCTION,
        EntryKind::Label => CompletionItemKind::REFERENCE,
        EntryKind::Type => CompletionItemKind::STRUCT,
        EntryKind::Member => CompletionItemKind::FIELD,
    };
    CompletionItem {
        label: e.name.clone(),
        kind: Some(kind),
        detail: (!e.ty.is_empty()).then(|| e.ty.clone()),
        documentation: Some(markdown(e.detail.clone())),
        ..CompletionItem::default()
    }
}

fn markdown(value: String) -> Documentation {
    Documentation::MarkupContent(MarkupContent {
        kind: MarkupKind::Markdown,
        value,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_name_before_a_dot() {
        assert_eq!(dotted_base("    p."), Some("p"));
        assert_eq!(dotted_base("x = pt.y"), Some("pt"));
        assert_eq!(dotted_base("x = 1."), None);
        assert_eq!(dotted_base("PRINT a"), None);
        assert_eq!(dotted_base("."), None);
    }

    #[test]
    fn built_ins_and_keywords_without_an_index() {
        let items = items(None, None, "x = _rg");
        let rgb = items.iter().find(|i| i.label == "_RGB32").expect("_RGB32");
        assert_eq!(rgb.kind, Some(CompletionItemKind::FUNCTION));
        assert!(items.iter().any(|i| i.label == "Left$"));
        assert!(
            items
                .iter()
                .any(|i| i.label == "FOR" && i.kind == Some(CompletionItemKind::KEYWORD))
        );
        let names: Vec<String> = items.iter().map(|i| i.label.to_ascii_uppercase()).collect();
        let unique: HashSet<&String> = names.iter().collect();
        assert_eq!(names.len(), unique.len(), "each name once");
    }
}
