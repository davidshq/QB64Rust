//! Built-in statements read by their `specialformat` template (design D6): `LINE (0, 0)-(9, 9), , BF`, `OPEN f$
//! FOR INPUT AS #1`, `PSET STEP(1, 1)`, `TIME$ = t$`. The old compiler reads these statements by the template too
//! (`seperateargs`); an expression list could not tell `(a, b)-(c, d)` or `STEP` apart.
//!
//! Matching works on the significant tokens ahead, before anything is added to the tree: an argument (`?`) takes
//! one expression, whose extent [`Parser::expr_extent`] finds by the shape of the expression grammar; words match
//! by text, without regard to case; `[…]` tries its items first, then nothing; the choices of `{…}` and the
//! entries of a name are tried in table order; the first full match wins (measured M5: with variables `B` and
//! `BF`, `LINE …, B` takes `B` as the colour, and `LINE …, BF, BF` the second as the box word, which is this
//! order). The match is then built as a `BuiltinStmt`: the name, `FormWord` nodes, punctuation tokens and
//! `FormArg` nodes (each holding the expression). With no match, the error is at the furthest token a form could
//! reach (the old compiler rejects these too: "Syntax error - Reference: …").

use super::Parser;
use super::expr::expr;
use crate::SyntaxKind::{self, *};
use qb64rust_builtins::template::{Atom, Item};
use qb64rust_builtins::{Builtin, statement_templates};

/// One step of a match: what the token(s) at a significant index are.
#[derive(Clone, Copy, Debug)]
enum Step {
    /// An expression from this significant index to (not including) the second.
    Arg(usize, usize),
    Word(usize),
    Punct(usize),
}

impl Parser<'_, '_> {
    /// At a statement whose first word names a built-in statement with a template: parses it as a `BuiltinStmt`
    /// and returns true; false (taking nothing) when the word is no such statement.
    pub(super) fn template_statement(&mut self) -> bool {
        let forms = statement_templates(self.nth_text(0));
        if forms.is_empty() {
            return false;
        }
        if self.at_word("KEY") && self.key_own_form() {
            return true;
        }
        if self.at_word("WIDTH") && self.nth(1) == Some(LParen) && self.parens_then_eq() {
            // `WIDTH (3) = 5`: a store into an array named `WIDTH`, used without `DIM` (measured,
            // `verification\v27_x51`, `v55_x66`, `x67`; design D8 of `m4-names-implicit-arrays`).
            return false;
        }
        if self.at_word("PALETTE") && self.nth_is_word(1, "USING") {
            // `PALETTE USING array(index)`: the old compiler reads it by hand, outside the statement's template
            // (`PALETTE [?,?]`), so no form of the table matches it; a statement of its own (design D11 of
            // `m4-statements-metacommands`), whose array `sema` reads.
            self.start_node(PaletteUsingStmt);
            self.bump(); // PALETTE
            self.bump(); // USING
            if !self.at_stmt_end() {
                super::expr::expr(self);
            }
            self.recover();
            self.finish_node();
            return true;
        }
        let mut furthest = 1;
        for (_, items) in &forms {
            let mut plan = Vec::new();
            if self.match_items(&mut vec![items.as_slice()], 1, None, &mut plan, &mut furthest) {
                self.build(&plan, 1, 0);
                return true;
            }
        }
        if self.nth(1) == Some(Eq) {
            // `key = 1`: an assignment to a reserved name, which `sema` reports as such.
            return false;
        }
        self.no_form(&forms, furthest);
        true
    }

    /// At `KEY`: the forms the old compiler reads before the statement's template (`qb64pe.bas` 7281–7330), as a
    /// `KeyStmt`: `KEY ON`, `KEY OFF` and `KEY LIST` alone, and `KEY n, text$` (a comma outside parentheses).
    /// Returns false, taking nothing, for any other `KEY` statement (`KEY(1) ON`), which the template reads.
    fn key_own_form(&mut self) -> bool {
        if let Some(word) = ["ON", "OFF", "LIST"].into_iter().find(|w| self.nth_is_word(1, w)) {
            self.start_node(KeyStmt);
            self.bump(); // KEY
            self.bump(); // ON, OFF or LIST
            if !self.at_stmt_end() {
                // Measured: "Expected KEY ON only" (`verification\v42_x25`).
                self.syntax_error(format!("expected the end of the statement after `KEY {word}`"));
                self.rest_into_error_node();
            }
            self.finish_node();
            return true;
        }
        if !self.comma_outside_parens() {
            return false;
        }
        self.start_node(KeyStmt);
        self.bump(); // KEY
        if expr(self) {
            if self.at(Comma) {
                self.bump();
                expr(self);
            } else {
                self.syntax_error("expected `,` and the key's text");
            }
        }
        self.recover();
        self.finish_node();
        true
    }

