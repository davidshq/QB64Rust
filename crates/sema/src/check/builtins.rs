//! Calls of built-in functions: one checker for every function `sema` compiles (design D2 of `m2-core-builtins`).
//! Which functions, their rules and result types: `crate::builtins`.

use super::places::numeric_type;
use super::{Checker, R};
use crate::builtins::{
    Rule, Slot, StmtRule, StmtSlot, StmtSupported, Supported, lookup, result_types, slots, stmt_lookup, stmt_slots,
};
use crate::{Expr, ExprKind, PLACE_ONLY_UNREACHABLE, Place, StmtArg, StmtKind, Ty};
use qb64rust_base::Span;
use qb64rust_builtins::template::{Atom, Item};
use qb64rust_syntax::SyntaxKind::Ident;
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
        | Rule::StrOrIndex => false,
    }
}

impl Checker<'_> {
    /// A call of a supported built-in. `list` is `None` for the bare name (`ERR`, `_PI`); a function that needs an
    /// argument gets the arity error here, like any other wrong number of arguments.
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
            | Rule::StringFill
            | Rule::StrOrIndex => {}
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
            // `ENVIRON$(name$)`: the entry that takes a string.
            (Rule::StrOrIndex, _) if string => return Ok(e),
            (_, Slot::Str) => true,
            (_, Slot::Long | Slot::Int64 | Slot::Double | Slot::Float | Slot::AnyNumeric) => false,
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
            Slot::Int64 => self.store(e, Ty::I64)?,
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
        for form in forms {
            let items = form.id.get().template();
            let mut matched = Vec::new();
            if match_items(&mut vec![&items[..]], &parts, 0, &mut matched) {
                return self.stmt_call(node, form, matched);
            }
        }
        Err(self.error(
            node.span(),
            "internal error: a built-in statement matches none of its forms",
        ))
    }

    /// A built-in statement written like a SUB call (`KILL f$`, `CALL KILL(f$)`: its entry has no template): one
    /// argument per slot. Measured (`verification\v22_x02`, `x03`): any other number of arguments is "Syntax error -
    /// Reference: KILL fileSpec$".
    pub(super) fn plain_stmt(&mut self, stmt: ast::CallStmt, form: StmtSupported) -> R<()> {
        let node = stmt.node();
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

    /// The typed statement of a form and what its template's arguments and choices matched.
    fn stmt_call(&mut self, node: Node, form: StmtSupported, matched: Vec<Matched>) -> R<()> {
        let slots = stmt_slots(form.id);
        let count = slots.iter().filter(|s| matches!(s, StmtSlot::Arg { .. })).count();
        let mut args = Vec::with_capacity(slots.len());
        let mut k = 0;
        for (slot, m) in slots.into_iter().zip(matched) {
            args.push(match (slot, m) {
                (StmtSlot::Arg { slot, .. }, Matched::Arg(Some(arg))) => {
                    k += 1;
                    let e = self.expr(arg)?;
                    let which = if count == 1 {
                        format!("`{}`", form.name)
                    } else {
                        format!("argument {k} of `{}`", form.name)
                    };
                    match form.rule {
                        StmtRule::Plain => StmtArg::Value(self.slot_arg(Rule::Plain, slot, k - 1, e, &which)?),
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
        self.push(node, StmtKind::Builtin { id: form.id, args });
        Ok(())
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
