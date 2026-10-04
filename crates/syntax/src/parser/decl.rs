//! `DIM name [AS type], ...` for scalars. Type names are kept as words; `sema` decides which it supports.

use super::Parser;
use crate::SyntaxKind::*;

pub(crate) fn dim_stmt(p: &mut Parser) {
    p.start_node(DimStmt);
    p.bump(); // DIM
    if p.at_word("SHARED") || p.at_word("_PRESERVE") {
        p.error(format!(
            "`DIM {}` is not supported yet",
            qb64rust_base::show_bytes(p.nth_text(0))
        ));
    } else {
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
    p.recover();
    p.finish_node();
}

fn dim_item(p: &mut Parser) -> bool {
    if !p.at(Ident) {
        p.error("expected a variable name");
        return false;
    }
    p.start_node(DimItem);
    p.bump();
    let mut ok = true;
    if p.at(LParen) {
        p.error("arrays are not supported yet");
        ok = false;
    } else if p.at_word("AS") {
        p.start_node(AsClause);
        p.bump();
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
    }
    p.finish_node();
    ok
}
