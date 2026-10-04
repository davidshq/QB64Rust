//! From the syntax tree to the typed [`Program`].

use crate::literal::{self, LitError, NumLit};
use crate::{BinOp, ConvKind, Expr, ExprKind, PrintItem, Program, Stmt, StmtKind, Ty, Var, VarId};
use qb64rust_base::{Diagnostics, SourceFile, Span, show_bytes};
use qb64rust_builtins::{BuiltinId, find_function};
use qb64rust_syntax::SyntaxKind::{self, *};
use qb64rust_syntax::tree::{Element, Node, Tok};
use std::collections::HashMap;

/// Checks a parsed file. Statements that already have a parse error are skipped (one error per statement).
pub fn check(root: Node, file: &SourceFile, parse_diags: &Diagnostics) -> (Program, Diagnostics) {
    check_with(root, file, parse_diags, true)
}

/// [`check`] with integer constant folding switched on or off. Off is for tests only: the slice programs must
/// print the same either way (design D5).
pub fn check_with(root: Node, file: &SourceFile, parse_diags: &Diagnostics, fold: bool) -> (Program, Diagnostics) {
    let mut c = Checker {
        fold,
        file,
        prog: Program::default(),
        vars: HashMap::new(),
        plain: HashMap::new(),
        diags: Diagnostics::new(),
        stmt_error: false,
        console_only: false,
    };
    let error_starts: Vec<u32> = parse_diags.list().iter().map(|d| d.span.start).collect();
    // Past the error cap the parser records no more errors, so statements from the cap on may be malformed.
    let cap_start = parse_diags
        .is_capped()
        .then(|| parse_diags.list().last().map_or(0, |d| d.span.start));
    for stmt in root.child_nodes() {
        let s = stmt.span();
        if cap_start.is_some_and(|c| s.end >= c) {
            break;
        }
        if error_starts.iter().any(|&o| o >= s.start && o <= s.end) || stmt.kind() == Error {
            continue;
        }
        c.stmt_error = false;
        c.statement(stmt);
    }
    if !c.console_only {
        let at = Span::new(root.file, 0, 0);
        c.diags
            .error(at, "programs without `$CONSOLE:ONLY` are not supported yet");
    }
    (c.prog, c.diags)
}

struct Checker<'a> {
    file: &'a SourceFile,
    prog: Program,
    /// (name, type) -> variable.
    vars: HashMap<(String, Ty), VarId>,
    /// Type of the plain (suffix-less) name after a `DIM name AS type`.
    plain: HashMap<String, Ty>,
    diags: Diagnostics,
    stmt_error: bool,
    console_only: bool,
    fold: bool,
}

/// An expression could not be typed; the error is already reported.
struct Failed;

type R<T> = Result<T, Failed>;

fn text(file: &SourceFile, span: Span) -> &[u8] {
    &file.bytes[span.start as usize..span.end as usize]
}

