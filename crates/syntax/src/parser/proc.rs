//! Procedures (design D1): `SUB`/`FUNCTION` blocks, their headers, `END SUB`/`END FUNCTION`, `EXIT SUB`/`EXIT
//! FUNCTION` and `DECLARE`.
//!
//! A `ProcDef` holds the `ProcHeader`, the body statements (parsed by the same statement loop as the main module)
//! and the closing `ProcEnd`. Recovery, so that one mistake never swallows the rest of the file:
//! - a `SUB`/`FUNCTION` inside a procedure is an error at the inner header, and that header ends the outer block;
//! - a missing `END SUB` is an error at the header; the block ends at the end of the file;
//! - `END FUNCTION` closing a SUB (or the reverse) is an error at that statement, which still ends the block;
//! - `END SUB` outside a procedure is an error at that statement (`Parser::end_stmt`).
//!
//! Everything after a procedure's `END SUB` belongs to the main module again.

use super::Parser;
use super::decl::as_clause;
use crate::SyntaxKind::*;

/// Parses a procedure at `SUB` or `FUNCTION`. Returns true when a nested header ended it; that header's error is
/// then already reported.
pub(crate) fn proc_def(p: &mut Parser) -> bool {
    p.start_node(ProcDef);
    let header_span = p.current_span().cover(p.next_span(1));
    let word = if p.at_word("SUB") { "SUB" } else { "FUNCTION" };
    proc_header(p);
    p.recover();
    if matches!(p.current(), Some(Newline | Colon)) {
        p.bump();
    }
    let mut nested = false;
    loop {
        p.stmt_error = false;
        if p.current().is_none() {
            p.error_at(header_span, format!("`{word}` without `END {word}`"));
            break;
        }
        if p.at_proc_start() {
            p.error(format!(
                "a SUB or FUNCTION cannot be defined inside another; expected `END {word}` before it"
            ));
            nested = true;
            break;
        }
        if p.at_word("END") && (p.nth_is_word(1, "SUB") || p.nth_is_word(1, "FUNCTION")) {
            proc_end(p, word);
            p.recover();
            if matches!(p.current(), Some(Newline | Colon)) {
                p.bump();
            }
            break;
        }
        p.statement_and_separator();
    }
    p.finish_node();
    nested
}

/// `END SUB` or `END FUNCTION`, closing a block opened by `word`.
fn proc_end(p: &mut Parser, word: &str) {
    p.start_node(ProcEnd);
    let span = p.current_span().cover(p.next_span(1));
    p.bump(); // END
    if !p.at_word(word) {
        let other = if word == "SUB" { "FUNCTION" } else { "SUB" };
        p.error_at(
            span,
            format!("`END {other}` cannot close a {word}; expected `END {word}`"),
        );
    }
    p.bump(); // SUB or FUNCTION
    p.finish_node();
}

/// `SUB name [(params)]` or `FUNCTION name[suffix] [(params)]`, at `SUB` or `FUNCTION`.
fn proc_header(p: &mut Parser) {
    p.start_node(ProcHeader);
    p.bump(); // SUB or FUNCTION
    if !p.at(Ident) {
        p.error("expected a procedure name");
        p.finish_node();
        return;
    }
    p.bump();
    if p.at(LParen) {
        param_list(p);
    }
    if p.at_word("STATIC") {
        p.error("`STATIC` after a procedure header is not supported yet");
    } else if p.at_word("AS") {
        p.error("a FUNCTION's type is given by a suffix on its name, not by `AS`");
    }
    p.finish_node();
}

fn param_list(p: &mut Parser) {
    p.start_node(ParamList);
    p.bump(); // (
    if !p.at(RParen) {
        loop {
            if !param(p) {
                break;
            }
            if p.at(Comma) {
                p.bump();
            } else {
                break;
            }
        }
    }
    p.expect(RParen, "`,` or `)`");
    p.finish_node();
}

/// `name[suffix] [AS type]`. Returns false after an error.
fn param(p: &mut Parser) -> bool {
    if !p.at(Ident) {
        p.error("expected a parameter name");
        return false;
    }
    p.start_node(Param);
    p.bump();
    let ok = if p.at(LParen) {
        p.error("array parameters are not supported yet");
        false
    } else if p.at_word("AS") {
        as_clause(p)
    } else {
        true
    };
    p.finish_node();
    ok
}

/// `EXIT SUB` / `EXIT FUNCTION`; other `EXIT` forms are not supported yet.
pub(crate) fn exit_stmt(p: &mut Parser) {
    if p.nth_is_word(1, "SUB") || p.nth_is_word(1, "FUNCTION") {
        p.start_node(ExitStmt);
        p.bump();
        p.bump();
        p.finish_node();
    } else {
        let span = p.current_span().cover(p.next_span(1));
        let text = format!("EXIT {}", qb64rust_base::show_bytes(p.nth_text(1)).to_ascii_uppercase());
        p.error_at(span, format!("`{}` is not supported yet", text.trim_end()));
        p.recover();
    }
}

/// `DECLARE SUB|FUNCTION <header>`; `DECLARE LIBRARY` and the like are not supported yet.
pub(crate) fn declare_stmt(p: &mut Parser) {
    if p.nth_is_word(1, "SUB") || p.nth_is_word(1, "FUNCTION") {
        p.start_node(DeclareStmt);
        p.bump(); // DECLARE
        proc_header(p);
        p.recover();
        p.finish_node();
    } else {
        let span = p.current_span().cover(p.next_span(1));
        let text = format!(
            "DECLARE {}",
            qb64rust_base::show_bytes(p.nth_text(1)).to_ascii_uppercase()
        );
        p.error_at(span, format!("`{}` is not supported yet", text.trim_end()));
        p.recover();
    }
}
