//! Calls of built-in functions: one checker for every function `sema` compiles (design D2 of `m2-core-builtins`).
//! Which functions, their rules and result types: `crate::builtins`.

use super::places::numeric_type;
use super::{Checker, R};
use crate::builtins::{Rule, Slot, Supported, lookup, result_types, slots};
use crate::{Expr, ExprKind, NEW_TYPE_UNREACHABLE, Place, Ty};
use qb64rust_base::Span;
use qb64rust_syntax::SyntaxKind::Ident;
use qb64rust_syntax::ast;

/// The supported built-in a name as written stands for: the required `$` where the function has one, no suffix
/// otherwise.
pub(super) fn supported_builtin(name: &str, suffix: Option<Ty>) -> Option<Supported> {
    match suffix {
        None => lookup(name, false),
        Some(Ty::Str) => lookup(name, true),
        // No built-in is written with a number suffix.
        Some(
            Ty::I8
            | Ty::U8
            | Ty::I16
            | Ty::U16
            | Ty::I32
            | Ty::U32
            | Ty::I64
            | Ty::U64
            | Ty::Off
            | Ty::UOff
            | Ty::Bit { .. }
            | Ty::F32
            | Ty::F64
            | Ty::F80
            | Ty::User(_),
        ) => None,
        Some(crate::unproduced_types!()) => unreachable!("{NEW_TYPE_UNREACHABLE}"),
    }
}

/// Whether the rule takes its argument as it is: the old compiler special-cases these functions before it converts
/// arguments to their slots (`qb64pe.bas` `evaluatefunc`; measured from the C++: `std::floor(*__SINGLE_F)`,
/// `func_hex(*__INTEGER_I,4)`, `func_cint_double( 2.5E+0 )`).
fn takes_as_is(rule: Rule) -> bool {
    match rule {
        Rule::IntFix | Rule::Exp | Rule::Convert(_) | Rule::Radix(_) => true,
        Rule::Plain
        | Rule::ResultOfArg
        | Rule::FloatByArg
        | Rule::Fixed(_)
        | Rule::Len
        | Rule::Val
        | Rule::Asc
        | Rule::StringFill => false,
    }
}

