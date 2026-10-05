//! `CALL name[(args)]` and `name [args]` (design D1). With `CALL`, the parentheses enclose the argument list.
//! Without it, the arguments follow the name without parentheses, so `bump (n)` has one parenthesized argument
//! (passed by value, D4).
//!
//! A statement that starts with a name is also how built-in statements look (`CLS`, `LOCATE , 5`, `COLOR 4, 1`),
//! and many of those have argument forms that are not expression lists. Only `sema` knows the built-in table, so
//! without `CALL` the arguments are parsed quietly: if they do not parse, the rest of the statement goes into an
//! `Error` node inside the `ArgList` and no diagnostic is reported here; `sema` reports the statement.

use super::Parser;
use super::expr::{arg_list, expr};
use crate::SyntaxKind::*;

pub(crate) fn call_stmt(p: &mut Parser) {
    p.start_node(CallStmt);
    let explicit = p.at_word("CALL");
    if explicit {
        p.bump();
    }
    if !p.at(Ident) {
        p.syntax_error("expected a SUB name after `CALL`");
        p.recover();
        p.finish_node();
        return;
    }
    p.start_node(NameRef);
    p.bump();
    p.finish_node();
    if explicit {
        if p.at(LParen) {
            arg_list(p);
        }
    } else if !p.at_stmt_end() {
        quiet_args(p);
    }
    p.recover();
    p.finish_node();
}

/// Arguments without parentheses: expressions separated by commas, up to the end of the statement.
fn quiet_args(p: &mut Parser) {
    p.start_node(ArgList);
    p.quiet = true;
    p.quiet_failed = false;
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
    // Also when nothing is left (`LOCATE 5,`): the `Error` node, empty then, is what tells `sema`.
    if p.quiet_failed || !p.at_stmt_end() {
        p.rest_into_error_node();
    }
    p.quiet = false;
    p.finish_node();
}
