//! `_ARRAYCOPY source TO destination` (design D1–D3 of `m3-arraycopy`; measured, `verification\v41_*`, `study\00`
//! §5): the form, the arrays of both sides and the old compiler's compile-time checks in its order
//! (`source\utilities\arrcpy.bm`). What runs is the IR's `Op::ArrayCopy`.

use super::{Checker, R};
use crate::{ArrayRef, Expr, Place, Program, StmtKind, SymbolKind, Ty};
use qb64rust_base::{Span, show_bytes};
use qb64rust_syntax::ast;

/// The six forms, by the shape of each side's last parentheses, in the order the old compiler tries them
/// (`qb64pe.bas` 10542).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Form {
    /// `s(lo TO hi) TO d(start)`.
    RangeAt,
    /// `s() TO d()`.
    Whole,
    /// `s() TO d(start, …)`, one start or more.
    WholeAt,
    /// `s(lo TO hi) TO d()`.
    RangeWhole,
    /// `s(lo TO hi, …) TO d(start, …)`, two ranges or more.
    RangesAt,
    /// `s(lo TO hi, …) TO d()`.
    RangesWhole,
}

/// The shape of a side's last parentheses (`ParseArrayCopyRef`).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    /// `()`.
    Whole,
    /// One item, with `TO` (source only).
    Range,
    /// Two items or more, each checked later (source only).
    Ranges,
    /// Items without `TO`, one or more (destination only).
    Starts(usize),
    /// Anything else.
    Other,
}

/// A side as the checks see it: its element type, its number of dimensions (0 where it is not known) and its
/// name as written.
struct Side {
    ty: Ty,
    dims: usize,
    shown: String,
}

/// The checks of a statement whose sides' numbers of dimensions were not known where it stands (an array parameter,
/// `COMMON a()`; design D3), run when every array parameter has its number ([`Checker::run_copy_checks`]).
pub(super) struct CopyCheck {
    form: Form,
    from: (ArrayRef, String),
    to: (ArrayRef, String),
    ranges: usize,
    starts: usize,
    span: Span,
}

/// The message of a statement that has none of the six forms.
const FORM: &str = "`_ARRAYCOPY` takes one of `s() TO d()`, `s(lo TO hi) TO d(start)`, `s() TO d(start, …)`, `s(lo TO \
                    hi) TO d()`, `s(lo TO hi, …) TO d(start, …)` and `s(lo TO hi, …) TO d()`";

fn plural(n: usize) -> &'static str {
    if n == 1 { "" } else { "s" }
}

fn shape(bounds: ast::ArrayBounds, source: bool) -> Shape {
    let items = bounds.ranges();
    match items.as_slice() {
        [] => Shape::Whole,
        [_, _, ..] if source => Shape::Ranges,
        [(Some(_), _)] if source => Shape::Range,
        _ if source => Shape::Other,
        _ if items.iter().all(|(lower, _)| lower.is_none()) => Shape::Starts(items.len()),
        _ => Shape::Other,
    }
}