impl Checker<'_> {
    fn error(&mut self, span: Span, msg: impl Into<String>) -> Failed {
        if !self.stmt_error {
            self.stmt_error = true;
            self.diags.error(span, msg);
        }
        Failed
    }

    fn text(&self, span: Span) -> &[u8] {
        text(self.file, span)
    }

    fn word(&self, t: Tok) -> String {
        String::from_utf8_lossy(self.text(t.span)).to_ascii_uppercase()
    }

    fn push(&mut self, node: Node, kind: StmtKind) {
        let span = node.span();
        let start = node.first_token().map_or(span.start, |t| t.span.start);
        let line = self.file.line_col(start).0;
        self.prog.stmts.push(Stmt { span, line, kind });
    }

    fn statement(&mut self, node: Node) {
        let _ = match node.kind() {
            MetaStmt => self.meta(node),
            PrintStmt => self.print(node),
            DimStmt => self.dim(node),
            AssignStmt => self.assign(node),
            EndStmt => {
                self.push(node, StmtKind::End);
                Ok(())
            }
            _ => Ok(()),
        };
    }

    fn meta(&mut self, node: Node) -> R<()> {
        let tok = node.first_token().unwrap();
        let raw = self.text(tok.span);
        let trimmed: Vec<u8> = raw.iter().copied().filter(|b| !b.is_ascii_whitespace()).collect();
        if trimmed.eq_ignore_ascii_case(b"$CONSOLE:ONLY") {
            if !self.console_only {
                self.console_only = true;
                self.push(node, StmtKind::ConsoleOnly);
            }
            Ok(())
        } else {
            let shown = show_bytes(raw.split(|&b| b == b':' || b == b' ').next().unwrap_or(raw));
            Err(self.error(tok.span, format!("metacommand `{shown}` is not supported yet")))
        }
    }

    // ---- variables ----

    /// Splits `name<suffix>` and gives the suffix's type (`None` without a suffix).
    fn split_name(&mut self, t: Tok) -> R<(String, Option<Ty>)> {
        let bytes = self.text(t.span);
        let end = bytes
            .iter()
            .position(|&b| !(b.is_ascii_alphanumeric() || b == b'_' || b == b'.'));
        let (name, suffix) = bytes.split_at(end.unwrap_or(bytes.len()));
        let name = String::from_utf8_lossy(name).to_ascii_uppercase();
        let ty = match suffix {
            b"" => None,
            b"%" => Some(Ty::I16),
            b"&" => Some(Ty::I32),
            b"&&" => Some(Ty::I64),
            b"!" => Some(Ty::F32),
            b"#" => Some(Ty::F64),
            b"##" => Some(Ty::F80),
            b"$" => Some(Ty::Str),
            other => {
                let msg = format!("the type suffix `{}` is not supported yet", show_bytes(other));
                return Err(self.error(t.span, msg));
            }
        };
        Ok((name, ty))
    }

    fn new_var(&mut self, name: String, ty: Ty) -> VarId {
        let id = VarId(self.prog.vars.len() as u32);
        self.prog.vars.push(Var { name: name.clone(), ty });
        self.vars.insert((name, ty), id);
        id
    }

    /// The variable a name refers to at this point of the program, created on first use.
    fn resolve(&mut self, t: Tok) -> R<VarId> {
        let (name, suffix) = self.split_name(t)?;
        let ty = suffix.or_else(|| self.plain.get(&name).copied()).unwrap_or(Ty::F32);
        if find_function(name.as_bytes()).is_some() && suffix.is_none() {
            let shown = show_bytes(self.text(t.span));
            return Err(self.error(t.span, format!("`{shown}` is not supported yet")));
        }
        Ok(match self.vars.get(&(name.clone(), ty)) {
            Some(&id) => id,
            None => self.new_var(name, ty),
        })
    }

    fn dim(&mut self, node: Node) -> R<()> {
        for item in node.child_nodes().filter(|n| n.kind() == DimItem) {
            let name_tok = item.child_tokens().next().unwrap();
            let (name, suffix) = self.split_name(name_tok)?;
            let as_clause = item.child_nodes().find(|n| n.kind() == AsClause);
            let ty = match (suffix, as_clause) {
                (Some(_), Some(a)) => {
                    return Err(self.error(a.span(), "a name with a type suffix cannot have an `AS` clause"));
                }
                (Some(t), None) => t,
                (None, None) => Ty::F32,
                (None, Some(a)) => {
                    let words: Vec<String> = a.child_tokens().skip(1).map(|t| self.word(t)).collect();
                    match words.join(" ").as_str() {
                        "INTEGER" => Ty::I16,
                        "LONG" => Ty::I32,
                        "_INTEGER64" => Ty::I64,
                        "SINGLE" => Ty::F32,
                        "DOUBLE" => Ty::F64,
                        "_FLOAT" => Ty::F80,
                        "STRING" => Ty::Str,
                        other => {
                            let msg = format!("the type `{other}` is not supported yet");
                            return Err(self.error(a.span(), msg));
                        }
                    }
                }
            };
            // Measured (verification\v13*): `DIM x AS T` fails once an earlier `DIM … AS` typed the plain
            // name, whatever the type; a plain `DIM x` after that is accepted and changes nothing.
            let typed_plain = suffix.is_none() && self.plain.contains_key(&name);
            if typed_plain && as_clause.is_none() {
                continue;
            }
            if typed_plain || self.vars.contains_key(&(name.clone(), ty)) {
                let msg = format!("name already in use: `{}`", show_bytes(self.text(name_tok.span)));
                return Err(self.error(name_tok.span, msg));
            }
            self.new_var(name.clone(), ty);
            // Only `DIM x AS T` changes what the plain name means; `DIM x` declares the default (SINGLE) one.
            if suffix.is_none() && as_clause.is_some() {
                self.plain.insert(name, ty);
            }
        }
        Ok(())
    }

    fn assign(&mut self, node: Node) -> R<()> {
        let mut parts = node.child_nodes();
        let (name, value_node) = (parts.next().unwrap(), parts.next().unwrap());
        let value = self.expr(value_node)?;
        let var = self.resolve(name.first_token().unwrap())?;
        let target = self.prog.var(var).ty;
        let value = self.store(value, target)?;
        self.push(node, StmtKind::Assign { var, value });
        Ok(())
    }

    fn print(&mut self, node: Node) -> R<()> {
        let mut items = Vec::new();
        let mut newline = true;
        for e in node.children().skip(1) {
            match e {
                Element::Token(t) if t.kind == Semicolon => newline = false,
                Element::Token(t) if t.kind == Comma => {
                    items.push(PrintItem::Zone);
                    newline = false;
                }
                Element::Node(n) => {
                    let e = self.expr(n)?;
                    items.push(if e.ty == Ty::Str {
                        PrintItem::Str(e)
                    } else {
                        let qb = e.qb;
                        PrintItem::Num(self.convert_exact(e, qb))
                    });
                    newline = true;
                }
                _ => {}
            }
        }
        self.push(node, StmtKind::Print { items, newline });
        Ok(())
    }

    // ---- expressions ----

    fn expr(&mut self, node: Node) -> R<Expr> {
        let span = node.span();
        match node.kind() {
            Literal => {
                let t = node.first_token().unwrap();
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
            NameRef => {
                let id = self.resolve(node.first_token().unwrap())?;
                let ty = self.prog.var(id).ty;
                Ok(Expr {
                    span,
                    ty,
                    qb: ty,
                    kind: ExprKind::Var(id),
                })
            }
            ParenExpr => {
                let inner = node.child_nodes().next().unwrap();
                let mut e = self.expr(inner)?;
                e.span = span;
                Ok(e)
            }
            PrefixExpr => self.prefix(node),
            BinExpr => self.binary(node),
            CallExpr => self.call(node),
            _ => Err(self.error(span, "expected an expression")),
        }
    }

    fn number(&mut self, t: Tok, negative: bool) -> R<Expr> {
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
            Err(LitError::Unsupported(what)) => Err(self.error(span, format!("{what} are not supported yet"))),
        }
    }

    fn prefix(&mut self, node: Node) -> R<Expr> {
        let span = node.span();
        let op = node.first_token().unwrap();
        let operand = node.child_nodes().next().unwrap();
        match op.kind {
            Plus => {
                let mut e = self.expr(operand)?;
                e.span = span;
                Ok(e)
            }
            Minus => {
                // A minus directly before a decimal literal is part of the literal (step C).
                if operand.kind() == Literal {
                    let t = operand.first_token().unwrap();
                    if t.kind == Number && self.text(t.span)[0] != b'&' {
                        let mut e = self.number(t, true)?;
                        e.span = span;
                        return Ok(e);
                    }
                }
                let e = self.expr(operand)?;
                if !e.ty.is_numeric() {
                    return Err(self.error(span, "unary `-` needs a number"));
                }
                // Negating an integer is believed `_INTEGER64`, a float keeps its type (measured: `-x%` with
                // `x% = -32768` prints ` 32768 `).
                let qb = if e.qb.is_int() { Ty::I64 } else { e.qb };
                let ty = promote(e.ty);
                let e = self.convert_exact(e, ty);
                if let (ExprKind::Int(v), true) = (&e.kind, self.fold) {
                    let v = *v;
                    return Ok(Expr {
                        span,
                        ty,
                        qb,
                        kind: ExprKind::Int(wrap(-(v as i128), ty)),
                    });
                }
                Ok(Expr {
                    span,
                    ty,
                    qb,
                    kind: ExprKind::Neg(Box::new(e)),
                })
            }
            _ => Err(self.error(op.span, format!("operator `{}` is not supported yet", self.word(op)))),
        }
    }

    fn binary(&mut self, node: Node) -> R<Expr> {
        let span = node.span();
        let mut operands = node.child_nodes();
        let (l, r) = (operands.next().unwrap(), operands.next().unwrap());
        let op_tok = node.child_tokens().next().unwrap();
        let op = match op_tok.kind {
            Plus => BinOp::Add,
            Minus => BinOp::Sub,
            Star => BinOp::Mul,
            Slash => BinOp::Div,
            _ => {
                let shown = show_bytes(self.text(op_tok.span)).to_ascii_uppercase();
                return Err(self.error(op_tok.span, format!("operator `{shown}` is not supported yet")));
            }
        };
        let lhs = self.expr(l)?;
        let rhs = self.expr(r)?;
        match (lhs.ty == Ty::Str, rhs.ty == Ty::Str) {
            (true, true) if op == BinOp::Add => {
                return Ok(Expr {
                    span,
                    ty: Ty::Str,
                    qb: Ty::Str,
                    kind: ExprKind::Concat(Box::new(lhs), Box::new(rhs)),
                });
            }
            (false, false) => {}
            (true, true) => return Err(self.error(op_tok.span, "this operator cannot be used on strings")),
            _ => return Err(self.error(span, "cannot mix strings and numbers")),
        }
        let float_qb = [lhs.qb, rhs.qb].into_iter().filter(|t| t.is_float()).max();
        let (ty, qb) = if op == BinOp::Div && float_qb.is_none() {
            // Integer / integer: the right operand is made `_FLOAT` (`study\02` §1.4).
            (Ty::F80, Ty::F80)
        } else {
            (promote(lhs.ty).max(promote(rhs.ty)), float_qb.unwrap_or(Ty::I64))
        };
        let lhs = self.convert_exact(lhs, ty);
        let rhs = self.convert_exact(rhs, ty);
        if let (ExprKind::Int(a), ExprKind::Int(b), true) = (&lhs.kind, &rhs.kind, self.fold) {
            let (a, b) = (*a as i128, *b as i128);
            let v = match op {
                BinOp::Add => a + b,
                BinOp::Sub => a - b,
                BinOp::Mul => a * b,
                BinOp::Div => unreachable!("integer division is computed in _FLOAT"),
            };
            return Ok(Expr {
                span,
                ty,
                qb,
                kind: ExprKind::Int(wrap(v, ty)),
            });
        }
        Ok(Expr {
            span,
            ty,
            qb,
            kind: ExprKind::Binary {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            },
        })
    }

    fn call(&mut self, node: Node) -> R<Expr> {
        let span = node.span();
        let name_tok = node.first_token().unwrap();
        let name = self.text(name_tok.span).to_vec();
        let Some(id) = find_function(&name).filter(|_| name.eq_ignore_ascii_case(b"INSTR")) else {
            let shown = show_bytes(&name);
            let msg = if find_function(&name).is_some() {
                format!("`{shown}` is not supported yet")
            } else {
                format!("`{shown}(...)`: arrays and functions are not supported yet")
            };
            return Err(self.error(name_tok.span, msg));
        };
        self.instr(span, id, node)
    }

    /// `INSTR([start,] base$, search$)`: table slots LONG, STRING, STRING; the first optional.
    fn instr(&mut self, span: Span, id: BuiltinId, node: Node) -> R<Expr> {
        let args_node = node.child_nodes().find(|n| n.kind() == ArgList).unwrap();
        let mut args = Vec::new();
        for a in args_node.child_nodes() {
            args.push(self.expr(a)?);
        }
        let mut slots: Vec<Option<Expr>> = match args.len() {
            2 => vec![None],
            3 => vec![Some(args.remove(0))],
            _ => return Err(self.error(span, "`INSTR` takes 2 or 3 arguments")),
        };
        if let Some(start) = slots[0].take() {
            if !start.ty.is_numeric() {
                return Err(self.error(start.span, "the start of `INSTR` must be a number"));
            }
            slots[0] = Some(self.store(start, Ty::I32)?);
        }
        for a in args {
            if a.ty != Ty::Str {
                return Err(self.error(a.span, "`INSTR` searches strings"));
            }
            slots.push(Some(a));
        }
        Ok(Expr {
            span,
            ty: Ty::I32,
            qb: Ty::I32,
            kind: ExprKind::Call {
                builtin: id,
                args: slots,
            },
        })
    }

    /// Converts a value for storing into a variable or argument of type `to` (spec: storing into an integer).
    fn store(&mut self, e: Expr, to: Ty) -> R<Expr> {
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
}

/// Integer operands are computed in at least 32 bits (C promotion).
fn promote(t: Ty) -> Ty {
    if t == Ty::I16 { Ty::I32 } else { t }
}

fn wrap(v: i128, ty: Ty) -> i64 {
    match ty {
        Ty::I16 => v as i16 as i64,
        Ty::I32 => v as i32 as i64,
        _ => v as i64,
    }
}

fn conv(e: Expr, to: Ty, how: ConvKind) -> Expr {
    Expr {
        span: e.span,
        ty: to,
        qb: to,
        kind: ExprKind::Convert { how, from: Box::new(e) },
    }
}

impl Checker<'_> {
    /// Converts between numeric types without rounding to an integer: integer widths, integer to float, float widths.
    /// Integer constants are folded.
    fn convert_exact(&self, e: Expr, to: Ty) -> Expr {
        if e.ty == to {
            return e;
        }
        debug_assert!(e.ty.is_numeric() && to.is_numeric() && !(e.ty.is_float() && to.is_int()));
        if let (ExprKind::Int(v), true, true) = (&e.kind, to.is_int(), self.fold) {
            let v = wrap(*v as i128, to);
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
