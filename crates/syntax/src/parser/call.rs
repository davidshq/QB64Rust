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
use qb64rust_builtins::statement_templates;
use qb64rust_builtins::template::Item;

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
    // After `CALL`, a built-in statement with a template keeps the template's forms between the parentheses
    // (measured, `verification\v26_x74`: `CALL _MOUSECURSOR(1, (2, 3))`, `CALL _MOUSEHIDE(_HIDE)`), which are not
    // an expression list. They are left unread, as arguments without `CALL` that do not parse are; `sema` reports
    // the statement. A template of one argument is an expression list (`CALL _DEST(h)`).
    let by_template = explicit && {
        let forms = statement_templates(p.nth_text(0));
        !forms.is_empty() && !forms.iter().any(|(_, items)| matches!(items[..], [Item::Arg]))
    };
    p.start_node(NameRef);
    p.bump();
    p.finish_node();
    if by_template {
        if p.at(LParen) {
            p.start_node(ArgList);
            p.rest_into_error_node();
            p.finish_node();
        }
    } else if explicit {
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
