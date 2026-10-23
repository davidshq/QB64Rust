//! `CONST` (design D6 of `m2-control-flow-slice`): the walk of the old compiler's evaluator over a value (the
//! arithmetic is [`crate::consteval`]), the scopes of constants, and their uses, which become literal nodes.
//!
//! Scopes, as measured (`verification\v17_e_*`): a constant is defined where its line stands, whatever control
//! flow surrounds it. A main-module constant is visible from its line on, in main and in the procedures defined
//! later; a name used as a variable before the line, anywhere, is "name already in use" at that use. A
//! procedure's constant is visible in that procedure from its line on, and may reuse the name of a main constant
//! or variable. A parameter with a main constant's name hides the constant.

use super::{Checker, R};
use crate::consteval::{self, FloatTy, Problem, Value};
use crate::literal::{self, NumLit};
use crate::{BinOp, Const, ConstId, Expr, ExprKind, Storage, SymbolKind, Ty, UnOp};
use qb64rust_base::{Span, show_bytes, to_u32};
use qb64rust_builtins::{find_any, find_function_form};
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::ast;
use qb64rust_syntax::is_keyword;
use qb64rust_syntax::tree::Tok;

impl Checker<'_> {
    /// `CONST name = value, ...`. Defines no statement: uses of the constants become literals.
    pub(super) fn const_stmt(&mut self, stmt: ast::ConstStmt) -> R<()> {
        // A label before it on its line is no problem here (the old compiler fails, `DIVERGENCES.md` D-008).
        self.stmt.const_at = Some(self.main_offset(stmt.node().key()));
        self.stmt.consts_before = self.consts_before.get(&stmt.node().key()).copied();
        let in_colors = self.color_file == Some(stmt.node().span().file);
        for item in stmt.items() {
            let name_tok = self.need(item.name(), item.node().span())?;
            let value_node = self.need(item.value(), item.node().span())?;
            let (name, suffix) = self.split_name(name_tok)?;
            if self.procs_by_name.contains_key(&name) {
                return Err(self.in_use(name_tok));
            }
            if !matches!(suffix, None | Some(Ty::I32))
                && !name.starts_with('_')
                && (is_keyword(name.as_bytes()) || find_any(name.as_bytes()).any(|b| b.musthave.is_none()))
            {
                // Unlike a variable's, a constant's name is looked up without its suffix: a keyword, or a built-in
                // without a required suffix, is taken whatever the suffix (`qb64pe.bas` 2651–2692; measured,
                // `verification\v52_x32`, `x67`, `x68`: `CONST key$`, `exp$`, `key!`).
                return Err(self.in_use(name_tok));
            }
            self.reserved(name_tok, &name, suffix)?;
            consteval::take_fallback();
            let value = self.const_value(value_node)?;
            let value = self.problem(consteval::reread(value, true), value_node.node().span())?;
            let (ty, value) = self.problem(consteval::settle(value, suffix), name_tok.span)?;
            let fell_back = consteval::take_fallback();
            if fell_back {
                // `DIVERGENCES.md` D-026: the old compiler's value depends on the CPU and the system's library.
                self.diags.warning(
                    value_node.node().span(),
                    "this `CONST` is computed with correctly rounded math functions; QB64pe's value depends on the \
                     CPU and the system's library and may differ",
                );
            }
            let existing = match self.cur {
                Some(_) => self.consts_local.get(&name),
                None => self.consts_main.get(&name),
            };
            if let Some(&id) = existing {
                // Measured: the same constant again is accepted, with another value "name already in use".
                let c = self.prog.constant(id);
                if c.ty != ty || c.value != value {
                    return Err(self.in_use(name_tok));
                }
                self.stmt.names.push((SymbolKind::Const(id), name_tok.span));
                continue;
            }
            self.before_const(name_tok, &name)?;
            let id = ConstId(to_u32(self.prog.consts.len()));
            self.prog.consts.push(Const {
                name: name.clone(),
                ty,
                value,
                proc: self.cur,
            });
            match self.cur {
                Some(_) => self.consts_local.insert(name, id),
                None => self.consts_main.insert(name, id),
            };
            if in_colors {
                self.color_consts.insert(id);
            }
            if fell_back {
                self.fell_back.insert(id);
            }
            self.stmt.names.push((SymbolKind::Const(id), name_tok.span));
        }
        Ok(())
    }

    /// Whether a `CONST` of this name stands above the one being checked, in any body ([`Checker::const_names`]);
    /// true where that is not known.
    fn const_above(&self, name: &str) -> bool {
        self.stmt
            .consts_before
            .is_none_or(|n| self.const_names[..n].iter().any(|(c, _)| c == name))
    }

    /// The checks of a new constant's name against what came before it.
    fn before_const(&mut self, name_tok: Tok, name: &str) -> R<()> {
        if self.cur.is_none() {
            // Measured: the error is reported at the earlier use, in main or in a procedure.
            if let Some(&used) = self.used_names.get(name) {
                let shown = show_bytes(self.text(used));
                let line = self.line(name_tok.span);
                let msg = format!("name already in use: `{shown}` (a `CONST` of this name follows on line {line})");
                return Err(self.error(used, msg));
            }
            return Ok(());
        }
        let local_var = self.local.vars.keys().any(|(n, _)| n == name);
        let shared = self.dim_shared_plain.contains_key(name)
            || self
                .main
                .vars
                .iter()
                .any(|((n, _), v)| n == name && self.dim_shared.contains(v));
        if local_var || shared || self.used_local.contains_key(name) {
            let shown = show_bytes(self.text(name_tok.span));
            let msg = format!("a `CONST` `{shown}` beside a variable of that name in a SUB or FUNCTION");
            return Err(self.unsupported(name_tok.span, msg));
        }
        Ok(())
    }

    /// The constant a name means here, if any: in a procedure its own constants, then (unless a variable of the
    /// procedure has the name, such as a parameter) the main module's.
    pub(super) fn visible_const(&self, name: &str) -> Option<ConstId> {
        if self.cur.is_some() {
            if let Some(&id) = self.consts_local.get(name) {
                return Some(id);
            }
            if self.local.vars.keys().any(|(n, _)| n == name) {
                return None;
            }
        }
        self.consts_main.get(name).copied()
    }

    /// Whether a variable of this name (any type) is visible here.
    fn is_variable_name(&self, name: &str) -> bool {
        let in_scope =
            |vars: &std::collections::HashMap<(String, Ty), crate::VarId>| vars.keys().any(|(n, _)| n == name);
        match self.cur {
            Some(_) => in_scope(&self.local.vars) || self.dim_shared_plain.contains_key(name),
            None => in_scope(&self.main.vars),
        }
    }

    /// Notes a variable name used or declared at `t`, for the check of a later `CONST` of the name. Parameters and
    /// FUNCTION results are not noted (not measured).
    pub(super) fn note_var_name(&mut self, name: &str, storage: Storage, t: Tok) {
        if matches!(storage, Storage::Param(_) | Storage::Result(_)) {
            return;
        }
        self.used_names.entry(name.to_string()).or_insert(t.span);
        if self.cur.is_some() {
            self.used_local.entry(name.to_string()).or_insert(t.span);
        }
    }

    /// A use of constant `id` as `name<suffix>` in an expression: a literal of its type, or of the suffix's type
    /// (measured: `c%`, `c&`, `c!`, `c#` of a plain constant are the constant; `c$` of a numeric one is an error).
    ///
    /// A constant with an integer suffix other than `&&` is a literal with that suffix and its value's digits,
    /// believed the suffix's type and held as C++ types the digits (design D7 of `m2-numeric-types`): `CONST c% =
    /// 40000` prints -25536, `c% + 0` is 40000.
    pub(super) fn const_use(&mut self, id: ConstId, t: Tok, suffix: Option<Ty>, span: Span) -> R<Expr> {
        let c = self.prog.constant(id).clone();
        if c.ty == Ty::Str && suffix.is_some_and(|s| s != Ty::Str) {
            let shown = show_bytes(self.text(t.span));
            return Err(self.unsupported(t.span, format!("a string constant with a number suffix: `{shown}`")));
        }
        let to = suffix.unwrap_or(c.ty);
        let bit = |t: Ty| matches!(t, Ty::Bit { .. });
        if to != c.ty && (bit(to) || bit(c.ty)) {
            let shown = show_bytes(self.text(t.span));
            let msg = format!("a constant used with another suffix, one of them `_BIT`: `{shown}`");
            return Err(self.unsupported(t.span, msg));
        }
        // With an integer suffix, its own or another, the constant is a literal of that suffix with its value's digits,
        // rounded half to even if a float, in range of the suffix's type or not (measured, `verification\v21_d_const`,
        // `v21_g_const_suffix`: `CONST u~& = 4294967295` used as `u%` prints -1, `u% + 0` is 4294967295). A plain
        // constant used plainly or as `&&` is an `_INTEGER64` as it is.
        if to.is_int() && !(to == Ty::I64 && c.ty == Ty::I64) && c.ty != Ty::Str {
            let (_, value) = self.problem(consteval::settle(c.value, Some(to)), t.span)?;
            let Value::Int(v) = value else {
                unreachable!("an integer suffix settles to an integer");
            };
            let NumLit::Int { value, ty, qb } = literal::constant_literal(v, to) else {
                unreachable!("an integer constant is an integer literal");
            };
            self.stmt.names.push((SymbolKind::Const(id), t.span));
            return Ok(Expr {
                span,
                ty,
                qb,
                kind: ExprKind::Int(value),
            });
        }
        let (ty, value) = self.problem(consteval::convert(c.ty, c.value, to), t.span)?;
        self.stmt.names.push((SymbolKind::Const(id), t.span));
        Ok(literal_expr(span, ty, value))
    }

    fn problem<T>(&mut self, r: Result<T, Problem>, span: Span) -> R<T> {
        r.map_err(|p| match p {
            Problem::Error(msg) => self.error(span, msg),
            Problem::Unsupported(what) => self.unsupported(span, what),
        })
    }

    /// The value of a `CONST` expression, as the old evaluator computes it. Operands are literals and constants
    /// already defined.
    fn const_value(&mut self, node: ast::Expr) -> R<Value> {
        let span = node.node().span();
        match node {
            ast::Expr::Literal(lit) => {
                let t = self.need(lit.token(), span)?;
                if t.kind == SyntaxKind::StringLit {
                    let raw = self.text(t.span);
                    let inner = &raw[1..];
                    return Ok(Value::Str(inner.strip_suffix(b"\"").unwrap_or(inner).to_vec()));
                }
                let text = self.text(t.span).to_vec();
                self.problem(consteval::literal(&text), t.span)
            }
            ast::Expr::NameRef(n) => {
                let t = self.need(n.name(), span)?;
                let (name, suffix) = self.split_name(t)?;
                let shown = show_bytes(self.text(t.span));
                self.auto_const(&name, t.span)?;
                match (self.visible_const(&name), suffix) {
                    // With a suffix or without, the constant itself, with its own value and type (the old
                    // evaluator drops the suffix, design D6; measured, `verification\v45_b_asc_suffix`), but a
                    // string suffix on a number or a number suffix on a string is an error (`v45_x21`–`x23`).
                    (Some(id), suffix) => {
                        let c = self.prog.constant(id);
                        let string = c.ty == Ty::Str;
                        if suffix.is_some_and(|s| (s == Ty::Str) != string) {
                            return Err(self.error(t.span, format!("type mismatch: `{shown}`")));
                        }
                        // The old compiler's first pass includes the colour file at the `$COLOR` line, so a `CONST`
                        // above it does not know the colours (measured, `verification\v45_x59`).
                        if self.color_consts.contains(&id)
                            && self
                                .first_color
                                .zip(self.stmt.const_at)
                                .is_some_and(|(color, at)| at < color)
                        {
                            let msg = format!(
                                "`{shown}` is a `$COLOR` constant, which a `CONST` above the `$COLOR` line cannot name"
                            );
                            return Err(self.error(t.span, msg));
                        }
                        let v = consteval::lookup(c.ty, &c.value);
                        // A constant that took the correctly rounded value passes it on (D-026).
                        if self.fell_back.contains(&id) {
                            consteval::fell_back();
                        }
                        self.stmt.names.push((SymbolKind::Const(id), t.span));
                        Ok(v)
                    }
                    // Measured, `verification\v43_x06`: `CHR$ 65` is an error.
                    (None, _) if self.word(t) == "CHR$" => {
                        Err(self.error(t.span, format!("`{shown}` needs its code in parentheses")))
                    }
                    // `_PI` alone is pi (`const_eval.bas` `Numeric&`); another function's name alone is an error.
                    (None, None) if self.word(t) == "_PI" => Ok(Value::float(consteval::pi(), FloatTy::Float)),
                    (None, _) if consteval::function_arity(&self.word(t)).is_some() => {
                        Err(self.error(t.span, format!("`{shown}` needs its arguments in parentheses")))
                    }
                    // Measured: a variable is an error (`v17_e_err_const_var`). Another name may be a constant
                    // of an auto-included file (`$COLOR`), or one whose `CONST` was not supported.
                    (None, _) if self.is_variable_name(&name) => {
                        Err(self.error(t.span, format!("`{shown}` is a variable, not a constant")))
                    }
                    // A built-in function's name is no constant's (measured, `verification\v43_x40`: `CONST c =
                    // _ERRORLINE` is an error).
                    (None, suffix) if find_function_form(name.as_bytes(), suffix == Some(Ty::Str)).is_some() => {
                        Err(self.error(t.span, format!("`{shown}` cannot be used in a `CONST`")))
                    }
                    // A `CONST` of this name above may be one this compiler did not take (an error, a mark, a body
                    // not reached); any other name is an error (measured: "Unexpected element 'B'" for an
                    // undefined name, a constant defined below, a SUB's, a label's, a FUNCTION's or an array's name,
                    // upstream `const/undefined_argument*`, `verification` `v51_x44`-`x49`, `x52`, `x53`).
                    (None, _) if self.const_above(&name) => {
                        Err(self.unsupported(t.span, format!("`{shown}`, not a constant defined so far")))
                    }
                    (None, _) => Err(self.error(t.span, format!("`{shown}` is no constant defined above"))),
                }
            }
            ast::Expr::Call(call) => {
                let t = self.need(call.name(), span)?;
                let shown = show_bytes(self.text(t.span));
                let word = self.word(t);
                if consteval::function_arity(&word).is_none() {
                    return Err(self.error(t.span, format!("`{shown}` cannot be used in a `CONST`")));
                }
                let args = call.arg_list().map(ast::ArgList::args).unwrap_or_default();
                if word == "CHR$" && !matches!(args[..], [Some(_)]) {
                    // Measured: `CHR$()` and `CHR$(1, 2)` are errors (`verification\v43_x04`, `x05`).
                    return Err(self.error(t.span, format!("`{shown}` in a `CONST` takes one code")));
                }
                let mut values = Vec::new();
                for arg in args {
                    // Measured, `verification\v45_x11`, `x14`: `SIN()` is an error.
                    let Some(arg) = arg else {
                        return Err(self.error(
                            t.span,
                            format!("`{shown}` in a `CONST` needs a value for each argument"),
                        ));
                    };
                    // Each argument is computed and read back as a parenthesised group is.
                    let v = self.const_value(arg)?;
                    values.push(self.problem(consteval::reread(v, false), arg.node().span())?);
                }
                let v = self.problem(consteval::function(&word, &values), span)?;
                // The result is printed and read back into the expression (`EvaluateFunction$`).
                self.problem(consteval::reread(v, false), span)
            }
            ast::Expr::Paren(p) => {
                let inner = self.need(p.inner(), span)?;
                let v = self.const_value(inner)?;
                // The old evaluator computes each parenthesised group and reads its printed result back.
                self.problem(consteval::reread(v, false), span)
            }
            ast::Expr::Prefix(prefix) => {
                let op_tok = self.need(prefix.op(), span)?;
                let operand = self.need(prefix.operand(), span)?;
                let op = match ast::signs_negative(prefix.node()) {
                    Some(true) => UnOp::Neg,
                    // Measured: `+5`, `--5` and `2 * - -3` are errors of the old compiler ("Unexpected element
                    // '+'"), `- - -3`, `-+5` and `+-3` are -3, -5 and -3 (`verification\v25_signs_const`,
                    // `v25_x*`).
                    Some(false) => {
                        let msg = "the signs before a value in a `CONST` amount to `+`, which cannot start a value";
                        return Err(self.error(span, msg));
                    }
                    None => self.const_unary_op(op_tok)?,
                };
                if prefix_op(self, operand) == Some(UnOp::Not) {
                    return Err(self.unsupported(span, "`NOT` as an operand in a `CONST`"));
                }
                let v = self.const_value(operand)?;
                self.problem(consteval::unary(op, &v), span)
            }
            ast::Expr::Bin(bin) => self.const_binary(bin),
            ast::Expr::Field(f) => Err(self.field(f)),
        }
    }

    /// A prefix operator that is no sign.
    fn const_unary_op(&mut self, t: Tok) -> R<UnOp> {
        match self.word(t).as_str() {
            "NOT" => Ok(UnOp::Not),
            other => Err(self.unsupported(t.span, format!("operator `{other}` in a `CONST`"))),
        }
    }

    fn const_binary(&mut self, bin: ast::BinExpr) -> R<Value> {
        let span = bin.node().span();
        let op_tok = self.need(bin.op(), span)?;
        // A run of signs between the operands is one sign: `2 - -3` is `2 + 3`, `2 + -3` is `2 - 3`.
        let op = match ast::signs_negative(bin.node()) {
            Some(true) => Some(BinOp::Sub),
            Some(false) => Some(BinOp::Add),
            None => self.bin_op(op_tok),
        };
        let Some(op) = op.or_else(|| (self.word(op_tok) == "ROOT").then_some(BinOp::Pow)) else {
            let shown = self.word(op_tok);
            return Err(self.unsupported(op_tok.span, format!("operator `{shown}`")));
        };
        let (l, r) = (self.need(bin.lhs(), span)?, self.need(bin.rhs(), span)?);
        let logical = matches!(op, BinOp::And | BinOp::Or | BinOp::Xor | BinOp::Eqv | BinOp::Imp);
        if !logical && (prefix_op(self, l) == Some(UnOp::Not) || prefix_op(self, r) == Some(UnOp::Not)) {
            return Err(self.unsupported(span, "`NOT` as an operand in a `CONST`"));
        }
        if let Some(first_root) = self.exponent_op(op_tok) {
            // `^` and `ROOT` are right-associative in the old evaluator: `a ^ b ^ c` (parsed `(a ^ b) ^ c`) is
            // `a ^ (b ^ c)`.
            let mut operands = vec![(first_root, r)];
            let mut left = l;
            while let ast::Expr::Bin(b) = left
                && let Some(root) = b.op().and_then(|t| self.exponent_op(t))
            {
                operands.push((root, self.need(b.rhs(), span)?));
                left = self.need(b.lhs(), span)?;
            }
            if operands.len() > 1 {
                // The spec's warning (`language/constants`): the same text in a statement means another value.
                self.diags.warning(
                    span,
                    "a chain of `^` in a `CONST` is computed right to left (`2 ^ 3 ^ 2` is 512), in a statement left \
                     to right (64); add parentheses to say which",
                );
            }
            // `operands` runs from the rightmost operand to the leftmost, each with the operator on its left.
            let mut v = self.const_value(operands[0].1)?;
            let mut root = operands[0].0;
            for &(next_root, o) in &operands[1..] {
                let base = self.const_value(o)?;
                v = self.exponent(root, &base, &v, span)?;
                root = next_root;
            }
            let base = self.const_value(left)?;
            return self.exponent(root, &base, &v, span);
        }
        let a = self.const_value(l)?;
        let b = self.const_value(r)?;
        self.problem(consteval::binary(op, &a, &b), op_tok.span)
    }

    /// Whether `t` is `^` (`Some(false)`) or `ROOT` (`Some(true)`).
    fn exponent_op(&self, t: Tok) -> Option<bool> {
        if t.kind == SyntaxKind::Caret {
            Some(false)
        } else {
            (self.word(t) == "ROOT").then_some(true)
        }
    }

    /// `base ^ v` or `base ROOT v`.
    fn exponent(&mut self, root: bool, base: &Value, v: &Value, span: Span) -> R<Value> {
        let r = if root {
            consteval::root(base, v)
        } else {
            consteval::binary(BinOp::Pow, base, v)
        };
        self.problem(r, span)
    }
}

