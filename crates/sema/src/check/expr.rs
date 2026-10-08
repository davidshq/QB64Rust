//! Expressions, and the conversions for storing a value.

use super::builtins::supported_builtin;
use super::ops::{Op, TypeError, Typed, Typing, fold_binary, fold_unary, op_typing, wrap};
use super::{Checker, Failed, R};
use crate::builtins::slots;
use crate::literal::{self, LitError, NumLit};
use crate::{BinOp, ConvKind, Expr, ExprKind, Place, ProcId, ProcKind, Ty, UnOp};
use qb64rust_base::show_bytes;
use qb64rust_builtins::find_any;
use qb64rust_syntax::SyntaxKind::{self, Minus, Number};
use qb64rust_syntax::ast;
use qb64rust_syntax::tree::Tok;

impl Checker<'_> {
    // ---- expressions ----

    pub(super) fn expr(&mut self, node: ast::Expr) -> R<Expr> {
        let span = node.node().span();
        match node {
            ast::Expr::Literal(lit) => {
                let t = self.need(lit.token(), span)?;
                if t.kind == SyntaxKind::StringLit {
                    let raw = self.text(t.span);
                    let inner = &raw[1..];
                    let inner = inner.strip_suffix(b"\"").unwrap_or(inner);
                    Ok(Expr {
                        span,
                        ty: Ty::Str,
                        qb: Ty::Str,
                        kind: ExprKind::Str(inner.to_vec()),
                    })
                } else {
                    self.number(t, false)
                }
            }
            ast::Expr::NameRef(name) => {
                let t = self.need(name.name(), span)?;
                let (name, suffix) = self.split_name(t)?;
                if let Some(p) = self.proc_in_expr(&name, suffix) {
                    return self.call_function(p, t, suffix, None, span);
                }
                if let Some(c) = self.visible_const(&name) {
                    return self.const_use(c, t, suffix, span);
                }
                // A built-in that needs no argument is called by its bare name (`ERR`, `ERL`, `_PI`).
                if let Some(s) = supported_builtin(&name, suffix)
                    && slots(s).iter().all(|&(_, optional)| optional)
                {
                    return self.builtin(s, None, span);
                }
                if is_builtin_function(&name, suffix) {
                    let shown = show_bytes(self.text(t.span));
                    return Err(self.unsupported(t.span, format!("`{shown}`")));
                }
                if let Some(place) = self.dotted(t, &name, suffix)? {
                    return self.load(place, span);
                }
                let id = self.variable(t, name, suffix)?;
                self.load(Place::Var(id), span)
            }
            ast::Expr::Paren(paren) => {
                let inner = self.need(paren.inner(), span)?;
                let mut e = self.expr(inner)?;
                e.span = span;
                Ok(e)
            }
            ast::Expr::Prefix(prefix) => self.prefix(prefix),
            ast::Expr::Bin(bin) => self.binary(bin),
            ast::Expr::Call(call) => self.call(call),
            ast::Expr::Field(field) => {
                let place = self.field_place(field)?;
                self.load(place, span)
            }
        }
    }

    /// A member of an element (`a(1).b`) where only constants may stand: not supported yet.
    pub(super) fn field(&mut self, node: ast::FieldExpr) -> Failed {
        let span = node.node().span();
        self.unsupported(span, "a `TYPE` member here")
    }

    /// The arguments of a call, in order; none without an argument list. An omitted argument (`f(a, , b)`) is not
    /// supported yet.
    pub(super) fn present_args<'t>(&mut self, list: Option<ast::ArgList<'t>>) -> R<Vec<ast::Expr<'t>>> {
        let Some(list) = list else {
            return Ok(Vec::new());
        };
        if let Some(t) = list.type_arg() {
            return Err(self.unsupported(t.span(), "a type name as an argument"));
        }
        let mut out = Vec::new();
        for a in list.args() {
            match a {
                Some(e) => out.push(e),
                None => return Err(self.unsupported(list.node().span(), "omitted arguments")),
            }
        }
        Ok(out)
    }

    pub(super) fn number(&mut self, t: Tok, negative: bool) -> R<Expr> {
        let span = t.span;
        match literal::number(self.text(span), negative) {
            Ok(NumLit::Int { value, ty }) => Ok(Expr {
                span,
                ty,
                qb: ty,
                kind: ExprKind::Int(value),
            }),
            Ok(NumLit::Float { text, ty }) => {
                // SINGLE literals are C `double` constants in the old compiler (`study\02` §1.4).
                let held = if ty == Ty::F32 { Ty::F64 } else { ty };
                Ok(Expr {
                    span,
                    ty: held,
                    qb: ty,
                    kind: ExprKind::Float(text),
                })
            }
            Err(LitError::Overflow) => Err(self.error(span, "overflow")),
            Err(LitError::Unsupported(what)) => Err(self.unsupported(span, what)),
        }
    }

    fn prefix(&mut self, node: ast::PrefixExpr) -> R<Expr> {
        let span = node.node().span();
        let op_tok = self.need(node.op(), span)?;
        let operand = self.need(node.operand(), span)?;
        let op = if op_tok.kind == Minus {
            UnOp::Neg
        } else {
            match self.word(op_tok).as_str() {
                "NOT" => UnOp::Not,
                "_NEGATE" => UnOp::Negate,
                other => return Err(self.unsupported(op_tok.span, format!("operator `{other}`"))),
            }
        };
        // A minus directly before a decimal literal is part of the literal (step C).
        if let (UnOp::Neg, ast::Expr::Literal(lit)) = (op, operand) {
            let t = self.need(lit.token(), span)?;
            if t.kind == Number && self.text(t.span)[0] != b'&' {
                let mut e = self.number(t, true)?;
                e.span = span;
                return Ok(e);
            }
        }
        let e = self.expr(operand)?;
        let Ok(Typed::Num(typing)) = op_typing(Op::Un(op), (e.ty, e.qb), None) else {
            let shown = if op == UnOp::Neg {
                "unary `-`".to_string()
            } else {
                format!("`{}`", self.word(op_tok))
            };
            return Err(self.error(span, format!("{shown} needs a number")));
        };
        let e = self.operand(e, &typing);
        if let (ExprKind::Int(v), true) = (&e.kind, self.fold) {
            return Ok(Expr {
                span,
                ty: typing.ty,
                qb: typing.qb,
                kind: ExprKind::Int(fold_unary(op, *v, typing.ty)),
            });
        }
        Ok(Expr {
            span,
            ty: typing.ty,
            qb: typing.qb,
            kind: ExprKind::Unary {
                op,
                operand: Box::new(e),
            },
        })
    }

    fn binary(&mut self, node: ast::BinExpr) -> R<Expr> {
        let span = node.node().span();
        let l = self.need(node.lhs(), span)?;
        let op_tok = self.need(node.op(), span)?;
        let r = self.need(node.rhs(), span)?;
        let Some(op) = self.bin_op(op_tok) else {
            let shown = self.word(op_tok);
            return Err(self.unsupported(op_tok.span, format!("operator `{shown}`")));
        };
        if op == BinOp::Imp && self.is_imp(l) {
            // Measured: the old compiler computes `a IMP b IMP c` as `a OR b OR c` (`v17_g_imp_eqv`).
            return Err(self.unsupported(op_tok.span, "`IMP` with an `IMP` as its left operand"));
        }
        let lhs = self.expr(l)?;
        let rhs = self.expr(r)?;
        let typing = match op_typing(Op::Bin(op), (lhs.ty, lhs.qb), Some((rhs.ty, rhs.qb))) {
            Ok(Typed::Num(t)) => t,
            Ok(Typed::Concat) => {
                return Ok(Expr {
                    span,
                    ty: Ty::Str,
                    qb: Ty::Str,
                    kind: ExprKind::Concat(Box::new(lhs), Box::new(rhs)),
                });
            }
            Ok(Typed::StrCompare) => {
                return Ok(Expr {
                    span,
                    ty: Ty::I32,
                    qb: Ty::I32,
                    kind: ExprKind::StrCompare {
                        op,
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    },
                });
            }
            Err(TypeError::OnStrings) => {
                return Err(self.error(op_tok.span, "this operator cannot be used on strings"));
            }
            Err(TypeError::Mixed) => return Err(self.error(span, "cannot mix strings and numbers")),
        };
        let lhs = self.operand(lhs, &typing);
        let rhs = self.operand(rhs, &typing);
        if let (ExprKind::Int(a), ExprKind::Int(b), true) = (&lhs.kind, &rhs.kind, self.fold)
            && let Some(v) = fold_binary(op, *a, *b, typing.ty)
        {
            return Ok(Expr {
                span,
                ty: typing.ty,
                qb: typing.qb,
                kind: ExprKind::Int(v),
            });
        }
        Ok(Expr {
            span,
            ty: typing.ty,
            qb: typing.qb,
            kind: ExprKind::Binary {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        })
    }

    /// The binary operator a token stands for; `None` for a token that is not one.
    pub(super) fn bin_op(&self, t: Tok) -> Option<BinOp> {
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "token kinds: every other token is not a binary operator"
        )]
        Some(match t.kind {
            SyntaxKind::Plus => BinOp::Add,
            SyntaxKind::Minus => BinOp::Sub,
            SyntaxKind::Star => BinOp::Mul,
            SyntaxKind::Slash => BinOp::Div,
            SyntaxKind::Backslash => BinOp::IDiv,
            SyntaxKind::Caret => BinOp::Pow,
            SyntaxKind::Eq => BinOp::Eq,
            SyntaxKind::Ne => BinOp::Ne,
            SyntaxKind::Lt => BinOp::Lt,
            SyntaxKind::Gt => BinOp::Gt,
            SyntaxKind::Le => BinOp::Le,
            SyntaxKind::Ge => BinOp::Ge,
            SyntaxKind::Ident => match self.word(t).as_str() {
                "MOD" => BinOp::Mod,
                "AND" => BinOp::And,
                "OR" => BinOp::Or,
                "XOR" => BinOp::Xor,
                "EQV" => BinOp::Eqv,
                "IMP" => BinOp::Imp,
                "_ANDALSO" => BinOp::AndAlso,
                "_ORELSE" => BinOp::OrElse,
                _ => return None,
            },
            _ => return None,
        })
    }

    /// Whether `e`, inside any parentheses, is an `IMP`.
    fn is_imp(&self, mut e: ast::Expr) -> bool {
        while let ast::Expr::Paren(p) = e {
            match p.inner() {
                Some(inner) => e = inner,
                None => return false,
            }
        }
        matches!(e, ast::Expr::Bin(b) if b.op().is_some_and(|t| self.bin_op(t) == Some(BinOp::Imp)))
    }

    /// Prepares an operand as `typing` says: a float rounded half to even to `_INTEGER64` first where the operator
    /// takes integers, then converted exactly to the computation type.
    fn operand(&self, e: Expr, typing: &Typing) -> Expr {
        let e = if typing.round && e.ty.is_float() {
            conv(e, Ty::I64, ConvKind::RoundEven)
        } else {
            e
        };
        self.convert_exact(e, typing.operands)
    }

    /// The procedure a name in an expression calls. A SUB named like a built-in function (`SUB loc`, allowed,
    /// `verification\v19_proc_names.txt`) leaves the name to the built-in there.
    fn proc_in_expr(&self, name: &str, suffix: Option<Ty>) -> Option<ProcId> {
        let &p = self.procs_by_name.get(name)?;
        let sub = matches!(self.prog.proc(p).kind, ProcKind::Sub);
        (!(sub && is_builtin_function(name, suffix))).then_some(p)
    }

    pub(super) fn call(&mut self, node: ast::CallExpr) -> R<Expr> {
        let span = node.node().span();
        let name_tok = self.need(node.name(), span)?;
        let (proc_name, suffix) = self.split_name(name_tok)?;
        if let Some(p) = self.proc_in_expr(&proc_name, suffix) {
            let args = self.need(node.arg_list(), span)?;
            return self.call_function(p, name_tok, suffix, Some(args), span);
        }
        match (proc_name.as_str(), suffix) {
            ("LBOUND", None) => return self.bound_fn(node, false, span),
            ("UBOUND", None) => return self.bound_fn(node, true, span),
            _ => {}
        }
        if let Some(s) = supported_builtin(&proc_name, suffix) {
            return self.builtin(s, node.arg_list(), span);
        }
        if is_builtin_function(&proc_name, suffix) {
            let shown = show_bytes(self.text(name_tok.span));
            return Err(self.unsupported(name_tok.span, format!("`{shown}`")));
        }
        match self.find_array(&proc_name, suffix) {
            Some(a) => {
                let place = self.element(node, a, name_tok)?;
                self.load(place, span)
            }
            None => Err(self.not_an_array(name_tok, &proc_name)),
        }
    }

    /// Converts a value for storing into a variable or argument of type `to` (spec: storing into an integer).
    pub(super) fn store(&mut self, e: Expr, to: Ty) -> R<Expr> {
        match (e.ty == Ty::Str, to == Ty::Str) {
            (true, true) => return Ok(e),
            (false, false) => {}
            (true, false) => return Err(self.error(e.span, "cannot store a string in a number variable")),
            (false, true) => return Err(self.error(e.span, "cannot store a number in a string variable")),
        }
        if e.ty.is_float() && to.is_int() {
            let e = if to == Ty::I16 {
                // INTEGER targets round the SINGLE value ("**32 rounding fix", `study\02` §1.5).
                let single = self.convert_exact(e, Ty::F32);
                conv(single, Ty::I32, ConvKind::RoundEven)
            } else {
                conv(e, Ty::I64, ConvKind::RoundEven)
            };
            return Ok(self.convert_exact(e, to));
        }
        Ok(self.convert_exact(e, to))
    }

    /// Converts between numeric types without rounding to an integer: integer widths, integer to float, float widths.
    /// Integer constants are folded.
    pub(super) fn convert_exact(&self, e: Expr, to: Ty) -> Expr {
        if e.ty == to {
            return e;
        }
        debug_assert!(e.ty.is_numeric() && to.is_numeric() && !(e.ty.is_float() && to.is_int()));
        if let (ExprKind::Int(v), true, true) = (&e.kind, to.is_int(), self.fold) {
            let v = wrap(*v, to);
            return Expr {
                span: e.span,
                ty: to,
                qb: to,
                kind: ExprKind::Int(v),
            };
        }
        let how = if e.ty.is_int() && to.is_int() {
            if to > e.ty { ConvKind::Widen } else { ConvKind::Truncate }
        } else if e.ty.is_float() && to > e.ty {
            ConvKind::Widen
        } else {
            ConvKind::Nearest
        };
        conv(e, to, how)
    }
}

/// Whether `name` with `suffix` names a built-in function as it must be written: `LEN` bare, `LEFT$` with its `$`
/// (a bare `left` is free, design D3).
fn is_builtin_function(name: &str, suffix: Option<Ty>) -> bool {
    find_any(name.as_bytes()).any(|b| {
        b.kind == qb64rust_builtins::Kind::Function
            && match b.musthave {
                None => suffix.is_none(),
                Some("$") => suffix == Some(Ty::Str),
                Some(_) => false,
            }
    })
}

pub(super) fn conv(e: Expr, to: Ty, how: ConvKind) -> Expr {
    Expr {
        span: e.span,
        ty: to,
        qb: to,
        kind: ExprKind::Convert { how, from: Box::new(e) },
    }
}
