//! Names and declarations: suffixes, reserved names, variables by scope, `DIM`, `STATIC`, `SHARED`, `OPTION`.

use super::{Checker, Failed, R, Scope};
use crate::{ProcKind, Storage, SymbolKind, Ty, Var, VarId};
use qb64rust_base::{show_bytes, to_u32};
use qb64rust_builtins::find_any;
use qb64rust_syntax::ast;
use qb64rust_syntax::is_keyword;
use qb64rust_syntax::tree::Tok;

impl Checker<'_> {
    // ---- names ----

    /// Splits `name<suffix>` and gives the suffix's type (`None` without a suffix).
    pub(super) fn split_name(&mut self, t: Tok) -> R<(String, Option<Ty>)> {
        let bytes = self.text(t.span);
        let end = bytes
            .iter()
            .position(|&b| !(b.is_ascii_alphanumeric() || b == b'_' || b == b'.'));
        let (name, suffix) = bytes.split_at(end.unwrap_or(bytes.len()));
        let name = show_bytes(&name.to_ascii_uppercase());
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
                let msg = format!("the type suffix `{}`", show_bytes(other));
                return Err(self.unsupported(t.span, msg));
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
    ///
    /// Only the bare name and the `&` suffix were measured for keywords and built-ins without a required suffix;
    /// other suffixes are "not supported yet". (The old compiler accepts `name$`, `not$` and `key$`, found in
    /// `qbasic_testcases` in task 6.4 of `m2-parser-breadth`; the rule above is not "whatever suffix".)
    pub(super) fn reserved(&mut self, t: Tok, name: &str, suffix: Option<Ty>) -> R<()> {
        if name.starts_with('_') {
            let msg = format!(
                "names starting with `_` are reserved: `{}`",
                show_bytes(self.text(t.span))
            );
            return Err(self.error(t.span, msg));
        }
        let word_taken =
            is_keyword(name.as_bytes()) || (name != "WIDTH" && find_any(name.as_bytes()).any(|b| b.musthave.is_none()));
        let with_dollar_taken = suffix == Some(Ty::Str) && find_any(name.as_bytes()).any(|b| b.musthave == Some("$"));
        if with_dollar_taken || (word_taken && matches!(suffix, None | Some(Ty::I32))) {
            Err(self.in_use(t))
        } else if word_taken {
            let msg = format!(
                "`{}` as a name (a reserved word with this suffix is not measured)",
                show_bytes(self.text(t.span))
            );
            Err(self.unsupported(t.span, msg))
        } else {
            Ok(())
        }
    }

    /// The type named by an `AS` clause.
    pub(super) fn type_of(&mut self, a: ast::AsClause) -> R<Ty> {
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
                let msg = format!("the type `{other}`");
                return Err(self.unsupported(a.node().span(), msg));
            }
        })
    }

    pub(super) fn new_var(&mut self, name: String, ty: Ty, storage: Storage) -> VarId {
        let id = VarId(to_u32(self.prog.vars.len()));
        self.prog.vars.push(Var { name, ty, storage });
        id
    }

    pub(super) fn scope(&mut self) -> &mut Scope {
        if self.cur.is_some() {
            &mut self.local
        } else {
            &mut self.main
        }
    }

    /// The variable a name (not a procedure's) refers to at this point of the program, created on first use. In
    /// a procedure: its own variables (parameters, `STATIC`, `SHARED`, `DIM`, implicit), then the main module's
    /// `DIM SHARED` variables declared earlier in the file, else a new local.
    pub(super) fn variable(&mut self, t: Tok, name: String, suffix: Option<Ty>) -> R<VarId> {
        let ty = match suffix {
            Some(t) => t,
            None => {
                let in_proc = self.cur.is_some();
                let plain = self.scope().plain.get(&name).copied();
                let shared = in_proc.then(|| self.dim_shared_plain.get(&name).copied()).flatten();
                plain.or(shared).unwrap_or(Ty::F32)
            }
        };
        let storage = match self.cur {
            Some(p) => Storage::Local(p),
            None => Storage::Main,
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
                if self.explicit {
                    return Err(self.undeclared(t, ty));
                }
                let id = self.new_var(key.0.clone(), ty, storage);
                self.scope().vars.insert(key, id);
                id
            }
        };
        let (name, storage) = (self.prog.var(id).name.clone(), self.prog.var(id).storage);
        self.note_var_name(&name, storage, t);
        self.names.push((SymbolKind::Var(id), t.span));
        Ok(id)
    }

    /// An assignment target: a variable, or inside `FUNCTION f` the name `f` (with or without its suffix), which
    /// is the function's result.
    pub(super) fn target(&mut self, t: Tok) -> R<VarId> {
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
        if self.visible_const(&name).is_some() {
            // Measured: "Expected variable =, look for conflict with a CONST name".
            let shown = show_bytes(self.text(t.span));
            return Err(self.error(t.span, format!("cannot assign to the constant `{shown}`")));
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
            if self.visible_const(&name).is_some() {
                // Measured for `DIM`, in main and in a procedure (`v17_e_err_dim_after_const`,
                // `v17_e_const_sub_dim_same`); not for `STATIC`.
                if matches!(storage, Storage::Static(_)) {
                    let shown = show_bytes(self.text(name_tok.span));
                    return Err(self.unsupported(name_tok.span, format!("`STATIC` of the constant name `{shown}`")));
                }
                return Err(self.in_use(name_tok));
            }
            // Measured (verification\v13*): `DIM x AS T` fails once an earlier `DIM … AS` typed the plain
            // name, whatever the type; a plain `DIM x` after that is accepted and changes nothing.
            let typed_plain = self.scope().plain.get(&name).copied();
            if let (true, Some(plain_ty), None) = (suffix.is_none(), typed_plain, as_clause) {
                // The name still refers to the typed variable.
                let id = self.scope().vars[&(name.clone(), plain_ty)];
                self.note_var_name(&name, storage, name_tok);
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
                    "a `STATIC` `{}` beside the `DIM SHARED` one",
                    show_bytes(self.text(name_tok.span))
                );
                return Err(self.unsupported(name_tok.span, msg));
            }
            self.reserved(name_tok, &name, suffix)?;
            let id = self.new_var(name.clone(), ty, storage);
            self.scope().vars.insert(key, id);
            self.note_var_name(&name, storage, name_tok);
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

    pub(super) fn dim(&mut self, stmt: ast::DimStmt) -> R<()> {
        let shared = stmt.shared().is_some();
        let storage = match (self.cur, shared) {
            (None, _) => Storage::Main,
            (Some(_), true) => {
                let span = stmt.node().span();
                return Err(self.unsupported(span, "`DIM SHARED` inside a SUB or FUNCTION"));
            }
            (Some(p), false) => Storage::Local(p),
        };
        self.declare_items(stmt.items(), storage, shared)
    }

    pub(super) fn static_stmt(&mut self, stmt: ast::StaticStmt) -> R<()> {
        let Some(p) = self.cur else {
            let span = stmt.node().span();
            return Err(self.unsupported(span, "`STATIC` in the main module"));
        };
        self.declare_items(stmt.items(), Storage::Static(p), false)
    }

    /// `OPTION _EXPLICIT` and `OPTION _EXPLICITARRAY` (design D7). The flag was set by the pre-pass, since it
    /// applies to the whole program; `_EXPLICITARRAY` only concerns arrays, which are "not supported yet" anyway.
    pub(super) fn option_stmt(&mut self, stmt: ast::OptionStmt) -> R<()> {
        let node = stmt.node();
        let word = self.need(stmt.word(), node.span())?;
        match self.word(word).as_str() {
            "_EXPLICIT" | "_EXPLICITARRAY" => Ok(()),
            "BASE" => Err(self.unsupported(word.span, "`OPTION BASE`")),
            // Measured without `$NOPREFIX` (`v17_f_explicit_no_underscore`): "Expected OPTION BASE or OPTION
            // _EXPLICIT or OPTION _EXPLICITARRAY".
            _ => Err(self.error(
                word.span,
                "expected `OPTION BASE`, `OPTION _EXPLICIT` or `OPTION _EXPLICITARRAY`",
            )),
        }
    }

    /// A variable used without a declaration under `OPTION _EXPLICIT`. Once something was marked "not supported
    /// yet", the use is only "not supported yet" too: that construct may declare the name (an `$INCLUDE`, `DIM AS
    /// LONG x`, a `TYPE` variable, a suffix not supported yet), so a real error could be false (the follow-on rule
    /// of `study\23` §2.3, applied here first). Counted: the parser's marks earlier in the same file, and every
    /// mark `sema` made so far. The latter are earlier in the file, except those of pass 1 (procedure headers)
    /// and of the label pre-pass, which may stand anywhere; counting them too only hides more real errors.
    fn undeclared(&mut self, t: Tok, ty: Ty) -> Failed {
        let shown = show_bytes(self.text(t.span));
        let parse_mark_before = self
            .parse_marks
            .iter()
            .any(|m| m.file == t.span.file && m.start < t.span.start);
        if self.diags.unsupported_count() > 0 || parse_mark_before {
            let msg = format!("`{shown}` under `OPTION _EXPLICIT` after a construct not supported yet");
            return self.unsupported(t.span, msg);
        }
        let msg = format!(
            "variable `{shown}` ({}) is not declared (`OPTION _EXPLICIT`)",
            ty.qb_name()
        );
        self.error(t.span, msg)
    }

    /// `SHARED name [AS type]` in a procedure: binds the main-module variable of that name and type, created if
    /// missing. Measured: without `AS` or a suffix the type is SINGLE, whatever the main module's plain name
    /// means (`v14_shared_plain_typed`); with `AS` it types the main module's plain name too, as a `DIM` there
    /// would (`v14_shared_plain_main`). Under `OPTION _EXPLICIT` a missing one is an error instead.
    pub(super) fn shared(&mut self, stmt: ast::SharedStmt) -> R<()> {
        if self.cur.is_none() {
            let span = stmt.node().span();
            return Err(self.unsupported(span, "`SHARED` in the main module"));
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
            if self.visible_const(&name).is_some() {
                let shown = show_bytes(self.text(name_tok.span));
                return Err(self.unsupported(name_tok.span, format!("`SHARED` of the constant name `{shown}`")));
            }
            let key = (name.clone(), ty);
            if self.local.vars.contains_key(&key) {
                return Err(self.in_use(name_tok));
            }
            if as_clause.is_some() && self.main.plain.get(&name).is_some_and(|&t| t != ty) {
                let msg = format!(
                    "`SHARED {}` with another type than the main module's",
                    show_bytes(self.text(name_tok.span))
                );
                return Err(self.unsupported(name_tok.span, msg));
            }
            let id = match self.main.vars.get(&key) {
                Some(&id) => id,
                None => {
                    self.reserved(name_tok, &name, suffix)?;
                    if self.explicit {
                        // Measured: also when the main module declares the name later in the file
                        // (`v17_f_explicit_shared_before_dim`) or with another type (`..._shared_other_type`).
                        return Err(self.undeclared(name_tok, ty));
                    }
                    let id = self.new_var(name.clone(), ty, Storage::Main);
                    self.main.vars.insert(key.clone(), id);
                    id
                }
            };
            self.local.vars.insert(key, id);
            self.note_var_name(&name, Storage::Main, name_tok);
            if as_clause.is_some() {
                self.main.plain.insert(name.clone(), ty);
                self.local.plain.insert(name, ty);
            }
            self.names.push((SymbolKind::Var(id), name_tok.span));
        }
        Ok(())
    }
}
