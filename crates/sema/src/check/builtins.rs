//! Calls of built-in functions: one checker for every function `sema` compiles (design D2 of `m2-core-builtins`).
//! Which functions, their rules and result types: `crate::builtins`.

use super::expr::conv;
use super::ops::{held, rotated, widest};
use super::places::numeric_type;
use super::{Checker, R};
use crate::builtins::{
    MEM_RESULTS, Rule, Slot, StmtRule, StmtSlot, StmtSupported, Supported, lookup, result_types, slots, stmt_lookup,
    stmt_slots,
};
use crate::{ConvKind, Expr, ExprKind, PLACE_ONLY_UNREACHABLE, Place, StmtArg, StmtKind, Ty};
use qb64rust_base::Span;
use qb64rust_builtins::template::{Atom, Item};
use qb64rust_syntax::SyntaxKind::{self, Ident, LParen, RParen};
use qb64rust_syntax::ast;
use qb64rust_syntax::tree::Node;

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
        Some(crate::place_only_types!()) => unreachable!("{PLACE_ONLY_UNREACHABLE}"),
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
        | Rule::StringFill
        | Rule::StrOrIndex
        | Rule::GivenArgs(_)
        // Their slots take a number as it is.
        | Rule::Widest
        | Rule::Rotate
        | Rule::Choice
        | Rule::Own
        | Rule::Mk(_)
        | Rule::Cv(_) => false,
    }
}

