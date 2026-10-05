//! Labels and error handling (design D1 of `m2-procedures-and-errors`): `name:`, `ON ERROR GOTO`, `RESUME` and
//! `ERROR`.
//!
//! A label is a name without suffix followed by `:` at the start of a statement, when the name is not a reserved
//! word (the old compiler's `validlabel`). The old compiler also refuses built-in names (`CLS:` is a call of `CLS`)
//! and may read a SUB name as a call. The parser knows the two built-in statements it parses itself (`END`,
//! `SYSTEM`); it cannot see the other built-ins or the procedures, so `sema` rejects those labels.

use super::expr::expr;
use super::{Parser, keywords, name_part};
use crate::SyntaxKind::*;

impl Parser<'_> {
    /// At `name:` that can be a label.
    pub(super) fn at_label(&self) -> bool {
        if !self.at(Ident) || self.nth(1) != Some(Colon) {
            return false;
        }
        let text = self.nth_text(0);
        name_part(text) == text && !keywords::is_keyword(text) && !self.at_word("END") && !self.at_word("SYSTEM")
    }
}

/// `name:` at a label.
pub(crate) fn label_def(p: &mut Parser) {
    p.start_node(LabelDef);
    p.bump(); // name
    p.bump(); // :
    p.finish_node();
}

/// `ON ERROR GOTO label` or `ON ERROR GOTO 0`; other `ON` statements are not supported yet.
pub(crate) fn on_stmt(p: &mut Parser) {
    for (n, lead, word) in [(1, "ON", "ERROR"), (2, "ON ERROR", "GOTO")] {
        if p.nth_is_word(n, word) {
            continue;
        }
        if matches!(p.nth(n), None | Some(Newline) | Some(Colon)) {
            let span = p.current_span().cover(p.next_span(n - 1));
            p.error_at(span, format!("expected `{word}` after `{lead}`"));
        } else {
            let span = p.current_span().cover(p.next_span(n));
            let next = qb64rust_base::show_bytes(&p.nth_text(n).to_ascii_uppercase());
            p.unsupported_at(span, format!("`{lead} {next}`"));
        }
        p.recover();
        return;
    }
    p.start_node(OnErrorStmt);
    p.bump(); // ON
    p.bump(); // ERROR
    p.bump(); // GOTO
    if p.at(Ident) || p.at(Number) {
        p.bump();
    } else {
        p.syntax_error("expected a label or `0` after `ON ERROR GOTO`");
    }
    p.recover();
    p.finish_node();
}

/// `RESUME`, `RESUME 0`, `RESUME NEXT` or `RESUME label`.
pub(crate) fn resume_stmt(p: &mut Parser) {
    p.start_node(ResumeStmt);
    p.bump(); // RESUME
    if p.at(Ident) || p.at(Number) {
        p.bump();
    }
    p.recover();
    p.finish_node();
}

/// `ERROR n`.
pub(crate) fn error_stmt(p: &mut Parser) {
    p.start_node(ErrorStmt);
    p.bump(); // ERROR
    expr(p);
    p.recover();
    p.finish_node();
}
