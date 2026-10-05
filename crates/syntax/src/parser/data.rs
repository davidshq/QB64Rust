//! `DATA`, `READ` and `RESTORE` (design D3, D5). The lexer gives everything after `DATA` as one `DataText` token;
//! the items are split from its bytes by `ast::DataStmt::items`, never re-tokenized.

use super::Parser;
use super::expr::expr;
use crate::SyntaxKind::*;
use qb64rust_base::Span;

/// `DATA` and its `DataText`, at `DATA`.
pub(crate) fn data_stmt(p: &mut Parser) {
    p.start_node(DataStmt);
    p.bump(); // DATA
    if p.at(DataText) {
        let span = p.current_span();
        p.bump();
        let scan = crate::data::scan(p.bytes, span.start as usize);
        if let Some(at) = scan.after_quote {
            let at = qb64rust_base::to_u32(at);
            let span = Span::new(span.file, at, at + 1);
            p.error_at(span, "expected `,` after a quoted `DATA` item");
        }
    }
    p.recover();
    p.finish_node();
}

/// `READ target, ...`, at `READ`. The targets are parsed as expressions; `sema` checks that each is a variable.
pub(crate) fn read_stmt(p: &mut Parser) {
    p.start_node(ReadStmt);
    p.bump(); // READ
    loop {
        if !expr(p) {
            break;
        }
        if p.at(Comma) {
            p.bump();
        } else {
            break;
        }
    }
    p.recover();
    p.finish_node();
}

/// `RESTORE [label | number]`, at `RESTORE`.
pub(crate) fn restore_stmt(p: &mut Parser) {
    p.start_node(RestoreStmt);
    p.bump(); // RESTORE
    if p.at(Ident) || p.at(Number) {
        p.bump();
    }
    p.recover();
    p.finish_node();
}
