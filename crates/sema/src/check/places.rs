//! User types, arrays and places (design D3–D5 of `m2-arrays-and-types`): `TYPE` blocks (pass 1), `DIM` of static
//! arrays and their bounds, array elements, members and dotted names, `LBOUND`/`UBOUND`. Every rule is measured
//! (`verification\v18_*`, `study\00` §5); what was not measured is "not supported yet".

use super::decl::Declarer;
use super::{Checker, Failed, R, Skips};
use crate::{AllocMode, BinOp, ConvKind, StmtKind, UnOp};
use crate::{Expr, ExprKind, Member, MemberId, Place, Shape, Storage, SymbolKind, Ty, TypeId, UserType, Var, VarId};
use qb64rust_base::{Span, show_bytes, to_u32};
use qb64rust_builtins::find_any;
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::ast;
use qb64rust_syntax::is_keyword;
use qb64rust_syntax::tree::{Node, Tok};

/// One dimension of a declaration ([`Checker::dim_bounds`]): its bounds as values of type `_INTEGER64`, and as
/// numbers when both are constant by their spelling.
struct DimBounds {
    lower: Expr,
    upper: Expr,
    constant: Option<(i64, i64)>,
}

/// How an array bound is written: one number, one name (constant when it names a `CONST`), or anything else.
enum Spelling {
    Number,
    Name,
    Operator,
}

/// The type words that name a built-in type, which a `TYPE` may not take as its name (not measured: "not supported
/// yet").
const TYPE_WORDS: &[&str] = &[
    "INTEGER",
    "LONG",
    "_INTEGER64",
    "SINGLE",
    "DOUBLE",
    "_FLOAT",
    "STRING",
    "_BYTE",
    "_BIT",
    "_OFFSET",
    "_UNSIGNED",
    "_MEM",
];

/// `TYPE` names measured as accepted, and as "Name already in use", though they are built-in or keyword names.
const TYPE_NAMES_FREE: &[&str] = &["POINT", "CLS"];
const TYPE_NAMES_TAKEN: &[&str] = &["LEN", "PRINT", "LONG"];

/// Member names measured likewise.
const MEMBER_NAMES_FREE: &[&str] = &["LEFT", "LEN", "COLOR", "NAME"];
const MEMBER_NAMES_TAKEN: &[&str] = &["PRINT"];

/// The most elements an array may have here (none of the inputs comes near it; measured, 10,000,001 LONGs work).
const MAX_ELEMENTS: i64 = i32::MAX as i64;

