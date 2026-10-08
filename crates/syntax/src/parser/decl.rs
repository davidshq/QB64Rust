//! Declarations (design D5): `DIM [SHARED] name[(bounds)] [AS type], ...` and `DIM [SHARED] AS type name, ...`,
//! `REDIM [_PRESERVE] [SHARED] ...`, `SHARED ...`, `STATIC ...`, `COMMON [SHARED] ...`, `ERASE name, ...`,
//! `DEFINT`/`DEFLNG`/`DEFSNG`/`DEFDBL`/`DEFSTR` and `_DEFINE` letter ranges. Type names are kept as words; `sema`
//! decides which it supports. Also `CONST name = expr, ...` and `OPTION word` (design D2 of
//! `m2-control-flow-slice`).

use super::Parser;
use super::blocks::array_bounds;
use super::expr::expr;
use crate::SyntaxKind::{self, *};

pub(crate) fn dim_stmt(p: &mut Parser) {
    p.start_node(DimStmt);
    p.bump(); // DIM
    if p.at_word("SHARED") {
        p.bump();
    }
    if p.at_word("_PRESERVE") {
        p.unsupported("`DIM _PRESERVE`");
    } else {
        leading_as_and_items(p);
    }
    p.recover();
    p.finish_node();
}

/// `REDIM [_PRESERVE] [SHARED] items` (the old compiler also takes `SHARED` before `_PRESERVE`).
pub(crate) fn redim_stmt(p: &mut Parser) {
    p.start_node(RedimStmt);
    p.bump(); // REDIM
    for _ in 0..2 {
        if ["_PRESERVE", "PRESERVE", "_RETAIN", "RETAIN", "SHARED"]
            .iter()
            .any(|w| p.at_word(w))
        {
            p.bump();
        }
    }
    p.redim_items = true;
    leading_as_and_items(p);
    p.redim_items = false;
    p.recover();
    p.finish_node();
}

/// `COMMON [SHARED] [/name/] items`.
pub(crate) fn common_stmt(p: &mut Parser) {
    p.start_node(CommonStmt);
    p.bump(); // COMMON
    if p.at_word("SHARED") {
        p.bump();
    }
    if p.at(Slash) {
        p.bump();
        if p.at(Ident) {
            p.bump();
        }
        p.expect(Slash, "`/` after the block name");
    }
    if !p.stmt_error {
        leading_as_and_items(p);
    }
    p.recover();
    p.finish_node();
}

/// `[AS type] item, ...`: with the type first, items take no `AS` of their own.
fn leading_as_and_items(p: &mut Parser) {
    if p.at_word("AS") && !as_clause_with(p, true) {
        return;
    }
    items(p);
}

