//! Names and declarations: suffixes, reserved names, variables by scope, `DIM`, `STATIC`, `SHARED`, `OPTION`.

use super::places::{numeric_type, unsigned_of};
use super::proc::{FIXED_LEN_ERROR, fixed_len, fixed_name, suffix_fits};
use super::{Checker, Failed, R, Scope};
use crate::consteval::Value;
use crate::literal::{self, NumLit};
use crate::{ProcId, ProcKind, Shape, StmtKind, Storage, SymbolKind, Ty, Var, VarId};
use qb64rust_base::{show_bytes, to_u32};
use qb64rust_builtins::{Kind, find_any, is_auto_include_name};
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::ast;
use qb64rust_syntax::is_keyword;
use qb64rust_syntax::tree::{Node, Tok};

/// The statement a declaration stands in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Declarer {
    Dim,
    Static,
    /// `REDIM`, or with `preserve` `REDIM _PRESERVE`.
    Redim {
        preserve: bool,
    },
}

impl Checker<'_> {
    // ---- names ----

    /// The one place where `name$n` is read as a procedure's name: where `name` names a procedure, `name$n` is that
    /// name with a `STRING * n` suffix (a FUNCTION `f$n`, design D6), never a variable, so the procedure's checks
    /// decide as for any other suffix. Measured: beside a SUB or a FUNCTION of another type, `x$n` is "Name already
    /// in use" as a value, an assignment or in `DIM` (`verification\v21_x50` to `x53`). Used for variable names
    /// ([`Self::split_var_name`]) and calls only; elsewhere `name$n` stays unsupported.
    pub(super) fn proc_fixed_name(&self, t: Tok) -> Option<(String, Ty)> {
        match fixed_name(self.text(t.span)) {
            Some((name, Some(n))) if self.procs_by_name.contains_key(&name) => Some((name, Ty::FixedStr(n))),
            Some(_) | None => None,
        }
    }

    /// [`Self::split_name`] for the name of a scalar variable, which may also be `name$n`: a fixed-length string of n
    /// bytes, a variable other than `name$` (design D6; measured, `verification\v21_c_fixed_basics`; n read as
    /// [`fixed_len`]), or a procedure's name ([`Self::proc_fixed_name`]). A `t$n` parameter is found by
    /// [`Self::fixed_param_ref`] before this.
    pub(super) fn split_var_name(&mut self, t: Tok) -> R<(String, Option<Ty>)> {
        if let Some((name, ty)) = self.proc_fixed_name(t) {
            return Ok((name, Some(ty)));
        }
        match fixed_name(self.text(t.span)) {
            Some((name, Some(n))) => {
                self.fixed_name_alone(t, &name)?;
                Ok((name, Some(Ty::FixedStr(n))))
            }
            Some((_, None)) => Err(self.error(t.span, FIXED_LEN_ERROR)),
            None => self.split_name(t),
        }
    }

    /// A name `x$n` where `x` might also name a constant, a member or a `STRING * n` parameter of the current
    /// procedure: none of that was measured. (A procedure's name is [`Self::proc_fixed_name`].)
    fn fixed_name_alone(&mut self, t: Tok, name: &str) -> R<()> {
        let param = self.cur.is_some_and(|p| {
            let params = &self.prog.proc(p).params;
            params
                .iter()
                .any(|v| self.param_len.contains_key(v) && self.prog.var(*v).name == name)
        });
        if param || self.visible_const(name).is_some() || name.contains('.') {
            let shown = show_bytes(self.text(t.span));
            let msg = format!("`{shown}` beside a constant or `STRING * n` parameter of its name, or with a `.`");
            return Err(self.unsupported(t.span, msg));
        }
        Ok(())
    }

    /// Splits `name<suffix>` and gives the suffix's type (`None` without a suffix): every suffix of
    /// [`literal::suffix_type`]; `name$n` is not supported yet here (a variable's name is read by
    /// [`Self::split_var_name`], a FUNCTION `f$n` called by [`Self::proc_fixed_name`]).
    pub(super) fn split_name(&mut self, t: Tok) -> R<(String, Option<Ty>)> {
        let bytes = self.text(t.span);
        let end = name_end(bytes);
        let (name, suffix) = bytes.split_at(end);
        let name = show_bytes(&name.to_ascii_uppercase());
        if suffix.is_empty() {
            return Ok((name, None));
        }
        match literal::suffix_type(suffix) {
            Some(Ok(ty)) => Ok((name, Some(ty))),
            // Measured: "Invalid symbol" for `` k`65 ``, "Cannot create a _BIT variable of size 0 bits" for `` k`0 ``
            // (`verification\v21_x14`, `x15`).
            Some(Err(msg)) => Err(self.error(t.span, msg)),
            None => {
                let what = if suffix.starts_with(b"$") {
                    "fixed-length strings".to_string()
                } else {
                    format!("the type suffix `{}`", show_bytes(suffix))
                };
                Err(self.unsupported(t.span, what))
            }
        }
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
        // A name QB64pe's auto-included files declare (`_TRUE`, `_CHR_CR`): those files are not included yet
        // (design D10 of `m2-parser-breadth`).
        if is_auto_include_name(name.as_bytes()) {
            let shown = show_bytes(self.text(t.span));
            let msg = format!("`{shown}` (a name of QB64pe's auto-included files)");
            return Err(self.unsupported(t.span, msg));
        }
        // `validname` (`qb64pe.bas` 28271–28274) refuses a single leading `_` only: `__name$` is a parameter of
        // `qb64pe.bas` itself.
        if name.starts_with('_') && !name.starts_with("__") {
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

    /// Reserved names for a procedure, measured for every keyword and built-in (`verification\v19_proc_names.txt`,
    /// checked against this function by `crates\driver\tests\names.rs`): as for variables, a name starting with
    /// `_` and a keyword are taken; otherwise only a built-in **of the same kind** without a required suffix takes
    /// the name. A SUB may be named like a built-in function (`SUB loc`, `SUB abs`), a FUNCTION like a built-in
    /// statement (`FUNCTION beep`, `FUNCTION width`), and both like a built-in that needs `$` (`SUB left`).
    /// Measured for a SUB without a suffix and a FUNCTION without one and with `&`; other FUNCTION suffixes on
    /// a built-in function's name are "not supported yet".
    pub(super) fn reserved_proc(&mut self, t: Tok, name: &str, suffix: Option<Ty>, function: bool) -> R<()> {
        if name == "_GL" && !function {
            // The SUB QB64 calls to draw with OpenGL (design D10 of `m2-parser-breadth`).
            return Err(self.unsupported(t.span, "`SUB _GL` (OpenGL)"));
        }
        if name.starts_with('_') || is_keyword(name.as_bytes()) {
            return self.reserved(t, name, suffix);
        }
        let kind = if function { Kind::Function } else { Kind::Sub };
        let mut same_kind = find_any(name.as_bytes()).filter(|b| b.kind == kind);
        match suffix {
            None | Some(Ty::I32) if same_kind.any(|b| b.musthave.is_none()) => Err(self.in_use(t)),
            None | Some(Ty::I32) => Ok(()),
            Some(_) if same_kind.next().is_some() => {
                let msg = format!(
                    "`{}` as a FUNCTION name (a built-in's name with this suffix is not measured)",
                    show_bytes(self.text(t.span))
                );
                Err(self.unsupported(t.span, msg))
            }
            Some(_) => Ok(()),
        }
    }

    /// The type named by an `AS` clause: a numeric type, `STRING`, or a user type. As measured
    /// (`verification\v21_a_decls`, `v21_x01`–`x08`): `_UNSIGNED` before an integer type, `_OFFSET` or `_BIT`, and
    /// ignored before `STRING`; `_BIT * n` with n a number from 1 to 64; `STRING * n` with n a number or (where
    /// `consts`) a constant's name, read as [`fixed_len`] says (`v21_c_fixed_basics`, `v21_x31`–`x34`).
    pub(super) fn type_of(&mut self, a: ast::AsClause) -> R<Ty> {
        self.type_of_in(a, true)
    }

    /// [`Self::type_of`]; `consts` false where no constant is known yet (a `TYPE` member, read in pass 1).
    pub(super) fn type_of_in(&mut self, a: ast::AsClause, consts: bool) -> R<Ty> {
        let span = a.node().span();
        let words: Vec<String> = a.type_words().map(|t| self.word(t)).collect();
        let (unsigned, rest) = match words.split_first() {
            Some((first, rest)) if first == "_UNSIGNED" => (true, rest),
            _ => (false, &words[..]),
        };
        let text = rest.join(" ");
        if unsigned && text.is_empty() {
            // Measured: "Unknown type" (`v21_x08_unsigned_alone`).
            return Err(self.error(span, "`_UNSIGNED` needs a type after it"));
        }
        if let Some(size) = a.size() {
            return match text.as_str() {
                "_BIT" => {
                    let width = self.bit_width(size)?;
                    Ok(Ty::Bit {
                        width,
                        signed: !unsigned,
                    })
                }
                // Measured: `_UNSIGNED STRING * n` is a `STRING * n` (`v21_c_fixed_basics`).
                "STRING" => Ok(Ty::FixedStr(self.fixed_len_of(size, consts)?)),
                _ => Err(self.unsupported(span, format!("`{} * …`", words.join(" ")))),
            };
        }
        if text == "STRING" {
            // Measured: `_UNSIGNED STRING` is a `STRING` (`v21_a_decls`).
            return Ok(Ty::Str);
        }
        if let Some(t) = numeric_type(&text) {
            if !unsigned {
                return Ok(t);
            }
            return match unsigned_of(t) {
                Some(u) => Ok(u),
                // Measured: "Type cannot be _UNSIGNED" (`v21_x07_unsigned_single`).
                None => Err(self.error(span, format!("`{text}` cannot be `_UNSIGNED`"))),
            };
        }
        if let Some(t) = self.user_type_of(&text) {
            if unsigned {
                // Measured: "Type cannot be _UNSIGNED" (`v21_x46_unsigned_user_type`).
                return Err(self.error(span, format!("`{text}` cannot be `_UNSIGNED`")));
            }
            return Ok(t);
        }
        let msg = format!("the type `{}`", words.join(" "));
        Err(self.unsupported(span, msg))
    }

    /// The n of `_BIT * n`: a number from 1 to 64 (measured: 0 and 65 are errors, a constant's name "Number expected
    /// after *", `v21_x01`–`x04`).
    fn bit_width(&mut self, size: Tok) -> R<u8> {
        let width = literal::decimal(self.text(size.span)).map(u8::try_from);
        match width {
            Some(Ok(w @ 1..=64)) => Ok(w),
            Some(_) => Err(self.error(size.span, "a `_BIT` type takes 1 to 64 bits")),
            None => Err(self.error(size.span, "a number of bits is expected after `_BIT *`")),
        }
    }

    /// The n of `STRING * n` (design D6): a number, or where `consts` the name of an integer constant, read as
    /// [`fixed_len`] says. Measured (`v21_x31`–`x34`): 0, an expression, a float and a negative number are errors.
    /// A constant's name where none is known yet, or of a float or string constant, is not supported yet.
    pub(super) fn fixed_len_of(&mut self, size: Tok, consts: bool) -> R<u32> {
        let shown = show_bytes(self.text(size.span));
        if size.kind == SyntaxKind::Ident {
            let (name, suffix) = self.split_name(size)?;
            let found = match (consts, suffix, self.visible_const(&name)) {
                (true, None, Some(c)) => match self.prog.constant(c).value {
                    Value::Int(v) => Some((c, v)),
                    Value::Float(_) | Value::Str(_) => None,
                },
                _ => None,
            };
            let Some((c, v)) = found else {
                let msg = format!("this name as the length of a `STRING * n`: `{shown}`");
                return Err(self.unsupported(size.span, msg));
            };
            self.names.push((SymbolKind::Const(c), size.span));
            return match fixed_len(i128::from(v)) {
                Some(n) => Ok(n),
                None => Err(self.error(size.span, FIXED_LEN_ERROR)),
            };
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

    // ---- default types ----

    /// The type of a name written without a suffix or an `AS` clause, here in the file: SINGLE, or what a `DEFxxx`
    /// or `_DEFINE` above this line set for its first letter (design D2). A name that starts with an underscore
    /// has an entry of its own.
    pub(super) fn default_ty(&self, name: &str) -> Ty {
        self.defaults[letter_slot(name.as_bytes().first().copied().unwrap_or(b'_'))]
    }

    /// `DEFINT a-z, x` (and `DEFLNG`, `DEFSNG`, `DEFDBL`, `DEFSTR`) and `_DEFINE a-z, x AS type`: from this line
    /// of the file on, whether or not the statement runs, a name beginning with one of the letters and written
    /// without a suffix or `AS` clause has this type. Measured (`verification\v24_a_*`, `v24_x01`–`x37`): a range
    /// may be written backwards; `_` is a letter of its own, after Z; the statement inside a block or a procedure
    /// goes on applying below it.
    pub(super) fn def_type(&mut self, stmt: ast::DefTypeStmt) -> R<()> {
        let span = stmt.node().span();
        let keyword = self.need(stmt.keyword(), span)?;
        let ty = match self.word(keyword).as_str() {
            "DEFINT" => Ty::I16,
            "DEFLNG" => Ty::I32,
            "DEFSNG" => Ty::F32,
            "DEFDBL" => Ty::F64,
            "DEFSTR" => Ty::Str,
            _ => {
                let a = self.need(stmt.as_clause(), span)?;
                self.default_type_of(a)?
            }
        };
        let mut ranges = Vec::new();
        for range in stmt.ranges() {
            let (first, last) = self.need(range.letters(), range.node().span())?;
            let (a, b) = (
                letter_slot(self.text(first.span)[0]),
                letter_slot(self.text(last.span)[0]),
            );
            ranges.push((a.min(b), a.max(b)));
        }
        for (a, b) in ranges {
            self.defaults[a..=b].fill(ty);
        }
        Ok(())
    }

    /// The type of `_DEFINE … AS type`: any type a variable may have by a suffix. Measured: `STRING * n`, `_BIT`
    /// and `_BIT * n` are taken (`verification\v24_x01`–`x04`); a user type and `_UNSIGNED STRING` are "Cannot
    /// convert type (…) to symbol" (`v24_x05`, `x08`), though `DIM` takes `_UNSIGNED STRING`.
    fn default_type_of(&mut self, a: ast::AsClause) -> R<Ty> {
        let span = a.node().span();
        let words: Vec<String> = a.type_words().map(|t| self.word(t)).collect();
        if words == ["_UNSIGNED", "STRING"] {
            return Err(self.error(span, "a default type cannot be `_UNSIGNED STRING`"));
        }
        let ty = self.type_of_in(a, false)?;
        if let Ty::User(_) = ty {
            return Err(self.error(span, "a default type cannot be a `TYPE`"));
        }
        Ok(ty)
    }

    pub(super) fn new_var(&mut self, name: String, ty: Ty, storage: Storage) -> VarId {
        let id = VarId(to_u32(self.prog.vars.len()));
        self.prog.vars.push(Var {
            name,
            ty,
            storage,
            shape: Shape::Scalar,
        });
        id
    }

    /// The storage of a procedure's own variable or array: per call, or for the program in a procedure with
    /// `STATIC` after its header, down to its first `STATIC` statement (measured, `verification\v24_g_local`,
    /// `v24_f_implicit_sub`, `v24_x279`; slice `s56_proc_arrays`).
    pub(super) fn local_storage(&self, p: ProcId) -> Storage {
        if self.static_scope {
            Storage::Static(p)
        } else {
            Storage::Local(p)
        }
    }

    pub(super) fn scope(&mut self) -> &mut Scope {
        if self.cur.is_some() {
            &mut self.local
        } else {
            &mut self.main
        }
    }

    /// The variable a name refers to at this point of the program, if it exists (as [`Self::variable`] finds it,
    /// without creating one).
    pub(super) fn lookup_var(&mut self, name: String, suffix: Option<Ty>) -> Option<VarId> {
        let key = self.var_key(name, suffix);
        self.find_var(&key)
    }

    /// A name's variable key: the name and the type it means here (its suffix, or the type a `DIM … AS` gave the
    /// plain name, or the default type of its first letter at this line). In a procedure a variable of its
    /// own (a parameter, a local one) of the name and the default type comes before a main-module `DIM SHARED … AS`
    /// of the name; where a `DEFxxx` in the body has changed the default, the name means the shared variable
    /// (measured, `verification\v24_a_param_shared`).
    fn var_key(&mut self, name: String, suffix: Option<Ty>) -> (String, Ty) {
        let ty = match suffix {
            Some(t) => t,
            None => {
                let in_proc = self.cur.is_some();
                let default = self.default_ty(&name);
                let plain = self.scope().plain.get(&name).copied();
                let key = (name, default);
                // A main-module variable bound by `SHARED x!` is in `local.vars` too and does not count.
                let own = in_proc
                    && self
                        .local
                        .vars
                        .get(&key)
                        .is_some_and(|&v| self.prog.var(v).storage != Storage::Main);
                let shared = (in_proc && !own)
                    .then(|| self.dim_shared_plain.get(&key.0).copied())
                    .flatten();
                return (key.0, plain.or(shared).unwrap_or(default));
            }
        };
        (name, ty)
    }

    /// The variable of `key` in the current scope, or in a procedure a main-module `DIM SHARED` one.
    fn find_var(&mut self, key: &(String, Ty)) -> Option<VarId> {
        self.scope().vars.get(key).copied().or_else(|| {
            let main = self.main.vars.get(key).copied();
            main.filter(|v| self.cur.is_some() && self.dim_shared.contains(v))
        })
    }

    /// The variable a name (not a procedure's) refers to at this point of the program, created on first use. In
    /// a procedure: its own variables (parameters, `STATIC`, `SHARED`, `DIM`, implicit), then the main module's
    /// `DIM SHARED` variables declared earlier in the file, else a new local.
    pub(super) fn variable(&mut self, t: Tok, name: String, suffix: Option<Ty>) -> R<VarId> {
        let key = self.var_key(name, suffix);
        let ty = key.1;
        let storage = match self.cur {
            Some(p) => self.local_storage(p),
            None => Storage::Main,
        };
        let found = self.find_var(&key);
        let id = match found {
            Some(id) => id,
            None => {
                let typed_array = self.scope().array_plain.contains_key(&key.0)
                    || (self.cur.is_some() && self.dim_shared_array_plain.contains_key(&key.0));
                if suffix.is_none() && typed_array {
                    // Whether `DIM a(3) AS LONG` types the plain scalar `a` too was not measured.
                    let shown = show_bytes(self.text(t.span));
                    let msg = format!("the plain scalar `{shown}` beside an array typed by `DIM … AS`");
                    return Err(self.unsupported(t.span, msg));
                }
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
        let (name, suffix) = self.split_var_name(t)?;
        if let Some(&p) = self.procs_by_name.get(&name) {
            let proc = self.prog.proc(p);
            if let (Some(cur), ProcKind::Function(ty)) = (self.cur, proc.kind)
                && cur == p
                && suffix.is_none_or(|s| suffix_fits(s, ty))
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

    /// `DIM`, `DIM SHARED` (main module only), `STATIC` (procedures only) and `REDIM` (`by`): `storage` is the
    /// class of the variables created. `list` is the type written before the names (`DIM AS LONG a, b(3)`): every
    /// name of the list is then declared as `name AS type` would declare it. `stmt` is the statement, which an
    /// array that is allocated when it runs becomes ([`Self::declare_array`]).
    fn declare_items<'t>(
        &mut self,
        items: impl Iterator<Item = ast::DimItem<'t>>,
        list: Option<ast::AsClause<'t>>,
        storage: Storage,
        shared: bool,
        by: Declarer,
        stmt: Node,
    ) -> R<()> {
        let list_ty = match list {
            Some(a) => Some(self.type_of(a)?),
            None => None,
        };
        for item in items {
            if item.is_member_path() {
                // `REDIM a(1).m(5)`.
                return Err(self.unsupported(item.node().span(), "arrays as `TYPE` members"));
            }
            let name_tok = self.need(item.name(), item.node().span())?;
            let (name, suffix) = self.split_var_name(name_tok)?;
            let as_clause = item.as_clause();
            let ty = match list_ty {
                Some(t) => self.list_item_type(name_tok, suffix, as_clause, t)?,
                None => match (suffix, as_clause) {
                    (Some(_), Some(a)) => {
                        // Measured also for a `TYPE` (`DIM p& AS pt`: "DIM: Expected ,", `v18_h_dim_type_suffix`).
                        let span = a.node().span();
                        return Err(self.error(span, "a name with a type suffix cannot have an `AS` clause"));
                    }
                    (Some(t), None) => t,
                    (None, None) => self.default_ty(&name),
                    (None, Some(a)) => self.type_of(a)?,
                },
            };
            // The type was written with `AS`, on the name or before the list: it types the plain name.
            let typed = as_clause.is_some() || list_ty.is_some();
            // With nothing between the parentheses, and in a `REDIM` without parentheses, the name is declared as
            // a scalar and no array is made (measured in the main module, `verification\v24_b_dim_empty*`,
            // `v24_x43`, `x44`, `x86`) and in a procedure (`v24_x273`). `STATIC a()` lists an array (measured,
            // `v24_g_local`).
            if let Some(bounds) = item.bounds()
                && (!bounds.ranges().is_empty() || by == Declarer::Static)
            {
                self.declare_array(typed, bounds, (name_tok, name, suffix, ty), storage, shared, by, stmt)?;
                continue;
            }
            if matches!(by, Declarer::Redim { .. }) && storage != Storage::Main && item.bounds().is_none() {
                let shown = show_bytes(self.text(name_tok.span));
                let msg = format!("`REDIM {shown}` without bounds in a SUB or FUNCTION");
                return Err(self.unsupported(name_tok.span, msg));
            }
            if name.contains('.') && matches!(ty, Ty::User(_)) {
                let shown = show_bytes(self.text(name_tok.span));
                return Err(self.unsupported(
                    name_tok.span,
                    format!("a `TYPE` variable with a dotted name: `{shown}`"),
                ));
            }
            if self.procs_by_name.contains_key(&name) {
                return Err(self.in_use(name_tok));
            }
            if self.visible_const(&name).is_some() {
                // Measured for `DIM`, in main and in a procedure (`v17_e_err_dim_after_const`,
                // `v17_e_const_sub_dim_same`), also one with `STATIC` after its header (`v24_x283`); not for
                // `STATIC`.
                if by == Declarer::Static {
                    let shown = show_bytes(self.text(name_tok.span));
                    return Err(self.unsupported(name_tok.span, format!("`STATIC` of the constant name `{shown}`")));
                }
                return Err(self.in_use(name_tok));
            }
            // Measured (verification\v13*): `DIM x AS T` fails once an earlier `DIM … AS` typed the plain
            // name, whatever the type; a plain `DIM x` after that is accepted and changes nothing.
            let typed_plain = self.scope().plain.get(&name).copied();
            if let (true, Some(plain_ty), false) = (suffix.is_none(), typed_plain, typed) {
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
            // A local `DIM` shadows a `DIM SHARED` variable of the same name (measured, `v14_dim_local_*`), also
            // in a procedure with `STATIC` after its header, where it lives for the program (`v24_x284`); a
            // `STATIC` beside one is not measured.
            let shadows_shared = self.cur.is_some()
                && (self.main.vars.get(&key).is_some_and(|v| self.dim_shared.contains(v))
                    || (suffix.is_none() && self.dim_shared_plain.contains_key(&name)));
            if shadows_shared && by == Declarer::Static {
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
            if suffix.is_none() && typed {
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

    /// The type of a name in a list with the type before the names. Measured (`verification\v24_x227`–`x229`): a
    /// type suffix on the name, also the list's own, is "Cannot use type symbol with DIM AS type variable-list";
    /// an `AS` clause on it "DIM: Expected ,".
    fn list_item_type(&mut self, name: Tok, suffix: Option<Ty>, own: Option<ast::AsClause>, list: Ty) -> R<Ty> {
        if suffix.is_some() {
            let shown = show_bytes(self.text(name.span));
            let msg = format!("a name in a list with the type before the names cannot have a type suffix: `{shown}`");
            return Err(self.error(name.span, msg));
        }
        if let Some(a) = own {
            let msg = "a name in a list with the type before the names cannot have an `AS` clause";
            return Err(self.error(a.node().span(), msg));
        }
        Ok(list)
    }

    pub(super) fn dim(&mut self, stmt: ast::DimStmt) -> R<()> {
        let shared = stmt.shared().is_some();
        let storage = match (self.cur, shared) {
            (None, _) => Storage::Main,
            (Some(_), true) => {
                // Measured: "DIM/REDIM SHARED invalid within a SUB/FUNCTION" (`verification\v24_x128`).
                let span = stmt.node().span();
                return Err(self.error(span, "`DIM SHARED` cannot stand inside a SUB or FUNCTION"));
            }
            (Some(p), false) => self.local_storage(p),
        };
        let node = stmt.node();
        self.declare_items(stmt.items(), stmt.as_clause(), storage, shared, Declarer::Dim, node)
    }

    pub(super) fn static_stmt(&mut self, stmt: ast::StaticStmt) -> R<()> {
        let Some(p) = self.cur else {
            // Measured: "STATIC must be used within a SUB/FUNCTION" (`verification\v24_x127`).
            let span = stmt.node().span();
            return Err(self.error(span, "`STATIC` must stand inside a SUB or FUNCTION"));
        };
        let node = stmt.node();
        let done = self.declare_items(
            stmt.items(),
            stmt.as_clause(),
            Storage::Static(p),
            false,
            Declarer::Static,
            node,
        );
        // Below its first `STATIC` statement a procedure with `STATIC` after its header is one like any other.
        self.static_scope = false;
        done
    }

    /// `REDIM [_PRESERVE] [SHARED] name(bounds) [AS type], …` (design D3, D4 of `m2-defxxx-common-arrays`): each
    /// name is a dynamic array, declared here when no array of the name and type is visible, and given new bounds
    /// and new elements when the statement runs. Measured (`verification\v24_c_redim*`, `v24_x39`–`x56`): the
    /// words come in this order; `REDIM SHARED _PRESERVE` is "Invalid variable name".
    pub(super) fn redim(&mut self, stmt: ast::RedimStmt) -> R<()> {
        let node = stmt.node();
        let words: Vec<Tok> = stmt.words().collect();
        let texts: Vec<String> = words.iter().map(|&t| self.word(t)).collect();
        let texts: Vec<&str> = texts.iter().map(String::as_str).collect();
        let (preserve, shared) = match texts[..] {
            [] => (false, false),
            ["_PRESERVE"] => (true, false),
            ["SHARED"] => (false, true),
            ["_PRESERVE", "SHARED"] => (true, true),
            ["SHARED", "_PRESERVE"] => {
                return Err(self.error(words[1].span, "`_PRESERVE` comes before `SHARED` in a `REDIM`"));
            }
            // `_RETAIN`, alone or with the other words.
            _ => return Err(self.unsupported(words[0].span, format!("`REDIM {}`", texts.join(" ")))),
        };
        let storage = match (self.cur, shared) {
            (None, _) => Storage::Main,
            (Some(_), true) => {
                // Measured: "DIM/REDIM SHARED invalid within a SUB/FUNCTION" (`verification\v24_x129`).
                return Err(self.error(node.span(), "`REDIM SHARED` cannot stand inside a SUB or FUNCTION"));
            }
            (Some(p), false) => self.local_storage(p),
        };
        let by = Declarer::Redim { preserve };
        self.declare_items(stmt.items(), stmt.as_clause(), storage, shared, by, node)
    }

    /// `ERASE name, …` (design D4 of `m2-defxxx-common-arrays`; measured, `verification\v24_d_*`, `v24_x57`–`x63`):
    /// a static array is cleared, a dynamic one released, in order. Anything that is no declared array is
    /// "Undefined array passed to ERASE": a scalar, a name never used (no array is made for it), `name()`, an
    /// element, a suffix of another type than the array's.
    pub(super) fn erase(&mut self, stmt: ast::EraseStmt) -> R<()> {
        let node = stmt.node();
        for target in stmt.targets() {
            let name = match target {
                ast::EraseTarget::Name { name, parens: false } => name,
                ast::EraseTarget::Name { name, parens: true } => {
                    let span = name.node().span();
                    return Err(self.error(span, "`ERASE` takes the array's name without `()`"));
                }
                ast::EraseTarget::Other(ast::Expr::Field(f)) => {
                    return Err(self.unsupported(f.node().span(), "arrays as `TYPE` members"));
                }
                ast::EraseTarget::Other(other) => {
                    return Err(self.error(other.node().span(), "`ERASE` takes a whole array, by its name"));
                }
            };
            let t = self.need(name.name(), node.span())?;
            let (array_name, suffix) = self.split_name(t)?;
            let Some(array) = self.find_array(&array_name, suffix) else {
                let shown = show_bytes(self.text(t.span));
                if self.implicit_names.contains(&array_name) {
                    let msg = format!("`ERASE` of `{shown}`, an array used without `DIM`");
                    return Err(self.unsupported(t.span, msg));
                }
                return Err(self.error(t.span, format!("`{shown}` is no declared array (`ERASE`)")));
            };
            self.names.push((SymbolKind::Var(array), t.span));
            let kind = match self.prog.var(array).shape {
                Shape::Static { .. } => StmtKind::Clear { array },
                Shape::Dynamic { .. } => StmtKind::Release { array },
                Shape::Scalar => unreachable!("an array is no scalar"),
            };
            self.push(node, kind);
        }
        Ok(())
    }

    /// `OPTION _EXPLICIT` and `OPTION _EXPLICITARRAY` (design D7). Their flags were set by the pre-pass, since
    /// they apply to the whole program; `_EXPLICITARRAY` concerns only arrays used without `DIM`
    /// ([`Self::implicit_array`]).
    /// `OPTION BASE 0` and `1` (design D3 of `m2-defxxx-common-arrays`): the lower bound of the dimensions declared
    /// without `TO` below this line of the file, wherever the statement stands and whether or not it runs
    /// (measured, `verification\v24_e_option_base`, `v24_x70`).
    pub(super) fn option_stmt(&mut self, stmt: ast::OptionStmt) -> R<()> {
        let node = stmt.node();
        let word = self.need(stmt.word(), node.span())?;
        match self.word(word).as_str() {
            "_EXPLICIT" | "_EXPLICITARRAY" => Ok(()),
            "BASE" => {
                let base = self.need(stmt.base(), node.span())?;
                // Measured: plain digits that are 0 or 1, leading zeros allowed (`01`, `00`, `v24_x239`, `x240`,
                // `x243`); "Expected OPTION BASE 0 or 1" for `2`, `1.0`, `1&` and `&H1` (`v24_x64`, `x68`, `x241`,
                // `x242`).
                let digits = self.text(base.span);
                let (zeros, last) = digits.split_at(digits.len().saturating_sub(1));
                self.option_base = match last {
                    b"0" if zeros.iter().all(|&b| b == b'0') => 0,
                    b"1" if zeros.iter().all(|&b| b == b'0') => 1,
                    _ => return Err(self.error(base.span, "`OPTION BASE` takes `0` or `1`")),
                };
                Ok(())
            }
            // Measured without `$NOPREFIX` (`v17_f_explicit_no_underscore`): "Expected OPTION BASE or OPTION
            // _EXPLICIT or OPTION _EXPLICITARRAY".
            _ => Err(self.error(
                word.span,
                "expected `OPTION BASE`, `OPTION _EXPLICIT` or `OPTION _EXPLICITARRAY`",
            )),
        }
    }

    /// A variable used without a declaration under `OPTION _EXPLICIT`: a real error. After a declaration marked
    /// "not supported yet" (which may have declared the name) the follow-on rule drops it (design D10, which
    /// replaced the narrower rule first applied here).
    fn undeclared(&mut self, t: Tok, ty: Ty) -> Failed {
        let shown = show_bytes(self.text(t.span));
        let msg = format!(
            "variable `{shown}` ({}) is not declared (`OPTION _EXPLICIT`)",
            ty.qb_name()
        );
        self.error(t.span, msg)
    }

    /// `SHARED name [AS type]` in a procedure: binds the main-module variable of that name and type, created if
    /// missing. Measured: without `AS` or a suffix the type is the default one, whatever the main module's plain name
    /// means (`v14_shared_plain_typed`); with `AS` it types the main module's plain name too, as a `DIM` there
    /// would (`v14_shared_plain_main`). Under `OPTION _EXPLICIT` a missing one is an error instead.
    pub(super) fn shared(&mut self, stmt: ast::SharedStmt) -> R<()> {
        if self.cur.is_none() {
            let span = stmt.node().span();
            return Err(self.unsupported(span, "`SHARED` in the main module"));
        }
        let list_ty = match stmt.as_clause() {
            Some(a) => Some(self.type_of(a)?),
            None => None,
        };
        for item in stmt.items() {
            let name_tok = self.need(item.name(), item.node().span())?;
            if let Some(b) = item.bounds().filter(|b| !b.ranges().is_empty()) {
                // Measured: "Expected ()" (`verification\v24_x133`).
                let msg = "`SHARED` names an array with nothing between its parentheses: `name()`";
                return Err(self.error(b.node().span(), msg));
            }
            let (name, suffix) = self.split_var_name(name_tok)?;
            let as_clause = item.as_clause();
            let ty = match list_ty {
                Some(t) => self.list_item_type(name_tok, suffix, as_clause, t)?,
                None => match (suffix, as_clause) {
                    (Some(_), Some(a)) => {
                        let span = a.node().span();
                        return Err(self.error(span, "a name with a type suffix cannot have an `AS` clause"));
                    }
                    (Some(t), None) => t,
                    (None, None) => self.default_ty(&name),
                    (None, Some(a)) => self.type_of(a)?,
                },
            };
            let typed = as_clause.is_some() || list_ty.is_some();
            if self.procs_by_name.contains_key(&name) {
                return Err(self.in_use(name_tok));
            }
            if self.visible_const(&name).is_some() {
                let shown = show_bytes(self.text(name_tok.span));
                return Err(self.unsupported(name_tok.span, format!("`SHARED` of the constant name `{shown}`")));
            }
            if item.bounds().is_some() {
                self.shared_array(name_tok, name, ty, suffix.is_none() && typed)?;
                continue;
            }
            let key = (name.clone(), ty);
            if self.local.vars.contains_key(&key) {
                return Err(self.in_use(name_tok));
            }
            if typed && self.main.plain.get(&name).is_some_and(|&t| t != ty) {
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
            if typed {
                self.main.plain.insert(name.clone(), ty);
                self.local.plain.insert(name, ty);
            }
            self.names.push((SymbolKind::Var(id), name_tok.span));
        }
        Ok(())
    }

    /// `SHARED name() [AS type]` (design D6 of `m2-defxxx-common-arrays`; measured, `verification\v24_g_shared`,
    /// `v24_x130`–`x134`, `x274`, `x275`, `x277`): the name means the main-module array of that name and type in
    /// this procedure, whether it was declared or used without `DIM`. It must stand above the procedure in the
    /// file, and none is made: "Array 'a' not defined" otherwise, also when the type differs (without `AS` or a
    /// suffix the type is the default one, as for a scalar). Where the procedure has an array of the name and
    /// type already, that one stays what the name means. `typed`: the type was written with `AS`, which types
    /// the plain name.
    fn shared_array(&mut self, name_tok: Tok, name: String, ty: Ty, typed: bool) -> R<()> {
        let shown = show_bytes(self.text(name_tok.span));
        let key = (name.clone(), ty);
        let Some(&id) = self.main.arrays.get(&key) else {
            let msg = format!(
                "array `{shown}` ({}) is not declared in the main module above this procedure (`SHARED`)",
                ty.qb_name()
            );
            return Err(self.error(name_tok.span, msg));
        };
        if let Some(&own) = self.local.arrays.get(&key) {
            self.names.push((SymbolKind::Var(own), name_tok.span));
            return Ok(());
        }
        if typed && self.local.array_plain.get(&name).is_some_and(|&t| t != ty) {
            let msg = format!("`SHARED {shown}()` beside an array of that name typed otherwise by `DIM … AS`");
            return Err(self.unsupported(name_tok.span, msg));
        }
        self.local.arrays.insert(key, id);
        if typed {
            self.local.array_plain.insert(name, ty);
        }
        self.names.push((SymbolKind::Var(id), name_tok.span));
        Ok(())
    }
}

/// The entry of the default-type table for a name's first character: A to Z, then one for `_` (and anything else).
fn letter_slot(first: u8) -> usize {
    match first.to_ascii_uppercase() {
        c @ b'A'..=b'Z' => usize::from(c - b'A'),
        _ => 26,
    }
}

/// Where the name part of a name token ends (letters, digits, `_`, `.`); its type suffix follows.
pub(super) fn name_end(bytes: &[u8]) -> usize {
    bytes
        .iter()
        .position(|&b| !(b.is_ascii_alphanumeric() || b == b'_' || b == b'.'))
        .unwrap_or(bytes.len())
}