impl Checker<'_> {
    // ---- user types ----

    /// Pass 1: the `TYPE` blocks of the main module, so a type may be used above its block (measured,
    /// `v18_h_type_before_block`). A block with an error is not entered; its uses are "not supported yet".
    pub(super) fn declare_types(&mut self, statements: &[Node], skips: &Skips) {
        for &s in statements {
            let Some(b) = ast::TypeBlock::cast(s) else {
                continue;
            };
            if skips.past_cap(s) {
                break;
            }
            self.types_seen.insert(s.key());
            if !skips.usable(s) {
                continue;
            }
            self.stmt_error = false;
            let marks = self.diags.unsupported_count();
            if self.declare_type(b).is_ok() {
                self.type_defs.insert(s.key());
            } else if self.diags.unsupported_count() > marks {
                // Pass 2 starts the follow-on rule when it reaches this block (design D10).
                self.types_marked.insert(s.key());
            }
            self.flush_names();
        }
    }

    fn declare_type(&mut self, b: ast::TypeBlock) -> R<()> {
        let span = b.node().span();
        let header = self.need(b.header(), span)?;
        let name_tok = self.need(header.name(), header.node().span())?;
        let (name, suffix) = self.split_name(name_tok)?;
        let shown = show_bytes(self.text(name_tok.span));
        if suffix.is_some() || name.contains('.') {
            return Err(self.unsupported(name_tok.span, format!("the `TYPE` name `{shown}`")));
        }
        // Measured (`v18_h_type_named_*`, corpus `30_type_udt`): `TYPE Point` and `TYPE cls` are accepted, `TYPE
        // len`, `TYPE print` and `TYPE long` are "Name already in use"; the rule behind them is not known.
        if TYPE_NAMES_TAKEN.contains(&name.as_str()) {
            return Err(self.in_use(name_tok));
        }
        if !TYPE_NAMES_FREE.contains(&name.as_str())
            && (is_keyword(name.as_bytes())
                || TYPE_WORDS.contains(&name.as_str())
                || find_any(name.as_bytes()).next().is_some())
        {
            let msg = format!("a `TYPE` named like a keyword or built-in: `{shown}`");
            return Err(self.unsupported(name_tok.span, msg));
        }
        if self.types_by_name.contains_key(&name) {
            return Err(self.unsupported(name_tok.span, format!("a second `TYPE {shown}`")));
        }
        let mut members: Vec<Member> = Vec::new();
        for field in b.fields() {
            let fspan = field.node().span();
            let a = self.need(field.as_clause(), fspan)?;
            let ty = self.member_type(a)?;
            for f in field.names() {
                let t = self.need(f.name(), f.node().span())?;
                let shown = show_bytes(self.text(t.span));
                if let Some(bounds) = f.bounds() {
                    return Err(self.unsupported(bounds.node().span(), "arrays as `TYPE` members"));
                }
                if let Some(m) = f.modifier() {
                    return Err(self.unsupported(m.span, "`_DYNAMIC`/`_STATIC` members"));
                }
                let (mname, msuffix) = self.split_name(t)?;
                if msuffix.is_some() {
                    // Measured: "Invalid name" and "Expected element-name AS type" (`v18_h_member_decl_suffix*`).
                    return Err(self.error(t.span, format!("a `TYPE` member cannot have a type suffix: `{shown}`")));
                }
                // Measured (`v18_h_member_names*`): `left`, `len`, `color`, `name` are fine, `print` is "Name already
                // in use".
                if MEMBER_NAMES_TAKEN.contains(&mname.as_str()) {
                    return Err(self.in_use(t));
                }
                let free = MEMBER_NAMES_FREE.contains(&mname.as_str());
                if mname.contains('.')
                    || (!free && (is_keyword(mname.as_bytes()) || find_any(mname.as_bytes()).next().is_some()))
                {
                    return Err(self.unsupported(t.span, format!("the member name `{shown}`")));
                }
                if members.iter().any(|m| m.name == mname) {
                    return Err(self.unsupported(t.span, format!("a second member `{shown}`")));
                }
                members.push(Member { name: mname, ty });
            }
        }
        if members.is_empty() {
            return Err(self.unsupported(header.node().span(), "a `TYPE` without members"));
        }
        let id = TypeId(to_u32(self.prog.types.len()));
        self.prog.types.push(UserType {
            name: name.clone(),
            members,
        });
        self.types_by_name.insert(name, id);
        Ok(())
    }

    /// The type of a member: a numeric type other than `_BIT`, a fixed-length string of n bytes (n a number: the
    /// block is read before any `CONST`), or a user type defined in an earlier block. A `_BIT` member is an error
    /// (measured: "Cannot use _BIT inside user defined types", `verification\v21_x25`, `x26`).
    fn member_type(&mut self, a: ast::AsClause) -> R<Ty> {
        let span = a.node().span();
        let words: Vec<String> = a.type_words().map(|t| self.word(t)).collect();
        let text = words.join(" ");
        if let Some(&id) = self.types_by_name.get(&text) {
            return Ok(Ty::User(id));
        }
        let built_in = words.first().is_some_and(|w| w == "_UNSIGNED") || numeric_type(&text).is_some();
        if !built_in && text != "STRING" {
            return Err(self.unsupported(span, format!("the member type `{text}`")));
        }
        let ty = self.type_of_in(a, false)?;
        if ty == Ty::Str {
            Err(self.unsupported(span, "`STRING` members"))
        } else if let Ty::Bit { .. } = ty {
            Err(self.error(span, "a `TYPE` member cannot be a `_BIT`"))
        } else {
            Ok(ty)
        }
    }

    /// A `TYPE` block met in pass 2: one of the main module's was handled in pass 1; one inside a procedure or a
    /// block (accepted by the old compiler inside a SUB, `v18_h_type_in_sub`) is not supported yet.
    pub(super) fn type_block(&mut self, b: ast::TypeBlock, skips: &Skips) {
        let node = b.node();
        if self.types_seen.contains(&node.key()) {
            if self.types_marked.contains(&node.key()) {
                self.follow_on = true;
            }
            return;
        }
        let Some(header) = b.header().map(|h| h.node()) else {
            return;
        };
        if skips.usable(header) {
            self.stmt_error = false;
            let _ = self.unsupported(header.span(), "`TYPE` blocks inside a SUB, FUNCTION or block");
            self.flush_names();
            self.follow_on = true;
        }
    }

    /// The user type an `AS` clause names, if it names one.
    pub(super) fn user_type_of(&self, words: &str) -> Option<Ty> {
        self.types_by_name.get(words).map(|&id| Ty::User(id))
    }

    // ---- arrays ----

    /// `DIM name(bounds) [AS type]`, also `DIM SHARED`, and `REDIM [_PRESERVE] [SHARED] name(bounds) [AS type]`
    /// (`by`; design D3 and D4 of `m2-defxxx-common-arrays`). `ty` is the element type from the suffix, the `AS`
    /// clause or the default type of the name; `typed` says it was written with `AS` (on the name, or before a
    /// list of names), which types the plain name of an array declared here.
    ///
    /// **What an array is, is settled by the first declaration of it in the file** (measured, `study\00` §5): a
    /// static array when that is a `DIM` of the main module with bounds that are all constant by their spelling
    /// ([`Self::dim_bounds`]) and `$DYNAMIC` is not in effect; a dynamic one otherwise. A static array's `DIM` is
    /// no statement. Every other declaration, the first one of a dynamic array and any later one, is an
    /// [`StmtKind::Allocate`] at `stmt`, which gives the bounds when it runs; only a `DIM` with constant bounds of
    /// an array that is static already is a compile error. A procedure's own arrays are not supported yet: there
    /// a `REDIM` reaches a `DIM SHARED` array of the main module.
    #[expect(
        clippy::too_many_arguments,
        reason = "one declaration item and the statement it stands in"
    )]
    pub(super) fn declare_array(
        &mut self,
        typed: bool,
        bounds: ast::ArrayBounds,
        (name_tok, name, suffix, ty): (Tok, String, Option<Ty>, Ty),
        storage: Storage,
        shared: bool,
        by: Declarer,
        stmt: Node,
    ) -> R<()> {
        let shown = show_bytes(self.text(name_tok.span));
        let mode = match by {
            Declarer::Dim | Declarer::Static => AllocMode::Dim,
            Declarer::Redim { preserve: false } => AllocMode::Redim,
            Declarer::Redim { preserve: true } => AllocMode::Preserve,
        };
        let redim = matches!(by, Declarer::Redim { .. });
        if storage != Storage::Main && !redim {
            return Err(self.unsupported(bounds.node().span(), "arrays in a SUB or FUNCTION"));
        }
        if self.procs_by_name.contains_key(&name) {
            // Measured: "Name already in use" (`v18_g_array_and_function`).
            return Err(self.in_use(name_tok));
        }
        if self.visible_const(&name).is_some() {
            return Err(self.unsupported(name_tok.span, format!("an array with a constant's name: `{shown}`")));
        }
        self.reserved(name_tok, &name, suffix)?;
        // Measured (`v09` test 3, `v24_x48`): a `REDIM` without a type takes the type of an array of the name
        // declared with `AS`, by `REDIM` or by `DIM`. With `AS` and another type than that array's it is "Name
        // already in use", between `STRING` and `STRING * n` or two lengths "Type mismatch" (`v24_x39`, `x41`,
        // `x42`).
        let plain_array = self.array_plain_ty(&name);
        let ty = match (redim && suffix.is_none(), typed, plain_array) {
            (true, false, Some(adopted)) => adopted,
            (true, true, Some(other)) if other != ty => {
                if other.is_string() && ty.is_string() {
                    let msg = format!("a `REDIM` cannot change the array `{shown}` between `STRING` and `STRING * n`");
                    return Err(self.error(name_tok.span, msg));
                }
                return Err(self.in_use(name_tok));
            }
            _ => ty,
        };
        if let Some(Ty::FixedStr(_)) = suffix {
            // `DIM a$3(2)` was not measured (`DIM a(2) AS STRING * 3` was).
            return Err(self.unsupported(name_tok.span, format!("an array named with `$n`: `{shown}`")));
        }
        let key = (name.clone(), ty);
        let existing = self.array_of(&key);
        if storage != Storage::Main && existing.is_none() {
            return Err(self.unsupported(bounds.node().span(), "arrays in a SUB or FUNCTION"));
        }
        let dims = self.dim_bounds(bounds, &shown)?;
        let constant: Option<Vec<(i64, i64)>> = dims.iter().map(|d| d.constant).collect();
        let static_bounds = constant.filter(|_| by == Declarer::Dim && !self.dynamic_mode);
        let bounds_span = bounds.node().span();
        let allocate = |array| StmtKind::Allocate {
            array,
            bounds: dims.iter().map(|d| (d.lower.clone(), d.upper.clone())).collect(),
            mode,
        };
        if let Some(id) = existing {
            let var = self.prog.var(id);
            let (count, is_static) = (var.dim_count(), matches!(var.shape, Shape::Static { .. }));
            if let Some(n) = count.filter(|&n| n != dims.len()) {
                // Measured: "Cannot change the number of elements an array has!" (`v24_x40`, `x50`).
                let plural = if n == 1 { "" } else { "s" };
                let msg = format!("`{shown}` has {n} dimension{plural}, not {}", dims.len());
                return Err(self.error(bounds_span, msg));
            }
            if is_static && static_bounds.is_some() {
                // Measured: "Cannot redefine a static array!" (`v18_e_dim_twice`).
                return Err(self.error(name_tok.span, format!("the static array `{shown}` is already declared")));
            }
            // `SHARED` on a later declaration changes nothing: an array that was not shared stays unseen by the
            // procedures (measured, `verification\v24_x248`).
            self.names.push((SymbolKind::Var(id), name_tok.span));
            self.push(stmt, allocate(id));
            return Ok(());
        }
        // A typed array beside a scalar of the name typed alike is measured (`v24_x56`,
        // `v24_b_dim_empty_then_dim`); how a plain array name and another typed scalar or typed array of the name
        // interact was not.
        let plain_scalar = self.scope().plain.get(&name).copied();
        if suffix.is_none() && (plain_array.is_some() || plain_scalar.is_some_and(|t| !(typed && t == ty))) {
            let msg = format!("an array `{shown}` beside a variable or array typed by `DIM … AS`");
            return Err(self.unsupported(name_tok.span, msg));
        }
        let shape = match static_bounds {
            Some(bounds) => {
                let mut count: i64 = 1;
                for &(l, u) in &bounds {
                    count = match (u - l).checked_add(1).and_then(|n| count.checked_mul(n)) {
                        Some(n) if n <= MAX_ELEMENTS => n,
                        _ => return Err(self.unsupported(bounds_span, "an array of more than 2^31 elements")),
                    };
                }
                Shape::Static { bounds }
            }
            None => Shape::Dynamic { dims: Some(dims.len()) },
        };
        let dynamic = matches!(shape, Shape::Dynamic { .. });
        let id = VarId(to_u32(self.prog.vars.len()));
        self.prog.vars.push(Var {
            name: name.clone(),
            ty,
            storage,
            shape,
        });
        self.scope().arrays.insert(key, id);
        if suffix.is_none() && typed {
            self.scope().array_plain.insert(name.clone(), ty);
            if shared {
                self.dim_shared_array_plain.insert(name, ty);
            }
        }
        if shared {
            self.dim_shared.insert(id);
        }
        self.names.push((SymbolKind::Var(id), name_tok.span));
        if dynamic {
            self.push(stmt, allocate(id));
        }
        Ok(())
    }

    /// The bounds of a declaration, one entry per dimension, lower bound before upper as they are evaluated.
    ///
    /// Without `TO` the lower bound is `OPTION BASE` as it stands here; a bound written as the number 0 in plain
    /// digits stays 0 to 0 (measured, `verification\v24_e_option_base`, `v24_x71`–`x83`: the old compiler compares
    /// the text, so `0&`, `(0)`, `0 + 0` and a `CONST` that is 0 do not count).
    ///
    /// **A constant bound is a spelling, not a value** (measured, `verification\v24_b_constant_bounds*`): one
    /// number, a minus before it included, or one `CONST` name, in any number of parentheses. A bound with an
    /// operator is not constant though its value is known (`2 + 3`, `k + 1`, `-k`, `-(1)`, `--1`).
    ///
    /// Measured, each dimension by itself (`qb64pe.bas` 15004–15015): "Invalid array bounds" for a single constant
    /// bound that is negative and for constant `lower TO upper` in the wrong order, in a static `DIM`, a `REDIM`
    /// and below `$DYNAMIC` alike, also beside a dimension whose bounds are not constant (`v18_e_bounds_negative`,
    /// `_reversed`, `v24_x53`–`x55`, `x244`, `x245`); a lower bound that is not constant is not compared (`x246`).
    /// Under `OPTION BASE 1` a single bound that is 0 without being written `0` is accepted: a static array is
    /// then 1 to 0 and the program raises error 5 when it starts (`v24_x71`, `x103`), where the emitter sets the
    /// array up; a dynamic one raises it at the statement.
    fn dim_bounds(&mut self, bounds: ast::ArrayBounds, shown: &str) -> R<Vec<DimBounds>> {
        let mut out = Vec::new();
        for (lower, upper) in bounds.ranges() {
            let upper = self.need(upper, bounds.node().span())?;
            let low = match lower {
                Some(l) => Some(self.bound(l)?),
                None => None,
            };
            let (up, up_value) = self.bound(upper)?;
            let (low, low_value) = match low {
                Some((e, value)) => (e, value),
                None => {
                    let base = if self.is_plain_zero(upper) { 0 } else { self.option_base };
                    let e = Expr {
                        span: up.span,
                        ty: Ty::I64,
                        qb: Ty::I64,
                        kind: ExprKind::Int(base),
                    };
                    (e, Some(base))
                }
            };
            let constant = low_value.zip(up_value);
            if let Some((l, u)) = constant
                && ((lower.is_some() && u < l) || (lower.is_none() && u < 0))
            {
                let span = upper.node().span();
                return Err(self.error(span, format!("invalid bounds for `{shown}`: {l} TO {u}")));
            }
            out.push(DimBounds {
                lower: low,
                upper: up,
                constant,
            });
        }
        Ok(out)
    }

    /// Whether a bound is the number 0 written in plain digits (`0`, `00`): the one spelling that keeps a dimension
    /// 0 to 0 under `OPTION BASE 1`.
    fn is_plain_zero(&self, bound: ast::Expr) -> bool {
        let ast::Expr::Literal(lit) = bound else {
            return false;
        };
        lit.token()
            .is_some_and(|t| self.text(t.span).iter().all(|&b| b == b'0'))
    }

    /// An array bound, converted to `_INTEGER64` (a float rounded half to even, measured: `DIM a(2.5)` is 0 to 2),
    /// and its value when it is constant by its spelling ([`Self::dim_bounds`]): then it is computed here, with
    /// the run-time meaning of its operators.
    fn bound(&mut self, node: ast::Expr) -> R<(Expr, Option<i64>)> {
        let e = self.expr(node)?;
        if e.ty == Ty::Str {
            return Err(self.unsupported(e.span, "a string as an array bound"));
        }
        let span = e.span;
        let spelled_constant = match self.bound_spelling(node) {
            Spelling::Number => true,
            // A `CONST` is a literal in the typed tree; a variable or a call is not.
            Spelling::Name => matches!(e.kind, ExprKind::Int(_) | ExprKind::Float(_)),
            Spelling::Operator => false,
        };
        let e = self.store(e, Ty::I64)?;
        if !spelled_constant {
            return Ok((e, None));
        }
        match constant(&e) {
            Some(Num::Int(v)) => Ok((e, Some(v))),
            Some(Num::Float(_)) | None => {
                Err(self.unsupported(span, "an array bound this large, or this close to a rounding boundary"))
            }
        }
    }

    /// How a bound is written ([`Self::dim_bounds`]).
    fn bound_spelling(&self, node: ast::Expr) -> Spelling {
        match node {
            ast::Expr::Literal(_) => Spelling::Number,
            ast::Expr::NameRef(_) => Spelling::Name,
            ast::Expr::Paren(p) => p.inner().map_or(Spelling::Operator, |inner| self.bound_spelling(inner)),
            // A minus directly before a number belongs to the number (`- 1` too; also before `&H5`, measured,
            // `verification\v24_x249`).
            ast::Expr::Prefix(p) => match (p.op(), p.operand()) {
                (Some(op), Some(ast::Expr::Literal(lit)))
                    if op.kind == SyntaxKind::Minus && lit.token().is_some_and(|t| t.kind == SyntaxKind::Number) =>
                {
                    Spelling::Number
                }
                _ => Spelling::Operator,
            },
            ast::Expr::Bin(_) | ast::Expr::Call(_) | ast::Expr::Field(_) => Spelling::Operator,
        }
    }

    /// The type an array's plain (suffix-less) name was given by a declaration with `AS` that is visible here.
    fn array_plain_ty(&mut self, name: &str) -> Option<Ty> {
        let own = self.scope().array_plain.get(name).copied();
        let shared = if self.cur.is_some() {
            self.dim_shared_array_plain.get(name).copied()
        } else {
            None
        };
        own.or(shared)
    }

    /// The type an array's plain (suffix-less) name means here.
    fn array_key_ty(&mut self, name: &str, suffix: Option<Ty>) -> Ty {
        if let Some(t) = suffix {
            return t;
        }
        self.array_plain_ty(name).unwrap_or_else(|| self.default_ty(name))
    }

    /// The array of a name and element type that is visible here: one of the current scope, or in a procedure a
    /// main-module `DIM SHARED` one declared earlier in the file.
    fn array_of(&mut self, key: &(String, Ty)) -> Option<VarId> {
        self.scope().arrays.get(key).copied().or_else(|| {
            let main = self.main.arrays.get(key).copied();
            main.filter(|v| self.cur.is_some() && self.dim_shared.contains(v))
        })
    }

    /// The array a name refers to here ([`Self::array_of`]).
    pub(super) fn find_array(&mut self, name: &str, suffix: Option<Ty>) -> Option<VarId> {
        let key = (name.to_string(), self.array_key_ty(name, suffix));
        self.array_of(&key)
    }

    /// Whether some array of this name is visible here, of any type.
    fn any_array_named(&self, name: &str) -> bool {
        let scope = if self.cur.is_some() { &self.local } else { &self.main };
        scope.arrays.keys().any(|(n, _)| n == name)
            || (self.cur.is_some()
                && self
                    .main
                    .arrays
                    .iter()
                    .any(|((n, _), v)| n == name && self.dim_shared.contains(v)))
    }

    /// The message for `name(...)` that names no visible array (and no FUNCTION or built-in): the old compiler makes
    /// an implicit array, which is not supported yet (design D4).
    pub(super) fn not_an_array(&mut self, t: Tok, name: &str) -> Failed {
        self.implicit_names.insert(name.to_string());
        let shown = show_bytes(self.text(t.span));
        let msg = if self.any_array_named(name) {
            format!("`{shown}(...)`: an array of another type, used without `DIM` (an implicit array)")
        } else {
            format!("`{shown}(...)` (an array used without `DIM`, or no FUNCTION of this name)")
        };
        self.unsupported(t.span, msg)
    }

    /// The element `array(index, …)` named by a call node: one index per dimension, each converted to `_INTEGER64`
    /// as for an assignment.
    pub(super) fn element(&mut self, call: ast::CallExpr, array: VarId, t: Tok) -> R<Place> {
        let span = call.node().span();
        let args = self.present_args(call.arg_list())?;
        let dims = self
            .prog
            .var(array)
            .dim_count()
            .expect("every array declared so far has its number of dimensions");
        if args.is_empty() {
            // `x()`: a whole array (whole-array assignment, array arguments), deferred (`SOMEDAY.md`).
            let shown = show_bytes(self.text(t.span));
            return Err(self.unsupported(span, format!("the whole array `{shown}()`")));
        }
        if args.len() != dims {
            // Measured: "Cannot change the number of elements an array has!" (`v18_d_index_count_*`).
            let shown = show_bytes(self.text(t.span));
            let plural = if dims == 1 { "" } else { "s" };
            return Err(self.error(
                span,
                format!("`{shown}` has {dims} dimension{plural}, not {}", args.len()),
            ));
        }
        let mut index = Vec::with_capacity(dims);
        for a in args {
            let e = self.expr(a)?;
            if e.ty == Ty::Str {
                // Measured: "Illegal string-number conversion" (`v18_d_index_string`).
                return Err(self.error(e.span, "an array index must be a number"));
            }
            index.push(self.store(e, Ty::I64)?);
        }
        self.names.push((SymbolKind::Var(array), t.span));
        Ok(Place::Element { array, index })
    }

    /// The place an element-shaped call names, for a store or the base of a member: an array element, else "not
    /// supported yet".
    pub(super) fn call_place(&mut self, call: ast::CallExpr) -> R<Place> {
        let span = call.node().span();
        let t = self.need(call.name(), span)?;
        let (name, suffix) = self.split_name(t)?;
        if self.procs_by_name.contains_key(&name) {
            let shown = show_bytes(self.text(t.span));
            return Err(self.unsupported(t.span, format!("`{shown}(...)` here (a FUNCTION of this name exists)")));
        }
        match self.find_array(&name, suffix) {
            Some(a) => self.element(call, a, t),
            None => Err(self.not_an_array(t, &name)),
        }
    }

    // ---- members ----

    /// A dotted name `a.b…` (design D3, measured M8 and `v18_h_dotted_before_dim`): member access when a scalar
    /// variable `a` of a user type is visible here, an error when only an array `a` of a user type is, else
    /// `None` (a plain variable named `a.b…`).
    pub(super) fn dotted(&mut self, t: Tok, name: &str, suffix: Option<Ty>) -> R<Option<Place>> {
        let Some(dot) = name.find('.') else {
            return Ok(None);
        };
        let (head, path) = (&name[..dot], &name[dot + 1..]);
        if let Some(v) = self.lookup_var(head.to_string(), None)
            && matches!(self.prog.var(v).ty, Ty::User(_))
        {
            self.names.push((SymbolKind::Var(v), t.span));
            return self.members(Place::Var(v), path, suffix, t).map(Some);
        }
        if matches!(self.array_key_ty(head, None), Ty::User(_)) && self.find_array(head, None).is_some() {
            // Measured: "Invalid expression" (`v18_h_member_of_scalar`).
            let shown = show_bytes(self.text(t.span));
            return Err(self.error(
                t.span,
                format!("`{shown}`: `{head}` is an array; a member needs an index"),
            ));
        }
        let hidden_main = self.cur.is_some()
            && self
                .main
                .vars
                .keys()
                .any(|(n, ty)| n == head && matches!(ty, Ty::User(_)));
        if hidden_main {
            // In a procedure, beside a main-module `TYPE` variable the procedure does not see: not measured.
            let shown = show_bytes(self.text(t.span));
            let msg = format!("`{shown}` in a SUB or FUNCTION that does not share the `TYPE` variable `{head}`");
            return Err(self.unsupported(t.span, msg));
        }
        Ok(None)
    }

    /// The members `path` (`b` or `b.c`, upper case) of `base`; `suffix` is on the last one, which must be its own
    /// type's (measured: another suffix is "Incorrect symbol after element name").
    fn members(&mut self, mut base: Place, path: &str, suffix: Option<Ty>, t: Tok) -> R<Place> {
        let shown = show_bytes(self.text(t.span));
        for seg in path.split('.') {
            let Ty::User(tid) = self.prog.place_ty(&base) else {
                return Err(self.unsupported(t.span, format!("`{shown}`: a member of a value that is not a `TYPE`")));
            };
            let ut = self.prog.user_type(tid);
            let Some(i) = ut.members.iter().position(|m| m.name == seg) else {
                // Measured: "Element not defined" (`v16_m8_dotted_next_to_type`, `v18_h_unknown_member_element`).
                let msg = format!("`{seg}` is not a member of `TYPE {}` (`{shown}`)", ut.name);
                return Err(self.error(t.span, msg));
            };
            base = Place::Member {
                base: Box::new(base),
                member: MemberId(to_u32(i)),
            };
        }
        if let Some(s) = suffix {
            let m = self.prog.place_ty(&base);
            if let Ty::User(_) = m {
                return Err(self.unsupported(t.span, format!("a type suffix on a `TYPE` member: `{shown}`")));
            }
            if let Ty::FixedStr(_) = m {
                // A suffix on a fixed-length string member was not measured.
                return Err(self.unsupported(t.span, format!("a type suffix on a `STRING * n` member: `{shown}`")));
            }
            if m != s {
                let msg = format!("`{shown}`: the member is {}, not {}", m.qb_name(), s.qb_name());
                return Err(self.error(t.span, msg));
            }
        }
        Ok(base)
    }

    /// `base(…).member…` (a `FieldExpr`).
    pub(super) fn field_place(&mut self, f: ast::FieldExpr) -> R<Place> {
        let span = f.node().span();
        if let Some(list) = f.arg_list() {
            return Err(self.unsupported(list.node().span(), "arrays as `TYPE` members"));
        }
        let base = match self.need(f.base(), span)? {
            ast::Expr::Call(c) => self.call_place(c)?,
            ast::Expr::Field(inner) => self.field_place(inner)?,
            ast::Expr::NameRef(n) => {
                // `a . s`: the old compiler drops the blanks, so this is the name `a.s` (`v19_dot_blanks_*`).
                return Err(self.unsupported(n.node().span(), "blanks around `.` after a name"));
            }
            other @ (ast::Expr::Literal(_) | ast::Expr::Paren(_) | ast::Expr::Prefix(_) | ast::Expr::Bin(_)) => {
                return Err(self.unsupported(other.node().span(), "a member of this expression"));
            }
        };
        if !matches!(self.prog.place_ty(&base), Ty::User(_)) {
            // Measured: "Invalid expression" (`v18_h_member_of_numeric_element`).
            return Err(self.error(span, "only a value of a `TYPE` has members"));
        }
        let t = self.need(f.member(), span)?;
        let (path, suffix) = self.split_name(t)?;
        self.members(base, &path, suffix, t)
    }

    /// The value of a place. A whole user-type value is no value (measured, "User defined types in expressions are
    /// invalid"); as an argument it is not supported yet (a `TYPE` parameter is). A `_BIT` place's value is held in
    /// its storage type and believed the place's `_BIT` type ([`Ty::held_value`]); a `STRING * n` place's value is a
    /// `STRING`.
    pub(super) fn load(&mut self, place: Place, span: Span) -> R<Expr> {
        let ty = self.prog.place_ty(&place);
        if self.len_place == Some(span) && matches!(ty, Ty::User(_)) {
            // `LEN` of a whole `TYPE` place (`check\builtins.rs`): its size, never a value.
            return Ok(Expr {
                span,
                ty,
                qb: ty,
                kind: ExprKind::Load(place),
            });
        }
        if let Ty::User(_) = ty {
            let shown = show_bytes(self.text(span));
            if self.whole_type_arg {
                return Err(self.unsupported(span, format!("a whole `TYPE` value as an argument: `{shown}`")));
            }
            return Err(self.error(span, format!("a whole `TYPE` value cannot be used here: `{shown}`")));
        }
        Ok(Expr {
            span,
            ty: ty.held_value(),
            qb: ty.believed_value(),
            kind: ExprKind::Load(place),
        })
    }

    /// `LBOUND(array[, dimension])` and `UBOUND(…)` (design D4, measured `v18_e_bounds*`, `v18_f_*`).
    pub(super) fn bound_fn(&mut self, call: ast::CallExpr, upper: bool, span: Span) -> R<Expr> {
        let word = if upper { "UBOUND" } else { "LBOUND" };
        // Its row in the supported list gives the arity and the result type (`crate::builtins`).
        let row = crate::builtins::lookup(word, false).expect("LBOUND and UBOUND are supported");
        let crate::builtins::Rule::Fixed(ty) = row.rule else {
            unreachable!("LBOUND and UBOUND have a fixed result type");
        };
        let slots = crate::builtins::slots(row);
        let required = slots.iter().filter(|(_, optional)| !optional).count();
        let args = self.present_args(call.arg_list())?;
        if !(required..=slots.len()).contains(&args.len()) {
            return Err(self.unsupported(span, format!("`{word}` with {} arguments", args.len())));
        }
        let array = match args[0] {
            ast::Expr::NameRef(n) => {
                let t = self.need(n.name(), span)?;
                let (name, suffix) = self.split_name(t)?;
                match self.find_array(&name, suffix) {
                    Some(a) => {
                        self.names.push((SymbolKind::Var(a), t.span));
                        a
                    }
                    None => {
                        // Measured: `LBOUND(x)` of a non-array gives 0, making an implicit array (`v18_f_*`).
                        self.implicit_names.insert(name);
                        let shown = show_bytes(self.text(t.span));
                        let msg = format!("`{word}` of `{shown}`, which is no array (an implicit array)");
                        return Err(self.unsupported(t.span, msg));
                    }
                }
            }
            ast::Expr::Call(c) if c.arg_list().is_some_and(|l| l.args().is_empty()) => {
                // Measured: "Expected ." (`v18_e_bounds_empty_parens`, `v18_e_bounds_type_array`).
                return Err(self.error(c.node().span(), format!("`{word}` takes the array's name without `()`")));
            }
            other @ (ast::Expr::Literal(_)
            | ast::Expr::Call(_)
            | ast::Expr::Field(_)
            | ast::Expr::Paren(_)
            | ast::Expr::Prefix(_)
            | ast::Expr::Bin(_)) => {
                return Err(self.unsupported(other.node().span(), format!("`{word}` of this expression")));
            }
        };
        let dim = match args.get(1) {
            Some(&d) => {
                let e = self.expr(d)?;
                if e.ty == Ty::Str {
                    return Err(self.unsupported(e.span, format!("a string dimension in `{word}`")));
                }
                Some(Box::new(self.store(e, Ty::I32)?))
            }
            None => None,
        };
        Ok(Expr {
            span,
            ty,
            qb: ty,
            kind: ExprKind::Bound { upper, array, dim },
        })
    }
}