    /// At `CALL name(…)` or `CALL name` where the name is a built-in statement with a template: the old compiler
    /// reads the template's forms between the parentheses (measured, `verification\v26_x74`, `v27_x02`, `v28_x65`,
    /// `v29_x27`, `x87`–`x98`; `qb64pe.bas` 10617–10627: `CALL LINE((1, 2)-(3, 4), 5, BF)`, `CALL CLS`, `CALL
    /// SOUND(440, 1 WAIT)`). Parses a match as a `BuiltinStmt` that starts with `CALL` and holds the parentheses.
    /// What the old compiler rejects is an error here: anything but one pair of parentheses with something
    /// between them after the name ("Expected CALL sub-name (...)": `CALL CLS()`, `CALL SOUND(440, 1) WAIT`,
    /// `CALL VIEW PRINT`), and a part no form matches (`CALL PSET(1, 2)`, `CALL PLAY`). Returns false, taking
    /// nothing, when the name is no such statement or has a template of one argument, which is an expression
    /// list (`CALL _DEST(h)`).
    pub(super) fn template_call(&mut self) -> bool {
        let forms = statement_templates(self.nth_text(1));
        if forms.is_empty() || forms.iter().any(|(_, items)| matches!(items[..], [Item::Arg])) {
            return false;
        }
        // The significant indexes of the first token inside the parentheses and of the closing parenthesis.
        let (from, end) = if self.ends_at(2) {
            (2, 2)
        } else {
            match self.skip_parens(2) {
                Some(after) if self.nth(2) == Some(LParen) && self.ends_at(after) && after > 4 => (3, after - 1),
                found => {
                    let name = qb64rust_base::show_bytes(self.nth_text(1)).to_ascii_uppercase();
                    // The old compiler has one message for both; `CALL CLS()` has its parentheses.
                    let empty = found == Some(4) && self.nth(2) == Some(LParen) && self.ends_at(4);
                    self.start_node(CallStmt);
                    self.bump(); // CALL
                    self.bump(); // the statement's name
                    let span = self.current_span();
                    self.error_at(
                        span,
                        if empty {
                            format!("expected something between the parentheses of `CALL {name}(…)`")
                        } else {
                            format!("expected `CALL {name}(…)`, the parentheses around all that follows")
                        },
                    );
                    self.rest_into_error_node();
                    self.finish_node();
                    return true;
                }
            }
        };
        let mut furthest = from;
        for (_, items) in &forms {
            let mut plan = Vec::new();
            if self.match_items(&mut vec![items.as_slice()], from, Some(end), &mut plan, &mut furthest) {
                self.build(&plan, 2, if from == end { end } else { end + 1 });
                return true;
            }
        }
        self.start_node(CallStmt);
        self.bump(); // CALL
        self.no_form(&forms, furthest - 1);
        self.finish_node();
        true
    }

    /// Matches the items of `stack` (the innermost sequence last) from significant index `pos` to `end` (the end of
    /// the statement when `None`), adding steps to `plan`. Backtracks: on failure `plan` is as it was.
    fn match_items(
        &self,
        stack: &mut Vec<&[Item]>,
        pos: usize,
        end: Option<usize>,
        plan: &mut Vec<Step>,
        furthest: &mut usize,
    ) -> bool {
        *furthest = (*furthest).max(pos);
        let Some(seq) = stack.pop() else {
            return match end {
                Some(end) => pos == end,
                None => self.ends_at(pos),
            };
        };
        let ok = match seq.split_first() {
            None => self.match_items(stack, pos, end, plan, furthest),
            Some((first, rest)) => {
                let mark = plan.len();
                stack.push(rest);
                let ok = match first {
                    Item::Arg => match self.expr_extent(pos) {
                        Some(after) => {
                            plan.push(Step::Arg(pos, after));
                            self.match_items(stack, after, end, plan, furthest)
                        }
                        None => false,
                    },
                    Item::Punct(c) => {
                        self.punct_at(pos, *c) && {
                            plan.push(Step::Punct(pos));
                            self.match_items(stack, pos + 1, end, plan, furthest)
                        }
                    }
                    Item::Choice(alts) => alts.iter().any(|alt| {
                        plan.truncate(mark);
                        self.atoms_at(pos, alt, plan) && self.match_items(stack, pos + alt.len(), end, plan, furthest)
                    }),
                    Item::Optional(inner) => {
                        stack.push(inner);
                        let with = self.match_items(stack, pos, end, plan, furthest);
                        stack.pop();
                        with || {
                            plan.truncate(mark);
                            self.match_items(stack, pos, end, plan, furthest)
                        }
                    }
                };
                stack.pop();
                if !ok {
                    plan.truncate(mark);
                }
                ok
            }
        };
        stack.push(seq);
        ok
    }

    /// The words and punctuation of one choice at `pos`; adds their steps.
    fn atoms_at(&self, pos: usize, alt: &[Atom], plan: &mut Vec<Step>) -> bool {
        for (i, atom) in alt.iter().enumerate() {
            let ok = match atom {
                Atom::Word(w) => self.nth_is_word(pos + i, w),
                Atom::Punct(c) => self.punct_at(pos + i, *c),
            };
            if !ok {
                return false;
            }
            plan.push(match atom {
                Atom::Word(_) => Step::Word(pos + i),
                Atom::Punct(_) => Step::Punct(pos + i),
            });
        }
        true
    }

