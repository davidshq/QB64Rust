//! Procedures: headers and parameters (pass 1), calls and their arguments, `EXIT SUB`/`FUNCTION`.

use super::decl::name_end;
use super::{Checker, DimFact, Failed, R, Scope};
use crate::builtins::stmt_lookup;
use crate::literal;
use crate::{Arg, ArrayRef, Expr, ExprKind, Place, Proc, ProcId, ProcKind, Shape, StmtKind, Storage, SymbolKind, Ty};
use crate::{Var, VarId};
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
        let function = self.word(keyword) == "FUNCTION";
        // `FUNCTION f$n`: a `STRING * n` result, returned as its n bytes (design D6; measured,
        // `verification\v21_c_fixed_args`).
        let fixed = if function { self.fixed_suffix(name_tok)? } else { None };
        let (name, suffix) = match fixed {
            Some((name, n)) => (name, Some(Ty::FixedStr(n))),
            None => self.split_name(name_tok)?,
        };
        let kind = if function {
            ProcKind::Function(suffix.unwrap_or_else(|| self.default_ty(&name)))
        } else if suffix.is_some() {
            return Err(self.unsupported(name_tok.span, "a SUB name with a type suffix"));
        } else {
            ProcKind::Sub
        };
        self.reserved_proc(name_tok, &name, suffix, matches!(kind, ProcKind::Function(_)))?;
        if self.procs_by_name.contains_key(&name) {
            return Err(self.in_use(name_tok));
        }
        let id = ProcId(to_u32(self.prog.procs.len()));
        if header.static_word().is_some() {
            self.static_procs.insert(id);
        }
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
            if param.array_parens().is_some() {
                self.declare_array_param(param, name_tok, id, &mut scope)?;
                continue;
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
            // The type the plain name is found by in the body: the parameter's, except for a default `STRING * n`.
            let mut key_ty = None;
            let as_clause = param.as_clause();
            let ty = match (suffix, as_clause) {
                (Some(_), Some(a)) => {
                    let span = a.node().span();
                    return Err(self.error(span, "a name with a type suffix cannot have an `AS` clause"));
                }
                (Some(t), None) => t,
                // Measured (`verification\v24_x33`): a parameter typed `STRING * n` by `_DEFINE` is one written
                // `AS STRING * n`: a `STRING` whose `LEN` is n.
                (None, None) => {
                    let default = self.default_ty(&name);
                    if let Ty::FixedStr(n) = default {
                        (fixed, key_ty) = (Some(n), Some(default));
                        Ty::Str
                    } else {
                        default
                    }
                }
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
            // A parameter of a user type is the place passed ([`Self::type_arg`]).
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
            scope.vars.insert((name.clone(), key_ty.unwrap_or(ty)), v);
            // Only an `AS` clause types the plain name. A parameter typed by a default is found as the name plus
            // the default type where it is named: after a `DEFxxx` in the body the plain name is another variable
            // (measured, `verification\v24_a_procs_subs`).
            if as_clause.is_some() {
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

    /// An array parameter, `name()` (design D6 of `m2-defxxx-common-arrays`; measured,
    /// `verification\v24_g_params`, `v24_x135`–`x160`, `x290`, `x291`): what stands between its parentheses is not
    /// read (`a(,)`, `a(1)`). Its element type is any an array may have, a `_BIT` and a user type included, which
    /// a scalar parameter cannot be. It is an array of the procedure's scope whose number of dimensions is found
    /// after the bodies ([`Checker::solve_dims`]). A scalar parameter of the same name is another variable; a
    /// second array parameter of the name and type is "Name already in use".
    fn declare_array_param(&mut self, param: ast::Param, name_tok: Tok, id: ProcId, scope: &mut Scope) -> R<()> {
        let shown = show_bytes(self.text(name_tok.span));
        if self.fixed_suffix(name_tok)?.is_some() {
            // As `DIM a$3(2)`: not measured.
            return Err(self.unsupported(name_tok.span, format!("an array named with `$n`: `{shown}`")));
        }
        let (name, suffix) = self.split_name(name_tok)?;
        let as_clause = param.as_clause();
        let ty = match (suffix, as_clause) {
            (Some(_), Some(a)) => {
                let span = a.node().span();
                return Err(self.error(span, "a name with a type suffix cannot have an `AS` clause"));
            }
            (Some(t), None) => t,
            (None, None) => self.default_ty(&name),
            (None, Some(a)) if self.unsigned_string(a) => {
                // A scalar parameter cannot be one (`verification\v21_x42`); an array of them was not measured.
                let span = a.node().span();
                return Err(self.unsupported(span, "an `_UNSIGNED STRING` array parameter"));
            }
            (None, Some(a)) => match (self.fixed_string_size(a), a.size()) {
                (true, Some(size)) => Ty::FixedStr(self.fixed_param_len(size)?),
                _ => self.type_of(a)?,
            },
        };
        self.reserved(name_tok, &name, suffix)?;
        if self.procs_by_name.contains_key(&name) || scope.arrays.contains_key(&(name.clone(), ty)) {
            return Err(self.in_use(name_tok));
        }
        if scope.arrays.keys().any(|(n, _)| *n == name) {
            // Two array parameters of one name and two types: not measured.
            let msg = format!("a second array parameter named `{shown}`, of another type");
            return Err(self.unsupported(name_tok.span, msg));
        }
        let v = VarId(to_u32(self.prog.vars.len()));
        self.prog.vars.push(Var {
            name: name.clone(),
            ty,
            storage: Storage::Param(id),
            shape: Shape::Dynamic {
                dims: None,
                at_start: None,
            },
        });
        scope.arrays.insert((name.clone(), ty), v);
        if as_clause.is_some() {
            scope.array_plain.insert(name, ty);
        }
        self.prog.procs[id.0 as usize].params.push(v);
        self.names.push((SymbolKind::Var(v), name_tok.span));
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
        self.fixed_len_of(size, false)
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
        let (name, suffix) = match self.proc_fixed_name(name_tok) {
            Some((name, ty)) => (name, Some(ty)),
            None => self.split_name(name_tok)?,
        };
        let shown = show_bytes(self.text(name_tok.span));
        // A built-in statement whose entry has no template is written like a SUB call (`KILL f$`).
        let plain = match suffix {
            None => stmt_lookup(&name, false),
            Some(_) => Vec::new(),
        };
        let plain = match plain[..] {
            [form] if form.id.get().specialformat.is_none() => Some(form),
            _ => None,
        };
        match self.procs_by_name.get(&name).map(|&p| (p, self.prog.proc(p).kind)) {
            None if plain.is_some() => self.plain_stmt(stmt, plain.expect("checked")),
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
            if self.prog.var(param).is_array() {
                args.push(self.array_arg(node, param)?);
                continue;
            }
            if let Some((_, t)) = self.whole_array(node) {
                // Measured: "Whole array cannot be passed to a scalar SUB parameter" (`verification\v24_x147`).
                let shown = show_bytes(self.text(t.span));
                let msg = format!("the whole array `{shown}()` cannot be passed to a parameter that is no array");
                return Err(self.error(node.node().span(), msg));
            }
            if let Ty::User(_) = pty {
                args.push(self.type_arg(node, pty)?);
                continue;
            }
            let e = self.expr(node)?;
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
            // A fixed-length string variable, element or member to a `STRING` parameter is passed by reference, also
            // in parentheses (measured, `v21_c_fixed_args`): the procedure's stores reach it cut and padded (design D6).
            let fixed = |t: Ty| pty == Ty::Str && matches!(t, Ty::FixedStr(_));
            if let ExprKind::Load(place) = &e.kind
                && let ty = self.prog.place_ty(place)
                && (ty == pty || fixed(ty) || same_storage(ty, pty))
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
                    (true, Place::Element { .. } | Place::Member { .. }) if fixed(ty) => {
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

    /// The argument for a parameter of the user type `pty` (design D9 of `m2-defxxx-common-arrays`; measured,
    /// `verification\v24_h_type_params`, `v24_x211`–`x213`, `x308`): a variable, an element or a member of exactly
    /// that type, passed by reference. An element with an index outside its bounds raises error 9 and the
    /// procedure is not entered.
    ///
    /// - A place of another type is "User defined types in expressions are invalid", a number "TYPE 't1' required
    ///   for sub".
    /// - A place in parentheses, `move (q)`, is "UNEXPECTED INTERNAL COMPILER ERROR!" in the old compiler: an
    ///   error here.
    fn type_arg(&mut self, node: ast::Expr, pty: Ty) -> R<Arg> {
        let span = node.node().span();
        let required = format!("this parameter takes a variable of `TYPE {}`", self.prog.type_name(pty));
        if let ast::Expr::Paren(p) = node
            && let Some(inner) = p.inner()
            && let Some(place) = self.whole_place(inner)?
        {
            // A place of another type in parentheses is no value of the type, as without them.
            let msg = if self.prog.place_ty(&place) == pty {
                format!("{required}, which cannot stand in parentheses")
            } else {
                required
            };
            return Err(self.error(span, msg));
        }
        match self.whole_place(node)? {
            Some(place) if self.prog.place_ty(&place) == pty => Ok(Arg::Ref(place)),
            Some(_) => Err(self.error(span, required)),
            None => {
                self.expr(node)?;
                Err(self.error(span, required))
            }
        }
    }

    /// `name()` naming an array that is visible here: the array and the name's token.
    fn whole_array(&mut self, node: ast::Expr) -> Option<(VarId, Tok)> {
        let ast::Expr::Call(call) = node else {
            return None;
        };
        if !call.arg_list().is_some_and(|l| l.args().is_empty()) {
            return None;
        }
        let t = call.name()?;
        let (name, suffix) = self.split_name(t).ok()?;
        self.find_array(&name, suffix).map(|a| (a, t))
    }

    /// The argument for the array parameter `param` (design D6 of `m2-defxxx-common-arrays`; measured,
    /// `verification\v24_g_params`, `v24_x135`–`x160`, `x178`–`x180`, `x286`–`x296`): a whole array written
    /// `name()`, static or dynamic, allocated or not, the procedure's own array parameter included.
    ///
    /// - Its element type has the parameter's kind and width ([`array_kind`]); else "Incorrect array type passed
    ///   to sub".
    /// - Anything that is no `name()` is "Expected arrayname()". In parentheses, `(x())`, the old compiler
    ///   accepts it and its C++ build fails (`x145`): an error here (`DECISIONS.md` 2026-10-08).
    /// - A name that names no array becomes one in the main module, as a use without `DIM` with one index does
    ///   (`x146`, `x286`, `x287`, `x293`, `x295`), and is "Expected (...)" in a procedure (`x292`).
    /// - Its number of dimensions is one fact about the parameter's ([`Checker::solve_dims`]); an array that has
    ///   no number yet (another parameter, an array listed by `COMMON a()` or `STATIC a()`, `x288`, `x289`) gets
    ///   the parameter's.
    ///
    /// A whole member array is passed as a static array with its bounds (`t.v()`, `a(1).v()`; measured,
    /// `verification\v24_x200`, `x315`).
    fn array_arg(&mut self, node: ast::Expr, param: VarId) -> R<Arg> {
        let span = node.node().span();
        if let Some((array, parens, t)) = self.member_array(node)? {
            let shown = show_bytes(self.text(t.span));
            if !parens {
                // Measured: "Expected: Array Name()" (`verification\v24_x319`).
                let msg = format!("this parameter takes a whole array, written `{shown}()`");
                return Err(self.error(span, msg));
            }
            let (aty, pty) = (self.prog.array_ty(&array), self.prog.var(param).ty);
            if array_kind(aty) != array_kind(pty) {
                let msg = format!(
                    "the array `{shown}` ({}) cannot be passed to an array parameter of type {}",
                    self.prog.type_name(aty),
                    self.prog.type_name(pty)
                );
                return Err(self.error(span, msg));
            }
            let count = self.prog.array_dims(&array).expect("a member array has its bounds");
            self.dim_facts.push(DimFact::Count {
                array: param,
                shown: self.prog.var(param).name.to_ascii_lowercase(),
                count,
                span,
                passed: true,
            });
            return Ok(Arg::Array(array));
        }
        let call = match node {
            ast::Expr::Call(call) if call.arg_list().is_some_and(|l| l.args().is_empty()) => call,
            ast::Expr::Paren(p) if p.inner().is_some_and(|inner| self.whole_array(inner).is_some()) => {
                return Err(self.error(span, "an array argument cannot stand in parentheses"));
            }
            ast::Expr::Literal(_)
            | ast::Expr::NameRef(_)
            | ast::Expr::Call(_)
            | ast::Expr::Field(_)
            | ast::Expr::Paren(_)
            | ast::Expr::Prefix(_)
            | ast::Expr::Bin(_) => {
                return Err(self.error(span, "this parameter takes a whole array, written `name()`"));
            }
        };
        let t = self.need(call.name(), span)?;
        let (name, suffix) = self.split_name(t)?;
        let shown = show_bytes(self.text(t.span));
        let array = match self.find_array(&name, suffix) {
            Some(a) => a,
            None if self.cur.is_some() => {
                let msg = format!("`{shown}()` names no array here (an array passed in a SUB or FUNCTION must exist)");
                return Err(self.error(span, msg));
            }
            None => self.implicit_array(t, &name, suffix, 1, call.node())?,
        };
        let (aty, pty) = (self.prog.var(array).ty, self.prog.var(param).ty);
        if array_kind(aty) != array_kind(pty) {
            let msg = format!(
                "the array `{shown}` ({}) cannot be passed to an array parameter of type {}",
                self.prog.type_name(aty),
                self.prog.type_name(pty)
            );
            return Err(self.error(span, msg));
        }
        let fact = match self.prog.var(array).dim_count() {
            Some(count) => DimFact::Count {
                array: param,
                shown: self.prog.var(param).name.to_ascii_lowercase(),
                count,
                span,
                passed: true,
            },
            None => {
                if !matches!(self.prog.var(array).storage, Storage::Param(_)) {
                    self.dims_open.insert(array);
                }
                DimFact::Same {
                    arg: array,
                    param,
                    shown,
                    span,
                }
            }
        };
        self.dim_facts.push(fact);
        self.names.push((SymbolKind::Var(array), t.span));
        Ok(Arg::Array(ArrayRef::Var(array)))
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
        if suffix.is_some_and(|s| !suffix_fits(s, ty)) || matches!(ty, Ty::Bit { .. }) {
            return Err(self.in_use(t));
        }
        let nodes = self.present_args(args)?;
        let args = self.args(p, t, &nodes, span)?;
        self.names.push((SymbolKind::Proc(p), t.span));
        Ok(Expr {
            span,
            ty: ty.held_value(),
            qb: ty.believed_value(),
            kind: ExprKind::CallProc { proc: p, args },
        })
    }
}

/// Whether a suffix on a FUNCTION's name fits its type: its own suffix, or `$` for a FUNCTION `f$n` (measured:
/// `fs$("a")` calls `fs$5`, `fs$4("a")` is "Name already in use", `verification\v21_g_fixed_function`).
pub(super) fn suffix_fits(suffix: Ty, function: Ty) -> bool {
    suffix == function || (suffix == Ty::Str && matches!(function, Ty::FixedStr(_)))
}

/// Whether a place of integer type `place` is passed by reference to a parameter of integer type `param` of
/// another type: the same width (the 64-bit integer and `_OFFSET` types are one width), as the old compiler passes
/// the same bytes with a pointer cast (measured, `verification\v21_b_passing`, `v21_g_passing_places`: variables,
/// elements and members). A `_BIT` place is always a copy: its value is not of its own width.
fn same_storage(place: Ty, param: Ty) -> bool {
    let bit = |t: Ty| matches!(t, Ty::Bit { .. });
    place.is_int() && param.is_int() && !bit(place) && !bit(param) && place.int_bits() == param.int_bits()
}

/// What the old compiler compares between the element type of an array argument and of its parameter
/// (`qb64pe.bas` 12029; measured, `verification\v24_x135`–`x142`, `x160`, `x178`, `x179`): the size in bits or the
/// user type, whether it is a `_BIT` type, a string, a fixed-length string (and its length), a float. Not
/// `_UNSIGNED`, and not `_OFFSET` beside `_INTEGER64`.
#[derive(PartialEq, Eq)]
enum ArrayKind {
    Int(u32),
    Bit(u8),
    Float(u32),
    Str,
    FixedStr(u32),
    User(crate::TypeId),
}

fn array_kind(t: Ty) -> ArrayKind {
    match t {
        Ty::Bit { width, .. } => ArrayKind::Bit(width),
        Ty::F32 => ArrayKind::Float(32),
        Ty::F64 => ArrayKind::Float(64),
        Ty::F80 => ArrayKind::Float(256),
        Ty::Str => ArrayKind::Str,
        Ty::FixedStr(n) => ArrayKind::FixedStr(n),
        Ty::User(id) => ArrayKind::User(id),
        Ty::I8 | Ty::U8 => ArrayKind::Int(8),
        Ty::I16 | Ty::U16 => ArrayKind::Int(16),
        Ty::I32 | Ty::U32 => ArrayKind::Int(32),
        Ty::I64 | Ty::U64 | Ty::Off | Ty::UOff => ArrayKind::Int(64),
    }
}

/// The error for a fixed-length string's length that is 0 or negative once read in 32 bits.
pub(super) const FIXED_LEN_ERROR: &str =
    "a fixed-length string's length must be 1 to 2147483647 (it is read in 32 bits)";

/// The length n of `STRING * n` or `name$n` as the old compiler reads it: its low 32 bits as a signed integer
/// (measured, `verification\v21_c_fixed_len_*`: 4294967297 is 1); `None` when that is 0 or negative (0 is "Cannot
/// create a fixed string of length 0", a negative one fails the old compiler's C++ build).
pub(super) fn fixed_len(v: i128) -> Option<u32> {
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
pub(super) fn fixed_name(raw: &[u8]) -> Option<(String, Option<u32>)> {
    let end = name_end(raw);
    let digits = raw[end..].strip_prefix(b"$").filter(|d| !d.is_empty())?;
    let name = show_bytes(&raw[..end].to_ascii_uppercase());
    let value = literal::decimal(digits).and_then(|v| i128::try_from(v).ok());
    Some((name, value.and_then(fixed_len)))
}
