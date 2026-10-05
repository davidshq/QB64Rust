//! Expressions: Pratt parsing with the old compiler's 16 precedence levels (`study\02` §1.3). All binary
//! operators are left-associative; unary minus (level 15) binds looser than `^` (16), so `-2 ^ 2` is `-(2 ^ 2)`
//! and `2 ^ -3 * 4` is `(2 ^ (-3)) * 4`; `NOT` (9) takes everything from comparisons up, so `NOT a = b` is
//! `NOT (a = b)`.

use super::Parser;
use crate::SyntaxKind::{self, *};

/// Precedence level of a binary operator at the current token, if it is one.
fn binary_level(p: &Parser) -> Option<u8> {
    let level = match p.current()? {
        Caret => 16,
        Star | Slash => 14,
        Backslash => 13,
        Plus | Minus => 11,
        Eq | Lt | Gt | Le | Ge | Ne => 10,
        Ident => {
            let t = p.nth_text(0);
            let is = |w: &str| t.eq_ignore_ascii_case(w.as_bytes());
            if is("MOD") {
                12
            } else if is("AND") {
                7
            } else if is("OR") {
                6
            } else if is("XOR") {
                5
            } else if is("EQV") {
                4
            } else if is("IMP") {
                3
            } else if is("_ANDALSO") {
                2
            } else if is("_ORELSE") {
                1
            } else {
                return None;
            }
        }
        _ => return None,
    };
    Some(level)
}

/// Precedence level of a prefix operator at the current token: the operand takes operators of this level and up.
/// There is no unary `+` (measured: `PRINT +5`, `x = +n` and `2 * +n` are compile errors, `verification\v15_plus_*`).
fn prefix_level(p: &Parser) -> Option<u8> {
    match p.current()? {
        Minus => Some(15),
        Ident if p.at_word("NOT") => Some(9),
        Ident if p.at_word("_NEGATE") => Some(8),
        _ => None,
    }
}

/// Word operators and other words that cannot start an operand.
const RESERVED_IN_EXPR: &[&str] = &[
    "MOD", "AND", "OR", "XOR", "EQV", "IMP", "_ANDALSO", "_ORELSE", "THEN", "TO",
];

pub(crate) fn expr(p: &mut Parser) -> bool {
    expr_bp(p, 0)
}

/// Parses an expression whose binary operators have level `min` or higher. Returns false after an error.
fn expr_bp(p: &mut Parser, min: u8) -> bool {
    p.eat_trivia();
    let cp = p.builder.checkpoint();
    if let Some(level) = prefix_level(p) {
        p.start_node(PrefixExpr);
        p.bump();
        let ok = expr_bp(p, level);
        p.finish_node();
        if !ok {
            return false;
        }
    } else if !primary(p) {
        return false;
    }
    while let Some(level) = binary_level(p) {
        if level < min {
            break;
        }
        p.builder.start_node_at(cp, BinExpr);
        p.bump();
        let ok = expr_bp(p, level + 1);
        p.finish_node();
        if !ok {
            return false;
        }
    }
    true
}

fn primary(p: &mut Parser) -> bool {
    match p.current() {
        Some(Number) | Some(SyntaxKind::StringLit) => {
            p.start_node(Literal);
            p.bump();
            p.finish_node();
            true
        }
        Some(LParen) => {
            p.start_node(ParenExpr);
            p.bump();
            let ok = expr(p) && p.expect(RParen, "`)`");
            p.finish_node();
            ok
        }
        Some(Ident) if !RESERVED_IN_EXPR.iter().any(|w| p.at_word(w)) => {
            if p.nth(1) == Some(LParen) {
                p.start_node(CallExpr);
                p.start_node(NameRef);
                p.bump();
                p.finish_node();
                let ok = arg_list(p);
                p.finish_node();
                ok
            } else {
                p.start_node(NameRef);
                p.bump();
                p.finish_node();
                true
            }
        }
        _ => {
            p.syntax_error("expected an expression");
            false
        }
    }
}

/// `(` expressions separated by commas `)`, at the `(`.
pub(super) fn arg_list(p: &mut Parser) -> bool {
    p.start_node(ArgList);
    p.bump(); // (
    let mut ok = true;
    if !p.at(RParen) {
        loop {
            if !expr(p) {
                ok = false;
                break;
            }
            if p.at(Comma) {
                p.bump();
            } else {
                break;
            }
        }
    }
    let ok = ok && p.expect(RParen, "`,` or `)`");
    p.finish_node();
    ok
}