impl Checker<'_> {
    /// A call of a supported built-in. `list` is `None` for the bare name (`ERR`, `_PI`); a function that needs an
    /// argument gets the arity error here, like any other wrong number of arguments.
    pub(super) fn builtin(&mut self, s: Supported, list: Option<ast::ArgList>, span: Span) -> R<Expr> {
        let name = s.name;
        // A block is only stored into a `_MEM` place (design D2 of `m3-mem`): measured, "TYPE '_MEM' required for
        // function" as an argument (`verification\v32_x03`, `x93`–`x96`), "Invalid expression" with a member read
        // (`x02`, `x114`), and a C++ build that fails when it is stored into a number (`x80`).
        let mem_store = std::mem::take(&mut self.mem_store);
        if MEM_RESULTS.contains(&name) && !mem_store {
            let msg = format!("`{name}` gives a `_MEM` block, which can only be stored into a `_MEM` variable");
            return Err(self.error(span, msg));
        }
        if list.is_some_and(|l| l.args().is_empty() && l.type_arg().is_none()) {
            // Measured: `_PI()`, `SIN()`, `LEN()` are "Expected (...)" (`v20_x22`, `x23`, `x27`).
            return Err(self.error(span, format!("`{name}` with empty parentheses")));
        }
        match s.rule {
            Rule::Len => return self.len(s, list, span),
            Rule::Val => return self.val(s, list, span),
            Rule::Fixed(_) if list.is_some() && slots(s).is_empty() => {
                // Measured: `_SCREENEXISTS(1)`, `ERR(1)`, `ERL(1)` are "Incorrect number of arguments"
                // (`verification\v26_x75`–`x77`).
                return Err(self.error(span, arity(name, 0, 0)));
            }
            Rule::Fixed(_) if list.is_some() => {
                return Err(self.unsupported(span, format!("`{name}` with arguments")));
            }
            Rule::Choice => return self.choice(s, list, span),
            // `_MEM`, `_MEMELEMENT`, `_OFFSET` named bare: with an argument list they are checked before the
            // table is looked at (`check\expr.rs`, `call`). Measured: "Incorrect number of arguments" (`_MEM`
            // with three, `verification\v32_x19`).
            Rule::Own => return Err(self.error(span, format!("`{name}` needs its argument"))),
            Rule::Mk(_) | Rule::Cv(_) => return self.number_bytes(s, list, span),
            Rule::Plain
            | Rule::ResultOfArg
            | Rule::IntFix
            | Rule::FloatByArg
            | Rule::Exp
            | Rule::Convert(_)
            | Rule::Fixed(_)
            | Rule::Radix(_)
            | Rule::Asc
            | Rule::StringFill
            | Rule::StrOrIndex
            | Rule::GivenArgs(_)
            | Rule::Widest
            | Rule::Rotate => {}
        }
        self.runtime_part(s.id, name, span)?;
        let slots = slots(s);
        let filled = self.filled_slots(s, &slots, list, span)?;
        let mut args = Vec::with_capacity(slots.len());
        for (k, (&(slot, _), node)) in slots.iter().zip(filled).enumerate() {
            let Some(node) = node else {
                args.push(None);
                continue;
            };
            if slot == Slot::Block {
                args.push(Some(self.long_array_arg(node, name)?));
                continue;
            }
            if slot == Slot::Mem {
                let place = self.mem_arg(node, name)?;
                let ty = self.prog.place_ty(&place);
                args.push(Some(Expr {
                    span: node.node().span(),
                    ty,
                    qb: ty,
                    kind: ExprKind::Load(place),
                }));
                continue;
            }
            let e = self.expr(node)?;
            let which = if slots.len() == 1 {
                format!("`{name}`")
            } else {
                format!("argument {} of `{name}`", k + 1)
            };
            if let (Rule::Radix(_), ExprKind::Call { builtin, .. }) = (s.rule, &e.kind)
                && builtin.get().ret.is_none()
            {
                // `_SCREENEXISTS`, which has no return type in the table: the old compiler passes the width 0
                // here, not a LONG's (measured from its C++, design D6 of `m2-console-screenhide`).
                let inner = builtin.get().name.to_ascii_uppercase();
                return Err(self.unsupported(e.span, format!("`{inner}` as the argument of `{name}`")));
            }
            if let (Rule::Radix(_), ExprKind::VarPtr { .. } | ExprKind::VarSeg { .. }) = (s.rule, &e.kind) {
                // The old compiler passes the width 0 here too, not an `_INTEGER64`'s (measured from its C++:
                // `func_hex(…,0)`; accepted, `verification\v28_x95`).
                let msg = format!("`VARPTR` or `VARSEG` as the argument of `{name}`");
                return Err(self.unsupported(e.span, msg));
            }
            args.push(Some(self.slot_arg(s.rule, slot, k, e, &which)?));
        }
        let (ty, qb) = if s.rule == Rule::Widest {
            // Measured: `_CLAMP(a, b, c)` is typed as `widest(widest(a, b), c)` (`qb64pe.bas` 20844).
            let w = args
                .iter()
                .flatten()
                .map(|a| a.qb)
                .reduce(widest)
                .expect("`_MIN`, `_MAX` and `_CLAMP` take arguments");
            (w.held_value(), w)
        } else if s.rule == Rule::Rotate {
            let t = self.rotation(name, &args)?;
            (t, t)
        } else if s.id.get().ret == Some("_MEM") {
            let t = self.mem_ty();
            (t, t)
        } else {
            result_types(s, &args)
        };
        Ok(Expr {
            span,
            ty,
            qb,
            kind: ExprKind::Call { builtin: s.id, args },
        })
    }

    /// The argument of each slot of a call, `None` for a slot left out. Without a gap in the list, the optional
    /// slots are filled left to right by the arguments beyond the required ones. A gap (`_LOADIMAGE(f$, , r$)`)
    /// leaves out the slot in its place, where the template has that slot alone in brackets (`?[,[?][,?]]`) and an
    /// argument follows (measured, design D1 of `m3-images-fonts-mouse`: `func__loadimage(f,N,r,0|2)`); any other
    /// gap is not supported yet.
    fn filled_slots<'t>(
        &mut self,
        s: Supported,
        slots: &[(Slot, bool)],
        list: Option<ast::ArgList<'t>>,
        span: Span,
    ) -> R<Vec<Option<ast::Expr<'t>>>> {
        let required = slots.iter().filter(|(_, optional)| !optional).count();
        let (positions, list_span) = match list {
            Some(l) if l.type_arg().is_none() && l.args().iter().any(Option::is_none) => (l.args(), l.node().span()),
            _ => {
                let nodes = self.present_args(list)?;
                if nodes.len() < required || nodes.len() > slots.len() {
                    return Err(self.error(span, arity(s.name, required, slots.len())));
                }
                if let Some(counts) = template_counts(s.id.get())
                    && counts & (1 << nodes.len()) == 0
                {
                    // `_SCREENIMAGE`'s `[?,?,?,?]`: none or all four (measured, `verification\v31_x81`).
                    return Err(self.error(span, format!("`{}` takes {} arguments", s.name, list_counts(counts))));
                }
                let mut extra = nodes.len() - required;
                let mut nodes = nodes.into_iter();
                return Ok(slots
                    .iter()
                    .map(|&(_, optional)| {
                        if optional {
                            if extra == 0 {
                                return None;
                            }
                            extra -= 1;
                        }
                        Some(nodes.next().expect("one argument per filled slot"))
                    })
                    .collect());
            }
        };
        if positions.len() > slots.len() {
            return Err(self.error(span, arity(s.name, required, slots.len())));
        }
        let last = positions.iter().rposition(Option::is_some);
        for (k, p) in positions.iter().enumerate() {
            let inner = last.is_some_and(|l| k < l);
            if p.is_none() && !(inner && slots[k].1 && may_be_left_out(s.id.get(), k)) {
                return Err(self.unsupported(list_span, "omitted arguments"));
            }
        }
        if slots[positions.len()..].iter().any(|&(_, optional)| !optional) {
            return Err(self.error(span, arity(s.name, required, slots.len())));
        }
        let mut out = positions;
        out.resize(slots.len(), None);
        Ok(out)
    }

    /// The type `_ROL` or `_ROR` turns its first argument in (design D7 of `m3-console-files`). Measured
    /// (`verification\v30_x84`–`x87`, `v30_result_types.txt`): a float as either argument is "Expected
    /// non-floating-point numeric argument", a `_BIT` type "Unsupported integral type size".
    fn rotation(&mut self, name: &str, args: &[Option<Expr>]) -> R<Ty> {
        for (a, k) in args.iter().flatten().zip(1..) {
            if a.qb.is_float() {
                let msg = format!("argument {k} of `{name}` needs an integer, not a float");
                return Err(self.error(a.span, msg));
            }
        }
        let first = args[0].as_ref().expect("`_ROL` and `_ROR` take two arguments");
        match rotated(first.qb) {
            Some(t) => Ok(t),
            None => {
                let msg = format!(
                    "`{name}` turns a value of 8, 16, 32 or 64 bits, not a `{}`",
                    self.prog.type_name(first.qb)
                );
                Err(self.error(first.span, msg))
            }
        }
    }

    /// `_IIF(condition, truePart, falsePart)` (design D6 of `m3-console-files`; `qb64pe.bas` 20737): C++'s `?:`,
    /// so only the part chosen is evaluated. Measured (`verification\v30_g_iif_min_max`, `v30_x70`–`x76`): the
    /// condition is a number, a float rounded (`qbr`); the parts are both strings or both numbers ("falsePart and
    /// truePart must be of the same type"), each as it is.
    fn choice(&mut self, s: Supported, list: Option<ast::ArgList>, span: Span) -> R<Expr> {
        let nodes = self.present_args(list)?;
        let [c, a, b] = nodes[..] else {
            return Err(self.error(span, arity(s.name, 3, 3)));
        };
        let condition = self.expr(c)?;
        if condition.ty == Ty::Str {
            return Err(self.error(condition.span, format!("argument 1 of `{}` needs a number", s.name)));
        }
        let condition = if condition.ty.is_float() {
            conv(condition, Ty::I64, ConvKind::RoundEven)
        } else {
            condition
        };
        let then = self.expr(a)?;
        let else_ = self.expr(b)?;
        let (ty, qb) = match (then.ty == Ty::Str, else_.ty == Ty::Str) {
            (true, true) => (Ty::Str, Ty::Str),
            (false, false) => {
                // C++ gives `c ? a : b` the type of its parts when they have one, else their common type.
                let ty = if then.ty == else_.ty {
                    then.ty
                } else {
                    held(then.ty, else_.ty)
                };
                (ty, widest(then.qb, else_.qb))
            }
            (true, false) | (false, true) => {
                let msg = format!("the two parts of `{}` must be both strings or both numbers", s.name);
                return Err(self.error(span, msg));
            }
        };
        Ok(Expr {
            span,
            ty,
            qb,
            kind: ExprKind::Choice {
                condition: Box::new(condition),
                then: Box::new(then),
                else_: Box::new(else_),
            },
        })
    }

    /// Records the optional part of the runtime a built-in needs (the table's `dependency`; design D3 of
    /// `m2-console-screenhide`). A part no build provides yet makes the built-in "not supported yet".
    fn runtime_part(&mut self, id: qb64rust_builtins::BuiltinId, name: &str, span: Span) -> R<()> {
        let Some(dependency) = id.get().dependency else {
            return Ok(());
        };
        match crate::RuntimePart::of_dependency(dependency) {
            Some(part) => {
                self.runtime_parts.insert(part);
                Ok(())
            }
            None => Err(self.unsupported(span, format!("`{name}`: a part of the runtime that is not built yet"))),
        }
    }

    /// An argument converted to its slot (design D3; measured `v20_a_slots`). `which` names it in errors.
    fn slot_arg(&mut self, rule: Rule, slot: Slot, k: usize, e: Expr, which: &str) -> R<Expr> {
        let string = e.ty == Ty::Str;
        let wants_string = match (rule, slot) {
            // `STRING$(n, s$)` takes the first byte of a string in its second slot.
            (Rule::StringFill, _) if k == 1 && string => return Ok(e),
            // `ENVIRON$(name$)`: the entry that takes a string.
            (Rule::StrOrIndex, _) if string => return Ok(e),
            // `PAINT (x, y), tile$`: the entry that takes a string.
            (_, Slot::ColorOrTile) if string => return Ok(e),
            (_, Slot::Str) => true,
            (
                _,
                Slot::Long
                | Slot::Integer
                | Slot::Int64
                | Slot::Double
                | Slot::Float
                | Slot::AnyNumeric
                | Slot::UInt64
                | Slot::ULong
                | Slot::ColorOrTile
                | Slot::Offset,
            ) => false,
            (_, Slot::Block | Slot::Bytes | Slot::Mem) => {
                unreachable!("the bytes of an array or a place, or a block, are no value (`block_arg`, `mem_arg`)")
            }
        };
        match (wants_string, string) {
            (true, false) => return Err(self.error(e.span, format!("{which} needs a string"))),
            (false, true) => return Err(self.error(e.span, format!("{which} needs a number"))),
            (true, true) => return Ok(e),
            (false, false) => {}
        }
        if takes_as_is(rule) {
            return Ok(e);
        }
        Ok(match slot {
            Slot::Long => self.store(e, Ty::I32)?,
            // `qbr_float_to_long` of the value as it is, as the old compiler writes it: the function takes a `float`,
            // so C++ narrows the value to SINGLE (as for `MKI$`, `check\mem.rs`).
            Slot::Integer if e.ty.is_float() => super::expr::conv(e, Ty::I32, crate::ConvKind::RoundEven),
            Slot::Integer => self.convert_exact(e, Ty::I32),
            Slot::Int64 => self.store(e, Ty::I64)?,
            Slot::Offset => self.store(e, Ty::Off)?,
            Slot::UInt64 => self.store(e, Ty::U64)?,
            Slot::ULong => self.store(e, Ty::U32)?,
            // A fill colour of `PAINT`: an integer as it is, a float rounded to an integer, as the old compiler
            // writes it; the emitter's `(uint32)` cast keeps the low 32 bits.
            Slot::ColorOrTile if e.ty.is_float() => self.store(e, Ty::I64)?,
            Slot::ColorOrTile => e,
            Slot::Double => self.convert_exact(e, Ty::F64),
            Slot::Float => e,
            Slot::AnyNumeric => {
                let qb = e.qb.printed();
                self.convert_exact(e, qb)
            }
            Slot::Str | Slot::Block | Slot::Bytes | Slot::Mem => unreachable!("handled above"),
        })
    }

    /// A block as an argument ([`Slot::Mem`]; design D4 of `m3-mem`): a variable, element or member of type
    /// `_MEM`. Anything else is the old compiler's "TYPE '_MEM' required for function" (or "… for sub", "1st
    /// function argument requires TYPE '_MEM'"; measured, `verification\v32_x03`, `x67`–`x73`).
    pub(super) fn mem_arg(&mut self, node: ast::Expr, name: &str) -> R<Place> {
        let span = node.node().span();
        if !self.gives_block(node) {
            // A place of a user type is a place here, not a value, as for `LEN` ([`Checker::load`]).
            let outer = self.len_place.replace(span);
            let e = self.expr(node);
            self.len_place = outer;
            if let ExprKind::Load(place) = e?.kind
                && self.prog.is_mem(self.prog.place_ty(&place))
            {
                return Ok(place);
            }
        }
        Err(self.error(
            span,
            format!("`{name}` needs a `_MEM` variable, element or member here"),
        ))
    }

    /// `LEN(x)`: a string expression's length, or the size of a variable, element or member of any other type,
    /// known when compiling (D4; measured `v20_g_len`). Anything else is an error, as in the old compiler ("String
    /// expression or variable name required in LEN statement").
    fn len(&mut self, s: Supported, list: Option<ast::ArgList>, span: Span) -> R<Expr> {
        let nodes = self.present_args(list)?;
        let [node] = nodes[..] else {
            return Err(self.error(span, arity(s.name, 1, 1)));
        };
        // A whole member array (`LEN(p(0).a())`), which the old compiler takes (`qb64pe.bas` 20652; upstream
        // `arrays/test_mem_vs_len`), is not measured here.
        let empty = |l: Option<ast::ArgList>| l.is_some_and(|l| l.args().is_empty());
        let whole = match node {
            ast::Expr::Call(c) => empty(c.arg_list()),
            ast::Expr::Field(f) => empty(f.arg_list()),
            ast::Expr::Literal(_)
            | ast::Expr::NameRef(_)
            | ast::Expr::Paren(_)
            | ast::Expr::Prefix(_)
            | ast::Expr::Bin(_) => false,
        };
        if whole && self.member_array(node)?.is_some() {
            return Err(self.unsupported(node.node().span(), "`LEN` of a whole member array"));
        }
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
        if crate::holds_strings(&self.prog.types, ty) {
            // Measured: "UDT must have fixed size" (`verification\v24_x206`).
            let msg = format!(
                "`LEN` of a `TYPE {}` value: a `TYPE` with a `STRING` member has no fixed size",
                self.prog.type_name(ty)
            );
            return Err(self.error(e.span, msg));
        }
        if crate::owns_desc(&self.prog.types, ty) {
            // Measured (`verification\v35_x20`), compiled with task 5.2 of `m3-typefields`.
            return Err(self.unsupported(e.span, "`LEN` of a value of a `TYPE` with a dynamic member array"));
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
                // `_UNSIGNED` before an integer type: `qbs_val<uint64_t>`, typed `_UNSIGNED _INTEGER64` (`qb64pe.bas`
                // 20358–20368; measured, `v21_d_builtins`: not narrowed, a minus sign dropped).
                let (unsigned, text) = match text.strip_prefix("_UNSIGNED ") {
                    Some(rest) => (true, rest.to_string()),
                    None => (false, text),
                };
                let ty = match numeric_type(&text) {
                    // Measured: "VAL TYPE unsupported" (`verification\v21_x35_val_bit`, `x36`).
                    Some(Ty::Bit { .. }) => return Err(self.error(t.span(), "`VAL` cannot give a `_BIT`")),
                    Some(Ty::I8 | Ty::I16 | Ty::I32 | Ty::I64 | Ty::Off) if unsigned => Ty::U64,
                    Some(Ty::I8 | Ty::I16 | Ty::I32 | Ty::I64 | Ty::Off) => Ty::I64,
                    Some(t @ (Ty::F32 | Ty::F64 | Ty::F80)) if !unsigned => t,
                    // `_UNSIGNED SINGLE` is an error in a declaration (`v21_x07`); not measured in `VAL`.
                    Some(Ty::F32 | Ty::F64 | Ty::F80) => {
                        return Err(self.unsupported(t.span(), format!("`VAL` with the type `_UNSIGNED {text}`")));
                    }
                    Some(Ty::U8 | Ty::U16 | Ty::U32 | Ty::U64 | Ty::UOff | Ty::Str | Ty::User(_)) => {
                        unreachable!("`numeric_type` gives signed numeric types only")
                    }
                    Some(crate::place_only_types!()) => unreachable!("{PLACE_ONLY_UNREACHABLE}"),
                    // Measured: "VAL TYPE unsupported" (`v20_x24_val_string_type`).
                    None if text == "STRING" && !unsigned => {
                        return Err(self.error(t.span(), "`VAL` cannot give a `STRING`"));
                    }
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

/// A part of a statement read by its template, for matching it against the template of a form.
enum Part<'t> {
    /// A word, in upper case.
    Word(String),
    Punct(u8),
    Arg(ast::Expr<'t>),
}

/// What an argument or a choice of a template is in a statement: the expression, or the number of the alternative
/// written (from 0); `None` when it was left out.
enum Matched<'t> {
    Arg(Option<ast::Expr<'t>>),
    Word(Option<u8>),
}

/// Matches the items of `stack` (the innermost sequence last) against `parts` from `pos` to their end, as the
/// parser matched the tokens (`syntax\src\parser\template.rs`: `[…]` tries its items first, then nothing; the
/// alternatives of a choice in order), adding one entry per argument and choice to `out` in template order. On
/// failure `out` is as it was.
fn match_items<'t>(stack: &mut Vec<&[Item]>, parts: &[Part<'t>], pos: usize, out: &mut Vec<Matched<'t>>) -> bool {
    let Some(seq) = stack.pop() else {
        return pos == parts.len();
    };
    let ok = match seq.split_first() {
        None => match_items(stack, parts, pos, out),
        Some((first, rest)) => {
            let mark = out.len();
            stack.push(rest);
            let ok = match first {
                Item::Arg => match parts.get(pos) {
                    Some(Part::Arg(e)) => {
                        out.push(Matched::Arg(Some(*e)));
                        match_items(stack, parts, pos + 1, out)
                    }
                    Some(Part::Word(_) | Part::Punct(_)) | None => false,
                },
                Item::Punct(c) => {
                    matches!(parts.get(pos), Some(Part::Punct(p)) if p == c) && match_items(stack, parts, pos + 1, out)
                }
                Item::Choice(alts) => (0u8..).zip(alts).any(|(k, alt)| {
                    out.truncate(mark);
                    let fits = alt
                        .iter()
                        .enumerate()
                        .all(|(i, atom)| match (atom, parts.get(pos + i)) {
                            (Atom::Word(w), Some(Part::Word(p))) => w.eq_ignore_ascii_case(p),
                            (Atom::Punct(c), Some(Part::Punct(p))) => c == p,
                            (Atom::Word(_) | Atom::Punct(_), _) => false,
                        });
                    fits && {
                        out.push(Matched::Word(Some(k)));
                        match_items(stack, parts, pos + alt.len(), out)
                    }
                }),
                Item::Optional(inner) => {
                    stack.push(inner);
                    let with = match_items(stack, parts, pos, out);
                    stack.pop();
                    with || {
                        out.truncate(mark);
                        absent(inner, out);
                        match_items(stack, parts, pos, out)
                    }
                }
            };
            stack.pop();
            if !ok {
                out.truncate(mark);
            }
            ok
        }
    };
    stack.push(seq);
    ok
}

/// The entries of the arguments and choices of items that were left out.
fn absent(items: &[Item], out: &mut Vec<Matched>) {
    for item in items {
        match item {
            Item::Arg => out.push(Matched::Arg(None)),
            Item::Choice(_) => out.push(Matched::Word(None)),
            Item::Optional(inner) => absent(inner, out),
            Item::Punct(_) => {}
        }
    }
}

impl Checker<'_> {
    /// A statement the parser read by its template (`OPEN`, `NAME`, `SEEK`, …): the first form of its name, in
    /// table order, that its words, punctuation and arguments match, as the parser matched it (design D3 of
    /// `m2-builtin-statements`). A name `sema` does not compile is "not supported yet" at its first word.
    pub(super) fn builtin_stmt(&mut self, stmt: ast::BuiltinStmt) -> R<()> {
        let node = stmt.node();
        let name_tok = self.need(stmt.name(), node.span())?;
        let shown = self.word(name_tok);
        let (bare, string) = match shown.strip_suffix('$') {
            Some(b) => (b, true),
            None => (shown.as_str(), false),
        };
        let forms = stmt_lookup(bare, string);
        if forms.is_empty() {
            return Err(self.unsupported(name_tok.span, format!("`{shown}`")));
        }
        self.not_an_assignment(node, name_tok, &shown)?;
        let mut parts = Vec::new();
        for part in stmt.parts() {
            parts.push(match part {
                ast::FormPart::Word(w) => {
                    let t = self.need(w.token(), node.span())?;
                    Part::Word(self.word(t))
                }
                ast::FormPart::Punct(t) => Part::Punct(self.text(t.span)[0]),
                ast::FormPart::Arg(a) => Part::Arg(self.need(a.expr(), node.span())?),
            });
        }
        // A statement of `GET` or `PUT` that starts with a point is the graphics form to the old compiler,
        // whatever follows, and a syntax error there when it is none (measured, `verification\v28_x93`, `x94`,
        // `v30_x36`: `GET (1), , x`); the file form is not tried for it.
        let graphics = forms.iter().any(|f| f.rule == StmtRule::Graphics);
        let point_first = match parts.first() {
            Some(Part::Punct(b'(') | Part::Arg(ast::Expr::Paren(_))) => true,
            Some(Part::Word(w)) => w == "STEP",
            Some(Part::Punct(_) | Part::Arg(_)) | None => false,
        };
        for &form in &forms {
            if graphics && point_first && form.rule != StmtRule::Graphics {
                continue;
            }
            let items = form.id.get().template();
            let mut matched = Vec::new();
            if match_items(&mut vec![&items[..]], &parts, 0, &mut matched) {
                return self.stmt_call(node, form, matched);
            }
        }
        if graphics {
            let which = if point_first { "graphics " } else { "" };
            return Err(self.error(node.span(), format!("this is no form of {which}`{shown}`")));
        }
        Err(self.error(
            node.span(),
            "internal error: a built-in statement matches none of its forms",
        ))
    }

    /// A built-in statement's name and a colon at the start of a line (`CLS: LOCATE 1, 1`), which the parser reads
    /// as a label: to the old compiler it is the statement with no argument, never a label (measured for 17
    /// statements, `verification\v27_x58`; `GOTO CLS` is "Invalid label"). A statement that needs an argument is
    /// an error there ("Syntax error - Reference: PSET …", `v27_x59`).
    pub(super) fn bare_stmt(&mut self, node: Node, name: qb64rust_syntax::tree::Tok) -> R<()> {
        let shown = self.word(name);
        for form in stmt_lookup(&shown, false) {
            match form.rule {
                StmtRule::Plain | StmtRule::Graphics | StmtRule::File => {
                    let items = form.id.get().template();
                    let mut matched = Vec::new();
                    if match_items(&mut vec![&items[..]], &[], 0, &mut matched) {
                        return self.stmt_call(node, form, matched);
                    }
                }
                // `CLOSE:` closes every file.
                StmtRule::Close => {
                    self.push(
                        node,
                        StmtKind::Builtin {
                            id: form.id,
                            args: Vec::new(),
                        },
                    );
                    return Ok(());
                }
                StmtRule::Swap | StmtRule::MidAssign => {}
            }
        }
        Err(self.error(name.span, format!("`{shown}` needs its arguments")))
    }

    /// A built-in statement that starts `name (…) =` is an assignment to the old compiler, whatever follows, and so
    /// an error: the name is the statement's (measured for every supported statement, 2026-10-12: "Name already in
    /// use (CLS)" for `CLS (1) = 5`, `LOCATE (1) = 2, 3`, `PCOPY (1) = 1, 2`; another error for a name that is
    /// also a function, `SCREEN (12) = 12`, or has a leading underscore). The parser reads such a line as the
    /// statement with a comparison as its first argument, which is right only when something stands between the
    /// `)` and the `=` (`CLS (1) + 2 = 5` is `CLS` with a comparison, measured). `WIDTH` alone is different: the
    /// old compiler makes `WIDTH (80) = 80` an element of an array named `WIDTH`.
    fn not_an_assignment(&mut self, node: Node, name: qb64rust_syntax::tree::Tok, shown: &str) -> R<()> {
        let tokens: Vec<_> = node.tokens().into_iter().filter(|t| !t.kind.is_trivia()).collect();
        if tokens.first().map(|t| t.span) != Some(name.span) || tokens.get(1).map(|t| t.kind) != Some(LParen) {
            return Ok(());
        }
        let mut depth = 0usize;
        let close = tokens.iter().skip(1).position(|t| {
            if t.kind == LParen {
                depth += 1;
            } else if t.kind == RParen {
                depth = depth.saturating_sub(1);
            }
            depth == 0
        });
        let Some(close) = close else {
            return Ok(());
        };
        if tokens.get(close + 2).map(|t| t.kind) != Some(SyntaxKind::Eq) {
            return Ok(());
        }
        if shown.eq_ignore_ascii_case("WIDTH") {
            return Err(self.unsupported(name.span, "an array named `WIDTH`"));
        }
        Err(self.error(name.span, format!("name already in use: `{shown}`")))
    }

    /// A built-in statement written like a SUB call (`KILL f$`, `CALL KILL(f$)`: its entry has no template): one
    /// argument per slot. Measured (`verification\v22_x02`, `x03`): any other number of arguments is "Syntax error -
    /// Reference: KILL fileSpec$".
    pub(super) fn plain_stmt(&mut self, stmt: ast::CallStmt, form: StmtSupported) -> R<()> {
        let node = stmt.node();
        if let Some(name_tok) = stmt.name() {
            self.not_an_assignment(node, name_tok, form.name)?;
        }
        if let Some(bad) = stmt.unparsed_args() {
            let msg = format!("cannot read the arguments of `{}`", form.name);
            return Err(self.error(bad.span(), msg));
        }
        let nodes = self.present_args(stmt.arg_list())?;
        let slots = stmt_slots(form.id).len();
        if nodes.len() != slots {
            return Err(self.error(node.span(), arity(form.name, slots, slots)));
        }
        let matched = nodes.into_iter().map(|n| Matched::Arg(Some(n))).collect();
        self.stmt_call(node, form, matched)
    }

    /// The array of a graphics `GET` or `PUT` (design D5 of `m3-get-put-bload`; measured,
    /// `verification\v28_x01`, `x10`–`x25`, `x58`–`x64`): an array as a whole (`a`, `a()`) or from an element
    /// (`a(5)`, `b(2, 3)`), of numbers; static or dynamic, of this body or shared, or an array parameter.
    ///
    /// The old compiler also takes an array of a user type or of strings and, for a name that is no array, the
    /// bytes of that plain variable; each is another text of its own and not supported yet. It writes the index
    /// twice, so a FUNCTION in it is called twice (measured from its C++): an index with a call is not supported
    /// yet either. Anything that is no variable is its error ("Expected variable name/array element").
    fn block_arg(&mut self, node: ast::Expr, name: &str) -> R<StmtArg> {
        let span = node.node().span();
        let whole = match node {
            ast::Expr::NameRef(n) => n.name(),
            ast::Expr::Call(c) if c.arg_list().is_some_and(|l| l.args().is_empty()) => c.name(),
            ast::Expr::Call(_)
            | ast::Expr::Literal(_)
            | ast::Expr::Field(_)
            | ast::Expr::Paren(_)
            | ast::Expr::Prefix(_)
            | ast::Expr::Bin(_) => None,
        };
        let (array, index) = if let Some(t) = whole {
            let (word, suffix) = self.split_name(t)?;
            let Some(array) = self.find_array(&word, suffix) else {
                if self.visible_const(&word).is_some() {
                    // Measured: "Expected variable name/array element" (`verification\v28_x64`).
                    return Err(self.error(span, format!("`{name}` needs an array here")));
                }
                let msg = format!("a variable that is no array as the array of `{name}`");
                return Err(self.unsupported(span, msg));
            };
            self.names.push((crate::SymbolKind::Var(array), t.span));
            (array, Vec::new())
        } else {
            // An element of a user type is a place here, not a value, as for `LEN` ([`Checker::load`]).
            let outer = self.len_place.replace(span);
            let e = self.expr(node);
            self.len_place = outer;
            match e?.kind {
                ExprKind::Load(Place::Element {
                    array: crate::ArrayRef::Var(array),
                    index,
                }) => (array, index),
                ExprKind::Load(_) => {
                    let msg = format!("a member of a `TYPE` as the array of `{name}`");
                    return Err(self.unsupported(span, msg));
                }
                ExprKind::Int(_)
                | ExprKind::Float(_)
                | ExprKind::Str(_)
                | ExprKind::Bound { .. }
                | ExprKind::VarPtr { .. }
                | ExprKind::VarSeg { .. }
                | ExprKind::Block { .. }
                | ExprKind::MemOf { .. }
                | ExprKind::MemAt { .. }
                | ExprKind::AddressOf(_)
                | ExprKind::MemRead { .. }
                | ExprKind::Convert { .. }
                | ExprKind::Binary { .. }
                | ExprKind::Unary { .. }
                | ExprKind::Concat(..)
                | ExprKind::StrCompare { .. }
                | ExprKind::Choice { .. }
                | ExprKind::Call { .. }
                | ExprKind::CallProc { .. } => {
                    return Err(self.error(span, format!("`{name}` needs an array here")));
                }
            }
        };
        let ty = self.prog.var(array).ty;
        if !(ty.is_int() || ty.is_float()) || matches!(ty, Ty::Bit { .. }) {
            let msg = format!("an array of strings, of a `TYPE` or of `_BIT` as the array of `{name}`");
            return Err(self.unsupported(span, msg));
        }
        if index.iter().any(calls_procedure) {
            let msg = format!("a call of a FUNCTION in the index of the array of `{name}`");
            return Err(self.unsupported(span, msg));
        }
        Ok(StmtArg::Block { array, index })
    }

    /// The array of `_UCHARPOS` (design D1 of `m3-images-fonts-mouse`; measured, `verification\v31_x01_accepted`,
    /// `x19`–`x22`, `x25`, `x26`): an array of LONG or `_UNSIGNED LONG`, whole (`p()`) or from an element (`p(2)`),
    /// of any number of dimensions, passed as a graphics `GET` passes its array. Anything else is the old
    /// compiler's "Expected LONG array-name" (`qb64pe.bas` 20901): the name alone, a scalar, a literal, an array
    /// of another type. An array used without `DIM` here (`nd&()`), which the old compiler takes, is not
    /// supported yet, as for `GET`.
    fn long_array_arg(&mut self, node: ast::Expr, name: &str) -> R<Expr> {
        let span = node.node().span();
        let expected = |c: &mut Self| c.error(span, format!("`{name}` needs an array of LONG here"));
        let ast::Expr::Call(c) = node else {
            return Err(expected(self));
        };
        let Some(t) = c.name() else {
            return Err(expected(self));
        };
        // An array of strings or of a `TYPE` is this error, before `block_arg` would mark it.
        let (word, suffix) = self.split_name(t)?;
        if let Some(array) = self.find_array(&word, suffix)
            && !matches!(self.prog.var(array).ty, Ty::I32 | Ty::U32)
        {
            return Err(expected(self));
        }
        let StmtArg::Block { array, index } = self.block_arg(node, name)? else {
            unreachable!("`block_arg` gives an array's bytes")
        };
        let ty = self.prog.var(array).ty;
        if !matches!(ty, Ty::I32 | Ty::U32) {
            return Err(expected(self));
        }
        Ok(Expr {
            span,
            ty,
            qb: ty,
            kind: ExprKind::Block { array, index },
        })
    }

    /// The variable of `GET` or `PUT` on a file (design D2 of `m3-console-files`; measured,
    /// `verification\v30_x20`–`x48`, `x67`): a variable, element or member, of any type but `_BIT` and a `TYPE`
    /// with a `STRING` in it, or a whole array `a()` of anything but variable-length strings. A name alone is a
    /// scalar variable, also when an array has the name (`x67`). Anything that is no variable is the old
    /// compiler's "Expected variable name/array element" (`GET #1, , "lit"` is its internal error, `x24`).
    fn bytes_arg(&mut self, node: ast::Expr, name: &str) -> R<StmtArg> {
        let span = node.node().span();
        if let ast::Expr::Call(c) = node
            && c.arg_list().is_some_and(|l| l.args().is_empty())
            && let Some(t) = c.name()
        {
            let (word, suffix) = self.split_name(t)?;
            let Some(array) = self.find_array(&word, suffix) else {
                if word.contains('.') {
                    // Measured: "Expected array index" (`v30_x48`: `PUT #1, 1, r.v()`).
                    return Err(self.error(span, format!("`{name}` of a whole member array")));
                }
                if self.procs_by_name.contains_key(&word) {
                    // Measured: "Expected (...)" for a FUNCTION's name (`v30_x171`, `x172`).
                    return Err(self.error(span, format!("`{name}` needs a variable here")));
                }
                let msg = format!("an array that is not declared as the variable of `{name}`");
                return Err(self.unsupported(span, msg));
            };
            self.names.push((crate::SymbolKind::Var(array), t.span));
            let ty = self.prog.var(array).ty;
            if ty == Ty::Str {
                // Measured: "Cannot pass array of variable-length strings" (`v30_x38`).
                return Err(self.error(span, format!("`{name}` of a whole array of variable-length strings")));
            }
            self.fixed_size(ty, name, span)?;
            return Ok(StmtArg::Block {
                array,
                index: Vec::new(),
            });
        }
        // A place of a user type is a place here, not a value, as for `LEN` ([`Checker::load`]).
        let outer = self.len_place.replace(span);
        let e = self.expr(node);
        self.len_place = outer;
        let ExprKind::Load(place) = e?.kind else {
            return Err(self.error(span, format!("`{name}` needs a variable here")));
        };
        self.fixed_size(self.prog.place_ty(&place), name, span)?;
        Ok(StmtArg::Bytes(place))
    }

    /// Whether `GET` and `PUT` can take the bytes of a place of type `ty`: not a `_BIT` ("Variable/element cannot
    /// be _BIT aligned", `v30_x25`, `x35`) and not a `TYPE` with a `STRING` in it ("UDT must have fixed size",
    /// `x40`).
    fn fixed_size(&mut self, ty: Ty, name: &str, span: Span) -> R<()> {
        if let Ty::Bit { .. } = ty {
            return Err(self.error(span, format!("`{name}` of a `_BIT` variable")));
        }
        if crate::holds_strings(&self.prog.types, ty) {
            let msg = format!(
                "`{name}` of a `TYPE {}` value: a `TYPE` with a `STRING` member has no fixed size",
                self.prog.type_name(ty)
            );
            return Err(self.error(span, msg));
        }
        if crate::owns_desc(&self.prog.types, ty) {
            // Measured (`verification\v35_e_dyn_statements`, `v35_x50`, `x54`, `x55`), compiled with tasks 5.1 and
            // 5.2 of `m3-typefields`.
            let msg = format!("`{name}` of a `TYPE` with a dynamic member array");
            return Err(self.unsupported(span, msg));
        }
        Ok(())
    }

    /// The supported statement `name` (one form, [`StmtRule::Plain`]) with every argument and choice left out,
    /// as the statement of `node`: what `$EXEICON` is where it stands, `_ICON` with no argument.
    pub(super) fn absent_stmt(&mut self, node: Node, name: &str) -> R<()> {
        let form = *stmt_lookup(name, false)
            .first()
            .unwrap_or_else(|| unreachable!("`{name}` is a supported statement"));
        let mut matched = Vec::new();
        absent(&form.id.get().template(), &mut matched);
        self.stmt_call(node, form, matched)
    }

    /// The typed statement of a form and what its template's arguments and choices matched.
    fn stmt_call(&mut self, node: Node, form: StmtSupported, matched: Vec<Matched>) -> R<()> {
        self.runtime_part(form.id, form.name, node.span())?;
        let slots = stmt_slots(form.id);
        let count = slots.iter().filter(|s| matches!(s, StmtSlot::Arg { .. })).count();
        let mut args = Vec::with_capacity(slots.len());
        let mut k = 0;
        for (slot, m) in slots.into_iter().zip(matched) {
            args.push(match (slot, m) {
                (StmtSlot::Arg { slot: Slot::Block, .. }, Matched::Arg(Some(arg))) => {
                    k += 1;
                    self.block_arg(arg, form.name)?
                }
                (StmtSlot::Arg { slot: Slot::Bytes, .. }, Matched::Arg(Some(arg))) => {
                    k += 1;
                    self.bytes_arg(arg, form.name)?
                }
                (StmtSlot::Arg { slot: Slot::Mem, .. }, Matched::Arg(Some(arg))) => {
                    k += 1;
                    StmtArg::Place(self.mem_arg(arg, form.name)?)
                }
                (StmtSlot::Arg { slot, .. }, Matched::Arg(Some(arg))) => {
                    k += 1;
                    let e = self.expr(arg)?;
                    let which = if count == 1 {
                        format!("`{}`", form.name)
                    } else {
                        format!("argument {k} of `{}`", form.name)
                    };
                    match form.rule {
                        StmtRule::Plain | StmtRule::Graphics | StmtRule::File => {
                            StmtArg::Value(self.slot_arg(Rule::Plain, slot, k - 1, e, &which)?)
                        }
                        StmtRule::Close | StmtRule::Swap | StmtRule::MidAssign => {
                            unreachable!("`{}` has a node of its own (`check\\io.rs`)", form.name)
                        }
                    }
                }
                (StmtSlot::Arg { .. }, Matched::Arg(None)) => {
                    k += 1;
                    StmtArg::Absent
                }
                (StmtSlot::Choice { .. }, Matched::Word(Some(w))) => StmtArg::Word(w),
                (StmtSlot::Choice { .. }, Matched::Word(None)) => StmtArg::Absent,
                (StmtSlot::Arg { .. }, Matched::Word(_)) | (StmtSlot::Choice { .. }, Matched::Arg(_)) => {
                    unreachable!("`{}`: the slots and the match follow one template", form.name)
                }
            });
        }
        if form.id.get().callname == "sub__echo" {
            self.echoes.push(node.span());
        }
        // `LOCK #1, TO 4`: the old compiler takes `TO` for the first record's name ("Name already in use (TO)",
        // measured, `verification\v30_x50`); the template alone would take the record as left out.
        if matches!(form.id.get().callname, "sub_lock" | "sub_unlock")
            && let [_, StmtArg::Absent, StmtArg::Word(_), _] = args[..]
        {
            let msg = format!("`{}` needs the first record before `TO`", form.name);
            return Err(self.error(node.span(), msg));
        }
        self.push(node, StmtKind::Builtin { id: form.id, args });
        Ok(())
    }
}

