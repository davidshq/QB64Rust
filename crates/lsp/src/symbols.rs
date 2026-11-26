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
    document_symbols_in(a, a.parsed.main().file)
}

/// The symbols of the program's file `file` (an included file's at its first inclusion).
pub fn document_symbols_in(a: &Analysis, file: qb64rust_base::FileId) -> Vec<DocumentSymbol> {
    let mut out = Vec::new();
    if let Some(tree) = a.tree(file) {
        collect(a, tree.root(), &mut out);
    }
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

/// One SUB or FUNCTION of a program, for `qb64rust/procedures` (OpenSpec change `editor-side-panel`, design D3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Procedure {
    /// The name as declared, with its suffix: `Speed!`.
    pub name: String,
    /// `SUB` or `FUNCTION`, in capitals.
    pub kind: &'static str,
    /// What the header has after the name, as written: `(level)`, `(x, y) STATIC`; empty for none.
    pub params: String,
    /// The header line, from its first to its last token.
    pub span: Span,
}

/// Every SUB and FUNCTION of the program, in the order the program reads them: the main file from its top, and an
/// included file's where its `$INCLUDE` stands, each file once. QB64pe's own files (the color file, a library's
/// files, `afterlastline.bm`) are left out. A procedure the parser could not read a name for is left out.
pub fn procedures(a: &Analysis) -> Vec<Procedure> {
    program_procedures(a).1
}

/// The program's own files, the main file first and each included file in the order the program reads it, and
/// its procedures ([`procedures`]). A program of many files, such as one that includes 47, has all of them here,
/// open in the editor or not: the client counts the errors of each.
pub fn program_procedures(a: &Analysis) -> (Vec<qb64rust_base::FileId>, Vec<Procedure>) {
    let mut out = Vec::new();
    let mut seen = vec![a.parsed.main().file];
    walk_procedures(a, a.parsed.main().root(), &mut seen, &mut out);
    (seen, out)
}

fn walk_procedures(a: &Analysis, node: Node, seen: &mut Vec<qb64rust_base::FileId>, out: &mut Vec<Procedure>) {
    for n in node.child_nodes() {
        if let Some(tree) = a.parsed.included(n) {
            if !seen.contains(&tree.file) {
                seen.push(tree.file);
                walk_procedures(a, tree.root(), seen, out);
            }
            continue;
        }
        if let Some(p) = ProcDef::cast(n)
            && let Some(header) = p.header()
            && let Some(name) = header.name()
        {
            let keyword = header.keyword().map(|k| text(&a.map, k).to_ascii_uppercase());
            let kind = if keyword.as_deref() == Some("FUNCTION") {
                "FUNCTION"
            } else {
                "SUB"
            };
            let line = content_span(header.node());
            let after = qb64rust_base::Span {
                start: name.span.end.min(line.end),
                ..line
            };
            out.push(Procedure {
                name: text(&a.map, name),
                kind,
                params: qb64rust_base::show_bytes(a.map.text(after)).trim().to_string(),
                span: line,
            });
        }
        // An `$INCLUDE` may stand inside a block or a procedure.
        if !is_expr(n) {
            walk_procedures(a, n, seen, out);
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
