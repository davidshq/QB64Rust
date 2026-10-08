//! `textDocument/documentSymbol` (design D4): procedures, `TYPE` blocks with their members, and labels, nested in
//! the procedure they stand in. `DECLARE` headers are left out: they declare nothing in QB64pe.

use crate::analysis::Analysis;
use crate::walk::{content_span, is_expr, text};
use lsp_types::{DocumentSymbol, SymbolKind};
use qb64rust_base::Span;
use qb64rust_syntax::ast::{LabelDef, ProcDef, TypeBlock};
use qb64rust_syntax::tree::{Node, Tok};

/// The symbols of the program's main file.
pub fn document_symbols(a: &Analysis) -> Vec<DocumentSymbol> {
    let mut out = Vec::new();
    collect(a, a.parsed.main().root(), &mut out);
    out
}

fn collect(a: &Analysis, node: Node, out: &mut Vec<DocumentSymbol>) {
    for n in node.child_nodes() {
        if let Some(p) = ProcDef::cast(n) {
            let mut children = Vec::new();
            collect(a, n, &mut children);
            let header = p.header();
            match header.and_then(|h| h.name()) {
                Some(name) => {
                    let keyword = header
                        .and_then(|h| h.keyword())
                        .map(|k| text(&a.map, k).to_ascii_uppercase());
                    let kind = match keyword.as_deref() {
                        Some("FUNCTION") => SymbolKind::FUNCTION,
                        _ => SymbolKind::METHOD,
                    };
                    let mut s = symbol(a, name, kind, content_span(n), children);
                    s.detail = keyword;
                    out.push(s);
                }
                None => out.extend(children),
            }
        } else if let Some(t) = TypeBlock::cast(n) {
            let Some(name) = t.header().and_then(|h| h.name()) else {
                continue;
            };
            let mut fields = Vec::new();
            for f in t.fields() {
                let names: Vec<_> = f.names().collect();
                for fname in &names {
                    let Some(tok) = fname.name() else {
                        continue;
                    };
                    // `a AS INTEGER`: the whole line; `AS INTEGER a, b`: each name.
                    let range = if names.len() == 1 {
                        content_span(f.node())
                    } else {
                        content_span(fname.node())
                    };
                    fields.push(symbol(a, tok, SymbolKind::FIELD, range, Vec::new()));
                }
            }
            out.push(symbol(a, name, SymbolKind::STRUCT, content_span(n), fields));
        } else if let Some(l) = LabelDef::cast(n) {
            if let Some(name) = l.name() {
                out.push(symbol(a, name, SymbolKind::KEY, content_span(n), Vec::new()));
            }
        } else if !is_expr(n) {
            collect(a, n, out);
        }
    }
}

fn symbol(a: &Analysis, name: Tok, kind: SymbolKind, range: Span, children: Vec<DocumentSymbol>) -> DocumentSymbol {
    #[expect(deprecated, reason = "a field of the protocol type; `tags` replaces it")]
    DocumentSymbol {
        name: text(&a.map, name),
        detail: None,
        kind,
        tags: None,
        deprecated: None,
        range: a.range(range.cover(name.span)),
        selection_range: a.range(name.span),
        children: if children.is_empty() { None } else { Some(children) },
    }
}
