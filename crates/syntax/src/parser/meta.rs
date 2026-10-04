//! Metacommands. The slice supports `$CONSOLE:ONLY` only; `sema` checks the text, the parser keeps any `$…` line
//! as one statement.

use super::Parser;
use crate::SyntaxKind::*;

pub(crate) fn metacommand(p: &mut Parser) {
    p.start_node(MetaStmt);
    p.bump();
    p.finish_node();
}