/// The numeric type a type name stands for (without `_UNSIGNED`; `_BIT` is one bit).
pub(super) fn numeric_type(words: &str) -> Option<Ty> {
    Some(match words {
        "_BYTE" => Ty::I8,
        "INTEGER" => Ty::I16,
        "LONG" => Ty::I32,
        "_INTEGER64" => Ty::I64,
        "_OFFSET" => Ty::Off,
        "_BIT" => Ty::Bit { width: 1, signed: true },
        "SINGLE" => Ty::F32,
        "DOUBLE" => Ty::F64,
        "_FLOAT" => Ty::F80,
        _ => return None,
    })
}

/// The `_UNSIGNED` form of a signed integer type; `None` for a float (measured: "Type cannot be _UNSIGNED").
pub(super) fn unsigned_of(t: Ty) -> Option<Ty> {
    match t {
        Ty::I8 => Some(Ty::U8),
        Ty::I16 => Some(Ty::U16),
        Ty::I32 => Some(Ty::U32),
        Ty::I64 => Some(Ty::U64),
        Ty::Off => Some(Ty::UOff),
        Ty::Bit { width, .. } => Some(Ty::Bit { width, signed: false }),
        Ty::F32 | Ty::F64 | Ty::F80 => None,
        Ty::U8 | Ty::U16 | Ty::U32 | Ty::U64 | Ty::UOff | Ty::Str | Ty::FixedStr(_) | Ty::User(_) => {
            unreachable!("{t:?} is not a type `numeric_type` gives")
        }
    }
}

