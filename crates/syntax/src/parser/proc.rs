//! Procedures (design D1): `SUB`/`FUNCTION` blocks, their headers, `END SUB`/`END FUNCTION`, `EXIT` and `DECLARE`,
//! including `DECLARE LIBRARY` blocks.
//!
//! A `ProcDef` holds the `ProcHeader`, the body statements (parsed by the statement loop of `blocks.rs`, as the
//! main module) and the closing `ProcEnd`. Recovery, so that one mistake never swallows the rest of the file:
//! - a `SUB`/`FUNCTION` inside a procedure is an error at the inner header, and that header ends the outer block
//!   (and every block open in it);
//! - a missing `END SUB` is an error at the header; the block ends at the end of the file;
//! - `END FUNCTION` closing a SUB (or the reverse) is an error at that statement, which still ends the block;
//! - `END SUB` outside a procedure is an error at that statement (a stray closer, `blocks.rs`).
//!
//! Everything after a procedure's `END SUB` belongs to the main module again.

use super::Parser;
use super::blocks::{Block, Closer, Stop};
use super::decl::as_clause;
use crate::SyntaxKind::*;

/// Parses a procedure at `SUB` or `FUNCTION`. Returns true when a nested header ended it; that header's error is
/// then already reported.
pub(crate) fn proc_def(p: &mut Parser) -> bool {
    p.start_node(ProcDef);
    let header_span = p.current_span().cover(p.next_span(1));
    let word = if p.at_word("SUB") { "SUB" } else { "FUNCTION" };
    proc_header(p, false);
    p.header_end();
    p.push_block(Block::Proc, header_span);
    let nested = loop {
        match p.body() {
            Stop::Eof => {
                p.block_error(header_span, format!("`{word}` without `END {word}`"));
                break false;
            }
            Stop::ProcHeader => {
                p.error(format!(
                    "a SUB or FUNCTION cannot be defined inside another; expected `END {word}` before it"
                ));
                break true;
            }
            Stop::Closer(Closer::EndProc) => {
                proc_end(p, word);
                p.header_end();
                break false;
            }
            // Cannot happen: only `END SUB`/`END FUNCTION` closes a procedure, and nothing is open around one. If it
            // did, take a token so that the loop always ends.
            Stop::Closer(_) | Stop::Outer | Stop::LineEnd | Stop::InnerNext => p.bump(),
        }
    };
    p.pop_block();
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

/// `SUB name [(params)]` or `FUNCTION name[suffix] [(params)]`, at `SUB` or `FUNCTION`. Inside `DECLARE LIBRARY`
/// (`library`) the name may be followed by `ALIAS "c_name"` or `ALIAS c_name`, and parameters may be `BYVAL`.
pub(super) fn proc_header(p: &mut Parser, library: bool) {
    p.start_node(ProcHeader);
    p.bump(); // SUB or FUNCTION
    if !p.at(Ident) {
        p.syntax_error("expected a procedure name");
        p.finish_node();
        return;
    }
    p.bump();
    if library && p.at_word("ALIAS") {
        p.bump();
        if p.at(StringLit) || p.at(Ident) {
            p.bump();
        } else {
            p.syntax_error("expected a name after `ALIAS`");
        }
    }
    if p.at(LParen) {
        param_list(p, library);
    }
    if p.at_word("STATIC") {
        p.unsupported("`STATIC` after a procedure header");
    } else if p.at_word("AS") {
        p.error("a FUNCTION's type is given by a suffix on its name, not by `AS`");
    }
    p.finish_node();
}

/// `(param, ...)`, at `(`.
pub(super) fn param_list(p: &mut Parser, library: bool) {
    p.start_node(ParamList);
    p.bump(); // (
    if !p.at(RParen) {
        loop {
            if !param(p, library) {
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

/// `[BYVAL] name[suffix] [AS type]` (`BYVAL` only with `library`). Returns false after an error.
fn param(p: &mut Parser, library: bool) -> bool {
    if !p.at(Ident) {
        p.syntax_error("expected a parameter name");
        return false;
    }
    p.start_node(Param);
    if library && p.at_word("BYVAL") && p.nth(1) == Some(Ident) {
        p.bump();
    }
    p.bump();
    let ok = if p.at(LParen) {
        p.unsupported("array parameters");
        false
    } else if p.at_word("AS") {
        as_clause(p)
    } else {
        true
    };
    p.finish_node();
    ok
}

/// `EXIT SUB|FUNCTION|FOR|DO|WHILE|SELECT|CASE|DEF`; other `EXIT` forms are not supported yet. A loop or `SELECT`
/// exit needs that block open in the current procedure (measured M4: `EXIT FOR` outside one is an error, also
/// from inside other blocks it is fine).
pub(crate) fn exit_stmt(p: &mut Parser) {
    let span = p.current_span().cover(p.next_span(1));
    if p.nth_is_word(1, "SUB") || p.nth_is_word(1, "FUNCTION") {
        p.start_node(ExitStmt);
        p.bump();
        p.bump();
        p.finish_node();
        return;
    }
    let target = [
        ("FOR", Block::For, "FOR"),
        ("DO", Block::Do, "DO"),
        ("WHILE", Block::While, "WHILE"),
        ("SELECT", Block::Select, "SELECT CASE"),
        ("CASE", Block::Select, "SELECT CASE"),
        ("DEF", Block::DefFn, "DEF FN"),
    ]
    .into_iter()
    .find(|(w, _, _)| p.nth_is_word(1, w));
    match target {
        Some((word, block, opener)) => {
            p.start_node(ExitStmt);
            if !p.inside(block) {
                p.error_at(span, format!("`EXIT {word}` without `{opener}`"));
            }
            p.bump();
            p.bump();
            p.recover();
            p.finish_node();
        }
        None => {
            let text = format!("EXIT {}", qb64rust_base::show_bytes(p.nth_text(1)).to_ascii_uppercase());
            p.unsupported_at(span, format!("`{}`", text.trim_end()));
            p.recover();
        }
    }
}

/// `DECLARE SUB|FUNCTION <header>`, or a `DECLARE … LIBRARY` block; other `DECLARE` forms are not supported yet.
/// Returns true for the block (see `Parser::statement`).
pub(crate) fn declare_stmt(p: &mut Parser) -> bool {
    if p.nth_is_word(1, "SUB") || p.nth_is_word(1, "FUNCTION") {
        p.start_node(DeclareStmt);
        p.bump(); // DECLARE
        proc_header(p, false);
        p.recover();
        p.finish_node();
        false
    } else if p.nth_is_word(1, "LIBRARY")
        || (["CUSTOMTYPE", "DYNAMIC", "STATIC"].iter().any(|w| p.nth_is_word(1, w)) && p.nth_is_word(2, "LIBRARY"))
    {
        declare_library_block(p);
        true
    } else {
        let span = p.current_span().cover(p.next_span(1));
        let text = format!(
            "DECLARE {}",
            qb64rust_base::show_bytes(p.nth_text(1)).to_ascii_uppercase()
        );
        p.unsupported_at(span, format!("`{}`", text.trim_end()));
        p.recover();
        false
    }
}

/// `DECLARE [CUSTOMTYPE|DYNAMIC|STATIC] LIBRARY ["name", ...]`, then procedure headers up to `END DECLARE`. Only
/// headers, comments and metacommands may stand inside (measured M4: a statement there is an error). After such
/// a statement, or at the end of the file, the block ends with an error and the rest is parsed as usual.
fn declare_library_block(p: &mut Parser) {
    p.start_node(DeclareLibraryBlock);
    let header_span = p.current_span().cover(p.next_span(1));
    p.start_node(DeclareLibraryHeader);
    p.bump(); // DECLARE
    if !p.at_word("LIBRARY") {
        p.bump(); // CUSTOMTYPE, DYNAMIC or STATIC
    }
    p.bump(); // LIBRARY
    while p.at(StringLit) {
        p.bump();
        if p.at(Comma) && p.nth(1) == Some(StringLit) {
            p.bump();
        }
    }
    p.finish_node();
    p.header_end();
    loop {
        p.stmt_error = false;
        if p.current().is_none() {
            p.block_error(header_span, "`DECLARE LIBRARY` without `END DECLARE`");
            break;
        }
        if p.skip_blank_or_meta() {
            continue;
        }
        if p.closer_here() == Some(Closer::EndDeclare) {
            p.block_end();
            break;
        }
        if p.at_proc_start() {
            proc_header(p, true);
            p.header_end();
            continue;
        }
        p.error("expected a SUB or FUNCTION declaration or `END DECLARE`");
        p.carry_error = true;
        break;
    }
    p.finish_node();
}
