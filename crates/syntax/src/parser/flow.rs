//! Line numbers and plain control transfer (design D3, D5): `10 PRINT`, `GOTO`, `GOSUB`, `RETURN`. The `ON …`
//! forms other than `ON ERROR` come with task 7.3.
//!
//! A line number is a `Number` at the start of a line (measured M3): decimals, suffixes and numbers above 32 bits
//! are accepted, and a label may follow it (`10 lab: PRINT`). A number anywhere else where a statement starts
//! (after a label or a `:`) is a syntax error in the old compiler, and here.

use super::Parser;
use crate::SyntaxKind::{self, *};

impl Parser<'_> {
    /// At the start of a line (only a line end, or nothing, before it).
    fn at_line_start(&self) -> bool {
        matches!(self.last_kind, None | Some(Newline))
    }

    /// A line number, if the line starts with one. Taken before looking for a `SUB`/`FUNCTION` header, which may
    /// follow it (`verification\v16_m3_sub_header`, `v16_m3_nested_sub`).
    pub(super) fn line_number(&mut self) {
        if self.at(Number) && self.at_line_start() {
            self.start_node(LineNumber);
            self.bump();
            self.finish_node();
        }
    }

    /// What may stand before a statement: a line number at the start of a line, then labels.
    pub(super) fn line_prefix(&mut self) {
        self.line_number();
        while self.at_label() {
            super::errors::label_def(self);
        }
    }
}

/// `GOTO`, `GOSUB` or `RETURN` (`kind` says which) and its target, at the word. `RETURN` may stand alone.
pub(crate) fn jump_stmt(p: &mut Parser, kind: SyntaxKind) {
    p.start_node(kind);
    p.bump(); // GOTO, GOSUB or RETURN
    if p.at(Ident) || p.at(Number) {
        p.bump();
    } else if kind != ReturnStmt || !p.at_stmt_end() {
        p.syntax_error("expected a label or line number");
    }
    p.recover();
    p.finish_node();
}
