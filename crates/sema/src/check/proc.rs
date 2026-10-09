//! Procedures: headers and parameters (pass 1), calls and their arguments, `EXIT SUB`/`FUNCTION`.

use super::decl::name_end;
use super::{Checker, Failed, R, Scope};
use crate::literal::{self, NumLit};
use crate::{Arg, Expr, ExprKind, Place, Proc, ProcId, ProcKind, StmtKind, Storage, SymbolKind, Ty, VarId};
use qb64rust_base::{Span, show_bytes, to_u32};
use qb64rust_builtins::{Kind, find_any};
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::ast;
use qb64rust_syntax::is_keyword;
use qb64rust_syntax::tree::{Node, Tok};

impl Checker<'_> {
    // ---- procedures: pass 1 ----

    /// Enters a procedure's name and kind into the table.
    pub(super) fn declare_proc(&mut self, header: ast::ProcHeader) -> R<ProcId> {
        let span = header.node().span();
        let keyword = self.need(header.keyword(), span)?;
        let name_tok = self.need(header.name(), span)?;
        let (name, suffix) = self.split_name(name_tok)?;
        let kind = if self.word(keyword) == "FUNCTION" {
            ProcKind::Function(suffix.unwrap_or(Ty::F32))
        } else if suffix.is_some() {
            return Err(self.unsupported(name_tok.span, "a SUB name with a type suffix"));
        } else {
            ProcKind::Sub
        };
        self.reserved_proc(name_tok, &name, suffix, matches!(kind, ProcKind::Function(_)))?;
        if self.procs_by_name.contains_key(&name) {
            return Err(self.in_use(name_tok));
        }
        if let Some(t) = header.static_word() {
            return Err(self.unsupported(t.span, "`STATIC` after a procedure header"));
        }
        let id = ProcId(to_u32(self.prog.procs.len()));
        self.prog.procs.push(Proc {
            name: name.clone(),
            kind,
            params: Vec::new(),
            result: None,
            stmts: Vec::new(),
            line: 0,
            end_line: 0,
        });
        self.param_scopes.push(Scope::default());
        self.procs_by_name.insert(name, id);
        self.names.push((SymbolKind::Proc(id), name_tok.span));
        Ok(id)
    }

    /// Enters the name of a procedure whose header has an error (already reported) as a broken procedure: calls
    /// of it then fail without a second error ("not a SUB", a wrong argument count). Nothing is entered when the
    /// header has no name or the name is already a procedure (a duplicate: calls go to the first one).
    pub(super) fn declare_broken(&mut self, header: ast::ProcHeader) {
        let (Some(keyword), Some(name_tok)) = (header.keyword(), header.name()) else {
            return;
        };
        let raw = self.text(name_tok.span);
        let name = show_bytes(&raw[..name_end(raw)].to_ascii_uppercase());
        if self.procs_by_name.contains_key(&name) {
            return;
        }
        // The kind matters only to keep a FUNCTION name from being used as a variable; the type is not known.
        let kind = if self.word(keyword) == "FUNCTION" {
            ProcKind::Function(Ty::F32)
        } else {
            ProcKind::Sub
        };
        let id = ProcId(to_u32(self.prog.procs.len()));
        self.prog.procs.push(Proc {
            name: name.clone(),
            kind,
            params: Vec::new(),
            result: None,
            stmts: Vec::new(),
            line: 0,
            end_line: 0,
        });
        self.param_scopes.push(Scope::default());
        self.procs_by_name.insert(name, id);
        self.broken.insert(id);
    }

    /// Fails a use of a broken procedure without a diagnostic: its header's error is already reported, and its
    /// parameters are unknown.
    fn not_broken(&mut self, p: ProcId) -> R<()> {
        if self.broken.contains(&p) {
            self.stmt_error = true;
            return Err(Failed);
        }
        Ok(())
    }

    /// Creates the parameter variables and the FUNCTION's result variable.
    pub(super) fn declare_params(&mut self, header: ast::ProcHeader, id: ProcId) -> R<()> {
        let mut scope = Scope::default();
        for param in header.params() {
            let name_tok = self.need(param.name(), param.node().span())?;
            if let Some(t) = param.array_parens() {
                return Err(self.unsupported(t.span, "array parameters"));
            }
            if let Some((name, n)) = self.fixed_suffix(name_tok)? {
                // `t$n`: a `STRING` parameter of a name of its own (`t$` is another variable), `LEN` n.
                if let Some(a) = param.as_clause() {
                    let span = a.node().span();
                    return Err(self.error(span, "a name with a type suffix cannot have an `AS` clause"));
                }
                self.reserved(name_tok, &name, Some(Ty::Str))?;
                let key = (id, name.clone(), n);
                if self.procs_by_name.contains_key(&name) || self.fixed_params.contains_key(&key) {
                    return Err(self.in_use(name_tok));
                }
                let v = self.new_var(name, Ty::Str, Storage::Param(id));
                self.fixed_params.insert(key, v);
                self.param_len.insert(v, n);
                self.prog.procs[id.0 as usize].params.push(v);
                self.names.push((SymbolKind::Var(v), name_tok.span));
                continue;
            }
            let (name, suffix) = self.split_name(name_tok)?;
            let mut fixed = None;
            let ty = match (suffix, param.as_clause()) {
                (Some(_), Some(a)) => {
                    let span = a.node().span();
                    return Err(self.error(span, "a name with a type suffix cannot have an `AS` clause"));
                }
                (Some(t), None) => t,
                (None, None) => Ty::F32,
                (None, Some(a)) if self.unsigned_string(a) => {
                    // Measured: "Illegal SUB/FUNCTION parameter" (`verification\v21_x42`, `x43`), though `DIM`
                    // takes `_UNSIGNED STRING`.
                    let span = a.node().span();
                    return Err(self.error(span, "a parameter cannot be `_UNSIGNED STRING`"));
                }
                (None, Some(a)) => match (self.fixed_string_size(a), a.size()) {
                    (true, Some(size)) => {
                        fixed = Some(self.fixed_param_len(size)?);
                        Ty::Str
                    }
                    _ => self.type_of(a)?,
                },
            };
            if let Ty::User(_) = ty {
                // Measured: accepted (by reference, `v18_h_whole_arg`); whole `TYPE` values come later.
                return Err(self.unsupported(param.node().span(), "a `TYPE` parameter"));
            }
            if let Ty::Bit { .. } = ty {
                // Decided (`DECISIONS.md` 2026-10-08, design D7): the old compiler fails its C++ build for any
                // procedure with a `_BIT` parameter, called or not (`verification\v21_x20`–`x24`).
                let shown = show_bytes(self.text(name_tok.span));
                return Err(self.error(
                    param.node().span(),
                    format!("a parameter cannot be a `_BIT`: `{shown}`"),
                ));
            }
            self.reserved(name_tok, &name, suffix)?;
            if self.procs_by_name.contains_key(&name) || scope.vars.contains_key(&(name.clone(), ty)) {
                return Err(self.in_use(name_tok));
            }
            let v = self.new_var(name.clone(), ty, Storage::Param(id));
            scope.vars.insert((name.clone(), ty), v);
            if suffix.is_none() {
                scope.plain.insert(name, ty);
            }
            if let Some(n) = fixed {
                self.param_len.insert(v, n);
            }
            self.prog.procs[id.0 as usize].params.push(v);
            self.names.push((SymbolKind::Var(v), name_tok.span));
        }
        if let ProcKind::Function(ty) = self.prog.proc(id).kind {
            let name = self.prog.proc(id).name.clone();
            let v = self.new_var(name, ty, Storage::Result(id));
            self.prog.procs[id.0 as usize].result = Some(v);
        }
        self.param_scopes[id.0 as usize] = scope;
        Ok(())
    }

    /// A name `t$n` (a fixed-length string's suffix): its name in upper case and n; `None` for any other name. n
    /// is read as the old compiler reads it ([`fixed_len`]).
    fn fixed_suffix(&mut self, t: Tok) -> R<Option<(String, u32)>> {
        match fixed_name(self.text(t.span)) {
            None => Ok(None),
            Some((name, Some(n))) => Ok(Some((name, n))),
            Some((_, None)) => Err(self.error(t.span, FIXED_LEN_ERROR)),
        }
    }

    /// Whether an `AS` clause is `STRING * n`.
    fn fixed_string_size(&self, a: ast::AsClause) -> bool {
        let words: Vec<String> = a.type_words().map(|t| self.word(t)).collect();
        a.size().is_some() && words == ["STRING"]
    }

    /// Whether an `AS` clause is `_UNSIGNED STRING`, with a length or not.
    fn unsigned_string(&self, a: ast::AsClause) -> bool {
        let words: Vec<String> = a.type_words().map(|t| self.word(t)).collect();
        words == ["_UNSIGNED", "STRING"]
    }

    /// The n of a parameter `AS STRING * n` (design D6): a number, read as [`fixed_len`] says. A constant's name
    /// there is not supported yet (parameters are read before any `CONST`).
    fn fixed_param_len(&mut self, size: Tok) -> R<u32> {
        let shown = show_bytes(self.text(size.span));
        if size.kind == SyntaxKind::Ident {
            let msg = format!("a constant as the length of a `STRING * n` parameter: `{shown}`");
            return Err(self.unsupported(size.span, msg));
        }
        match literal::number(self.text(size.span), false) {
            Ok(NumLit::Int { value, .. }) => match fixed_len(i128::from(value)) {
                Some(n) => Ok(n),
                None => Err(self.error(size.span, FIXED_LEN_ERROR)),
            },
            // Measured: "Number/Constant expected after *" (`verification\v21_x33_fixed_len_float`).
            Ok(NumLit::Float { .. }) => {
                Err(self.error(size.span, "a fixed-length string's length must be a whole number"))
            }
            Err(_) => Err(self.error(size.span, FIXED_LEN_ERROR)),
        }
    }

    /// In a procedure, the `t$n` parameter a name token names, if any.
    pub(super) fn fixed_param_ref(&mut self, t: Tok) -> Option<VarId> {
        let p = self.cur?;
        let (name, n) = fixed_name(self.text(t.span))?;
        let v = *self.fixed_params.get(&(p, name, n?))?;
        self.names.push((SymbolKind::Var(v), t.span));
        Some(v)
    }

    /// `CALL s(...)` or `s ...`: a SUB call, or a built-in statement (not supported yet).
    pub(super) fn call_stmt(&mut self, stmt: ast::CallStmt) -> R<()> {
        let node = stmt.node();
        let name_tok = self.need(stmt.name(), node.span())?;
        let (name, suffix) = self.split_name(name_tok)?;
        let shown = show_bytes(self.text(name_tok.span));
        match self.procs_by_name.get(&name).map(|&p| (p, self.prog.proc(p).kind)) {
            Some((p, _)) if self.broken.contains(&p) => self.not_broken(p),
            Some((p, ProcKind::Sub)) if suffix.is_none() => {
                if let Some(bad) = stmt.unparsed_args() {
                    let span = bad.span();
                    let msg = format!("cannot read the arguments of `{shown}`; expected expressions separated by `,`");
                    return Err(self.error(span, msg));
                }
                let nodes = self.present_args(stmt.arg_list())?;
                let args = self.args(p, name_tok, &nodes, node.span())?;
                self.names.push((SymbolKind::Proc(p), name_tok.span));
                self.push(node, StmtKind::Call { proc: p, args });
                Ok(())
            }
            Some((_, ProcKind::Function(_))) => {
                let msg = format!("calling the FUNCTION `{shown}` as a statement");
                Err(self.unsupported(name_tok.span, msg))
            }
            // A built-in function that is no statement too (measured: `LEN("a")` alone is "Syntax error",
            // `verification\v20_x06_function_as_statement`).
            None if super::builtins::supported_builtin(&name, suffix).is_some()
                && !find_any(name.as_bytes()).any(|b| b.kind == Kind::Sub) =>
            {
                Err(self.error(name_tok.span, format!("`{shown}` is a function, not a statement")))
            }
            _ if is_keyword(name.as_bytes()) || find_any(name.as_bytes()).next().is_some() => {
                Err(self.unsupported(name_tok.span, format!("`{shown}`")))
            }
            _ => {
                let msg = format!("`{shown}` as a statement (no SUB of this name)");
                Err(self.unsupported(name_tok.span, msg))
            }
        }
    }

    pub(super) fn exit(&mut self, node: Node) -> R<()> {
        let word = ast::ExitStmt::cast(node)
            .and_then(|e| e.keyword())
            .map(|t| self.word(t));
        if !matches!(word.as_deref(), Some("SUB" | "FUNCTION")) {
            // `EXIT FOR`, `EXIT DO`... (the parser checked that the block is open).
            return self.exit_loop(node, word.as_deref().unwrap_or(""));
        }
        if self.cur.is_none() {
            let words = node.child_tokens().map(|t| self.word(t)).collect::<Vec<_>>().join(" ");
            let msg = format!("`{words}` must be inside a SUB or FUNCTION");
            return Err(self.error(node.span(), msg));
        }
        self.push(node, StmtKind::Exit);
        Ok(())
    }

    /// The arguments of a call of procedure `p`, passed as design D4 says: a plain variable of exactly the
    /// parameter's type by reference, anything else as a converted copy.
    pub(super) fn args(&mut self, p: ProcId, name_tok: Tok, nodes: &[ast::Expr], span: Span) -> R<Vec<Arg>> {
        let params = self.prog.proc(p).params.clone();
        if nodes.len() != params.len() {
            let n = params.len();
            let msg = format!(
                "`{}` takes {n} argument{}, not {}",
                show_bytes(self.text(name_tok.span)),
                if n == 1 { "" } else { "s" },
                nodes.len()
            );
            return Err(self.error(span, msg));
        }
        let mut args = Vec::with_capacity(nodes.len());
        for (&node, param) in nodes.iter().zip(params) {
            let pty = self.prog.var(param).ty;
            self.whole_type_arg = true;
            let e = self.expr(node);
            self.whole_type_arg = false;
            let e = e?;
            match (e.ty == Ty::Str, pty == Ty::Str) {
                (true, false) => return Err(self.error(e.span, "a number is required for this parameter")),
                (false, true) => return Err(self.error(e.span, "a string is required for this parameter")),
                _ => {}
            }
            // A place of exactly the parameter's type is passed by reference: a variable, an element, a member
            // (measured, `v18_c_byref`). In parentheses, `(n)`, it is a copy; a string variable is passed by reference
            // even in parentheses (measured, `s08_byref`: `addbang (s$)` changes `s$`), a string element or member
            // in parentheses was not measured.
            let parenthesized = matches!(node, ast::Expr::Paren(_));
            if let ExprKind::Load(place) = &e.kind
                && e.ty == pty
            {
                match (parenthesized, place) {
                    (false, _) => {
                        args.push(Arg::Ref(place.clone()));
                        continue;
                    }
                    (true, Place::Var(_)) if pty == Ty::Str => {
                        args.push(Arg::Ref(place.clone()));
                        continue;
                    }
                    (true, Place::Element { .. } | Place::Member { .. }) if pty == Ty::Str => {
                        let msg = "a string element or member in parentheses as an argument";
                        return Err(self.unsupported(e.span, msg));
                    }
                    (true, _) => {}
                }
            }
            args.push(Arg::Temp(self.store(e, pty)?));
        }
        Ok(args)
    }

    /// A call of FUNCTION `p` (`args` is `None` for the bare name). A SUB, or the name with another suffix than
    /// the function's, is an error.
    pub(super) fn call_function(
        &mut self,
        p: ProcId,
        t: Tok,
        suffix: Option<Ty>,
        args: Option<ast::ArgList>,
        span: Span,
    ) -> R<Expr> {
        self.not_broken(p)?;
        let ProcKind::Function(ty) = self.prog.proc(p).kind else {
            return Err(self.in_use(t));
        };
        // Measured: a `_BIT` FUNCTION is declared, but any call of it is "Name already in use" (`verification\v21_x16`,
        // `x17`, `x47`; uncalled it compiles, `x48`).
        if suffix.is_some_and(|s| s != ty) || matches!(ty, Ty::Bit { .. }) {
            return Err(self.in_use(t));
        }
        self.gate(ty, span)?;
        let nodes = self.present_args(args)?;
        let args = self.args(p, t, &nodes, span)?;
        self.names.push((SymbolKind::Proc(p), t.span));
        Ok(Expr {
            span,
            ty,
            qb: ty,
            kind: ExprKind::CallProc { proc: p, args },
        })
    }
}

