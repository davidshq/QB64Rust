//! Expressions: Pratt parsing with the old compiler's 16 precedence levels (`study\02` §1.3). All binary
//! operators are left-associative; unary minus (level 15) binds looser than `^` (16), so `-2 ^ 2` is `-(2 ^ 2)`
//! and `2 ^ -3 * 4` is `(2 ^ (-3)) * 4`; `NOT` (9) takes everything from comparisons up, so `NOT a = b` is
//! `NOT (a = b)`.

use super::Parser;
use crate::SyntaxKind::{self, *};
use crate::pp::Feature;
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
            } else if p.in_const && is("ROOT") {
                // The old compiler's constant evaluator: `exponent := numeric '^' unary | numeric 'ROOT' unary`
                // (`const_eval.bas` 157–158).
                16
            } else {
                return None;
            }
        }
        _ => return None,
    };
    Some(level)
}

/// Precedence level of a prefix operator at the current token: the operand takes operators of this level and up.
/// There is no unary `+` (measured: `PRINT +5`, `x = +n` and `2 * +n` are compile errors, `verification\v15_plus_*`);
/// a `+` is only dropped after another sign ([`signs_after`]), and in a `CONST` it may start a run of signs
/// ([`sign_run`]).
fn prefix_level(p: &Parser) -> Option<u8> {
    match p.current()? {
        Minus => Some(15),
        Plus if p.in_const => Some(15),
        Ident if p.at_word("NOT") => Some(9),
        Ident if p.at_word("_NEGATE") => Some(8),
        _ => None,
    }
}

/// The n-th significant token ahead is a binary operator (as [`binary_level`] sees it at the current token).
pub(super) fn binary_at(p: &Parser, n: usize) -> bool {
    match p.nth(n) {
        Some(Caret | Star | Slash | Backslash | Plus | Minus | Eq | Lt | Gt | Le | Ge | Ne) => true,
        Some(Ident) => {
            ["MOD", "AND", "OR", "XOR", "EQV", "IMP", "_ANDALSO", "_ORELSE"]
                .iter()
                .any(|w| p.nth_is_word(n, w))
                || (p.in_const && p.nth_is_word(n, "ROOT"))
        }
        _ => false,
    }
}