/// Whether evaluating the expression calls a SUB or FUNCTION of the program.
fn calls_procedure(e: &Expr) -> bool {
    match &e.kind {
        ExprKind::CallProc { .. } => true,
        ExprKind::Int(_) | ExprKind::Float(_) | ExprKind::Str(_) | ExprKind::VarSeg { .. } => false,
        ExprKind::Load(place) => place.indexes().iter().any(|i| calls_procedure(i)),
        ExprKind::Bound { dim, .. } => dim.as_deref().is_some_and(calls_procedure),
        ExprKind::VarPtr { index, .. } | ExprKind::Block { index, .. } => index.iter().any(calls_procedure),
        ExprKind::Convert { from, .. } | ExprKind::Unary { operand: from, .. } => calls_procedure(from),
        ExprKind::Binary { lhs, rhs, .. } | ExprKind::Concat(lhs, rhs) | ExprKind::StrCompare { lhs, rhs, .. } => {
            calls_procedure(lhs) || calls_procedure(rhs)
        }
        ExprKind::Choice { condition, then, else_ } => {
            calls_procedure(condition) || calls_procedure(then) || calls_procedure(else_)
        }
        ExprKind::Call { args, .. } => args.iter().flatten().any(calls_procedure),
        ExprKind::MemOf { of, .. } | ExprKind::AddressOf(of) => of.indexes().iter().any(|i| calls_procedure(i)),
        ExprKind::MemAt { offset, size } => calls_procedure(offset) || calls_procedure(size),
        ExprKind::MemRead { block, offset, .. } => {
            block.indexes().iter().any(|i| calls_procedure(i)) || calls_procedure(offset)
        }
    }
}