/// The error for a fixed-length string's length that is 0 or negative once read in 32 bits.
const FIXED_LEN_ERROR: &str = "a fixed-length string's length must be 1 to 2147483647 (it is read in 32 bits)";

/// The length n of `STRING * n` or `name$n` as the old compiler reads it: its low 32 bits as a signed integer
/// (measured, `verification\v21_c_fixed_len_*`: 4294967297 is 1); `None` when that is 0 or negative (0 is "Cannot
/// create a fixed string of length 0", a negative one fails the old compiler's C++ build).
fn fixed_len(v: i128) -> Option<u32> {
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss,
        reason = "the old compiler keeps the low 32 bits, read as signed: the truncation is the point"
    )]
    let n = v as u32 as i32;
    u32::try_from(n).ok().filter(|&n| n > 0)
}

/// A name `t$n`: its name in upper case and n (`None` when n is 0 or negative in 32 bits, or too long to read);
/// `None` for any other name.
fn fixed_name(raw: &[u8]) -> Option<(String, Option<u32>)> {
    let end = name_end(raw);
    let digits = raw[end..].strip_prefix(b"$").filter(|d| !d.is_empty())?;
    let name = show_bytes(&raw[..end].to_ascii_uppercase());
    let value = literal::decimal(digits).and_then(|v| i128::try_from(v).ok());
    Some((name, value.and_then(fixed_len)))
}
