//! Values: expressions and the arguments of procedure calls.

use crate::Emitter;
use crate::decl::{dim_slot, is_qbs, lock_slot};
use crate::names::{c_string, c_type, int_const, proc_name};
use qb64rust_ir::{Arg, ArrayRef, BinOp, Conv, Expr, ExprKind, MemberId, Place, ProcId, Ty, UnOp};

impl Emitter<'_> {
    /// Arguments of a call of `proc`: a place's own pointer (cast to the parameter's type when the place has the same
    /// width and the other signedness, as the old compiler passes it), a string temporary as it is, a numeric copy
    /// in a new `passN`, or a whole array's descriptor.
    pub(crate) fn args(&mut self, proc: ProcId, args: &[Arg]) -> String {
        let params = self.p.proc(proc).params.clone();
        let parts: Vec<String> = args
            .iter()
            .zip(params)
            .map(|(a, param)| match a {
                Arg::Ref(place) => {
                    let (from, to) = (self.p.place_ty(place), self.p.var(param).ty);
                    let r = self.place_ref(place);
                    if from == to || is_qbs(to) {
                        r
                    } else {
                        format!("({}*)({r})", c_type(to))
                    }
                }
                Arg::Temp(v) if is_qbs(v.ty) => self.value(v),
                Arg::Temp(v) => {
                    self.pass += 1;
                    let n = format!("pass{}", self.pass);
                    self.pass_decls.push(format!("{} {n};\n", c_type(v.ty)));
                    format!("&({n}={})", self.value(v))
                }
                Arg::Array(ArrayRef::Var(a)) => self.name(*a),
                Arg::Array(ArrayRef::Member { base, member }) => self.member_array_arg(base, *member),
            })
            .collect();
        parts.join(",")
    }

    /// A whole member array as an array argument (design D8 of `m2-defxxx-common-arrays`; measured,
    /// `verification\v24_x315`): a descriptor made for the call, of a static array that is allocated, with the
    /// member's bounds, over the bytes of the member array. It is a `passN` of the body that makes the call,
    /// its bounds written where it is declared; the argument sets its block, which finds the place the array is
    /// part of once. In the procedure an `ERASE` of it clears and a `REDIM` raises error 10, as for any static
    /// array passed.
    fn member_array_arg(&mut self, base: &Place, member: MemberId) -> String {
        let dims = self.member_dims(base, member);
        let n = dims.len();
        // The block, an unused slot, the flags (allocated, static), an unused slot; four slots per dimension;
        // the slot of the `_MEM` lock, which nothing reads of a static array.
        let mut slots = vec!["0".to_string(); lock_slot(n) + 1];
        slots[2] = "1+2".to_string();
        for (k, (lower, count, multiplier)) in dims.into_iter().enumerate() {
            let s = dim_slot(n, k);
            slots[s] = int_const(lower, Ty::I64);
            slots[s + 1] = count.to_string();
            slots[s + 2] = multiplier.to_string();
        }
        self.pass += 1;
        let name = format!("pass{}", self.pass);
        self.pass_decls
            .push(format!("ptrszint {name}[{}]={{{}}};\n", slots.len(), slots.join(",")));
        let first = self.member_array_at(base, member);
        format!("({name}[0]=(ptrszint)({first}),{name})")
    }

    /// `LBOUND` or `UBOUND` of a member array: its member's bound, a constant; the place it is part of is not
    /// found (measured, `verification\v24_x311`). A dimension outside 1 to the number of dimensions raises
    /// error 9 and gives 0, as libqb's `func_lbound` does.
    fn member_bound(&mut self, upper: bool, base: &Place, member: MemberId, dim: Option<&Expr>) -> String {
        let bounds: Vec<String> = self
            .p
            .member_of(base, member)
            .bounds
            .iter()
            .map(|&(lower, high)| int_const(if upper { high } else { lower }, Ty::I64))
            .collect();
        let outside = "(error(9),(int64)0)".to_string();
        let nth = |d: i64| {
            let bound = usize::try_from(d)
                .ok()
                .and_then(|d| d.checked_sub(1))
                .and_then(|k| bounds.get(k));
            bound.map_or(outside.clone(), |b| format!("((int64){b})"))
        };
        match dim.map(|d| (&d.kind, d)) {
            None => nth(1),
            Some((ExprKind::Int(d), _)) => nth(*d),
            // The dimension is evaluated once, into a `passN`.
            Some((_, d)) => {
                let value = self.value(d);
                self.pass += 1;
                let name = format!("pass{}", self.pass);
                self.pass_decls.push(format!("int32 {name};\n"));
                let mut text = format!("(({name}=({value})),");
                for (k, b) in bounds.iter().enumerate() {
                    text.push_str(&format!("{name}=={}?(int64){b}:", k + 1));
                }
                text.push_str(&outside);
                text.push(')');
                text
            }
        }
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
            ExprKind::Bound {
                upper,
                array: ArrayRef::Var(array),
                dim,
            } => {
                let f = if *upper { "func_ubound" } else { "func_lbound" };
                let d = dim.as_deref().map_or("1".to_string(), |d| self.value(d));
                let n = crate::decl::dim_count(self.p.var(*array));
                format!("((int64){f}({},{d},{n}))", self.name(*array))
            }
            ExprKind::Bound {
                upper,
                array: ArrayRef::Member { base, member },
                dim,
            } => self.member_bound(*upper, base, *member, dim.as_deref()),
            // As the old compiler writes them (`qb64pe.bas` 21328; measured, `study\00` §5): the element's checked
            // flat position times the size of an element, and the array's distance from the start of libqb's
            // conventional memory in 16-byte segments.
            ExprKind::VarPtr { array, index } => {
                let flat = self.flat_index(*array, index);
                format!("(({flat})*{})", self.ty_size(self.p.var(*array).ty))
            }
            ExprKind::VarSeg { array } => {
                format!(
                    "( ( ((ptrszint)({}[0])) - ((ptrszint)(&cmem[0])) ) /16)",
                    self.name(*array)
                )
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
                    // `~a^b`: the left operand is complemented in its own type (`sema`'s `left_operand`).
                    BinOp::Eqv => format!("((~({a}))^{b})"),
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
                let a = self.args(*proc, args);
                format!("{}({a})", proc_name(self.p.proc(*proc)))
            }
        }
    }
}
