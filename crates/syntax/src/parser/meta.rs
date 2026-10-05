//! Metacommands and metacommand comments. The slice supports `$CONSOLE:ONLY` only; `sema` checks the text, the
//! parser keeps any `$…` line as one statement, and a metacommand comment as another (`crate::meta`).

use super::Parser;
use crate::SyntaxKind::*;

pub(crate) fn metacommand(p: &mut Parser) {
    p.start_node(MetaStmt);
    p.bump();
    p.finish_node();
}

/// A metacommand comment (`'$INCLUDE:'x.bi'`) as a statement of its own; `sema` reads its directives.
pub(crate) fn meta_comment(p: &mut Parser) {
    p.start_node(MetaCommentStmt);
    p.bump();
    p.finish_node();
}
