//! `[LET] target = expression`. The target is a name, or an index or member access (`a(1)`, `a(1).b`), which
//! `sema` does not support yet.

use super::Parser;
use super::expr::{arg_list, expr, fields};
use crate::SyntaxKind::*;

pub(crate) fn assign_stmt(p: &mut Parser) {
    p.start_node(AssignStmt);
    if p.at_word("LET") {
        p.bump();
    }
    if p.at(Ident) && p.nth(1) == Some(Eq) {
        p.start_node(NameRef);
        p.bump();
        p.finish_node();
        p.bump(); // =
        expr(p);
    } else if p.at(Ident) && p.nth(1) == Some(LParen) {
        if indexed_target(p) && p.expect(Eq, "`=`") {
            expr(p);
        }
    } else {
        p.syntax_error("expected `<name> = <expression>`");
    }
    p.recover();
    p.finish_node();
}

/// `name(args)` and any member accesses after it, as a `CallExpr` or `FieldExpr`. Returns false after an error.
fn indexed_target(p: &mut Parser) -> bool {
    p.eat_trivia();
    let cp = p.builder.checkpoint();
    p.start_node(CallExpr);
    p.start_node(NameRef);
    p.bump();
    p.finish_node();
    let ok = arg_list(p);
    p.finish_node();
    ok && fields(p, cp)
}
