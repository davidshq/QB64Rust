//! Whole-array assignment, `to() = from()` (design D1 and D2 of `m3-whole-array-assign`; measured,
//! `verification\v36_*`, `study\00` §5): the form, the arrays of both sides, the old compiler's compile-time
//! checks (`EmitAssignWhole`, `qb64pe.bas` 16675) and its guard of an assignment with a whole array on one side
//! only (`CheckAsgRHS`, 16809). What runs is the IR's `Op::CopyArray`.

use super::{Checker, DimFact, Failed, R};
use crate::{ArrayRef, ExprKind, Place, ProcKind, StmtKind, Storage, SymbolKind};
use qb64rust_base::{Span, show_bytes};
use qb64rust_syntax::ast;
use qb64rust_syntax::tree::{Node, Tok};

/// Whether an expression is written as a whole array: a name with empty parentheses (`a()`, `x.v()`) or a chain of
/// members ending in them (`e(1).v()`).
pub(super) fn written_whole(e: ast::Expr) -> bool {
    match e {
        ast::Expr::Call(c) => c.arg_list().is_some_and(|l| l.args().is_empty()),
        ast::Expr::Field(f) => f.arg_list().is_some_and(|l| l.args().is_empty()),
        ast::Expr::NameRef(_)
        | ast::Expr::Literal(_)
        | ast::Expr::Paren(_)
        | ast::Expr::Prefix(_)
        | ast::Expr::Bin(_) => false,
    }
}

/// An expression without the parentheses around it, at any depth (`a() = ((b()))`, measured with one pair,
/// `verification\v36_x14`).
pub(super) fn without_parens(mut e: ast::Expr) -> ast::Expr {
    while let ast::Expr::Paren(p) = e
        && let Some(inner) = p.inner()
    {
        e = inner;
    }
    e
}