/// `ERASE name, ...`; a name may be followed by `()`. A member array (`a(0).b`, `a . b`) is an expression, a
/// `FieldExpr`.
pub(crate) fn erase_stmt(p: &mut Parser) {
    p.start_node(EraseStmt);
    p.bump(); // ERASE
    loop {
        if !p.at(Ident) {
            p.syntax_error("expected an array name");
            break;
        }
        let plain = match p.nth(1) {
            Some(LParen) => p.nth(2) == Some(RParen) && p.nth(3) != Some(Dot),
            Some(Dot) => false,
            _ => true,
        };
        if !plain {
            if !expr(p) {
                break;
            }
        } else {
            p.start_node(NameRef);
            p.bump();
            p.finish_node();
            if p.at(LParen) {
                p.bump();
                p.bump();
            }
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

/// The `DEFxxx` words, upper case.
pub(crate) const DEF_TYPE_WORDS: [&str; 5] = ["DEFINT", "DEFLNG", "DEFSNG", "DEFDBL", "DEFSTR"];

/// `DEFINT a-z, x` (and the other `DEFxxx`), or `_DEFINE a-z, x AS type`: letter ranges, each a `LetterRange`.
pub(crate) fn def_type_stmt(p: &mut Parser) {
    p.start_node(DefTypeStmt);
    let define = p.at_word("_DEFINE") || p.at_word("DEFINE");
    p.bump(); // DEFINT ... or _DEFINE
    let mut ok = true;
    loop {
        if !letter_range(p) {
            ok = false;
            break;
        }
        if p.at(Comma) {
            p.bump();
        } else {
            break;
        }
    }
    if ok && define {
        if p.at_word("AS") {
            as_clause(p);
        } else {
            p.syntax_error("expected `AS` and a type after the letters");
        }
    }
    p.recover();
    p.finish_node();
}

/// `a` or `a-z` (letters only; `_DEFINE` names may start with `_` too, which the lexer keeps in the name).
fn letter_range(p: &mut Parser) -> bool {
    let letter = |p: &Parser, n: usize| {
        p.nth(n) == Some(Ident) && {
            let t = p.nth_text(n);
            t.len() == 1 && t[0].is_ascii_alphabetic()
        }
    };
    if !letter(p, 0) {
        p.syntax_error("expected a letter or a range of letters (`a-z`)");
        return false;
    }
    p.start_node(LetterRange);
    p.bump();
    let mut ok = true;
    if p.at(Minus) {
        p.bump();
        if letter(p, 0) {
            p.bump();
        } else {
            p.syntax_error("expected a letter after `-`");
            ok = false;
        }
    }
    p.finish_node();
    ok
}

/// `SHARED` or `STATIC` and a list of `DimItem`s (`kind` is `SharedStmt` or `StaticStmt`).
pub(crate) fn list_stmt(p: &mut Parser, kind: SyntaxKind) {
    p.start_node(kind);
    p.bump(); // SHARED or STATIC
    leading_as_and_items(p);
    p.recover();
    p.finish_node();
}

fn items(p: &mut Parser) {
    loop {
        if !dim_item(p) {
            break;
        }
        if p.at(Comma) {
            p.bump();
        } else {
            break;
        }
    }
}

/// `name[(bounds)] [AS type]`; the bounds as in a `TYPE` field (design D2 of `m2-arrays-and-types`). In a `REDIM`
/// the name may be a member array: `name(index).member[(index)]….member(bounds)`, each part's parentheses an
/// `ArrayBounds` (`DimItem::member_path` tells them apart).
pub(super) fn dim_item(p: &mut Parser) -> bool {
    if !p.at(Ident) {
        p.syntax_error("expected a variable name");
        return false;
    }
    p.start_node(DimItem);
    p.bump();
    let mut ok = !p.at(LParen) || array_bounds(p);
    while ok && p.redim_items && p.at(Dot) {
        p.bump();
        ok = p.expect(Ident, "a member name after `.`") && (!p.at(LParen) || array_bounds(p));
    }
    let ok = ok && (!p.at_word("AS") || as_clause(p));
    p.finish_node();
    ok
}

/// `CONST name[suffix] = expr {, name[suffix] = expr}`; each item a `ConstItem`.
pub(crate) fn const_stmt(p: &mut Parser) {
    p.start_node(ConstStmt);
    p.bump(); // CONST
    p.in_const = true;
    loop {
        if !const_item(p) {
            break;
        }
        if p.at(Comma) {
            p.bump();
        } else {
            break;
        }
    }
    p.recover();
    p.in_const = false;
    p.finish_node();
}

fn const_item(p: &mut Parser) -> bool {
    if !p.at(Ident) {
        p.syntax_error("expected a constant name");
        return false;
    }
    p.start_node(ConstItem);
    p.bump();
    let ok = p.expect(Eq, "`=`") && expr(p);
    p.finish_node();
    ok
}

/// `OPTION` and one word: `BASE n`, `_EXPLICIT` or `_EXPLICITARRAY`. The spellings without the underscore are
/// kept too: they are valid only after `$NOPREFIX`, which `sema` knows about, not the parser.
pub(crate) fn option_stmt(p: &mut Parser) {
    p.start_node(OptionStmt);
    p.bump(); // OPTION
    if p.at_word("BASE") {
        p.bump();
        p.expect(Number, "`0` or `1` after `OPTION BASE`");
    } else if ["_EXPLICIT", "_EXPLICITARRAY", "EXPLICIT", "EXPLICITARRAY"]
        .iter()
        .any(|w| p.at_word(w))
    {
        p.bump();
    } else {
        p.error("expected `BASE`, `_EXPLICIT` or `_EXPLICITARRAY` after `OPTION`");
    }
    p.recover();
    p.finish_node();
}

/// `AS <type words> [* size]` at `AS`; the size is a number or a constant's name (`AS STRING * 8`). Returns false
/// after an error.
pub(super) fn as_clause(p: &mut Parser) -> bool {
    as_clause_with(p, false)
}

/// [`as_clause`]; before the names (`leading`: `DIM AS LONG a, b`) the type is one word, or two after
/// `_UNSIGNED`, and the names follow.
fn as_clause_with(p: &mut Parser, leading: bool) -> bool {
    p.start_node(AsClause);
    p.bump();
    let mut ok = true;
    if !p.at(Ident) {
        p.syntax_error("expected a type name after `AS`");
        ok = false;
    } else if leading {
        if (p.at_word("_UNSIGNED") || p.at_word("UNSIGNED")) && p.nth(1) == Some(Ident) {
            p.bump();
        }
        p.bump();
    }
    while !leading && p.at(Ident) {
        p.bump();
    }
    if ok && p.at(Star) {
        p.bump();
        if p.at(Number) || p.at(Ident) {
            p.bump();
        } else {
            p.syntax_error("expected a size after `*`");
            ok = false;
        }
    }
    p.finish_node();
    ok
}
