//! `PRINT` / `?` and `LPRINT`: `[#file,] [USING format;] items`, items separated by `;` or `,`. Next to a string
//! literal the separator may be left out; it then means `;` (the old compiler's auto-semicolon, `study\10` §2.2).
//! The file number (`FileNumber`, with its `,`) and the `USING` clause (`UsingClause`, with its `;`) are nodes of
//! their own, so the items are the statement's other children.

use super::Parser;
use super::expr::expr;
use crate::SyntaxKind::{self, *};

pub(crate) fn print_stmt(p: &mut Parser) {
    print_like(p, PrintStmt, true);
}

pub(crate) fn lprint_stmt(p: &mut Parser) {
    print_like(p, LprintStmt, false);
}

/// `PRINT` (`file` allowed) or `LPRINT`.
fn print_like(p: &mut Parser, kind: SyntaxKind, file: bool) {
    p.start_node(kind);
    p.bump(); // PRINT, ? or LPRINT
    if file && p.at(Hash) {
        file_number(p);
    }
    if !p.stmt_error && p.at_word("USING") {
        p.start_node(UsingClause);
        p.bump();
        if expr(p) {
            if p.at(Semicolon) || p.at(Comma) {
                p.bump();
            } else {
                p.syntax_error("expected `;` after the `USING` format");
            }
        }
        p.finish_node();
    }
    while !p.stmt_error && !p.at_stmt_end() {
        if p.at(Semicolon) || p.at(Comma) {
            p.bump();
            continue;
        }
        if !expr(p) {
            break;
        }
        let after_string = p.last_kind == Some(StringLit);
        if !(p.at_stmt_end() || p.at(Semicolon) || p.at(Comma) || after_string || p.at(StringLit)) {
            p.syntax_error("expected `;` or `,` between PRINT items");
        }
    }
    p.recover();
    p.finish_node();
}

/// `#n,` at `#`: a `FileNumber` node holding the `#`, the number and the `,` after it. Returns false after an
/// error.
pub(super) fn file_number(p: &mut Parser) -> bool {
    p.start_node(FileNumber);
    p.bump(); // #
    let ok = expr(p) && p.expect(Comma, "`,` after the file number");
    p.finish_node();
    ok
}