impl Checker<'_> {
    /// A call of a supported built-in. `list` is `None` for the bare name (`ERR`, `_PI`), which the caller allows
    /// only where no argument is required.
    pub(super) fn builtin(&mut self, s: Supported, list: Option<ast::ArgList>, span: Span) -> R<Expr> {
        let name = s.name;
        if list.is_some_and(|l| l.args().is_empty() && l.type_arg().is_none()) {
            // Measured: `_PI()`, `SIN()`, `LEN()` are "Expected (...)" (`v20_x22`, `x23`, `x27`).
            return Err(self.error(span, format!("`{name}` with empty parentheses")));
        }
        match s.rule {
            Rule::Len => return self.len(s, list, span),
            Rule::Val => return self.val(s, list, span),
            Rule::Fixed(_) if list.is_some() => {
                // `ERR(1)`: not measured.
                return Err(self.unsupported(span, format!("`{name}` with arguments")));
            }
            Rule::Plain
            | Rule::ResultOfArg
            | Rule::IntFix
            | Rule::FloatByArg
            | Rule::Exp
            | Rule::Convert(_)
            | Rule::Fixed(_)
            | Rule::Radix(_)
            | Rule::Asc
            | Rule::StringFill => {}
        }
        let nodes = self.present_args(list)?;
        let slots = slots(s);
        let required = slots.iter().filter(|(_, optional)| !optional).count();
        if nodes.len() < required || nodes.len() > slots.len() {
            return Err(self.error(span, arity(name, required, slots.len())));
        }
        // Optional slots are filled left to right by the arguments beyond the required ones.
        let mut extra = nodes.len() - required;
        let mut nodes = nodes.into_iter();
        let mut args = Vec::with_capacity(slots.len());
        for (k, &(slot, optional)) in slots.iter().enumerate() {
            if optional {
                if extra == 0 {
                    args.push(None);
                    continue;
                }
                extra -= 1;
            }
            let node = nodes.next().expect("one argument per filled slot");
            let e = self.expr(node)?;
            let which = if slots.len() == 1 {
                format!("`{name}`")
            } else {
                format!("argument {} of `{name}`", k + 1)
            };
            args.push(Some(self.slot_arg(s.rule, slot, k, e, &which)?));
        }
        let (ty, qb) = result_types(s, &args);
        Ok(Expr {
            span,
            ty,
            qb,
            kind: ExprKind::Call { builtin: s.id, args },
        })
    }

    /// An argument converted to its slot (design D3; measured `v20_a_slots`). `which` names it in errors.
    fn slot_arg(&mut self, rule: Rule, slot: Slot, k: usize, e: Expr, which: &str) -> R<Expr> {
        let string = e.ty == Ty::Str;
        let wants_string = match (rule, slot) {
            // `STRING$(n, s$)` takes the first byte of a string in its second slot.
            (Rule::StringFill, _) if k == 1 && string => return Ok(e),
            (_, Slot::Str) => true,
            (_, Slot::Long | Slot::Double | Slot::Float | Slot::AnyNumeric) => false,
        };
        match (wants_string, string) {
            (true, false) => return Err(self.error(e.span, format!("{which} needs a string"))),
            (false, true) => return Err(self.error(e.span, format!("{which} needs a number"))),
            (true, true) => return Ok(e),
            (false, false) => {}
        }
        // Arguments of the new numeric types (task 8.4 of `m2-numeric-types`): only where the slot converts them, and
        // to `STR$`, whose any-numeric slot keeps the believed type (`qbs_str` has every width).
        let converted = matches!(slot, Slot::Long | Slot::Double) || (rule == Rule::Plain && which == "`STR$`");
        if (e.ty.is_new_numeric() || e.qb.is_new_numeric()) && !converted {
            let name = self.prog.type_name(if e.qb.is_new_numeric() { e.qb } else { e.ty });
            return Err(self.unsupported(e.span, format!("a `{name}` value as {which}")));
        }
        if takes_as_is(rule) {
            return Ok(e);
        }
        Ok(match slot {
            Slot::Long => self.store(e, Ty::I32)?,
            Slot::Double => self.convert_exact(e, Ty::F64),
            Slot::Float => e,
            Slot::AnyNumeric => {
                let qb = e.qb.printed();
                self.convert_exact(e, qb)
            }
            Slot::Str => unreachable!("handled above"),
        })
    }

    /// `LEN(x)`: a string expression's length, or the size of a variable, element or member of any other type,
    /// known when compiling (D4; measured `v20_g_len`). Anything else is an error, as in the old compiler ("String
    /// expression or variable name required in LEN statement").
    fn len(&mut self, s: Supported, list: Option<ast::ArgList>, span: Span) -> R<Expr> {
        let nodes = self.present_args(list)?;
        let [node] = nodes[..] else {
            return Err(self.error(span, arity(s.name, 1, 1)));
        };
        // A whole `TYPE` variable, element or member is a place here, not a value (`Checker::load`). A `LEN` in an
        // index of this argument sets its own and gives this one back, as the place is loaded after its index.
        let outer = self.len_place.replace(node.node().span());
        let e = self.expr(node);
        self.len_place = outer;
        let e = e?;
        // A parameter declared `STRING * n` or `t$n`, named alone: n (design D6; measured, `v21_f_fixed_param`: only
        // here, `LEN(t + "!")` is the string's real length).
        if let (ExprKind::Load(Place::Var(v)), ast::Expr::NameRef(_)) = (&e.kind, node)
            && let Some(&n) = self.param_len.get(v)
        {
            return Ok(Expr {
                span,
                ty: Ty::I32,
                qb: Ty::I32,
                kind: ExprKind::Int(i64::from(n)),
            });
        }
        if e.ty == Ty::Str {
            return Ok(Expr {
                span,
                ty: Ty::I32,
                qb: Ty::I32,
                kind: ExprKind::Call {
                    builtin: s.id,
                    args: vec![Some(e)],
                },
            });
        }
        let ExprKind::Load(place) = &e.kind else {
            return Err(self.error(e.span, "`LEN` needs a string or a variable, element or member"));
        };
        let ty = self.prog.place_ty(place);
        if let Ty::Bit { .. } = ty {
            // Measured: "Variable/element cannot be _BIT aligned" (`verification\v21_x18`, `x19`).
            return Err(self.error(e.span, "`LEN` of a `_BIT` variable"));
        }
        // The index of an element is not evaluated (the old compiler takes the element type's size).
        let size = crate::size_of(&self.prog.types, ty);
        Ok(Expr {
            span,
            ty: Ty::I32,
            qb: Ty::I32,
            kind: ExprKind::Int(i64::from(size)),
        })
    }

    /// `VAL(s$)` is `_FLOAT`; `VAL(s$, type)` with a type name (D4; measured `v20_b_result_types`, the C++): SINGLE,
    /// DOUBLE and `_FLOAT` as named, every integer type `_INTEGER64` (`qbs_val<int64_t>`). The type slot stays empty;
    /// the result type says which parse.
    fn val(&mut self, s: Supported, list: Option<ast::ArgList>, span: Span) -> R<Expr> {
        let Some(list) = list else {
            return Err(self.error(span, arity(s.name, 1, 2)));
        };
        let positions = list.args();
        let (node, ty) = match (&positions[..], list.type_arg()) {
            (&[Some(node)], None) => (node, Ty::F80),
            (&[Some(node), None], Some(t)) => {
                let words: Vec<String> = t
                    .child_tokens()
                    .filter(|w| w.kind == Ident)
                    .map(|w| self.word(w))
                    .collect();
                let text = words.join(" ");
                let ty = match numeric_type(&text) {
                    Some(Ty::I16 | Ty::I32 | Ty::I64) => Ty::I64,
                    Some(t @ (Ty::F32 | Ty::F64 | Ty::F80)) => t,
                    Some(Ty::Str | Ty::User(_)) => unreachable!("numeric types only"),
                    Some(crate::unproduced_types!()) => unreachable!("{NEW_TYPE_UNREACHABLE}"),
                    // Measured: "VAL TYPE unsupported" (`verification\v21_x35_val_bit`, `x36`).
                    Some(Ty::Bit { .. }) => return Err(self.error(t.span(), "`VAL` cannot give a `_BIT`")),
                    // The other new numeric types: task 8.4 of `m2-numeric-types`.
                    Some(Ty::I8 | Ty::U8 | Ty::U16 | Ty::U32 | Ty::U64 | Ty::Off | Ty::UOff) => {
                        return Err(self.unsupported(t.span(), format!("`VAL` with the type `{text}`")));
                    }
                    // Measured: "VAL TYPE unsupported" (`v20_x24_val_string_type`).
                    None if text == "STRING" => return Err(self.error(t.span(), "`VAL` cannot give a `STRING`")),
                    None => return Err(self.unsupported(t.span(), format!("`VAL` with the type `{text}`"))),
                };
                (node, ty)
            }
            (&[Some(_), Some(_)], None) => {
                return Err(self.error(span, "the second argument of `VAL` is a type name"));
            }
            (&[_] | &[_, _], _) if positions.iter().any(Option::is_none) => {
                return Err(self.unsupported(span, "omitted arguments"));
            }
            _ => return Err(self.error(span, arity(s.name, 1, 2))),
        };
        let e = self.expr(node)?;
        if e.ty != Ty::Str {
            return Err(self.error(e.span, "`VAL` needs a string"));
        }
        Ok(Expr {
            span,
            ty,
            qb: ty,
            kind: ExprKind::Call {
                builtin: s.id,
                args: vec![Some(e), None],
            },
        })
    }
}

/// "`LEFT$` takes 2 arguments", "`INSTR` takes 2 or 3 arguments".
fn arity(name: &str, required: usize, slots: usize) -> String {
    let count = if required == slots {
        format!("{slots} argument{}", if slots == 1 { "" } else { "s" })
    } else if slots == required + 1 {
        format!("{required} or {slots} arguments")
    } else {
        format!("{required} to {slots} arguments")
    };
    format!("`{name}` takes {count}")
}
