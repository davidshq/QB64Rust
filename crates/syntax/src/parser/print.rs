//! `PRINT` / `?` and `LPRINT`: `[#file,] [items] [USING format; values]`, items and values separated by `;` or
//! `,`. Next to a string literal the separator may be left out; it then means `;` (the old compiler's
//! auto-semicolon, `study\10` §2.2). The file number (`FileNumber`, with its `,`) and the `USING` clause
//! (`UsingClause`: the word, the format and its `;`) are nodes of their own, so the items before the clause and
//! the values after it are the statement's other children.

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
    } else if p.at(Hash) {
        // Measured: "Invalid expression" for `LPRINT #1, "a"`.
        p.error("`LPRINT` takes no file number");
    }
    while !p.stmt_error && !p.at_stmt_end() {
        if p.at(Semicolon) || p.at(Comma) {
            p.bump();
            continue;
        }
        // `USING format;` stands first or after items, with or without a separator before it (measured,
        // `verification\v30_x*`: `PRINT "a"; USING "##"; 5`, `PRINT 1 USING "##"; 5`). A second one is an error
        // there; it is read here too, and `sema` reports it.
        if p.at_word("USING") {
            p.start_node(UsingClause);
            p.bump();
            if p.at(Semicolon) || p.at(Comma) || p.at(Hash) || p.at_stmt_end() {
                // Measured: "Syntax error" for `PRINT USING ; 5`, "Expected PRINT USING formatstring ; ..." for
                // `PRINT USING #1, "##"; 5` and for `PRINT USING` alone (`verification\v30_x05`, `x17`).
                p.error("expected the format string after `USING`");
            } else if expr(p) {
                if p.at(Semicolon) || p.at(Comma) {
                    p.bump();
                } else {
                    p.syntax_error("expected `;` after the `USING` format");
                }
            }
            p.finish_node();
            continue;
        }
        if !expr(p) {
            break;
        }
        let after_string = p.last_kind == Some(StringLit);
        let separated = p.at_stmt_end() || p.at(Semicolon) || p.at(Comma) || p.at_word("USING");
        if !(separated || after_string || p.at(StringLit)) {
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
