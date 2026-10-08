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
        let mut furthest = 1;
        for (_, items) in &forms {
            let mut plan = Vec::new();
            if self.match_items(&mut vec![items.as_slice()], 1, &mut plan, &mut furthest) {
                self.build(&plan);
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

    /// Matches the items of `stack` (the innermost sequence last) from significant index `pos` to the end of the
    /// statement, adding steps to `plan`. Backtracks: on failure `plan` is as it was.
    fn match_items(&self, stack: &mut Vec<&[Item]>, pos: usize, plan: &mut Vec<Step>, furthest: &mut usize) -> bool {
        *furthest = (*furthest).max(pos);
        let Some(seq) = stack.pop() else {
            return self.ends_at(pos);
        };
        let ok = match seq.split_first() {
            None => self.match_items(stack, pos, plan, furthest),
            Some((first, rest)) => {
                let mark = plan.len();
                stack.push(rest);
                let ok = match first {
                    Item::Arg => match self.expr_extent(pos) {
                        Some(end) => {
                            plan.push(Step::Arg(pos, end));
                            self.match_items(stack, end, plan, furthest)
                        }
                        None => false,
                    },
                    Item::Punct(c) => {
                        self.punct_at(pos, *c) && {
                            plan.push(Step::Punct(pos));
                            self.match_items(stack, pos + 1, plan, furthest)
                        }
                    }
                    Item::Choice(alts) => alts.iter().any(|alt| {
                        plan.truncate(mark);
                        self.atoms_at(pos, alt, plan) && self.match_items(stack, pos + alt.len(), plan, furthest)
                    }),
                    Item::Optional(inner) => {
                        stack.push(inner);
                        let with = self.match_items(stack, pos, plan, furthest);
                        stack.pop();
                        with || {
                            plan.truncate(mark);
                            self.match_items(stack, pos, plan, furthest)
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

    /// Builds the `BuiltinStmt` of a match.
    fn build(&mut self, plan: &[Step]) {
        self.start_node(BuiltinStmt);
        self.bump(); // the statement's name
        let mut at = 1;
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
        let shown: Vec<String> = forms
            .iter()
            .map(|(b, _)| format!("`{name} {}`", b.specialformat.unwrap_or_default()))
            .collect();
        let span = self.next_span(furthest.min(self.statement_len()));
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
