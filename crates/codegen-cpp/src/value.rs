//! Values: expressions and the arguments of procedure calls.

use crate::Emitter;
use crate::decl::is_qbs;
use crate::names::{c_string, c_type, int_const, proc_name};
use qb64rust_ir::{Arg, BinOp, Conv, Expr, ExprKind, Ty, UnOp};

impl Emitter<'_> {
    /// Arguments of a procedure call: a place's own pointer, a string temporary as it is, or a numeric copy in a
    /// new `passN`.
    pub(crate) fn args(&mut self, args: &[Arg]) -> String {
        let parts: Vec<String> = args
            .iter()
            .map(|a| match a {
                Arg::Ref(place) => self.place_ref(place),
                Arg::Temp(v) if is_qbs(v.ty) => self.value(v),
                Arg::Temp(v) => {
                    self.pass += 1;
                    let n = format!("pass{}", self.pass);
                    self.pass_decls.push(format!("{} {n};\n", c_type(v.ty)));
                    format!("&({n}={})", self.value(v))
                }
            })
            .collect();
        parts.join(",")
    }

    pub(crate) fn value(&mut self, v: &Expr) -> String {
        match &v.kind {
            ExprKind::Int(i) => int_const(*i, v.ty),
            ExprKind::Float(t) => {
                let suffix = if v.ty == Ty::F80 { "L" } else { "" };
                if t.starts_with('-') {
                    format!("({t}{suffix})")
                } else {
                    format!("{t}{suffix}")
                }
            }
            ExprKind::Str(s) => format!("qbs_new_txt_len({},{})", c_string(s), s.len()),
            ExprKind::Load(place) => self.load_place(place),
            // libqb's functions take the descriptor, the dimension and the number of dimensions.
            ExprKind::Bound { upper, array, dim } => {
                let f = if *upper { "func_ubound" } else { "func_lbound" };
                let d = dim.as_deref().map_or("1".to_string(), |d| self.value(d));
                let n = self.p.var(*array).dims.len();
                format!("((int64){f}({},{d},{n}))", self.name(*array))
            }
            ExprKind::Convert { how, from } => {
                let x = self.value(from);
                match (how, v.ty) {
                    (Conv::RoundEven, Ty::I32) => format!("qbr_float_to_long({x})"),
                    (Conv::RoundEven, _) => format!("qbr({x})"),
                    _ => format!("(({})({x}))", c_type(v.ty)),
                }
            }
            ExprKind::Binary { op, lhs, rhs } => {
                let (a, b) = (self.value(lhs), self.value(rhs));
                // As the old compiler writes them (`study\02` §1.4); comparisons turn C's 1 into -1. `/` is followed
                // by a space: `*a/*b` would open a comment.
                match op {
                    BinOp::Add => format!("({a}+{b})"),
                    BinOp::Sub => format!("({a}-{b})"),
                    BinOp::Mul => format!("({a}*{b})"),
                    BinOp::Div => format!("({a}/ {b})"),
                    BinOp::Eq => format!("(-({a}=={b}))"),
                    BinOp::Ne => format!("(-({a}!={b}))"),
                    BinOp::Lt => format!("(-({a}<{b}))"),
                    BinOp::Gt => format!("(-({a}>{b}))"),
                    BinOp::Le => format!("(-({a}<={b}))"),
                    BinOp::Ge => format!("(-({a}>={b}))"),
                    BinOp::And => format!("({a}&{b})"),
                    BinOp::Or => format!("({a}|{b})"),
                    BinOp::Xor => format!("({a}^{b})"),
                    BinOp::Eqv => format!("(~({a}^{b}))"),
                    BinOp::Imp => format!("((~({a}))|{b})"),
                    BinOp::AndAlso => format!("(-({a}&&{b}))"),
                    BinOp::OrElse => format!("(-({a}||{b}))"),
                    BinOp::IDiv => format!("qb_safe_idiv({a},{b})"),
                    BinOp::Mod => format!("qb_safe_mod({a},{b})"),
                    BinOp::Pow => format!("pow2({a},{b})"),
                }
            }
            ExprKind::Unary { op, operand } => {
                let x = self.value(operand);
                match op {
                    UnOp::Neg => format!("(-({x}))"),
                    UnOp::Not => format!("(~({x}))"),
                    UnOp::Negate => format!("(-(!({x})))"),
                }
            }
            ExprKind::Concat(a, b) => format!("qbs_add({},{})", self.value(a), self.value(b)),
            ExprKind::StrCompare { op, lhs, rhs } => {
                let f = match op {
                    BinOp::Eq => "qbs_equal",
                    BinOp::Ne => "qbs_notequal",
                    BinOp::Lt => "qbs_lessthan",
                    BinOp::Gt => "qbs_greaterthan",
                    BinOp::Le => "qbs_lessorequal",
                    BinOp::Ge => "qbs_greaterorequal",
                    BinOp::Add
                    | BinOp::Sub
                    | BinOp::Mul
                    | BinOp::Div
                    | BinOp::And
                    | BinOp::Or
                    | BinOp::Xor
                    | BinOp::Eqv
                    | BinOp::Imp
                    | BinOp::AndAlso
                    | BinOp::OrElse
                    | BinOp::IDiv
                    | BinOp::Mod
                    | BinOp::Pow => unreachable!("a string comparison with {op:?}"),
                };
                format!("{f}({},{})", self.value(lhs), self.value(rhs))
            }
            ExprKind::Call { builtin: id, args } => self.call(*id, args, v.ty, v.qb),
            ExprKind::CallProc { proc, args } => {
                let a = self.args(args);
                format!("{}({a})", proc_name(self.p.proc(*proc)))
            }
        }
    }
}
