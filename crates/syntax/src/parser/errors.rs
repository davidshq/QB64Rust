//! Labels and error handling (design D1 of `m2-procedures-and-errors`): `name:`, `ON ERROR GOTO`, `RESUME` and
//! `ERROR`.
//!
//! A label is a name without suffix followed by `:` at the start of a statement, when the name is not a reserved
//! word (the old compiler's `validlabel`). The old compiler also refuses built-in names (`CLS:` is a call of `CLS`)
//! and may read a SUB name as a call. The parser knows the two built-in statements it parses itself (`END`,
//! `SYSTEM`); it cannot see the other built-ins or the procedures, so `sema` rejects those labels.

use super::expr::{arg_list, expr};
use super::{Parser, keywords, name_part};
use crate::SyntaxKind::*;

impl Parser<'_, '_> {
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

/// The events of `ON <event> GOSUB` and of the switches `<event>[(n)] ON|OFF|STOP|FREE`.
pub(crate) const EVENT_WORDS: [&str; 7] = ["TIMER", "KEY", "STRIG", "PLAY", "PEN", "COM", "UEVENT"];

/// `ON ERROR GOTO …`, `ON <event>[(args)] GOSUB label` (or a SUB's name: `ON TIMER(1) handler`), or
/// `ON expr GOTO|GOSUB target, ...`.
pub(crate) fn on_stmt(p: &mut Parser) {
    if p.nth_is_word(1, "ERROR") {
        on_error(p);
    } else if EVENT_WORDS.iter().any(|w| p.nth_is_word(1, w)) && !p.nth_is_word(1, "KEY") || on_key_event(p) {
        on_event(p);
    } else {
        on_jump(p);
    }
}

/// `ON KEY(n) …` is an event; `ON KEY` alone or `ON key …` (a variable `key` is not allowed: `KEY` is reserved)
/// cannot be anything else, so `KEY` followed by `(` is the event form.
fn on_key_event(p: &Parser) -> bool {
    p.nth_is_word(1, "KEY") && p.nth(2) == Some(LParen)
}

fn on_event(p: &mut Parser) {
    p.start_node(OnEventStmt);
    p.bump(); // ON
    p.bump(); // TIMER, KEY, ...
    if p.at(LParen) && !arg_list(p) {
        p.recover();
        p.finish_node();
        return;
    }
    if p.at_word("GOSUB") {
        p.bump();
    }
    if p.at(Ident) || p.at(Number) {
        p.bump();
    } else {
        p.syntax_error("expected `GOSUB` and a label, or a SUB's name");
    }
    p.recover();
    p.finish_node();
}

/// `ON expr GOTO|GOSUB target, ...`; a target is a label or a line number, and may be left out.
fn on_jump(p: &mut Parser) {
    p.start_node(OnJumpStmt);
    p.bump(); // ON
    if expr(p) {
        if p.at_word("GOTO") || p.at_word("GOSUB") {
            p.bump();
            loop {
                if p.at(Ident) || p.at(Number) {
                    p.bump();
                }
                if p.at(Comma) {
                    p.bump();
                } else {
                    break;
                }
            }
        } else {
            p.syntax_error("expected `GOTO` or `GOSUB`");
        }
    }
    p.recover();
    p.finish_node();
}

/// `<event>[(args)] ON|OFF|STOP|FREE` (`TIMER ON`, `KEY(1) OFF`, `TIMER(t) FREE`), at an event word. Returns
/// false (taking nothing) when the statement is not of that shape (`KEY 1, "x"`, `PLAY "abc"`).
pub(crate) fn event_switch(p: &mut Parser) -> bool {
    let mut n = 1;
    if p.nth(1) == Some(LParen) {
        match p.skip_parens(1) {
            Some(after) => n = after,
            None => return false,
        }
    }
    let switch = ["ON", "OFF", "STOP", "FREE"].iter().any(|w| p.nth_is_word(n, w));
    if !switch || !p.ends_at(n + 1) {
        return false;
    }
    p.start_node(EventSwitchStmt);
    p.bump(); // the event
    if p.at(LParen) {
        arg_list(p);
    }
    p.bump(); // ON, OFF, STOP or FREE
    p.recover();
    p.finish_node();
    true
}

/// `ON ERROR GOTO label`, `ON ERROR GOTO 0`, and QB64's `ON ERROR GOTO _NEWHANDLER label` / `_LASTHANDLER`.
fn on_error(p: &mut Parser) {
    for (n, lead, word) in [(1, "ON", "ERROR"), (2, "ON ERROR", "GOTO")] {
        if p.nth_is_word(n, word) {
            continue;
        }
        if p.ends_at(n) {
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
    if (p.at_word("_NEWHANDLER") || p.at_word("_LASTHANDLER")) && p.nth(1) == Some(Ident) {
        p.bump();
    }
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
