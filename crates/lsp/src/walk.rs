//! Helpers for the walks over the trees (design D4).

use qb64rust_base::{SourceMap, Span, show_bytes};
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::ast::Expr;
use qb64rust_syntax::tree::{Element, Node, Tok};

/// A token that is part of a statement: not trivia, not a line end.
fn significant(t: &Tok) -> bool {
    !t.kind.is_trivia() && t.kind != SyntaxKind::Newline
}

/// The first significant token below `node`.
pub fn first_token(node: Node) -> Option<Tok> {
    for c in node.children() {
        match c {
            Element::Node(n) => {
                if let Some(t) = first_token(n) {
                    return Some(t);
                }
            }
            Element::Token(t) if significant(&t) => return Some(t),
            Element::Token(_) => {}
        }
    }
    None
}

/// The last significant token below `node`.
pub fn last_token(node: Node) -> Option<Tok> {
    let children: Vec<Element> = node.children().collect();
    for c in children.into_iter().rev() {
        match c {
            Element::Node(n) => {
                if let Some(t) = last_token(n) {
                    return Some(t);
                }
            }
            Element::Token(t) if significant(&t) => return Some(t),
            Element::Token(_) => {}
        }
    }
    None
}

/// From the first to the last significant token: the node without the line end and comments after it.
pub fn content_span(node: Node) -> Span {
    match (first_token(node), last_token(node)) {
        (Some(a), Some(b)) => a.span.cover(b.span),
        _ => node.span(),
    }
}

/// An expression: no walk needs to look inside one, except go to definition, which descends by position.
pub fn is_expr(node: Node) -> bool {
    Expr::cast(node).is_some()
}

/// A token's text for the editor (names are ASCII; anything else shown as `\xNN`).
pub fn text(map: &SourceMap, t: Tok) -> String {
    show_bytes(map.text(t.span))
}

/// The 0-based line of an offset.
pub fn line(map: &SourceMap, span: Span) -> u32 {
    map.file(span.file).line_col(span.start).0 - 1
}