impl Checker<'_> {
    /// `to() = from()` with both sides written whole; `false` when it is no copy, for `assign` to go on with as an
    /// ordinary assignment. In order:
    ///
    /// 1. A type suffix on the last name of one side only is the error at the other (`x41`, `x42`). On both it is
    ///    no copy (`b%() = a%()`): the old compiler takes the path of an ordinary assignment and stores the first
    ///    element of `from` into the first of `to`, converting (measured, `v36_x24`, `x38`, `x40`, `x43`, `x44`;
    ///    built as QB64pe has it, user, 2026-10-22, design D4 of `m4-line-numbers-whole-arrays`). A member array
    ///    there gets the error of an element with no index (`v54_x45`).
    /// 2. Each side is an array (the target's resolved first): a member array, or an array visible here. A name
    ///    that names none (undeclared, a `CONST`) is the old compiler's "Whole-array assignment requires empty
    ///    parentheses on both sides" (`x15`, `x16`, `x29`); a SUB or FUNCTION is an error of its own (`x18`, `x55`).
    /// 3. One element type: "Cannot assign arrays of different types", for two lengths of `STRING * n` "… with
    ///    different element sizes" (`x05`–`x12`, `x19`, `x21`–`x23`, `x34`, `x35`).
    /// 4. One number of dimensions; where one is not known yet (an array parameter, `COMMON a()`), a fact about it
    ///    ([`Checker::solve_dims`]; `x08`, `x17`, `x39`, `x45`).
    pub(super) fn copy_array(&mut self, node: Node, target: ast::Expr, source: ast::Expr) -> R<bool> {
        let span = node.span();
        let (tt, st) = (self.last_name(target, span)?, self.last_name(source, span)?);
        let suffixed = |c: &mut Self, t: Tok| c.split_name(t).map(|(_, suffix)| suffix.is_some());
        match (suffixed(self, tt)?, suffixed(self, st)?) {
            (true, true) => return Ok(false),
            (false, true) => return Err(self.suffixed_side(target, tt, st)),
            (true, false) => return Err(self.suffixed_side(source, st, tt)),
            (false, false) => {}
        }
        let to = self.whole_side(target, tt, span)?;
        let from = self.whole_side(source, st, span)?;
        let (to_ty, from_ty) = (self.prog.array_ty(&to), self.prog.array_ty(&from));
        if to_ty != from_ty {
            let (tn, fname) = (self.prog.type_name(to_ty), self.prog.type_name(from_ty));
            let msg = match (to_ty, from_ty) {
                (crate::Ty::FixedStr(_), crate::Ty::FixedStr(_)) => {
                    format!("cannot assign an array of `{fname}` to an array of `{tn}`: the lengths differ")
                }
                _ => format!("cannot assign an array of `{fname}` to an array of `{tn}`"),
            };
            return Err(self.error(span, msg));
        }
        self.copy_dims((&to, tt), (&from, st), span)?;
        self.push(node, StmtKind::CopyArray { from, to });
        Ok(true)
    }

    /// The last name of a side written whole: a name's token, or the last member's.
    fn last_name(&mut self, e: ast::Expr, span: Span) -> R<Tok> {
        match e {
            ast::Expr::Field(f) => self.need(f.member(), span),
            ast::Expr::Call(c) => self.need(c.name(), span),
            ast::Expr::NameRef(_)
            | ast::Expr::Literal(_)
            | ast::Expr::Paren(_)
            | ast::Expr::Prefix(_)
            | ast::Expr::Bin(_) => unreachable!("a side written whole"),
        }
    }

    /// The error of a whole-array assignment with a type suffix on one side only: at the side `whole` without
    /// one (named `t`), which the old compiler finds alone (`suffixed`, named `s`, is element 0 there).
    fn suffixed_side(&mut self, whole: ast::Expr, t: Tok, s: Tok) -> Failed {
        let (t, s) = (show_bytes(self.text(t.span)), show_bytes(self.text(s.span)));
        let msg = format!("`{t}()` is a whole array, and `{s}()`, named with a type suffix, is not one here");
        self.error(whole.node().span(), msg)
    }

    /// The array a side written whole names (`t` its last name): a member array, or an array visible here.
    fn whole_side(&mut self, e: ast::Expr, t: Tok, span: Span) -> R<ArrayRef> {
        if let Some((array, _, _)) = self.member_array(e)? {
            return Ok(array);
        }
        let (name, suffix) = self.split_name(t)?;
        if let Some(v) = self.find_array(&name, suffix) {
            self.stmt.names.push((SymbolKind::Var(v), t.span));
            return Ok(ArrayRef::Var(v));
        }
        let shown = show_bytes(self.text(t.span));
        if let Some(&p) = self.procs_by_name.get(&name) {
            // Measured: a FUNCTION "Expected (...)" (`v36_x18`), a SUB "Name already in use" (`x55`).
            let kind = match self.prog.proc(p).kind {
                ProcKind::Sub => "SUB",
                ProcKind::Function(_) => "FUNCTION",
            };
            let msg = format!("`{shown}` is a {kind}; a whole-array assignment needs an array on each side");
            return Err(self.error(e.node().span(), msg));
        }
        // Measured: "Whole-array assignment requires empty parentheses on both sides" (`v36_x15`, `x16`, `x29`).
        let msg = format!("`{shown}()` names no array here; a whole-array assignment needs an array on each side");
        Err(self.error(span, msg))
    }

    /// The number of dimensions of the two sides of a copy, each with its last name: equal where both are known;
    /// where one is not, a fact about it ([`Checker::settle_dims`]); where neither is, one number for both, as for
    /// an array passed to an array parameter.
    fn copy_dims(&mut self, (to, tt): (&ArrayRef, Tok), (from, st): (&ArrayRef, Tok), span: Span) -> R<()> {
        let plural = |n: usize| if n == 1 { "" } else { "s" };
        match (self.prog.array_dims(to), self.prog.array_dims(from), to, from) {
            (Some(n), Some(m), _, _) if n != m => {
                // Measured: "Cannot assign arrays with different dimensions" (`v36_x08`).
                let msg = format!(
                    "cannot assign an array of {m} dimension{} to an array of {n} dimension{}",
                    plural(m),
                    plural(n)
                );
                Err(self.error(span, msg))
            }
            (Some(_), Some(_), _, _) => Ok(()),
            (None, Some(m), ArrayRef::Var(v), _) => {
                self.settle_dims(*v, m, tt, span);
                Ok(())
            }
            (Some(n), None, _, ArrayRef::Var(v)) => {
                self.settle_dims(*v, n, st, span);
                Ok(())
            }
            (None, None, ArrayRef::Var(t), ArrayRef::Var(f)) => {
                for v in [*t, *f] {
                    if !matches!(self.prog.var(v).storage, Storage::Param(_)) {
                        self.dims.dims_open.insert(v);
                    }
                }
                self.dims.dim_facts.push(DimFact::Same {
                    arg: *f,
                    param: *t,
                    shown: show_bytes(self.text(st.span)),
                    span,
                    copied: true,
                });
                Ok(())
            }
            _ => unreachable!("a member array has its number of dimensions"),
        }
    }

    /// An assignment with a whole array `name()` (`whole`, named `t`) on one side and `other` on the other, not
    /// written whole: the old compiler's guard (`CheckAsgRHS`; design D2). When `other` names an array too, an
    /// element or a member array without parentheses, it is "Whole-array assignment requires empty parentheses on
    /// both sides" at the statement (`v36_x02`, `x26`; `tests\upstream` `t026`, `t027`); else "Expected array
    /// index after" the whole array's name, at it (`x01`, `x03`, `x04`, `x13`; `t024`, `t025`, `t037`, `t038`,
    /// `t040`, `t041`). `other_is_target`: `other` is the place stored to.
    pub(super) fn half_whole(
        &mut self,
        node: Node,
        (whole, t): (ast::Expr, Tok),
        other: ast::Expr,
        other_is_target: bool,
    ) -> Failed {
        match self.names_array(other, other_is_target) {
            Ok(true) => self.error(
                node.span(),
                "both sides of a whole-array assignment need an array name with empty parentheses",
            ),
            Ok(false) => {
                let shown = show_bytes(self.text(t.span));
                let msg = format!("`{shown}()` is a whole array, and the other side of `=` is not one");
                self.error(whole.node().span(), msg)
            }
            Err(failed) => failed,
        }
    }

    /// Whether the other side of [`Self::half_whole`] names an array: an element (`b(1)`, `x.v(1)`), or a member
    /// array without parentheses (`x.v`). A plain name is a scalar of its own beside an array of that name.
    fn names_array(&mut self, e: ast::Expr, target: bool) -> R<bool> {
        match e {
            ast::Expr::NameRef(n) => {
                let t = self.need(n.name(), n.node().span())?;
                let (name, suffix) = self.split_name(t)?;
                Ok(self
                    .dotted_any(t, &name, suffix)?
                    .is_some_and(|place| self.is_member_array(&place)))
            }
            ast::Expr::Field(f) if f.arg_list().is_none() => {
                let (place, _) = self.field_member(f)?;
                Ok(self.is_member_array(&place))
            }
            ast::Expr::Call(c) if target => Ok(matches!(self.call_place(c)?, Place::Element { .. })),
            ast::Expr::Field(f) if target => Ok(matches!(self.field_place(f)?, Place::Element { .. })),
            ast::Expr::Call(_) | ast::Expr::Field(_) => {
                Ok(matches!(self.expr(e)?.kind, ExprKind::Load(Place::Element { .. })))
            }
            ast::Expr::Literal(_) | ast::Expr::Paren(_) | ast::Expr::Prefix(_) | ast::Expr::Bin(_) => Ok(false),
        }
    }
}
