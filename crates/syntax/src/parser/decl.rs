//! `DIM [SHARED] name [AS type], ...`, `SHARED ...` and `STATIC ...` for scalars. Type names are kept as words;
//! `sema` decides which it supports.

use super::Parser;
use crate::SyntaxKind::{self, *};

pub(crate) fn dim_stmt(p: &mut Parser) {
    p.start_node(DimStmt);
    p.bump(); // DIM
    if p.at_word("SHARED") {
        p.bump();
    }
    if p.at_word("_PRESERVE") {
        p.error("`DIM _PRESERVE` is not supported yet");
    } else {
        items(p);
    }
    p.recover();
    p.finish_node();
}

/// `SHARED` or `STATIC` and a list of `DimItem`s (`kind` is `SharedStmt` or `StaticStmt`).
pub(crate) fn list_stmt(p: &mut Parser, kind: SyntaxKind) {
    p.start_node(kind);
    p.bump(); // SHARED or STATIC
    items(p);
    p.recover();
    p.finish_node();
}

fn items(p: &mut Parser) {
    loop {
        if !dim_item(p) {
            break;
        }
        if p.at(Comma) {
            p.bump();
        } else {
            break;
        }
    }
}

fn dim_item(p: &mut Parser) -> bool {
    if !p.at(Ident) {
        p.error("expected a variable name");
        return false;
    }
    p.start_node(DimItem);
    p.bump();
    let ok = if p.at(LParen) {
        p.error("arrays are not supported yet");
        false
    } else if p.at_word("AS") {
        as_clause(p)
    } else {
        true
    };
    p.finish_node();
    ok
}

/// `AS <type words>` at `AS`. Returns false after an error.
pub(super) fn as_clause(p: &mut Parser) -> bool {
    p.start_node(AsClause);
    p.bump();
    let mut ok = true;
    if !p.at(Ident) {
        p.error("expected a type name after `AS`");
        ok = false;
    }
    while p.at(Ident) {
        p.bump();
    }
    if p.at(Star) {
        p.error("fixed-length strings are not supported yet");
        ok = false;
    }
    p.finish_node();
    ok
}
