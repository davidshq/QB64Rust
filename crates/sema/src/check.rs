//! From the syntax tree to the typed [`Program`].
//!
//! Two passes (design D2 of `m2-procedures-and-errors`): pass 1 collects the procedure headers, so a call may come
//! before the definition; pass 2 checks every statement **in file order**, main-module statements and procedure
//! bodies as they come. File order is what the old compiler's scopes follow (measured, `verification\v14_*`): a
//! `DIM SHARED` after a procedure is not seen by it, and a `SHARED x AS T` in an earlier procedure acts like a
//! main-module `DIM x AS T` for the main-module code after it.

use crate::literal::{self, LitError, NumLit};
use crate::{
    Arg, BinOp, ConvKind, Expr, ExprKind, PrintItem, Proc, ProcId, ProcKind, Program, Stmt, StmtKind, Storage,
    SymbolKind, Ty, Var, VarId,
};
use qb64rust_base::{Diagnostics, SourceFile, Span, show_bytes};
use qb64rust_builtins::{BuiltinId, find_any, find_function};
use qb64rust_syntax::SyntaxKind::{self, Minus, Number, Plus, Slash, Star};
use qb64rust_syntax::ast::{self, PrintPart};
use qb64rust_syntax::is_keyword;
use qb64rust_syntax::tree::{Node, Tok};
use std::collections::{HashMap, HashSet};

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
        main: Scope::default(),
        local: Scope::default(),
        cur: None,
        procs_by_name: HashMap::new(),
        proc_of_def: HashMap::new(),
        param_scopes: Vec::new(),
        broken: HashSet::new(),
        dim_shared: HashSet::new(),
        dim_shared_plain: HashMap::new(),
        diags: Diagnostics::new(),
        stmt_error: false,
        console_only: false,
        names: Vec::new(),
    };
    let skips = Skips {
        error_starts: parse_diags.list().iter().map(|d| d.span.start).collect(),
        // Past the error cap the parser records no more errors, so statements from the cap on may be malformed.
        cap_start: parse_diags
            .is_capped()
            .then(|| parse_diags.list().last().map_or(0, |d| d.span.start)),
    };
    let statements: Vec<Node> = ast::SourceFile::cast(root)
        .into_iter()
        .flat_map(|f| f.statements())
        .collect();

    // Pass 1: procedure names, then their parameters (a parameter may not have the name of any procedure).
    // A header with an error (a parse error, a reserved name...) still enters its name as a broken procedure, so
    // calls of it fail without a second error.
    let defs: Vec<ast::ProcDef> = statements
        .iter()
        .filter_map(|&s| ast::ProcDef::cast(s))
        .filter(|d| d.header().is_some() && !skips.past_cap(d.node()))
        .collect();
    let mut declared = Vec::new();
    for &def in &defs {
        let header = def.header().unwrap();
        let id = if skips.usable(header.node()) {
            c.stmt_error = false;
            let id = c.declare_proc(header);
            c.flush_names();
            id
        } else {
            Err(Failed)
        };
        match id {
            Ok(id) => declared.push((def, id)),
            Err(Failed) => c.declare_broken(header),
        }
    }
    for (def, id) in declared {
        c.stmt_error = false;
        let ok = c.declare_params(def.header().unwrap(), id).is_ok();
        c.flush_names();
        if ok {
            c.proc_of_def.insert(def.node().offset, id);
        } else {
            c.broken.insert(id);
        }
    }

    // Pass 2: everything in file order.
    for stmt in statements {
        if skips.past_cap(stmt) {
            break;
        }
        match ast::ProcDef::cast(stmt) {
            Some(def) => {
                let Some(&id) = c.proc_of_def.get(&def.node().offset) else {
                    continue;
                };
                c.cur = Some(id);
                c.local = c.param_scopes[id.0 as usize].clone();
                for s in def.body() {
                    if skips.past_cap(s) {
                        break;
                    }
                    c.check_statement(s, &skips);
                }
                c.cur = None;
            }
            None => c.check_statement(stmt, &skips),
        }
    }
    if !c.console_only {
        let at = Span::new(root.file, 0, 0);
        c.diags
            .error(at, "programs without `$CONSOLE:ONLY` are not supported yet");
    }
    (c.prog, c.diags)
}