/// The prefix operator of `e` (`-` or `NOT`), if it is a prefix expression.
fn prefix_op(c: &Checker, e: ast::Expr) -> Option<UnOp> {
    let ast::Expr::Prefix(p) = e else {
        return None;
    };
    let t = p.op()?;
    if t.kind == SyntaxKind::Minus {
        Some(UnOp::Neg)
    } else if c.word(t) == "NOT" {
        Some(UnOp::Not)
    } else {
        None
    }
}

/// A constant's value as a literal node of type `ty`, as the old compiler substitutes it: an integer typed `ty`,
/// a float as the text the old compiler writes (its 19 digits, design D2; C++ reads it as a `long double` for a
/// `_FLOAT`, else as a `double`, a SINGLE one held as a DOUBLE), a string.
fn literal_expr(span: Span, ty: Ty, value: Value) -> Expr {
    let (held, kind) = match value {
        Value::Int(i) => (ty, ExprKind::Int(i)),
        Value::UInt(u) => (ty, ExprKind::Int(u.cast_signed())),
        Value::Float(r, _) => {
            let v = r.one().expect("a constant's value is one value (`consteval::settle`)");
            (
                if ty == Ty::F32 { Ty::F64 } else { ty },
                ExprKind::Float(consteval::c_text(v)),
            )
        }
        Value::Str(s) => (Ty::Str, ExprKind::Str(s)),
    };
    Expr {
        span,
        ty: held,
        qb: ty,
        kind,
    }
}
