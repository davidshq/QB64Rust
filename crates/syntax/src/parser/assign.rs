//! `[LET] name = expression`.

use super::Parser;
use super::expr::expr;
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
        p.error("arrays are not supported yet");
    } else {
        p.error("expected `<name> = <expression>`");
    }
    p.recover();
    p.finish_node();
}
