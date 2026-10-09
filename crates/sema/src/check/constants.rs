//! `CONST` (design D6 of `m2-control-flow-slice`): the walk of the old compiler's evaluator over a value (the
//! arithmetic is [`crate::consteval`]), the scopes of constants, and their uses, which become literal nodes.
//!
//! Scopes, as measured (`verification\v17_e_*`): a constant is defined where its line stands, whatever control
//! flow surrounds it. A main-module constant is visible from its line on, in main and in the procedures defined
//! later; a name used as a variable before the line, anywhere, is "name already in use" at that use. A
//! procedure's constant is visible in that procedure from its line on, and may reuse the name of a main constant
//! or variable. A parameter with a main constant's name hides the constant.

use super::{Checker, R};
use crate::consteval::{self, Problem, Value, exact_text, shortest_text};
use crate::literal::{self, LitError, NumLit};
use crate::{BinOp, Const, ConstId, Expr, ExprKind, Storage, SymbolKind, Ty, UnOp};
use qb64rust_base::{Span, show_bytes, to_u32};
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::ast;
use qb64rust_syntax::tree::Tok;

/// The functions of the old evaluator (`Set_ConstFunctions`, `study\02` §7): "not supported yet" in a `CONST`.
/// Any other function is an error there.
const FUNCTIONS: &[&str] = &[
    "_PI", "_ACOS", "_ASIN", "_ARCSEC", "_ARCCSC", "_ARCCOT", "_SECH", "_CSCH", "_COTH", "COS", "SIN", "TAN", "LOG",
    "EXP", "ATN", "SQR", "_D2R", "_D2G", "_R2D", "_R2G", "_G2D", "_G2R", "ABS", "SGN", "INT", "_ROUND", "_CEIL", "FIX",
    "_SEC", "_CSC", "_COT", "_RGB32", "_RGBA32", "_RGB", "_RGBA", "_RED32", "_GREEN32", "_BLUE32", "_ALPHA32", "_RED",
    "_GREEN", "_BLUE", "_ALPHA", "CHR$", "ASC",
];