/// The parentheses after the last part of a side's name.
fn last(item: ast::DimItem<'_>) -> Option<ast::ArrayBounds<'_>> {
    item.path().last().and_then(|&(_, parens)| parens)
}

fn form(source: Shape, destination: Shape) -> Option<Form> {
    match (source, destination) {
        (Shape::Range, Shape::Starts(1)) => Some(Form::RangeAt),
        (Shape::Whole, Shape::Whole) => Some(Form::Whole),
        (Shape::Whole, Shape::Starts(_)) => Some(Form::WholeAt),
        (Shape::Range, Shape::Whole) => Some(Form::RangeWhole),
        (Shape::Ranges, Shape::Starts(_)) => Some(Form::RangesAt),
        (Shape::Ranges, Shape::Whole) => Some(Form::RangesWhole),
        _ => None,
    }
}

/// The number of dimensions of a side is not known: nothing else gives it (`x75`, "_ArrayCopy cannot determine the
/// array dimensions").
fn unknown(side: &Side) -> String {
    format!(
        "the number of dimensions of `{}` is not known here: nothing else in the program gives it, and `_ARRAYCOPY` \
         does not",
        side.shown
    )
}

/// Two element types the copy does not take together ("… requires arrays with the same element type", "…
/// fixed-length STRING arrays with the same element size").
fn other_types(p: &Program, s: &Side, d: &Side) -> String {
    format!(
        "`_ARRAYCOPY` copies between arrays of one element type, not `{}` and `{}`",
        p.type_name(s.ty),
        p.type_name(d.ty)
    )
}

/// One number of dimensions on both sides ("… whole-array source and destination must have the same number of
/// dimensions"); a side whose number is not known is [`unknown`].
fn other_dims(s: &Side, d: &Side) -> String {
    if let Some(side) = [s, d].into_iter().find(|side| side.dims == 0) {
        return unknown(side);
    }
    format!(
        "`{}` has {} dimension{} and `{}` has {}: `_ARRAYCOPY` needs as many on both sides here",
        s.shown,
        s.dims,
        plural(s.dims),
        d.shown,
        d.dims
    )
}

/// `CheckArrayCopyElementPair`: one type, `STRING * n` of one length, and a type the copy takes.
fn element_pair(p: &Program, s: &Side, d: &Side) -> Option<String> {
    if s.ty != d.ty {
        return Some(other_types(p, s, d));
    }
    // "_ArrayCopy supports only byte-aligned numeric or fixed-length STRING arrays" (`x21`–`x24`, `x55`).
    matches!(s.ty, Ty::User(_) | Ty::Bit { .. }).then(|| {
        format!(
            "`_ARRAYCOPY` copies arrays of numbers, `_OFFSET` and strings, not of `{}`",
            p.type_name(s.ty)
        )
    })
}

/// `CheckArrayCopyWholePair`: known and equal numbers of dimensions, then the element pair.
fn whole_pair(p: &Program, s: &Side, d: &Side) -> Option<String> {
    if s.dims == 0 {
        return Some(unknown(s));
    }
    if d.dims == 0 {
        return Some(unknown(d));
    }
    if s.dims != d.dims {
        return Some(other_dims(s, d));
    }
    element_pair(p, s, d)
}

/// `CheckArrayCopy1DPair`: a range of one dimension, or one start, is for arrays of one dimension ("…
/// variable-length STRING step supports only one-dimensional arrays", "… TYPE-member step supports only
/// one-dimensional arrays"; `x25`, `x40`).
fn one_dim_pair(p: &Program, s: &Side, d: &Side) -> Option<String> {
    let one_dim = || {
        [s, d].into_iter().find(|side| side.dims != 1).map(|side| {
            if side.dims == 0 {
                return unknown(side);
            }
            format!(
                "`{}` has {} dimensions: a range `lo TO hi` alone, or a start alone, is for arrays of one dimension",
                side.shown, side.dims
            )
        })
    };
    match (s.ty == Ty::Str, d.ty == Ty::Str) {
        (true, true) => one_dim(),
        (false, false) => one_dim().or_else(|| element_pair(p, s, d)),
        _ => Some(other_types(p, s, d)),
    }
}

/// The checks of the pair by the form, in the old compiler's order (`EmitArrayCopy1D` … `EmitArrayCopyRangeWholeND`),
/// up to the ranges.
fn pair_error(p: &Program, form: Form, s: &Side, d: &Side) -> Option<String> {
    let strings = (s.ty == Ty::Str, d.ty == Ty::Str);
    let mismatch = |strings: (bool, bool)| strings.0 != strings.1;
    if !matches!(form, Form::RangeAt | Form::RangeWhole) && mismatch(strings) {
        return Some(other_types(p, s, d));
    }
    let both_strings = strings.0 && strings.1;
    let same_many = s.dims > 1 && d.dims > 1 && s.dims == d.dims;
    match form {
        Form::RangeAt | Form::RangeWhole => one_dim_pair(p, s, d),
        Form::Whole if s.dims == 1 && d.dims == 1 => (!both_strings).then(|| element_pair(p, s, d)).flatten(),
        Form::Whole | Form::WholeAt if both_strings => {
            let one_dim = form == Form::WholeAt && s.dims == 1 && d.dims == 1;
            (!one_dim && !same_many).then(|| other_dims(s, d))
        }
        Form::Whole | Form::WholeAt => whole_pair(p, s, d),
        Form::RangesAt | Form::RangesWhole => {
            let pair = if both_strings {
                // "… must have the same multidimensional shape rank" (`x42`).
                (!same_many).then(|| {
                    if let Some(side) = [s, d].into_iter().find(|side| side.dims == 0) {
                        return unknown(side);
                    }
                    format!(
                        "`{}` has {} dimension{} and `{}` has {}: `_ARRAYCOPY` of `STRING` arrays with ranges needs as \
                         many on both sides, two or more",
                        s.shown,
                        s.dims,
                        plural(s.dims),
                        d.shown,
                        d.dims
                    )
                })
            } else {
                whole_pair(p, s, d)
            };
            pair.or_else(|| {
                // "_ArrayCopy multidimensional source ranges require at least two dimensions" (`x30`, `x38`).
                (s.dims <= 1).then(|| {
                    format!(
                        "`{}` has {} dimension{}: `_ARRAYCOPY` with more than one range needs a source of two \
                         dimensions or more",
                        s.shown,
                        s.dims,
                        plural(s.dims)
                    )
                })
            })
        }
    }
}

/// The count of ranges ("… source range count must match source dimensions", `x31`).
fn range_count(form: Form, s: &Side, ranges: usize) -> Option<String> {
    (matches!(form, Form::RangesAt | Form::RangesWhole) && ranges != s.dims).then(|| {
        format!(
            "`{}` has {} dimension{}: `_ARRAYCOPY` needs a range for each, not {ranges}",
            s.shown,
            s.dims,
            plural(s.dims)
        )
    })
}

/// The count of starts ("… destination start coordinate count must match destination dimensions", `x28`, `x29`,
/// `x32`, `x33`, `x71`).
fn start_count(form: Form, d: &Side, starts: usize) -> Option<String> {
    (matches!(form, Form::WholeAt | Form::RangesAt) && starts != d.dims).then(|| {
        format!(
            "`{}` has {} dimension{}: `_ARRAYCOPY` needs a start for each, not {starts}",
            d.shown,
            d.dims,
            plural(d.dims)
        )
    })
}

impl Checker<'_> {
    /// `_ARRAYCOPY source TO destination`. In the old compiler's order: the form (the shapes of both sides' last
    /// parentheses); the source's array, then the destination's; the checks of the pair by the form; each range
    /// (written `lower TO upper`, its bounds numbers) and their count; the starts (numbers) and their count. Where a
    /// side's number of dimensions is not known yet, the checks that need it run after every body is checked
    /// (design D3). The statement adds nothing to what is known of an array parameter.
    pub(super) fn array_copy(&mut self, stmt: ast::ArrayCopyStmt) -> R<()> {
        let span = stmt.node().span();
        let mut sides = stmt.sides();
        let (source, destination) = match (sides.next(), sides.next()) {
            (Some(s), Some(d)) => (s, d),
            _ => return Err(self.error(span, "internal error: incomplete syntax tree")),
        };
        let shapes = match (last(source), last(destination)) {
            (Some(s), Some(d)) if source.as_clause().is_none() && destination.as_clause().is_none() => {
                Some((shape(s, true), shape(d, false)))
            }
            _ => None,
        };
        let Some(form) = shapes.and_then(|(s, d)| form(s, d)) else {
            return Err(self.error(span, FORM));
        };
        let (Some(ranges_at), Some(starts_at)) = (last(source), last(destination)) else {
            unreachable!("a form has the parentheses of both sides");
        };
        let from = self.copy_side(source, "source")?;
        let to = self.copy_side(destination, "destination")?;
        let side = |c: &Self, (array, shown): &(ArrayRef, String)| Side {
            ty: c.prog.array_ty(array),
            dims: c.prog.array_dims(array).unwrap_or(0),
            shown: shown.clone(),
        };
        let (s, d) = (side(self, &from), side(self, &to));
        let known = self.prog.array_dims(&from.0).is_some() && self.prog.array_dims(&to.0).is_some();
        if known && let Some(msg) = pair_error(&self.prog, form, &s, &d) {
            return Err(self.error(span, msg));
        }
        let mut ranges = Vec::new();
        for (lower, upper) in ranges_at.ranges() {
            let upper = self.need(upper, span)?;
            let Some(lower) = lower else {
                // "_ArrayCopy source dimension requires lower TO upper" (`x34`).
                return Err(self.error(
                    upper.node().span(),
                    "each range of `_ARRAYCOPY` is written `lower TO upper`",
                ));
            };
            let lower = self.copy_value(lower, "bound")?;
            let upper = self.copy_value(upper, "bound")?;
            ranges.push((lower, upper));
        }
        if known && let Some(msg) = range_count(form, &s, ranges.len()) {
            return Err(self.error(span, msg));
        }
        let mut starts = Vec::new();
        for (_, start) in starts_at.ranges() {
            let start = self.need(start, span)?;
            starts.push(self.copy_value(start, "start")?);
        }
        if known && let Some(msg) = start_count(form, &d, starts.len()) {
            return Err(self.error(span, msg));
        }
        if !known {
            self.copy_checks.push(CopyCheck {
                form,
                from: from.clone(),
                to: to.clone(),
                ranges: ranges.len(),
                starts: starts.len(),
                span,
            });
        }
        let kind = StmtKind::ArrayCopy {
            from: from.0,
            to: to.0,
            ranges,
            starts,
        };
        self.push(stmt.node(), kind);
        Ok(())
    }

    /// The array a side names before its last parentheses (`role`: "source" or "destination"), with its last name
    /// as written: a member array, or an array visible here named without a type suffix ("_ArrayCopy source must be
    /// an array" for a scalar, a scalar member, an undeclared name, a FUNCTION, a `CONST` and a suffixed name, also
    /// the suffix the array was declared with; `x11`–`x14`, `x44`, `x45`, `x47`, `x48`, `x51`).
    fn copy_side(&mut self, item: ast::DimItem, role: &str) -> R<(ArrayRef, String)> {
        let span = item.node().span();
        if let Some((place, _, t)) = self.member_path_array(item)? {
            let shown = show_bytes(self.text(t.span));
            if self.is_member_array(&place) {
                let Place::Member { base, member } = place else {
                    unreachable!("a member array is a member");
                };
                return Ok((ArrayRef::Member { base, member }, shown));
            }
            let msg = format!("the {role} of `_ARRAYCOPY` must be an array: `{shown}` is a member that is no array");
            return Err(self.error(span, msg));
        }
        let t = self.need(item.name(), span)?;
        let (name, suffix) = self.split_name(t)?;
        let shown = show_bytes(self.text(t.span));
        if suffix.is_none()
            && let Some(v) = self.find_array(&name, None)
        {
            self.names.push((SymbolKind::Var(v), t.span));
            return Ok((ArrayRef::Var(v), shown));
        }
        let msg = if suffix.is_some() {
            format!("the {role} of `_ARRAYCOPY` must be an array named without a type suffix: `{shown}`")
        } else {
            format!("the {role} of `_ARRAYCOPY` must be an array: `{shown}` names none here")
        };
        Err(self.error(span, msg))
    }

    /// A bound or start of `_ARRAYCOPY` (`what`), converted to `_INTEGER64` as an index is (half to even; `v41_h`).
    fn copy_value(&mut self, e: ast::Expr, what: &str) -> R<Expr> {
        let v = self.expr(e)?;
        if v.ty == Ty::Str {
            // Measured: "Illegal string-number conversion" (`x49`, `x50`).
            return Err(self.error(v.span, format!("a {what} of `_ARRAYCOPY` must be a number")));
        }
        self.store(v, Ty::I64)
    }

    /// The checks left by statements whose sides' numbers of dimensions were not known there (design D3), with
    /// every array parameter's number found ([`Checker::solve_dims`]); a number still not known counts as 0, which
    /// is the old compiler's "cannot determine the array dimensions" (`x75`).
    pub(super) fn run_copy_checks(&mut self) {
        for check in std::mem::take(&mut self.copy_checks) {
            let side = |c: &Self, (array, shown): &(ArrayRef, String)| Side {
                ty: c.prog.array_ty(array),
                dims: c.prog.array_dims(array).unwrap_or(0),
                shown: shown.clone(),
            };
            let (s, d) = (side(self, &check.from), side(self, &check.to));
            let msg = pair_error(&self.prog, check.form, &s, &d)
                .or_else(|| range_count(check.form, &s, check.ranges))
                .or_else(|| start_count(check.form, &d, check.starts));
            if let Some(msg) = msg {
                self.stmt_error = false;
                let _ = self.error(check.span, msg);
            }
        }
    }
}
