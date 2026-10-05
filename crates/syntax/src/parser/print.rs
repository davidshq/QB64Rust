//! `PRINT` / `?`: items separated by `;` or `,`. Next to a string literal the separator may be left out; it then
//! means `;` (the old compiler's auto-semicolon, `study\10` §2.2).

use super::Parser;
use super::expr::expr;
use crate::SyntaxKind::*;

pub(crate) fn print_stmt(p: &mut Parser) {
    p.start_node(PrintStmt);
    p.bump(); // PRINT or ?
    if p.at(Hash) {
        p.unsupported("`PRINT #`");
    }
    while !p.stmt_error && !p.at_stmt_end() {
        if p.at(Semicolon) || p.at(Comma) {
            p.bump();
            continue;
        }
        if p.at_word("USING") {
            p.unsupported("`PRINT USING`");
            break;
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