/// Which statements the checker skips: those with a parse error, and those past the parser's error cap.
struct Skips {
    error_starts: Vec<u32>,
    cap_start: Option<u32>,
}

impl Skips {
    fn past_cap(&self, node: Node) -> bool {
        self.cap_start.is_some_and(|c| node.span().end >= c)
    }

    fn usable(&self, node: Node) -> bool {
        let s = node.span();
        node.kind() != SyntaxKind::Error
            && !self.past_cap(node)
            && !self.error_starts.iter().any(|&o| o >= s.start && o <= s.end)
    }
}

/// The variables a name can mean in one scope: a name plus a type (design D5 of the last change).
#[derive(Clone, Default)]
struct Scope {
    /// (name, type) -> variable.
    vars: HashMap<(String, Ty), VarId>,
    /// Type of the plain (suffix-less) name after a `DIM name AS type` (or a parameter or `SHARED` with `AS`).
    plain: HashMap<String, Ty>,
}

struct Checker<'a> {
    file: &'a SourceFile,
    prog: Program,
    main: Scope,
    /// The scope of the procedure being checked (only meaningful while `cur` is set).
    local: Scope,
    /// The procedure being checked; `None` in the main module.
    cur: Option<ProcId>,
    procs_by_name: HashMap<String, ProcId>,
    /// Offset of a `ProcDef` node whose header was declared without errors -> its procedure.
    proc_of_def: HashMap<u32, ProcId>,
    /// Per procedure: the scope holding its parameters, which each check of the body starts from.
    param_scopes: Vec<Scope>,
    /// Procedures whose parameter list has an error: calls of them fail without a second error.
    broken: HashSet<ProcId>,
    /// Main-module variables declared with `DIM SHARED` so far (in file order), and the plain names they typed.
    dim_shared: HashSet<VarId>,
    dim_shared_plain: HashMap<String, Ty>,
    diags: Diagnostics,
    stmt_error: bool,
    console_only: bool,
    fold: bool,
    /// Names resolved in the current statement, for the symbol table.
    names: Vec<(SymbolKind, Span)>,
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

    fn in_use(&mut self, t: Tok) -> Failed {
        let msg = format!("name already in use: `{}`", show_bytes(self.text(t.span)));
        self.error(t.span, msg)
    }

    fn text(&self, span: Span) -> &[u8] {
        text(self.file, span)
    }

    fn word(&self, t: Tok) -> String {
        String::from_utf8_lossy(self.text(t.span)).to_ascii_uppercase()
    }

    /// Records the names of the statement just checked, unless it had an error (one error per statement).
    fn flush_names(&mut self) {
        let names = std::mem::take(&mut self.names);
        if !self.stmt_error {
            self.prog.symbols.record(names);
        }
    }

    fn check_statement(&mut self, stmt: Node, skips: &Skips) {
        if !skips.usable(stmt) {
            return;
        }
        self.stmt_error = false;
        self.statement(stmt);
        self.flush_names();
    }

    fn push(&mut self, node: Node, kind: StmtKind) {
        let span = node.span();
        let start = node.first_token().map_or(span.start, |t| t.span.start);
        let line = self.file.line_col(start).0;
        let stmt = Stmt { span, line, kind };
        match self.cur {
            Some(p) => self.prog.procs[p.0 as usize].stmts.push(stmt),
            None => self.prog.stmts.push(stmt),
        }
    }

    /// A child the parser always builds for a statement without a parse error. Missing means a parser bug; it is
    /// reported rather than skipped, so no statement is silently dropped.
    fn need<T>(&mut self, x: Option<T>, span: Span) -> R<T> {
        x.ok_or_else(|| self.error(span, "internal error: incomplete syntax tree"))
    }

    fn statement(&mut self, node: Node) {
        let _ = if let Some(s) = ast::MetaStmt::cast(node) {
            self.meta(s)
        } else if let Some(s) = ast::PrintStmt::cast(node) {
            self.print(s)
        } else if let Some(s) = ast::DimStmt::cast(node) {
            self.dim(s)
        } else if let Some(s) = ast::AssignStmt::cast(node) {
            self.assign(s)
        } else if ast::EndStmt::cast(node).is_some() {
            self.push(node, StmtKind::End);
            Ok(())
        } else if let Some(s) = ast::CallStmt::cast(node) {
            self.call_stmt(s)
        } else if ast::ExitStmt::cast(node).is_some() {
            self.exit(node)
        } else if ast::DeclareStmt::cast(node).is_some() {
            // Ignored, as by the old compiler: calls are checked against the definition (measured).
            Ok(())
        } else if let Some(s) = ast::SharedStmt::cast(node) {
            self.shared(s)
        } else if let Some(s) = ast::StaticStmt::cast(node) {
            self.static_stmt(s)
        } else {
            Err(self.error(
                node.span(),
                format!("internal error: unexpected {:?} node", node.kind()),
            ))
        };
    }

    fn meta(&mut self, stmt: ast::MetaStmt) -> R<()> {
        let node = stmt.node();
        let tok = self.need(stmt.token(), node.span())?;
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

    // ---- procedures: pass 1 ----

    /// Enters a procedure's name and kind into the table.
    fn declare_proc(&mut self, header: ast::ProcHeader) -> R<ProcId> {
        let span = header.node().span();
        let keyword = self.need(header.keyword(), span)?;
        let name_tok = self.need(header.name(), span)?;
        let (name, suffix) = self.split_name(name_tok)?;
        let kind = if self.word(keyword) == "FUNCTION" {
            ProcKind::Function(suffix.unwrap_or(Ty::F32))
        } else if suffix.is_some() {
            return Err(self.error(name_tok.span, "a SUB name with a type suffix is not supported yet"));
        } else {
            ProcKind::Sub
        };
        self.reserved(name_tok, &name, suffix)?;
        if self.procs_by_name.contains_key(&name) {
            return Err(self.in_use(name_tok));
        }
        let id = ProcId(self.prog.procs.len() as u32);
        self.prog.procs.push(Proc {
            name: name.clone(),
            kind,
            params: Vec::new(),
            result: None,
            stmts: Vec::new(),
        });
        self.param_scopes.push(Scope::default());
        self.procs_by_name.insert(name, id);
        self.names.push((SymbolKind::Proc(id), name_tok.span));
        Ok(id)
    }

    /// Enters the name of a procedure whose header has an error (already reported) as a broken procedure: calls
    /// of it then fail without a second error ("not a SUB", a wrong argument count). Nothing is entered when the
    /// header has no name or the name is already a procedure (a duplicate: calls go to the first one).
    fn declare_broken(&mut self, header: ast::ProcHeader) {
        let (Some(keyword), Some(name_tok)) = (header.keyword(), header.name()) else {
            return;
        };
        let raw = self.text(name_tok.span);
        let end = raw
            .iter()
            .position(|&b| !(b.is_ascii_alphanumeric() || b == b'_' || b == b'.'))
            .unwrap_or(raw.len());
        let name = String::from_utf8_lossy(&raw[..end]).to_ascii_uppercase();
        if self.procs_by_name.contains_key(&name) {
            return;
        }
        // The kind matters only to keep a FUNCTION name from being used as a variable; the type is not known.
        let kind = if self.word(keyword) == "FUNCTION" {
            ProcKind::Function(Ty::F32)
        } else {
            ProcKind::Sub
        };
        let id = ProcId(self.prog.procs.len() as u32);
        self.prog.procs.push(Proc {
            name: name.clone(),
            kind,
            params: Vec::new(),
            result: None,
            stmts: Vec::new(),
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
    fn declare_params(&mut self, header: ast::ProcHeader, id: ProcId) -> R<()> {
        let mut scope = Scope::default();
        for param in header.params() {
            let name_tok = self.need(param.name(), param.node().span())?;
            let (name, suffix) = self.split_name(name_tok)?;
            let ty = match (suffix, param.as_clause()) {
                (Some(_), Some(a)) => {
                    let span = a.node().span();
                    return Err(self.error(span, "a name with a type suffix cannot have an `AS` clause"));
                }
                (Some(t), None) => t,
                (None, None) => Ty::F32,
                (None, Some(a)) => self.type_of(a)?,
            };
            self.reserved(name_tok, &name, suffix)?;
            if self.procs_by_name.contains_key(&name) || scope.vars.contains_key(&(name.clone(), ty)) {
                return Err(self.in_use(name_tok));
            }
            let v = self.new_var(name.clone(), ty, Storage::Param(id));
            scope.vars.insert((name.clone(), ty), v);
            if suffix.is_none() {
                scope.plain.insert(name, ty);
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

    // ---- names ----

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

    /// Reserved names (design D3), measured for every keyword and built-in (`verification\v15_builtin_names.txt`,
    /// checked against this function by `crates\driver\tests\names.rs`):
    /// - a name starting with `_` is never a variable, parameter or procedure name (the old compiler: "Invalid
    ///   variable name", "Invalid name"), built-in or not;
    /// - a keyword, or a built-in written without a required suffix, is taken whatever suffix the name carries
    ///   (`len&`, `cls`); a built-in with its required suffix (`left$`) is taken, the bare name (`left`) is free;
    /// - `WIDTH` is the one built-in without a required suffix that is still free (`width = 5` makes a variable).
    fn reserved(&mut self, t: Tok, name: &str, suffix: Option<Ty>) -> R<()> {
        if name.starts_with('_') {
            let msg = format!(
                "names starting with `_` are reserved: `{}`",
                show_bytes(self.text(t.span))
            );
            return Err(self.error(t.span, msg));
        }
        let taken = is_keyword(name.as_bytes())
            || (name != "WIDTH"
                && find_any(name.as_bytes()).any(|b| match b.musthave {
                    None => true,
                    Some("$") => suffix == Some(Ty::Str),
                    Some(_) => false,
                }));
        if taken { Err(self.in_use(t)) } else { Ok(()) }
    }

    /// The type named by an `AS` clause.
    fn type_of(&mut self, a: ast::AsClause) -> R<Ty> {
        let words: Vec<String> = a.type_words().map(|t| self.word(t)).collect();
        Ok(match words.join(" ").as_str() {
            "INTEGER" => Ty::I16,
            "LONG" => Ty::I32,
            "_INTEGER64" => Ty::I64,
            "SINGLE" => Ty::F32,
            "DOUBLE" => Ty::F64,
            "_FLOAT" => Ty::F80,
            "STRING" => Ty::Str,
            other => {
                let msg = format!("the type `{other}` is not supported yet");
                return Err(self.error(a.node().span(), msg));
            }
        })
    }

    fn new_var(&mut self, name: String, ty: Ty, storage: Storage) -> VarId {
        let id = VarId(self.prog.vars.len() as u32);
        self.prog.vars.push(Var { name, ty, storage });
        id
    }

    fn scope(&mut self) -> &mut Scope {
        if self.cur.is_some() {
            &mut self.local
        } else {
            &mut self.main
        }
    }

    /// The variable a name (not a procedure's) refers to at this point of the program, created on first use. In
    /// a procedure: its own variables (parameters, `STATIC`, `SHARED`, `DIM`, implicit), then the main module's
    /// `DIM SHARED` variables declared earlier in the file, else a new local.
    fn variable(&mut self, t: Tok, name: String, suffix: Option<Ty>) -> R<VarId> {
        let ty = match suffix {
            Some(t) => t,
            None => {
                let in_proc = self.cur.is_some();
                let plain = self.scope().plain.get(&name).copied();
                let shared = in_proc.then(|| self.dim_shared_plain.get(&name).copied()).flatten();
                plain.or(shared).unwrap_or(Ty::F32)
            }
        };
        let key = (name, ty);
        let found = self.scope().vars.get(&key).copied().or_else(|| {
            let main = self.main.vars.get(&key).copied();
            main.filter(|v| self.cur.is_some() && self.dim_shared.contains(v))
        });
        let id = match found {
            Some(id) => id,
            None => {
                self.reserved(t, &key.0, suffix)?;
                let storage = match self.cur {
                    Some(p) => Storage::Local(p),
                    None => Storage::Main,
                };
                let id = self.new_var(key.0.clone(), ty, storage);
                self.scope().vars.insert(key, id);
                id
            }
        };
        self.names.push((SymbolKind::Var(id), t.span));
        Ok(id)
    }

    /// An assignment target: a variable, or inside `FUNCTION f` the name `f` (with or without its suffix), which
    /// is the function's result.
    fn target(&mut self, t: Tok) -> R<VarId> {
        let (name, suffix) = self.split_name(t)?;
        if let Some(&p) = self.procs_by_name.get(&name) {
            let proc = self.prog.proc(p);
            if let (Some(cur), ProcKind::Function(ty)) = (self.cur, proc.kind)
                && cur == p
                && suffix.is_none_or(|s| s == ty)
            {
                let v = proc.result.expect("a FUNCTION has a result variable");
                self.names.push((SymbolKind::Var(v), t.span));
                return Ok(v);
            }
            return Err(self.in_use(t));
        }
        self.variable(t, name, suffix)
    }

    // ---- declarations ----

    /// `DIM`, `DIM SHARED` (main module only) and `STATIC` (procedures only): `storage` is the class of the
    /// variables created.
    fn declare_items<'t>(
        &mut self,
        items: impl Iterator<Item = ast::DimItem<'t>>,
        storage: Storage,
        shared: bool,
    ) -> R<()> {
        for item in items {
            let name_tok = self.need(item.name(), item.node().span())?;
            let (name, suffix) = self.split_name(name_tok)?;
            let as_clause = item.as_clause();
            let ty = match (suffix, as_clause) {
                (Some(_), Some(a)) => {
                    let span = a.node().span();
                    return Err(self.error(span, "a name with a type suffix cannot have an `AS` clause"));
                }
                (Some(t), None) => t,
                (None, None) => Ty::F32,
                (None, Some(a)) => self.type_of(a)?,
            };
            if self.procs_by_name.contains_key(&name) {
                return Err(self.in_use(name_tok));
            }
            // Measured (verification\v13*): `DIM x AS T` fails once an earlier `DIM … AS` typed the plain
            // name, whatever the type; a plain `DIM x` after that is accepted and changes nothing.
            let typed_plain = self.scope().plain.get(&name).copied();
            if let (true, Some(plain_ty), None) = (suffix.is_none(), typed_plain, as_clause) {
                // The name still refers to the typed variable.
                let id = self.scope().vars[&(name.clone(), plain_ty)];
                self.names.push((SymbolKind::Var(id), name_tok.span));
                continue;
            }
            let key = (name.clone(), ty);
            if (suffix.is_none() && typed_plain.is_some()) || self.scope().vars.contains_key(&key) {
                return Err(self.in_use(name_tok));
            }
            // A local `DIM` shadows a `DIM SHARED` variable of the same name (measured, `v14_dim_local_*`); a
            // `STATIC` beside one is not measured.
            let shadows_shared = self.cur.is_some()
                && (self.main.vars.get(&key).is_some_and(|v| self.dim_shared.contains(v))
                    || (suffix.is_none() && self.dim_shared_plain.contains_key(&name)));
            if shadows_shared && matches!(storage, Storage::Static(_)) {
                let msg = format!(
                    "a `STATIC` `{}` beside the `DIM SHARED` one is not supported yet",
                    show_bytes(self.text(name_tok.span))
                );
                return Err(self.error(name_tok.span, msg));
            }
            self.reserved(name_tok, &name, suffix)?;
            let id = self.new_var(name.clone(), ty, storage);
            self.scope().vars.insert(key, id);
            self.names.push((SymbolKind::Var(id), name_tok.span));
            if suffix.is_none() && as_clause.is_some() {
                // `DIM x AS T` changes what the plain name means.
                self.scope().plain.insert(name.clone(), ty);
                if shared {
                    self.dim_shared_plain.insert(name, ty);
                }
            } else if suffix.is_none() && shadows_shared {
                // `DIM x` declares the default (SINGLE) one. In a procedure where a `DIM SHARED x AS T` typed the
                // plain name, the plain name then means the local (`v14_dim_local_plain`: `g` is the local `g!`).
                self.local.plain.insert(name, ty);
            }
            if shared {
                self.dim_shared.insert(id);
            }
        }
        Ok(())
    }

    fn dim(&mut self, stmt: ast::DimStmt) -> R<()> {
        let shared = stmt.shared().is_some();
        let storage = match (self.cur, shared) {
            (None, _) => Storage::Main,
            (Some(_), true) => {
                let span = stmt.node().span();
                return Err(self.error(span, "`DIM SHARED` inside a SUB or FUNCTION is not supported yet"));
            }
            (Some(p), false) => Storage::Local(p),
        };
        self.declare_items(stmt.items(), storage, shared)
    }

    fn static_stmt(&mut self, stmt: ast::StaticStmt) -> R<()> {
        let Some(p) = self.cur else {
            let span = stmt.node().span();
            return Err(self.error(span, "`STATIC` in the main module is not supported yet"));
        };
        self.declare_items(stmt.items(), Storage::Static(p), false)
    }

    /// `SHARED name [AS type]` in a procedure: binds the main-module variable of that name and type, created if
    /// missing. Measured: without `AS` or a suffix the type is SINGLE, whatever the main module's plain name
    /// means (`v14_shared_plain_typed`); with `AS` it types the main module's plain name too, as a `DIM` there
    /// would (`v14_shared_plain_main`).
    fn shared(&mut self, stmt: ast::SharedStmt) -> R<()> {
        if self.cur.is_none() {
            let span = stmt.node().span();
            return Err(self.error(span, "`SHARED` in the main module is not supported yet"));
        }
        for item in stmt.items() {
            let name_tok = self.need(item.name(), item.node().span())?;
            let (name, suffix) = self.split_name(name_tok)?;
            let as_clause = item.as_clause();
            let ty = match (suffix, as_clause) {
                (Some(_), Some(a)) => {
                    let span = a.node().span();
                    return Err(self.error(span, "a name with a type suffix cannot have an `AS` clause"));
                }
                (Some(t), None) => t,
                (None, None) => Ty::F32,
                (None, Some(a)) => self.type_of(a)?,
            };
            if self.procs_by_name.contains_key(&name) {
                return Err(self.in_use(name_tok));
            }
            let key = (name.clone(), ty);
            if self.local.vars.contains_key(&key) {
                return Err(self.in_use(name_tok));
            }
            if as_clause.is_some() && self.main.plain.get(&name).is_some_and(|&t| t != ty) {
                let msg = format!(
                    "`SHARED {}` with another type than the main module's is not supported yet",
                    show_bytes(self.text(name_tok.span))
                );
                return Err(self.error(name_tok.span, msg));
            }
            let id = match self.main.vars.get(&key) {
                Some(&id) => id,
                None => {
                    self.reserved(name_tok, &name, suffix)?;
                    let id = self.new_var(name.clone(), ty, Storage::Main);
                    self.main.vars.insert(key.clone(), id);
                    id
                }
            };
            self.local.vars.insert(key, id);
            if as_clause.is_some() {
                self.main.plain.insert(name.clone(), ty);
                self.local.plain.insert(name, ty);
            }
            self.names.push((SymbolKind::Var(id), name_tok.span));
        }
        Ok(())
    }

    // ---- statements ----

    fn assign(&mut self, stmt: ast::AssignStmt) -> R<()> {
        let node = stmt.node();
        let target = self.need(stmt.target().and_then(|n| n.name()), node.span())?;
        let value_node = self.need(stmt.value(), node.span())?;
        let value = self.expr(value_node)?;
        let var = self.target(target)?;
        let target = self.prog.var(var).ty;
        let value = self.store(value, target)?;
        self.push(node, StmtKind::Assign { var, value });
        Ok(())
    }

    fn print(&mut self, stmt: ast::PrintStmt) -> R<()> {
        let mut items = Vec::new();
        let mut newline = true;
        for part in stmt.parts() {
            match part {
                PrintPart::Semicolon(_) => newline = false,
                PrintPart::Comma(_) => {
                    items.push(PrintItem::Zone);
                    newline = false;
                }
                PrintPart::Expr(n) => {
                    let e = self.expr(n)?;
                    items.push(if e.ty == Ty::Str {
                        PrintItem::Str(e)
                    } else {
                        let qb = e.qb;
                        PrintItem::Num(self.convert_exact(e, qb))
                    });
                    newline = true;
                }
            }
        }
        self.push(stmt.node(), StmtKind::Print { items, newline });
        Ok(())
    }

    /// `CALL s(...)` or `s ...`: a SUB call, or a built-in statement (not supported yet).
    fn call_stmt(&mut self, stmt: ast::CallStmt) -> R<()> {
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
                let nodes: Vec<ast::Expr> = stmt.arg_list().map(|l| l.args().collect()).unwrap_or_default();
                let args = self.args(p, name_tok, &nodes, node.span())?;
                self.names.push((SymbolKind::Proc(p), name_tok.span));
                self.push(node, StmtKind::Call { proc: p, args });
                Ok(())
            }
            Some((_, ProcKind::Function(_))) => {
                let msg = format!("calling the FUNCTION `{shown}` as a statement is not supported yet");
                Err(self.error(name_tok.span, msg))
            }
            _ if is_keyword(name.as_bytes()) || find_any(name.as_bytes()).next().is_some() => {
                Err(self.error(name_tok.span, format!("`{shown}` is not supported yet")))
            }
            _ => {
                let msg = format!("`{shown}` is not a SUB, or not supported yet");
                Err(self.error(name_tok.span, msg))
            }
        }
    }

    fn exit(&mut self, node: Node) -> R<()> {
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
    fn args(&mut self, p: ProcId, name_tok: Tok, nodes: &[ast::Expr], span: Span) -> R<Vec<Arg>> {
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
            let e = self.expr(node)?;
            match (e.ty == Ty::Str, pty == Ty::Str) {
                (true, false) => return Err(self.error(e.span, "a number is required for this parameter")),
                (false, true) => return Err(self.error(e.span, "a string is required for this parameter")),
                _ => {}
            }
            // A number in parentheses, `(n)`, is a ParenExpr in the tree and passed as a copy; a string variable
            // is passed by reference even in parentheses (measured, `s08_byref`: `addbang (s$)` changes `s$`).
            let plain_name = matches!(node, ast::Expr::NameRef(_));
            args.push(match e.kind {
                ExprKind::Var(v) if e.ty == pty && (plain_name || pty == Ty::Str) => Arg::Ref(v),
                _ => Arg::Temp(self.store(e, pty)?),
            });
        }
        Ok(args)
    }

    /// A call of FUNCTION `p` (`args` is `None` for the bare name). A SUB, or the name with another suffix than
    /// the function's, is an error.
    fn call_function(
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
        if suffix.is_some_and(|s| s != ty) {
            return Err(self.in_use(t));
        }
        let nodes: Vec<ast::Expr> = args.map(|l| l.args().collect()).unwrap_or_default();
        let args = self.args(p, t, &nodes, span)?;
        self.names.push((SymbolKind::Proc(p), t.span));
        Ok(Expr {
            span,
            ty,
            qb: ty,
            kind: ExprKind::CallProc { proc: p, args },
        })
    }

    // ---- expressions ----

    fn expr(&mut self, node: ast::Expr) -> R<Expr> {
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
                if let Some(&p) = self.procs_by_name.get(&name) {
                    return self.call_function(p, t, suffix, None, span);
                }
                if is_builtin_function(&name, suffix) {
                    let shown = show_bytes(self.text(t.span));
                    return Err(self.error(t.span, format!("`{shown}` is not supported yet")));
                }
                let id = self.variable(t, name, suffix)?;
                let ty = self.prog.var(id).ty;
                Ok(Expr {
                    span,
                    ty,
                    qb: ty,
                    kind: ExprKind::Var(id),
                })
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

    fn prefix(&mut self, node: ast::PrefixExpr) -> R<Expr> {
        let span = node.node().span();
        let op = self.need(node.op(), span)?;
        let operand = self.need(node.operand(), span)?;
        match op.kind {
            Minus => {
                // A minus directly before a decimal literal is part of the literal (step C).
                if let ast::Expr::Literal(lit) = operand {
                    let t = self.need(lit.token(), span)?;
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
                        kind: ExprKind::Int(wrap(v.wrapping_neg(), ty)),
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

    fn binary(&mut self, node: ast::BinExpr) -> R<Expr> {
        let span = node.node().span();
        let l = self.need(node.lhs(), span)?;
        let op_tok = self.need(node.op(), span)?;
        let r = self.need(node.rhs(), span)?;
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
            // Wrapping in 64 bits, then to `ty`, keeps the same low bits as the exact result (D-001, D-002).
            let (a, b) = (*a, *b);
            let v = match op {
                BinOp::Add => a.wrapping_add(b),
                BinOp::Sub => a.wrapping_sub(b),
                BinOp::Mul => a.wrapping_mul(b),
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

    fn call(&mut self, node: ast::CallExpr) -> R<Expr> {
        let span = node.node().span();
        let name_tok = self.need(node.name(), span)?;
        let (proc_name, suffix) = self.split_name(name_tok)?;
        if let Some(&p) = self.procs_by_name.get(&proc_name) {
            let args = self.need(node.arg_list(), span)?;
            return self.call_function(p, name_tok, suffix, Some(args), span);
        }
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
    fn instr(&mut self, span: Span, id: BuiltinId, node: ast::CallExpr) -> R<Expr> {
        let args_node = self.need(node.arg_list(), span)?;
        let mut args = Vec::new();
        for a in args_node.args() {
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

/// Integer operands are computed in at least 32 bits (C promotion).
fn promote(t: Ty) -> Ty {
    if t == Ty::I16 { Ty::I32 } else { t }
}

/// Keeps the low bits of `v` that fit `ty` (integer overflow wraps, D-001, D-002).
fn wrap(v: i64, ty: Ty) -> i64 {
    match ty {
        Ty::I16 => i64::from(v as i16),
        Ty::I32 => i64::from(v as i32),
        Ty::I64 => v,
        Ty::F32 | Ty::F64 | Ty::F80 | Ty::Str => unreachable!("integer constant folded to {ty:?}"),
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
