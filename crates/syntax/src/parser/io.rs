//! I/O statements without a `specialformat` template (design D5; those with one are task 7.5): `WRITE`, `INPUT`,
//! `LINE INPUT`, `CLOSE`, `FIELD`; and the assignment-like `LSET`, `RSET`, `SWAP`. A file number with its `,` is a
//! `FileNumber` node (`print.rs`); a prompt is a `StringLit` token of the statement, so that the expression
//! children are exactly the targets or items.

use super::Parser;
use super::expr::expr;
use super::print::file_number;
use crate::SyntaxKind::*;

/// `WRITE [#n,] [item, ...] [,]`.
pub(crate) fn write_stmt(p: &mut Parser) {
    p.start_node(WriteStmt);
    p.bump(); // WRITE
    if p.at(Hash) {
        file_number(p);
    }
    // A trailing comma is accepted (measured, `verification\v22_x59`, `x60`: `WRITE 1,` prints `1,` without a line
    // end).
    while !p.stmt_error && !p.at_stmt_end() {
        if !expr(p) {
            break;
        }
        if p.at(Comma) {
            p.bump();
        } else {
            break;
        }
    }
    p.recover();
    p.finish_node();
}

/// `INPUT [;] ["prompt" {;|,}] target, ...` or `INPUT #n, target, ...`.
pub(crate) fn input_stmt(p: &mut Parser) {
    p.start_node(InputStmt);
    p.bump(); // INPUT
    input_rest(p);
    p.recover();
    p.finish_node();
}

/// `LINE INPUT [;] ["prompt" {;|,}] target` or `LINE INPUT #n, target`.
pub(crate) fn line_input_stmt(p: &mut Parser) {
    p.start_node(LineInputStmt);
    p.bump(); // LINE
    p.bump(); // INPUT
    input_rest(p);
    p.recover();
    p.finish_node();
}

fn input_rest(p: &mut Parser) {
    if p.at(Hash) {
        if !file_number(p) {
            return;
        }
    } else {
        if p.at(Semicolon) {
            p.bump();
        }
        if p.at(StringLit) && matches!(p.nth(1), Some(Semicolon | Comma)) {
            p.bump();
            p.bump();
        }
    }
    expr_list(p);
}

/// `CLOSE [[#]n, ...]`.
pub(crate) fn close_stmt(p: &mut Parser) {
    p.start_node(CloseStmt);
    p.bump(); // CLOSE
    while !p.at_stmt_end() {
        if p.at(Hash) {
            p.bump();
        }
        if !expr(p) {
            break;
        }
        if p.at(Comma) {
            p.bump();
        } else {
            break;
        }
    }
    p.recover();
    p.finish_node();
}

/// `FIELD [#]n, width AS target, ...`.
pub(crate) fn field_stmt(p: &mut Parser) {
    p.start_node(FieldStmt);
    p.bump(); // FIELD
    let ok = if p.at(Hash) {
        file_number(p)
    } else {
        expr(p) && p.expect(Comma, "`,` after the file number")
    };
    if ok {
        loop {
            if !expr(p) {
                break;
            }
            if !p.at_word("AS") {
                p.syntax_error("expected `AS` and a variable");
                break;
            }
            p.bump();
            if !expr(p) {
                break;
            }
            if p.at(Comma) {
                p.bump();
            } else {
                break;
            }
        }
    }
    p.recover();
    p.finish_node();
}

/// `LSET target = value` or `RSET target = value`.
pub(crate) fn set_stmt(p: &mut Parser) {
    p.start_node(LsetStmt);
    p.bump(); // LSET or RSET
    if expr_target(p) && p.expect(Eq, "`=`") {
        expr(p);
    }
    p.recover();
    p.finish_node();
}

/// `SWAP a, b`.
pub(crate) fn swap_stmt(p: &mut Parser) {
    p.start_node(SwapStmt);
    p.bump(); // SWAP
    if expr_target(p) && p.expect(Comma, "`,` between the two variables") {
        expr_target(p);
    }
    p.recover();
    p.finish_node();
}

/// `_MEMPUT mem, offset, value [AS type]` or `_MEMFILL mem, offset, bytes, value [AS type]`: arguments, then the
/// type of the value (built-ins without a template; the type ends the last argument).
pub(crate) fn mem_stmt(p: &mut Parser) {
    p.start_node(MemStmt);
    p.bump(); // _MEMPUT or _MEMFILL
    expr_list(p);
    if !p.stmt_error && p.at_word("AS") {
        super::decl::as_clause(p);
    }
    p.recover();
    p.finish_node();
}

/// `_ARRAYCOPY source TO target`; each side an array, a slice of one (`a(2 TO 3, 2 TO 4)`) or a member array, in
/// the shape of a `REDIM` item (a `DimItem` with `ArrayBounds`).
pub(crate) fn array_copy_stmt(p: &mut Parser) {
    p.start_node(ArrayCopyStmt);
    p.bump(); // _ARRAYCOPY
    p.redim_items = true;
    if super::decl::dim_item(p) {
        if p.at_word("TO") {
            p.bump();
            super::decl::dim_item(p);
        } else {
            p.syntax_error("expected `TO` and the target array");
        }
    }
    p.redim_items = false;
    p.recover();
    p.finish_node();
}

/// A variable, element or member as an expression. The expression parser would read `a = b` as a comparison, so
/// the target is parsed above the comparison operators.
fn expr_target(p: &mut Parser) -> bool {
    super::expr::expr_above_comparison(p)
}

/// `expr, expr, ...`; an item may be left out (`INPUT a, , b` is an error later, `WRITE` takes none).
fn expr_list(p: &mut Parser) {
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
}
