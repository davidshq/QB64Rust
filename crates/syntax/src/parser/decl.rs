//! `DIM [SHARED] name [AS type], ...`, `SHARED ...` and `STATIC ...` for scalars. Type names are kept as words;
//! `sema` decides which it supports. Also `CONST name = expr, ...` and `OPTION word` (design D2 of
//! `m2-control-flow-slice`).

use super::Parser;
use super::expr::expr;
use crate::SyntaxKind::{self, *};

pub(crate) fn dim_stmt(p: &mut Parser) {
    p.start_node(DimStmt);
    p.bump(); // DIM
    if p.at_word("SHARED") {
        p.bump();
    }
    if p.at_word("_PRESERVE") {
        p.unsupported("`DIM _PRESERVE`");
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
        p.syntax_error("expected a variable name");
        return false;
    }
    p.start_node(DimItem);
    p.bump();
    let ok = if p.at(LParen) {
        p.unsupported("arrays");
        false
    } else if p.at_word("AS") {
        as_clause(p)
    } else {
        true
    };
    p.finish_node();
    ok
}

/// `CONST name[suffix] = expr {, name[suffix] = expr}`; each item a `ConstItem`.
pub(crate) fn const_stmt(p: &mut Parser) {
    p.start_node(ConstStmt);
    p.bump(); // CONST
    p.in_const = true;
    loop {
        if !const_item(p) {
            break;
        }
        if p.at(Comma) {
            p.bump();
        } else {
            break;
        }
    }
    p.recover();
    p.in_const = false;
    p.finish_node();
}

fn const_item(p: &mut Parser) -> bool {
    if !p.at(Ident) {
        p.syntax_error("expected a constant name");
        return false;
    }
    p.start_node(ConstItem);
    p.bump();
    let ok = p.expect(Eq, "`=`") && expr(p);
    p.finish_node();
    ok
}

/// `OPTION` and one word: `BASE n`, `_EXPLICIT` or `_EXPLICITARRAY`. The spellings without the underscore are
/// kept too: they are valid only after `$NOPREFIX`, which `sema` knows about, not the parser.
pub(crate) fn option_stmt(p: &mut Parser) {
    p.start_node(OptionStmt);
    p.bump(); // OPTION
    if p.at_word("BASE") {
        p.bump();
        p.expect(Number, "`0` or `1` after `OPTION BASE`");
    } else if ["_EXPLICIT", "_EXPLICITARRAY", "EXPLICIT", "EXPLICITARRAY"]
        .iter()
        .any(|w| p.at_word(w))
    {
        p.bump();
    } else {
        p.error("expected `BASE`, `_EXPLICIT` or `_EXPLICITARRAY` after `OPTION`");
    }
    p.recover();
    p.finish_node();
}

/// `AS <type words>` at `AS`. Returns false after an error.
pub(super) fn as_clause(p: &mut Parser) -> bool {
    p.start_node(AsClause);
    p.bump();
    let mut ok = true;
    if !p.at(Ident) {
        p.syntax_error("expected a type name after `AS`");
        ok = false;
    }
    while p.at(Ident) {
        p.bump();
    }
    if p.at(Star) {
        p.unsupported("fixed-length strings");
        ok = false;
    }
    p.finish_node();
    ok
}