    fn punct_at(&self, n: usize, c: u8) -> bool {
        let want = match c {
            b',' => Comma,
            b'(' => LParen,
            b')' => RParen,
            b'-' => Minus,
            b'=' => Eq,
            b'#' => Hash,
            _ => return false,
        };
        self.nth(n) == Some(want)
    }

    /// Where an expression starting at significant index `n` ends (the index after its last token), by the shape
    /// of the expression grammar (`expr.rs`): prefix operators, an operand (literal, name with an optional
    /// argument list, parenthesised expression) with member accesses, and binary operators between operands.
    /// `None` when no expression starts there.
    pub(super) fn expr_extent(&self, mut n: usize) -> Option<usize> {
        loop {
            while self.nth(n) == Some(Minus) || self.nth_is_word(n, "NOT") || self.nth_is_word(n, "_NEGATE") {
                n += 1;
            }
            n = match self.nth(n)? {
                Number | StringLit => n + 1,
                LParen => self.skip_parens(n)?,
                Ident if !self.ends_at(n) && !super::expr::reserved_in_expr(self, n) => {
                    if self.nth(n + 1) == Some(LParen) {
                        self.skip_parens(n + 1)?
                    } else {
                        n + 1
                    }
                }
                _ => return None,
            };
            while self.nth(n) == Some(Dot) && self.nth(n + 1) == Some(Ident) {
                n += 2;
                if self.nth(n) == Some(LParen) {
                    n = self.skip_parens(n)?;
                }
            }
            if !super::expr::binary_at(self, n) {
                return Some(n);
            }
            n += 1;
        }
    }

    /// Builds the `BuiltinStmt` of a match: `lead` tokens before the template's part (the statement's name, after
    /// `CALL` when it is there), the plan's steps, then the tokens up to significant index `until` (the closing
    /// parenthesis after `CALL`).
    fn build(&mut self, plan: &[Step], lead: usize, until: usize) {
        self.start_node(BuiltinStmt);
        for _ in 0..lead {
            self.bump();
        }
        let mut at = lead;
        for &step in plan {
            match step {
                Step::Arg(start, end) => {
                    self.skip_to(&mut at, start);
                    self.start_node(FormArg);
                    let ok = expr(self);
                    self.finish_node();
                    at = end;
                    if !ok {
                        break;
                    }
                }
                Step::Word(n) => {
                    self.skip_to(&mut at, n);
                    self.wrap_token(FormWord);
                    at = n + 1;
                }
                Step::Punct(n) => {
                    self.skip_to(&mut at, n);
                    self.bump();
                    at = n + 1;
                }
            }
        }
        self.skip_to(&mut at, until);
        self.recover();
        self.finish_node();
    }

    /// The plan's steps follow each other, so the parser is always at `want` already; `at` tracks it.
    fn skip_to(&mut self, at: &mut usize, want: usize) {
        while *at < want && self.current().is_some() {
            self.bump();
            *at += 1;
        }
    }

    fn wrap_token(&mut self, kind: SyntaxKind) {
        self.start_node(kind);
        self.bump();
        self.finish_node();
    }

    /// No form matched: an error at the furthest token any form reached, naming the forms.
    fn no_form(&mut self, forms: &[(&Builtin, Vec<Item>)], furthest: usize) {
        let name = qb64rust_base::show_bytes(self.nth_text(0)).to_ascii_uppercase();
        // Two entries of a name may have one template (graphics `GET` and `PUT`): each form is named once.
        let mut shown: Vec<String> = Vec::new();
        for (b, _) in forms {
            let form = format!("`{name} {}`", b.specialformat.unwrap_or_default());
            if !shown.contains(&form) {
                shown.push(form);
            }
        }
        let span = self.next_span(furthest.min(self.statement_len()));
        let function = forms.iter().any(|(b, _)| b.kind == qb64rust_builtins::Kind::Function)
            || qb64rust_builtins::find_any(name.as_bytes()).any(|b| b.kind == qb64rust_builtins::Kind::Function);
        if function && self.nth(1) == Some(LParen) && self.parens_then_eq() {
            // A store to a name that is a statement and a function (`TIMER(1) = 2`): "Expected variable ="
            // (measured, `verification\v55_x01`, `x04`), not the statement's reference.
            self.error_at(
                span,
                format!("`{name}(...) =`: `{name}` is a built-in function, not a variable"),
            );
            self.recover();
            return;
        }
        self.error_at(
            span,
            format!(
                "this `{name}` statement matches none of its forms: {}",
                shown.join(", ")
            ),
        );
        self.recover();
    }

    /// How many significant tokens the statement has from the current one on.
    fn statement_len(&self) -> usize {
        let mut n = 0;
        while !self.ends_at(n) {
            n += 1;
        }
        n
    }
}