impl Checker<'_> {
    /// `CONST name = value, ...`. Defines no statement: uses of the constants become literals.
    pub(super) fn const_stmt(&mut self, stmt: ast::ConstStmt) -> R<()> {
        let node = stmt.node();
        let span = node.span();
        if self.label_line == Some((span.file, self.line(span))) {
            // Measured: "NULL string; nothing to evaluate" in the old compiler (`study\00` §6, "Fix").
            return Err(self.unsupported(span, "`CONST` after a label on the same line"));
        }
        for item in stmt.items() {
            let name_tok = self.need(item.name(), item.node().span())?;
            let value_node = self.need(item.value(), item.node().span())?;
            let (name, suffix) = self.split_name(name_tok)?;
            if self.procs_by_name.contains_key(&name) {
                return Err(self.in_use(name_tok));
            }
            self.reserved(name_tok, &name, suffix)?;
            let value = self.const_value(value_node)?;
            let value = self.problem(consteval::reread(value), value_node.node().span())?;
            let (ty, value) = self.problem(consteval::settle(value, suffix), name_tok.span)?;
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
                self.names.push((SymbolKind::Const(id), name_tok.span));
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
            self.names.push((SymbolKind::Const(id), name_tok.span));
        }
        Ok(())
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
        if let (true, true, &Value::Int(v)) = (to == c.ty, c.ty.is_int() && c.ty != Ty::I64, &c.value) {
            let NumLit::Int { value, ty, qb } = literal::constant_literal(v, c.ty) else {
                unreachable!("an integer constant is an integer literal");
            };
            self.names.push((SymbolKind::Const(id), t.span));
            return Ok(Expr {
                span,
                ty,
                qb,
                kind: ExprKind::Int(value),
            });
        }
        if to != c.ty {
            // A constant used with a suffix of a new numeric type, or one of such a type with another suffix: not
            // measured (task 8.3 of `m2-numeric-types`).
            self.later(c.ty, t.span, "another suffix on a constant of type")?;
            self.later(to, t.span, "a constant used as type")?;
        }
        let (ty, value) = self.problem(consteval::convert(c.ty, c.value, to), t.span)?;
        self.names.push((SymbolKind::Const(id), t.span));
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
                match literal::number(self.text(t.span), false) {
                    Ok(NumLit::Int { value, .. }) => Ok(Value::Int(value)),
                    Ok(NumLit::Float { text, .. }) => {
                        let f = self.problem(consteval::float_literal(&text), t.span)?;
                        Ok(Value::Float(f))
                    }
                    Err(LitError::Overflow) => Err(self.error(t.span, "overflow")),
                    Err(LitError::Error(msg)) => Err(self.error(t.span, msg)),
                    Err(LitError::Unsupported(what)) => Err(self.unsupported(t.span, what)),
                }
            }
            ast::Expr::NameRef(n) => {
                let t = self.need(n.name(), span)?;
                let (name, suffix) = self.split_name(t)?;
                let shown = show_bytes(self.text(t.span));
                match (self.visible_const(&name), suffix) {
                    (Some(id), None) => {
                        self.names.push((SymbolKind::Const(id), t.span));
                        Ok(self.prog.constant(id).value.clone())
                    }
                    (Some(_), Some(_)) => {
                        let msg = format!("a constant with a type suffix inside a `CONST`: `{shown}`");
                        Err(self.unsupported(t.span, msg))
                    }
                    (None, _) if is_const_function(&self.word(t)) => {
                        Err(self.unsupported(t.span, format!("the function `{shown}` in a `CONST`")))
                    }
                    // Measured: a variable is an error (`v17_e_err_const_var`). Another name may be a constant
                    // of an auto-included file (`$COLOR`), or one whose `CONST` was not supported.
                    (None, _) if self.is_variable_name(&name) => {
                        Err(self.error(t.span, format!("`{shown}` is a variable, not a constant")))
                    }
                    (None, _) => Err(self.unsupported(t.span, format!("`{shown}`, not a constant defined so far"))),
                }
            }
            ast::Expr::Call(call) => {
                let t = self.need(call.name(), span)?;
                let shown = show_bytes(self.text(t.span));
                if is_const_function(&self.word(t)) {
                    Err(self.unsupported(t.span, format!("the function `{shown}` in a `CONST`")))
                } else {
                    Err(self.error(t.span, format!("`{shown}` cannot be used in a `CONST`")))
                }
            }
            ast::Expr::Paren(p) => {
                let inner = self.need(p.inner(), span)?;
                let v = self.const_value(inner)?;
                // The old evaluator computes each parenthesised group and reads its printed result back.
                self.problem(consteval::reread(v), span)
            }
            ast::Expr::Prefix(prefix) => {
                let op_tok = self.need(prefix.op(), span)?;
                let operand = self.need(prefix.operand(), span)?;
                let op = self.const_unary_op(op_tok)?;
                match (op, prefix_op(self, operand)) {
                    // Measured: `--5` is an error of the old compiler.
                    (UnOp::Neg, Some(UnOp::Neg)) => {
                        return Err(self.error(span, "`-` directly before another `-` in a `CONST`"));
                    }
                    (_, Some(UnOp::Not)) => {
                        return Err(self.unsupported(span, "`NOT` as an operand in a `CONST`"));
                    }
                    _ => {}
                }
                let v = self.const_value(operand)?;
                self.problem(consteval::unary(op, &v), span)
            }
            ast::Expr::Bin(bin) => self.const_binary(bin),
            ast::Expr::Field(f) => Err(self.field(f)),
        }
    }

    fn const_unary_op(&mut self, t: Tok) -> R<UnOp> {
        if t.kind == SyntaxKind::Minus {
            return Ok(UnOp::Neg);
        }
        match self.word(t).as_str() {
            "NOT" => Ok(UnOp::Not),
            other => Err(self.unsupported(t.span, format!("operator `{other}` in a `CONST`"))),
        }
    }

    fn const_binary(&mut self, bin: ast::BinExpr) -> R<Value> {
        let span = bin.node().span();
        let op_tok = self.need(bin.op(), span)?;
        let Some(op) = self.bin_op(op_tok) else {
            let shown = self.word(op_tok);
            return Err(self.unsupported(op_tok.span, format!("operator `{shown}`")));
        };
        let (l, r) = (self.need(bin.lhs(), span)?, self.need(bin.rhs(), span)?);
        let logical = matches!(op, BinOp::And | BinOp::Or | BinOp::Xor | BinOp::Eqv | BinOp::Imp);
        if !logical && (prefix_op(self, l) == Some(UnOp::Not) || prefix_op(self, r) == Some(UnOp::Not)) {
            return Err(self.unsupported(span, "`NOT` as an operand in a `CONST`"));
        }
        if op == BinOp::Pow {
            // `^` is right-associative in the old evaluator: `a ^ b ^ c` (parsed `(a ^ b) ^ c`) is `a ^ (b ^ c)`.
            let mut operands = vec![r];
            let mut left = l;
            while let ast::Expr::Bin(b) = left
                && b.op().is_some_and(|t| self.bin_op(t) == Some(BinOp::Pow))
            {
                operands.push(self.need(b.rhs(), span)?);
                left = self.need(b.lhs(), span)?;
            }
            operands.push(left);
            // `operands` runs from the rightmost operand to the leftmost.
            let mut v = self.const_value(operands[0])?;
            for &o in &operands[1..] {
                let base = self.const_value(o)?;
                v = self.problem(consteval::binary(BinOp::Pow, &base, &v), span)?;
            }
            return Ok(v);
        }
        let a = self.const_value(l)?;
        let b = self.const_value(r)?;
        self.problem(consteval::binary(op, &a, &b), op_tok.span)
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

fn is_const_function(word: &str) -> bool {
    FUNCTIONS.contains(&word)
}

/// A constant's value as a literal node of type `ty`, as the old compiler substitutes it: an integer typed `ty`,
/// a float like a literal of `ty` (a SINGLE one held as a DOUBLE), a string.
fn literal_expr(span: Span, ty: Ty, value: Value) -> Expr {
    let (held, kind) = match value {
        Value::Int(i) => (ty, ExprKind::Int(i)),
        Value::Float(f) => {
            // A `_FLOAT` is exact here (`consteval::convert`); the others are read by C++ as a `double`.
            let text = if ty == Ty::F80 {
                exact_text(f.v)
            } else {
                shortest_text(f.v)
            };
            (if ty == Ty::F32 { Ty::F64 } else { ty }, ExprKind::Float(text))
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