/// Whether slot `k` of a function stands alone in brackets after a comma in its template (`,[?]`), so that an
/// argument list may leave it empty: the second slot of `?[,[?][,?]]`, not the second of `?[,?]`, nor the first of
/// `INSTR`'s `[?],?,?` (`INSTR(, a$, b$)` is not measured and stays "not supported yet"). The rule goes by the
/// template, not by name: a function added later whose template has such a slot (`_MESSAGEBOX`, `_SNDNEW`, …)
/// accepts the gap at once, so its measurement includes one call with that slot left empty.
fn may_be_left_out(b: &qb64rust_builtins::Builtin, k: usize) -> bool {
    let Some(format) = b.specialformat else {
        return false;
    };
    let bytes = format.as_bytes();
    bytes
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c == b'?')
        .nth(k)
        .is_some_and(|(i, _)| i > 1 && bytes[i - 2..i] == *b",[" && bytes.get(i + 1) == Some(&b']'))
}

/// The numbers of arguments a function's template allows, bit `n` for `n` arguments; `None` without a template.
/// An optional group of several slots is given whole or not at all: `_SCREENIMAGE`'s `[?,?,?,?]` takes none or four
/// (the only such function template in the table; `verification\v31_x81`).
fn template_counts(b: &qb64rust_builtins::Builtin) -> Option<u64> {
    fn counts(items: &[Item]) -> u64 {
        items.iter().fold(1, |acc, item| {
            let step = match item {
                Item::Arg => 0b10,
                Item::Punct(_) | Item::Choice(_) => 1,
                Item::Optional(inner) => counts(inner) | 1,
            };
            (0..64)
                .filter(|&k| step & (1 << k) != 0)
                .fold(0, |sum, k| sum | (acc << k))
        })
    }
    let items = qb64rust_builtins::template::parse(b.specialformat?).ok()?;
    Some(counts(&items))
}

/// "0 or 4", "1, 2 or 3": the counts of [`template_counts`].
fn list_counts(counts: u64) -> String {
    let all: Vec<String> = (0..64)
        .filter(|&k| counts & (1 << k) != 0)
        .map(|k| k.to_string())
        .collect();
    match all.split_last() {
        Some((last, rest)) if !rest.is_empty() => format!("{} or {last}", rest.join(", ")),
        _ => all.concat(),
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