/// A value of [`constant`].
#[derive(Clone, Copy, Debug, PartialEq)]
enum Num {
    Int(i64),
    Float(f64),
}

/// 2^53: integers up to this size are exact in `f64`.
const EXACT: f64 = 9_007_199_254_740_992.0;

/// A whole number of magnitude below [`EXACT`] as an integer (the caller checks both).
#[expect(
    clippy::cast_possible_truncation,
    reason = "a whole number below 2^53 converts exactly"
)]
fn whole_to_int(r: f64) -> i64 {
    r as i64
}

/// The value of a typed expression made only of literals and constants, with the meaning its operators have at
/// run time (an array bound is computed by the generated code, not by the `CONST` evaluator: `^` is
/// left-associative here). Only what `f64` computes exactly is taken: integer arithmetic as the folding does it,
/// floats only where every operand is a whole number (or the value is a literal) and the result is exact. `None`
/// otherwise.
fn constant(e: &Expr) -> Option<Num> {
    let whole = |f: f64| f.fract() == 0.0 && f.abs() < EXACT;
    match &e.kind {
        ExprKind::Int(v) => Some(Num::Int(*v)),
        ExprKind::Float(text) => text.parse::<f64>().ok().filter(|f| f.is_finite()).map(Num::Float),
        ExprKind::Convert { how, from } => {
            let v = constant(from)?;
            match (how, v) {
                (ConvKind::Widen | ConvKind::Truncate, Num::Int(i)) => Some(Num::Int(super::ops::wrap(i, e.ty))),
                (ConvKind::RoundEven, Num::Float(f)) => {
                    let r = f.round_ties_even();
                    // Exactly representable bounds only; anything near the 64-bit limits is left to run time.
                    (r.abs() < EXACT).then_some(Num::Int(whole_to_int(r)))
                }
                // A negative `i64` of an unsigned type (`_UNSIGNED _INTEGER64`, `_UNSIGNED _OFFSET`, a wide
                // `_UNSIGNED _BIT`) is a value above 2^63: not exact.
                (ConvKind::Nearest, Num::Int(i)) => {
                    let exact = (i as f64).abs() < EXACT && !(from.ty.is_unsigned() && i < 0);
                    exact.then_some(Num::Float(i as f64))
                }
                (ConvKind::Widen, Num::Float(f)) => Some(Num::Float(f)),
                (ConvKind::Nearest, Num::Float(f)) => whole(f).then_some(Num::Float(f)),
                (ConvKind::RoundEven, Num::Int(_)) | (ConvKind::Truncate, Num::Float(_)) => None,
            }
        }
        ExprKind::Unary { op, operand } => match (op, constant(operand)?) {
            (UnOp::Neg, Num::Float(f)) => Some(Num::Float(-f)),
            (op, Num::Int(i)) => Some(Num::Int(super::ops::fold_unary(*op, i, e.ty))),
            (UnOp::Not | UnOp::Negate, Num::Float(_)) => None,
        },
        ExprKind::Binary { op, lhs, rhs } => match (constant(lhs)?, constant(rhs)?) {
            (Num::Int(a), Num::Int(b)) => super::ops::fold_binary(*op, (a, lhs.ty), (b, rhs.ty), e.ty).map(Num::Int),
            (Num::Float(a), Num::Float(b)) if whole(a) && whole(b) => {
                let r = match op {
                    BinOp::Add => a + b,
                    BinOp::Sub => a - b,
                    BinOp::Mul => a * b,
                    // A quotient is taken when it is a whole number or a half (so rounding it is exact too).
                    BinOp::Div if b != 0.0 && ((2.0 * a) % b) == 0.0 => a / b,
                    BinOp::Pow if b >= 0.0 => a.powf(b),
                    BinOp::Div
                    | BinOp::Pow
                    | BinOp::Eq
                    | BinOp::Ne
                    | BinOp::Lt
                    | BinOp::Gt
                    | BinOp::Le
                    | BinOp::Ge
                    | BinOp::And
                    | BinOp::Or
                    | BinOp::Xor
                    | BinOp::Eqv
                    | BinOp::Imp
                    | BinOp::AndAlso
                    | BinOp::OrElse
                    | BinOp::IDiv
                    | BinOp::Mod => return None,
                };
                (r.abs() < EXACT && (r * 2.0).fract() == 0.0).then_some(Num::Float(r))
            }
            (Num::Int(_) | Num::Float(_), Num::Int(_) | Num::Float(_)) => None,
        },
        ExprKind::Str(_)
        | ExprKind::Load(_)
        | ExprKind::Bound { .. }
        | ExprKind::Concat(..)
        | ExprKind::StrCompare { .. }
        | ExprKind::Call { .. }
        | ExprKind::CallProc { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{Num, constant};
    use crate::{BinOp, ConvKind, Expr, ExprKind, Ty, UnOp};
    use qb64rust_base::{FileId, Span};

    fn e(ty: Ty, kind: ExprKind) -> Expr {
        Expr {
            span: Span::new(FileId(0), 0, 0),
            ty,
            qb: ty,
            kind,
        }
    }

    fn float(t: &str) -> Expr {
        e(Ty::F64, ExprKind::Float(t.into()))
    }

    fn round(x: Expr) -> Expr {
        e(
            Ty::I64,
            ExprKind::Convert {
                how: ConvKind::RoundEven,
                from: Box::new(x),
            },
        )
    }

    fn bin(op: BinOp, ty: Ty, a: Expr, b: Expr) -> Expr {
        e(
            ty,
            ExprKind::Binary {
                op,
                lhs: Box::new(a),
                rhs: Box::new(b),
            },
        )
    }

    #[test]
    fn float_bounds_round_half_to_even() {
        assert_eq!(constant(&round(float("2.5"))), Some(Num::Int(2)));
        assert_eq!(constant(&round(float("3.5"))), Some(Num::Int(4)));
        assert_eq!(constant(&round(float("-0.5"))), Some(Num::Int(0)));
    }

    #[test]
    fn power_is_whole_numbers_only() {
        let two_sq = bin(BinOp::Pow, Ty::F80, float("2"), float("2"));
        let neg = e(
            Ty::F80,
            ExprKind::Unary {
                op: UnOp::Neg,
                operand: Box::new(two_sq),
            },
        );
        assert_eq!(constant(&round(neg)), Some(Num::Int(-4)));
        assert_eq!(constant(&bin(BinOp::Pow, Ty::F80, float("2"), float("0.5"))), None);
    }

    #[test]
    fn division_exact_or_half() {
        assert_eq!(
            constant(&round(bin(BinOp::Div, Ty::F64, float("10"), float("4")))),
            Some(Num::Int(2))
        );
        assert_eq!(constant(&bin(BinOp::Div, Ty::F64, float("10"), float("3"))), None);
    }

    #[test]
    fn integer_operations_fold() {
        let int = |v| e(Ty::I32, ExprKind::Int(v));
        assert_eq!(constant(&bin(BinOp::IDiv, Ty::I32, int(10), int(3))), Some(Num::Int(3)));
        assert_eq!(constant(&bin(BinOp::IDiv, Ty::I32, int(1), int(0))), None);
    }
}