/// The n-th significant token ahead is a word that cannot start an operand.
pub(super) fn reserved_in_expr(p: &Parser, n: usize) -> bool {
    RESERVED_IN_EXPR.iter().any(|w| p.nth_is_word(n, w))
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

/// An expression whose operators bind tighter than the comparisons, so that a following `=` is left alone
/// (`LSET a$ = b$`, the target of `SWAP`).
pub(crate) fn expr_above_comparison(p: &mut Parser) -> bool {
    expr_bp(p, 11).is_some()
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

/// The level of `^`, and the level its right operand is parsed at.
const POWER: u8 = 16;
const POWER_RHS: u8 = POWER + 1;

fn expr_bp_inner(p: &mut Parser, min: u8) -> Option<u32> {
    let cp = p.builder.checkpoint();
    let mut height = if let Some(level) = prefix_level(p) {
        // The old compiler's power-negation rule (`study\02` §1.2, step D; measured,
        // `verification\v24_j_power_unary`): a minus right of `^` takes what follows up to the next operator that
        // is not `^`, a negation or `NOT`. Inside it a `NOT` reaches no further either, so `2 ^ -NOT x + 3` is
        // `(2 ^ (-(NOT x))) + 3`, though `-NOT x + 3` alone is `-(NOT (x + 3))` and `2 ^ NOT x + 3` is
        // `2 ^ (NOT (x + 3))`. `POWER` as `min` is used by nothing else: it means "inside such a minus".
        let operand_min = match (min, p.current()) {
            (POWER_RHS, Some(Minus | Plus)) | (POWER, _) => POWER,
            _ => level,
        };
        if min == POWER_RHS && p.at_word("_NEGATE") {
            // Measured: "Expected variable/value after '^'" (`verification\v24_x238_power_negate`; design D2 of
            // `m4-statements-metacommands`).
            p.error("expected a value after `^`, not `_NEGATE`");
            return None;
        }
        let not = p.at_word("NOT");
        p.start_node(PrefixExpr);
        operator(p);
        p.eat_trivia();
        let operand = if not && p.in_const && at_binary_word(p) {
            // The same in a `CONST` after `NOT` (measured: "Expected variable/value after 'NOT'", upstream
            // `const/not_bad_identifier`, `verification\v51_x51`).
            let word = upper_word(p);
            p.error(format!("a value is expected after `NOT`, not the operator `{word}`"));
            None
        } else {
            expr_bp(p, operand_min)
        };
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
        operator(p);
        let rhs = expr_bp(p, level + 1);
        p.finish_node();
        height = height.max(rhs?) + 1;
    }
    Some(height)
}

/// Takes the operator at the current token into the open node, with the signs after it that the old compiler
/// drops or reads as one with it.
fn operator(p: &mut Parser) {
    let sign = matches!(p.current(), Some(Plus | Minus));
    p.bump();
    if p.in_const {
        if sign {
            sign_run(p);
        }
    } else {
        signs_after(p, sign);
    }
}

/// The signs the old compiler removes from an expression before it reads it, here those after an operator that
/// was just taken (`sign`: it is `+` or `-`). They stay in the operator's node as tokens after the operator, which
/// nothing reads (`fixoperationorder`, `qb64pe.bas` 23558–23601; measured, `verification\v25_signs`):
/// - a `+` right after a `+` or a `-` goes, so `a + +b` is `a + b`, `a - +b` is `a - b` and `-+b` is `-b`;
/// - then two `-` right after any operator go, so `a * - -b` is `a * b` and `s$ + - -t$` joins two strings.
///   The minus of `- -b` at the start of an expression is such an operator only for what follows it: `- - -b`
///   is `-b`, and `- -b` stays two negations.
fn signs_after(p: &mut Parser, sign: bool) {
    while sign && p.at(Plus) {
        p.bump();
    }
    while p.at(Minus) {
        // The second minus, past the `+` signs the first rule removes after the first.
        let mut n = 1;
        while p.nth(n) == Some(Plus) {
            n += 1;
        }
        if p.nth(n) != Some(Minus) {
            break;
        }
        for _ in 0..=n {
            p.bump();
        }
        while p.at(Plus) {
            p.bump();
        }
    }
}

/// In a `CONST` the old compiler's evaluator reads every run of `+` and `-` as one sign: `-` when the number of
/// `-` in it is odd, else `+` (`DWD`, `const_eval.bas` 1059–1085; measured, `verification\v25_signs_const`). The
/// run is taken into the open node after its first sign; `sema` reads it ([`crate::ast::signs_negative`]).
fn sign_run(p: &mut Parser) {
    while matches!(p.current(), Some(Plus | Minus)) {
        p.bump();
    }
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
            // The old compiler sets `_SOCKETS_` when it compiles a call of either, wherever it stands (design D6
            // of `m3-network-embed`; measured, `verification\v44_c_flags`, `v44_x51`–`x53`).
            if p.at_word("_OPENHOST") || p.at_word("_OPENCLIENT") {
                p.pp.met(Feature::Sockets, 1);
            }
            note_url_name(p);
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
        // In a `CONST`, an operator word where a value is expected is an error (measured: "Unexpected element
        // 'OR'", upstream `const/bad_single_arg`, `verification\v51_x40`, `x41`); after `NOT` see
        // `expr_bp_inner`. Elsewhere `OR` and `AND` are the old compiler's "Expected variable/value before 'OR'"
        // (design D2 of `m4-statements-metacommands`; measured, `x50`, `v57_e08`); the other words stay marked.
        _ if p.in_const && at_binary_word(p) => {
            let word = upper_word(p);
            p.error(format!("the operator `{word}` cannot start a value"));
            None
        }
        _ if p.at_word("OR") || p.at_word("AND") => {
            let word = upper_word(p);
            p.error(format!("expected a value before the operator `{word}`"));
            None
        }
        _ => {
            p.syntax_error("expected an expression");
            None
        }
    }
}

/// The current token is a binary operator written as a word.
fn at_binary_word(p: &Parser) -> bool {
    ["OR", "AND", "XOR", "EQV", "IMP", "MOD"].iter().any(|w| p.at_word(w))
}

/// The current token's text in capitals.
fn upper_word(p: &Parser) -> String {
    qb64rust_base::show_bytes(p.nth_text(0)).to_ascii_uppercase()
}

/// Notes a name at the current token, any suffix, of a function of QB64pe's file included after the last line
/// (`_ENCODEURL`, `_DECODEURL`): the program needs the file (design D7 of `m3-network-embed`).
pub(super) fn note_url_name(p: &mut Parser) {
    let word = p.nth_text(0);
    let end = word
        .iter()
        .position(|&b| !(b.is_ascii_alphanumeric() || b == b'_'))
        .unwrap_or(word.len());
    if word[..end].eq_ignore_ascii_case(b"_ENCODEURL") || word[..end].eq_ignore_ascii_case(b"_DECODEURL") {
        p.pp.url_named = true;
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

/// The type names a built-in may take as an argument (`VAL(s, _UNSIGNED _INTEGER64)`, `_MEMGET(m, o, LONG)`,
/// `_CAST(_BYTE, x)`).
const TYPE_ARG_WORDS: [&str; 11] = [
    "_BIT",
    "_BYTE",
    "INTEGER",
    "LONG",
    "_INTEGER64",
    "_OFFSET",
    "SINGLE",
    "DOUBLE",
    "_FLOAT",
    "STRING",
    "_MEM",
];

/// A type name as an argument, `[_UNSIGNED] type`, as a `TypeArg`, when one stands before the next `,` or `)`;
/// `STRING` and `_BIT` with `* n` (`_MEMGET(m, o, STRING * 4)`, `_MK$(_BIT * 4, x)`; `m3-mem`). Returns false
/// (taking nothing) otherwise.
fn type_arg(p: &mut Parser) -> bool {
    let n = usize::from(p.at_word("_UNSIGNED"));
    let is_type = TYPE_ARG_WORDS.iter().any(|w| p.nth_is_word(n, w));
    let sized = (p.nth_is_word(n, "STRING") || p.nth_is_word(n, "_BIT"))
        && p.nth(n + 1) == Some(Star)
        && p.nth(n + 2) == Some(Number);
    let last = if sized { n + 2 } else { n };
    if !is_type || !matches!(p.nth(last + 1), Some(Comma | RParen)) {
        return false;
    }
    p.start_node(TypeArg);
    for _ in 0..=last {
        p.bump();
    }
    p.finish_node();
    true
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
            if !omitted && type_arg(p) {
                height = height.max(1);
            } else if !omitted {
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
