//! Expressions: Pratt parsing with the old compiler's 16 precedence levels (`study\02` §1.3). All binary
//! operators are left-associative; unary minus (level 15) binds looser than `^` (16), so `-2 ^ 2` is `-(2 ^ 2)`
//! and `2 ^ -3 * 4` is `(2 ^ (-3)) * 4`; `NOT` (9) takes everything from comparisons up, so `NOT a = b` is
//! `NOT (a = b)`.

use super::Parser;
use crate::SyntaxKind::{self, *};
use crate::tree::Checkpoint;

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
    "MOD", "AND", "OR", "XOR", "EQV", "IMP", "_ANDALSO", "_ORELSE", "THEN", "TO", "ELSE",
];

/// How deep the nodes of one expression may nest (parentheses, calls, operators, member accesses, counted as
/// tree levels). Every later stage walks expressions recursively, so this bounds their stack use. A left-associative
/// chain (`1 + 1 + …`) is parsed in a loop but nests one level per operator, so it counts too. The old compiler
/// overflows its own stack at 150 to 200 nested parentheses and at under 50 nested calls, but takes a flat chain of
/// 3,000 operators (measured 2026-10-07); a deeper expression is marked "not supported yet". Of the 1,780 `.bas`,
/// `.bi` and `.bm` files of the corpus, the upstream tests and the reference clone, one goes beyond 40 levels.
pub(super) const MAX_EXPR_DEPTH: u32 = 1000;

pub(crate) fn expr(p: &mut Parser) -> bool {
    expr_bp(p, 0).is_some()
}

/// Parses an expression whose binary operators have level `min` or higher. Returns the height of the node it built
/// (a name is 1), or `None` after an error.
fn expr_bp(p: &mut Parser, min: u8) -> Option<u32> {
    p.eat_trivia();
    if !p.within_expr_depth(1) {
        return None;
    }
    p.expr_nesting += 1;
    let height = expr_bp_inner(p, min);
    p.expr_nesting -= 1;
    height
}

fn expr_bp_inner(p: &mut Parser, min: u8) -> Option<u32> {
    let cp = p.builder.checkpoint();
    let mut height = if let Some(level) = prefix_level(p) {
        p.start_node(PrefixExpr);
        p.bump();
        let operand = expr_bp(p, level);
        p.finish_node();
        operand? + 1
    } else {
        let h = primary(p)?;
        fields(p, cp, h)?
    };
    while let Some(level) = binary_level(p) {
        if level < min {
            break;
        }
        // The `BinExpr` takes the place of the operand built so far (counted in `expr_nesting`), one level higher.
        if !p.within_expr_depth(height) {
            return None;
        }
        p.builder.start_node_at(cp, BinExpr);
        p.bump();
        let rhs = expr_bp(p, level + 1);
        p.finish_node();
        height = height.max(rhs?) + 1;
    }
    Some(height)
}

/// Height of the operand built, or `None` after an error.
fn primary(p: &mut Parser) -> Option<u32> {
    match p.current() {
        Some(Number) | Some(SyntaxKind::StringLit) => {
            p.start_node(Literal);
            p.bump();
            p.finish_node();
            Some(1)
        }
        Some(LParen) => {
            p.start_node(ParenExpr);
            p.bump();
            let inner = expr_bp(p, 0);
            let ok = inner.is_some() && p.expect(RParen, "`)`");
            p.finish_node();
            ok.then(|| inner.unwrap_or(0) + 1)
        }
        Some(Ident) if !RESERVED_IN_EXPR.iter().any(|w| p.at_word(w)) => {
            if p.nth(1) == Some(LParen) {
                p.start_node(CallExpr);
                p.start_node(NameRef);
                p.bump();
                p.finish_node();
                let args = args_height(p);
                p.finish_node();
                Some(args? + 1)
            } else {
                p.start_node(NameRef);
                p.bump();
                p.finish_node();
                Some(1)
            }
        }
        _ => {
            p.syntax_error("expected an expression");
            None
        }
    }
}

/// Member accesses after the operand that started at `cp`, of height `height`: each `. name [(args)]` wraps what
/// came before in a `FieldExpr`. The lexer gives a `Dot` only after `)` or a member name. Returns the new height,
/// or `None` after an error. The operand's place must be counted in `expr_nesting`.
pub(super) fn fields(p: &mut Parser, cp: Checkpoint, mut height: u32) -> Option<u32> {
    while p.at(Dot) {
        // The `FieldExpr` takes the operand's place, one level higher.
        if !p.within_expr_depth(height) {
            return None;
        }
        p.builder.start_node_at(cp, FieldExpr);
        p.bump(); // .
        let mut ok = p.expect(Ident, "a member name after `.`");
        if ok && p.at(LParen) {
            match args_height(p) {
                Some(args) => height = height.max(args),
                None => ok = false,
            }
        }
        p.finish_node();
        if !ok {
            return None;
        }
        height += 1;
    }
    Some(height)
}

/// `(` arguments separated by commas `)`, at the `(`. An argument may be left out (`f(a, , b)`, `f(, b)`); `()`
/// is an empty list, not one omitted argument.
pub(super) fn arg_list(p: &mut Parser) -> bool {
    args_height(p).is_some()
}

/// [`arg_list`], returning the height of the `ArgList` node (1 when it is empty) or `None` after an error. Inside
/// an expression the call or member access that holds the list is counted in `expr_nesting`; a statement's own
/// list (`CALL s(…)`) is not inside one.
pub(super) fn args_height(p: &mut Parser) -> Option<u32> {
    p.start_node(ArgList);
    p.bump(); // (
    let mut ok = true;
    let mut height = 0;
    if !p.at(RParen) {
        loop {
            let omitted = p.at(Comma) || p.at(RParen);
            if !omitted {
                // The arguments sit one level below the `ArgList`.
                p.expr_nesting += 1;
                let arg = expr_bp(p, 0);
                p.expr_nesting -= 1;
                match arg {
                    Some(h) => height = height.max(h),
                    None => {
                        ok = false;
                        break;
                    }
                }
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
    ok.then_some(height + 1)
}
