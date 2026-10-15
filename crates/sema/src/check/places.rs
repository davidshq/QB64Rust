//! User types, arrays and places (design D3–D5 of `m2-arrays-and-types`): `TYPE` blocks (pass 1), `DIM` of static
//! arrays and their bounds, array elements, members and dotted names, `LBOUND`/`UBOUND`. Every rule is measured
//! (`verification\v18_*`, `study\00` §5); what was not measured is "not supported yet".

use super::decl::Declarer;
use super::expr::is_builtin_function;
use super::{Checker, DimFact, Failed, NoJoin, R, Scope, Skips};
use crate::{AllocMode, BinOp, ConvKind, StmtKind, UnOp};
use crate::{ArrayRef, Expr, ExprKind, Member, MemberId, Place, Shape, Storage, SymbolKind, Ty, TypeId, UserType};
use crate::{MEM_TYPE, Var, VarId, mem_type};
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
        // `OPTION BASE` as it stands at the block: the lower bound of a member array's dimension without `TO`.
        let base = self.type_bases.get(&b.node().key()).copied().unwrap_or(0);
        let mut members: Vec<Member> = Vec::new();
        for field in b.fields() {
            let fspan = field.node().span();
            let a = self.need(field.as_clause(), fspan)?;
            let ty = self.member_type(a)?;
            for f in field.names() {
                let t = self.need(f.name(), f.node().span())?;
                let shown = show_bytes(self.text(t.span));
                let dynamic = self.member_markers(b, field, f)?;
                if dynamic {
                    let at = f.markers().chain(field.markers()).next().map_or(t.span, |m| m.span);
                    return Err(self.unsupported(at, "`_DYNAMIC` member arrays"));
                }
                let bounds = match f.bounds() {
                    Some(bounds) => self.member_bounds(bounds, base, &shown)?,
                    None => Vec::new(),
                };
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
                members.push(Member {
                    name: mname,
                    ty,
                    bounds,
                    dynamic,
                });
            }
        }
        if members.is_empty() {
            return Err(self.unsupported(header.node().span(), "a `TYPE` without members"));
        }
        // The bytes of a value, a `STRING` member as its pointer: sizes are kept in 32 bits. A member is below
        // 2^62 bytes (a type and a count of at most 2^31 each); their sum may pass 64 bits.
        let bytes: u64 = members
            .iter()
            .map(|m| layout_bytes(&self.prog.types, m.ty) * u64::from(m.count()))
            .fold(0, u64::saturating_add);
        if bytes > MAX_ELEMENTS as u64 {
            return Err(self.unsupported(header.node().span(), "a `TYPE` of more than 2^31 bytes"));
        }
        let id = TypeId(to_u32(self.prog.types.len()));
        self.prog.types.push(UserType {
            name: name.clone(),
            members,
        });
        self.types_by_name.insert(name, id);
        Ok(())
    }

    /// The markers of one member name (design D2 of `m3-typefields`; `qb64pe.bas` 2213–2304; measured,
    /// `verification\v35_x01`–`x09`, `x44`, `x60`, `v24_x204`, `x205`): those of the field (before a single name,
    /// after the type) and the name's own. A marker needs `$UNSTABLE:TYPEFIELDS` above the block ("_Static/_Dynamic
    /// require $Unstable:TypeFields") and does not stand before an `AS type` list; a member takes one ("A TYPE
    /// member array cannot use both _Dynamic and _Static", also for one written twice), on an array only
    /// ("_Dynamic and _Static can only be used on array members inside TYPE"). True for `_DYNAMIC`; `_STATIC`
    /// changes nothing.
    fn member_markers(&mut self, block: ast::TypeBlock, field: ast::TypeField, name: ast::FieldName) -> R<bool> {
        let mut marks = field.markers().chain(name.markers()).collect::<Vec<_>>();
        marks.sort_by_key(|t| t.span.start);
        let Some(&first) = marks.first() else {
            return Ok(false);
        };
        if !self.type_flags.contains(&block.node().key()) {
            let msg = "`_DYNAMIC` and `_STATIC` members need `$UNSTABLE:TYPEFIELDS` above the `TYPE` block";
            return Err(self.error(first.span, msg));
        }
        // Before an `AS type` list the old compiler reads the marker as the start of `marker name AS type` and
        // `AS` as the name: "Expected element-name AS type, AS type element-list, or END TYPE" (measured,
        // `verification\v35_x60`).
        if let Some(a) = field.as_clause().map(|a| a.node().span())
            && a.start < name.node().span().start
            && let Some(m) = field.markers().find(|m| m.span.start < a.start)
        {
            let msg = "a member's `_DYNAMIC` or `_STATIC` cannot stand before `AS` in an `AS type` list";
            return Err(self.error(m.span, msg));
        }
        if let Some(second) = marks.get(1) {
            return Err(self.error(second.span, "a member takes one of `_DYNAMIC` and `_STATIC`"));
        }
        if name.bounds().is_none() {
            return Err(self.error(first.span, "`_DYNAMIC` and `_STATIC` mark member arrays only"));
        }
        Ok(self.text(first.span).eq_ignore_ascii_case(b"_DYNAMIC"))
    }

    /// The bounds of a member array (design D8 of `m2-defxxx-common-arrays`; measured,
    /// `verification\v24_h_member_arrays`, `v24_x182`, `x183`, `x195`–`x198`, `x217`), one pair per dimension.
    /// `base` is `OPTION BASE` where the block stands: the lower bound of a dimension without `TO`. The number 0
    /// has no rule of its own here (`z(0)` under base 1 is "Invalid array bounds", as bounds in the wrong order
    /// are); nothing between the parentheses is "Array bounds missing".
    ///
    /// A bound is a constant expression. The blocks are read before the program's `CONST`s, so a bound that
    /// names one (accepted by the old compiler, `v24_x196`) cannot be told from a variable (its "TYPE member
    /// array bounds must be numeric constant", `x195`): a bound that is not written with numbers alone is not
    /// supported yet.
    fn member_bounds(&mut self, bounds: ast::ArrayBounds, base: i64, shown: &str) -> R<Vec<(i64, i64)>> {
        let span = bounds.node().span();
        let ranges = bounds.ranges();
        if ranges.is_empty() {
            return Err(self.error(span, format!("the member array `{shown}` needs bounds")));
        }
        let mut out = Vec::new();
        let mut count: i64 = 1;
        for (lower, upper) in ranges {
            let upper = self.need(upper, span)?;
            for bound in lower.into_iter().chain([upper]) {
                if !only_numbers(bound) {
                    let msg = "a bound of a member array that is not written with numbers alone (a `CONST`)";
                    return Err(self.unsupported(bound.node().span(), msg));
                }
            }
            let l = match lower {
                Some(l) => self.constant_bound(l)?,
                None => base,
            };
            let u = self.constant_bound(upper)?;
            if u < l {
                let msg = format!("invalid bounds for `{shown}`: {l} TO {u}");
                return Err(self.error(upper.node().span(), msg));
            }
            count = match u
                .checked_sub(l)
                .and_then(|n| n.checked_add(1))
                .and_then(|n| count.checked_mul(n))
            {
                Some(n) if n <= MAX_ELEMENTS => n,
                _ => return Err(self.unsupported(span, "an array of more than 2^31 elements")),
            };
            out.push((l, u));
        }
        Ok(out)
    }

    /// A bound that must be known when compiling (of a member array, in its `TYPE` block or in a `REDIM` of it),
    /// rounded half to even as any bound is. Measured for one that is not: "TYPE member array bounds must be
    /// numeric constant" (`verification\v24_x195`, `x220`).
    fn constant_bound(&mut self, node: ast::Expr) -> R<i64> {
        let e = self.expr(node)?;
        let span = e.span;
        if e.ty == Ty::Str {
            return Err(self.error(span, "an array bound must be a number"));
        }
        let e = self.store(e, Ty::I64)?;
        match constant(&e) {
            Some(Num::Int(v)) => Ok(v),
            Some(Num::Float(_)) | None => Err(self.error(span, "a bound of a member array must be a constant")),
        }
    }

    /// The type of a member: a numeric type other than `_BIT`, a `STRING` (measured,
    /// `verification\v24_h_string_members`), a fixed-length string of n bytes (n a number: the block is read
    /// before any `CONST`), or a user type defined in an earlier block. A `_BIT` member is an error (measured:
    /// "Cannot use _BIT inside user defined types", `verification\v21_x25`, `x26`).
    fn member_type(&mut self, a: ast::AsClause) -> R<Ty> {
        let span = a.node().span();
        let words: Vec<String> = a.type_words().map(|t| self.word(t)).collect();
        let text = words.join(" ");
        if let Some(t) = self.user_type_of(&text) {
            return Ok(t);
        }
        let built_in = words.first().is_some_and(|w| w == "_UNSIGNED") || numeric_type(&text).is_some();
        if !built_in && text != "STRING" {
            return Err(self.unsupported(span, format!("the member type `{text}`")));
        }
        let ty = self.type_of_in(a, false)?;
        if let Ty::Bit { .. } = ty {
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

    /// The user type an `AS` clause names, if it names one: a `TYPE` block's, or the predefined `_MEM`.
    pub(super) fn user_type_of(&mut self, words: &str) -> Option<Ty> {
        if words == MEM_TYPE {
            return Some(self.mem_ty());
        }
        self.types_by_name.get(words).map(|&id| Ty::User(id))
    }

    /// The predefined type `_MEM` (design D1 of `m3-mem`), added to the program's types the first time a program
    /// names it or a built-in that gives one, so a program without it has the types of its `TYPE` blocks alone.
    /// No `TYPE` block takes its name (`TYPE_WORDS`).
    pub(super) fn mem_ty(&mut self) -> Ty {
        if let Some(&id) = self.types_by_name.get(MEM_TYPE) {
            return Ty::User(id);
        }
        let id = TypeId(to_u32(self.prog.types.len()));
        self.prog.types.push(mem_type());
        self.types_by_name.insert(MEM_TYPE.to_string(), id);
        Ty::User(id)
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
            Declarer::Dim | Declarer::Static | Declarer::Common => AllocMode::Dim,
            Declarer::Redim { mode } => mode,
        };
        let redim = matches!(by, Declarer::Redim { .. });
        // `STATIC a()` and `COMMON a()`: the array is listed, and a `DIM` or `REDIM` below gives it its bounds.
        // What stands between the parentheses of a `COMMON` is not read (measured, `verification\v24_x166`:
        // `COMMON a(3)` allocates nothing).
        let common = by == Declarer::Common;
        let listed = common || bounds.ranges().is_empty();
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
        let bounds_span = bounds.node().span();
        if by == Declarer::Static {
            self.static_array_free(existing, name_tok, &shown, listed)?;
        }
        if common && existing.is_some() {
            // Not measured: `COMMON a()` twice, or below a `DIM` or a use of the array.
            let msg = format!("`COMMON {shown}()` of an array that exists already");
            return Err(self.unsupported(name_tok.span, msg));
        }
        let dims = if common {
            Vec::new()
        } else if redim {
            self.unjoined(NoJoin::RedimBound, |c| c.dim_bounds(bounds, &shown))?
        } else {
            self.dim_bounds(bounds, &shown)?
        };
        let constant: Option<Vec<(i64, i64)>> = dims.iter().map(|d| d.constant).collect();
        // An array that lives for the program may be static: the main module's, a `STATIC` one and every array
        // of a procedure with `STATIC` after its header. A procedure's other arrays are made on each call.
        let lives = matches!(storage, Storage::Main | Storage::Static(_));
        let static_bounds = constant
            .clone()
            .filter(|_| by == Declarer::Dim && lives && !self.dynamic_mode);
        // `STATIC a(bounds)` is no static array: a dynamic one that is allocated when the program starts, below
        // `$DYNAMIC` or not ([`Shape::Dynamic`], `at_start`; read from the old compiler's C++ and measured,
        // `verification\v24_x278`, `v24_b_metacommand_sub`, slice `s56_proc_arrays`).
        let by_static = by == Declarer::Static && !listed;
        let start_bounds = constant.filter(|_| by_static);
        if by_static && start_bounds.is_none() {
            // That allocation stands in the main module's code, where the old compiler writes the bounds as they
            // are: with a parameter its C++ build fails (`verification\v24_x121`), with `2 + 3` it works (`x280`).
            // Bounds that are not constants by their spelling are not compiled yet.
            let msg = format!("`STATIC {shown}(...)` with a bound that is not one number or one constant");
            return Err(self.unsupported(bounds_span, msg));
        }
        let allocate = |array| StmtKind::Allocate {
            array,
            bounds: dims.iter().map(|d| (d.lower.clone(), d.upper.clone())).collect(),
            mode,
            joined: false,
        };
        if let Some(id) = existing {
            let var = self.prog.var(id);
            let (count, is_static) = (var.dim_count(), matches!(var.shape, Shape::Static { .. }));
            // A `DIM` of an array that `STATIC a(n)` declared is an allocation like any other: the array is
            // allocated already, so it raises error 10 when it runs (measured, `verification\v24_x285`).
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
            if count.is_none() {
                // An array listed by `STATIC a()` or `COMMON a()`: its first `DIM` or `REDIM` gives the number
                // of dimensions (measured, `verification\v24_g_local`, `v24_x177`). For an array parameter it
                // is one fact about that number (`v24_x155`).
                self.settle_dims(id, dims.len(), name_tok, bounds_span);
            }
            // `SHARED` on a later declaration changes nothing: an array that was not shared stays unseen by the
            // procedures (measured, `verification\v24_x248`). In a procedure this may be a shared array of the
            // main module: a `DIM` of it there, with constant bounds too, raises error 10 when it runs (`x270`,
            // `x271`).
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
        let shape = match static_bounds.or(start_bounds) {
            Some(bounds) => {
                let mut count: i64 = 1;
                for &(l, u) in &bounds {
                    count = match u
                        .checked_sub(l)
                        .and_then(|n| n.checked_add(1))
                        .and_then(|n| count.checked_mul(n))
                    {
                        Some(n) if n <= MAX_ELEMENTS => n,
                        _ => return Err(self.unsupported(bounds_span, "an array of more than 2^31 elements")),
                    };
                }
                if by_static {
                    Shape::Dynamic {
                        dims: Some(bounds.len()),
                        at_start: Some(bounds),
                    }
                } else {
                    Shape::Static { bounds }
                }
            }
            // `STATIC a()` (measured, `verification\v24_g_local`, `v24_f_implicit_sub`) and `COMMON a()`
            // (`v24_g_common`, `v24_x167`): a dynamic array that lives for the program and is not allocated;
            // what gives it bounds or indexes first gives it its number of dimensions.
            None if listed => Shape::Dynamic {
                dims: None,
                at_start: None,
            },
            None => Shape::Dynamic {
                dims: Some(dims.len()),
                at_start: None,
            },
        };
        // A listed array and one allocated at the start have no allocation where they are declared.
        let dynamic = matches!(shape, Shape::Dynamic { at_start: None, .. }) && !listed;
        let id = VarId(to_u32(self.prog.vars.len()));
        self.prog.vars.push(Var {
            conventional: false,
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

    /// Whether `STATIC name(…)` may declare its array, given the array of that name and type visible here
    /// (`listed`: nothing stands between the parentheses). Measured: "Name already in use" after a `STATIC`, a
    /// `DIM` or a use without `DIM` of it in the procedure (`verification\v24_x122`, `x123`, `x125`, `x269`),
    /// "Array already listed as STATIC" for bounds after `STATIC a()` (`x124`).
    fn static_array_free(&mut self, existing: Option<VarId>, name_tok: Tok, shown: &str, listed: bool) -> R<()> {
        if let Some(id) = existing {
            let var = self.prog.var(id);
            if matches!(var.storage, Storage::Param(_)) {
                return Err(self.in_use(name_tok));
            }
            if var.storage == Storage::Main {
                // Measured (`v24_x272`): the old compiler accepts it and the program ends when it starts
                // ("Duplicate definition").
                let msg = format!("`STATIC {shown}(...)` beside a shared array of the main module");
                return Err(self.unsupported(name_tok.span, msg));
            }
            if var.dim_count().is_none() && !listed {
                let msg = format!("`{shown}` is already listed by `STATIC {shown}()`: `DIM` gives it its bounds");
                return Err(self.error(name_tok.span, msg));
            }
            return Err(self.in_use(name_tok));
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

    /// The array of a name that names no visible array: an array used without `DIM` (design D5 of
    /// `m2-defxxx-common-arrays`; measured, `verification\v24_f_*`, `v24_x105`–`x120`, `x256`–`x263`). `dims` is
    /// the number of indexes written (1 for `LBOUND(name)`), `at` the node that names it.
    ///
    /// The name becomes an array where it may be a variable ([`Self::reserved`]), no SUB or FUNCTION has it and
    /// the spelling calls no built-in function; a `CONST` or a scalar of the name, typed with `AS` or not, does
    /// not stand in the way and does not type it (`x258`, `x259`). The element type is the suffix or the default
    /// type here. Under `OPTION _EXPLICIT` or `OPTION _EXPLICITARRAY`, wherever they stand, it is an error.
    ///
    /// What the array is depends on where the name stands:
    /// - in the main module a static array, `OPTION BASE` to 10 in each dimension, that exists from the start;
    /// - in the main module below `$DYNAMIC` a dynamic array, allocated with those bounds by an
    ///   [`StmtKind::Allocate`] that runs as the first part of the statement that names it first (`joined`);
    /// - in a procedure a local dynamic array that nothing allocates, so that every use raises error 9 (as the
    ///   old compiler does; `DIVERGENCES-QB45.md` Q-009) until a `DIM` or `REDIM` of it in the procedure runs;
    /// - in a procedure with `STATIC` after its header a static array like the main module's that lives for the
    ///   program, and below `$DYNAMIC` a dynamic one that lives for the program and that nothing allocates.
    ///
    /// Where the old compiler's allocation would not be the first thing its statement does, or would be another
    /// kind of allocation, nothing is made and the use is marked "not supported yet" ([`NoJoin`]).
    ///
    /// After a declaration that was marked "not supported yet" (which may have declared this array) nothing is
    /// made: the name keeps the mark of [`Self::not_an_array`].
    pub(super) fn implicit_array(&mut self, t: Tok, name: &str, suffix: Option<Ty>, dims: usize, at: Node) -> R<VarId> {
        let shown = show_bytes(self.text(t.span));
        // A name with one leading underscore is "Invalid array name" in the old compiler (`v24_x112`) unless it
        // calls a built-in, and the table does not know every spelling of those (`_CONNECTIONADDRESS$`, the
        // functions of the auto-included files): it keeps its mark.
        let underscore = name.starts_with('_') && !name.starts_with("__");
        if self.follow_on || underscore || self.procs_by_name.contains_key(name) {
            return Err(self.not_an_array(t, name));
        }
        if is_builtin_function(name, suffix) {
            // Measured for `rnd(1) = 2`: "Expected variable =" (`v24_x119`); the statements written like a store
            // to a function (`ASC(s$, 2) = 65`) are not compiled yet.
            self.implicit_names.insert(name.to_string());
            let msg = format!("`{shown}(...)` as a place or an array (a built-in function's name)");
            return Err(self.unsupported(t.span, msg));
        }
        if let Some(head) = name.split_once('.').map(|(head, _)| head) {
            // `p.m(1)` beside a `TYPE` variable or array `p` that the name does not reach as a member
            // ([`Self::dotted_call`] reads the ones it does): not measured.
            let user = |ty: &Ty| matches!(ty, Ty::User(_));
            let typed = |s: &Scope| {
                s.vars.keys().any(|(n, ty)| n == head && user(ty))
                    || s.arrays.keys().any(|(n, ty)| n == head && user(ty))
            };
            if typed(&self.main) || (self.cur.is_some() && typed(&self.local)) {
                self.implicit_names.insert(name.to_string());
                let msg = format!("`{shown}(...)` beside a `TYPE` variable or array `{head}`");
                return Err(self.unsupported(t.span, msg));
            }
        }
        if dims == 0 {
            // `x()`, a whole array, where it is no argument for an array parameter ([`Self::array_arg`]).
            self.implicit_names.insert(name.to_string());
            return Err(self.unsupported(at.span(), format!("the whole array `{shown}()`")));
        }
        self.reserved(t, name, suffix)?;
        let ty = self.array_key_ty(name, suffix);
        if self.explicit_array {
            // Measured: "Array 'z' (SINGLE) not defined" (`v24_x105`, `x256`, `x257`).
            let msg = format!(
                "array `{shown}` ({}) is not declared (`OPTION _EXPLICIT` or `OPTION _EXPLICITARRAY`)",
                ty.qb_name()
            );
            return Err(self.error(t.span, msg));
        }
        if self.no_join == Some(NoJoin::RedimBound) {
            // Measured (`v24_x266`, `x268`): dynamic wherever it stands, and allocated as the `REDIM` allocates.
            self.implicit_names.insert(name.to_string());
            let msg = format!("`{shown}(...)`: an array first used in a bound of `REDIM`, without `DIM`");
            return Err(self.unsupported(t.span, msg));
        }
        let base = self.option_base;
        // In a procedure with `STATIC` after its header the array lives for the program: a static one that
        // works, or below `$DYNAMIC` a dynamic one that nothing allocates (measured, `v24_f_implicit_sub`,
        // `v24_x120`).
        let static_proc = self.cur.filter(|_| self.static_scope);
        let (storage, shape) = match (self.cur, static_proc) {
            (Some(p), None) => (
                Storage::Local(p),
                Shape::Dynamic {
                    dims: Some(dims),
                    at_start: None,
                },
            ),
            (_, Some(p)) if self.dynamic_mode => (
                Storage::Static(p),
                Shape::Dynamic {
                    dims: Some(dims),
                    at_start: None,
                },
            ),
            (None, _) if self.dynamic_mode => (
                Storage::Main,
                Shape::Dynamic {
                    dims: Some(dims),
                    at_start: None,
                },
            ),
            (_, owner) => {
                let count = u32::try_from(dims).ok().and_then(|n| (11 - base).checked_pow(n));
                if count.is_none_or(|n| n > MAX_ELEMENTS) {
                    self.implicit_names.insert(name.to_string());
                    return Err(self.unsupported(at.span(), "an array of more than 2^31 elements"));
                }
                let bounds = vec![(base, 10); dims];
                (owner.map_or(Storage::Main, Storage::Static), Shape::Static { bounds })
            }
        };
        let allocated_here = self.cur.is_none() && self.dynamic_mode;
        // The allocation runs where the name is evaluated, which is the start of the statement only when the
        // statement writes its code after evaluating all its parts ([`NoJoin`]). Measured for the line of a
        // block (`v24_x262`): before an `IF`, a `FOR`, a `WHILE`, a `DO WHILE` and a `SELECT CASE`, inside the
        // block for an `ELSEIF` (not reached: not allocated) and a `LOOP UNTIL` (on every round); what an error
        // in it skips there was not measured.
        let place = match self.no_join {
            Some(NoJoin::Header) if allocated_here => Some("the line of a block (`IF`, `FOR`, `DO`, …)"),
            Some(NoJoin::Item) if allocated_here => {
                Some("a later item of `PRINT` or `READ`, or an item of `WRITE`, `PRINT #` or `INPUT`")
            }
            _ => None,
        };
        if let Some(place) = place {
            self.implicit_names.insert(name.to_string());
            let msg = format!("`{shown}(...)`: an array below `$DYNAMIC` first used in {place}, without `DIM`");
            return Err(self.unsupported(t.span, msg));
        }
        let id = VarId(to_u32(self.prog.vars.len()));
        self.prog.vars.push(Var {
            conventional: false,
            name: name.to_string(),
            ty,
            storage,
            shape,
        });
        self.scope().arrays.insert((name.to_string(), ty), id);
        if allocated_here {
            let bound = |v: i64| Expr {
                span: t.span,
                ty: Ty::I64,
                qb: Ty::I64,
                kind: ExprKind::Int(v),
            };
            let kind = StmtKind::Allocate {
                array: id,
                bounds: vec![(bound(base), bound(10)); dims],
                mode: AllocMode::Dim,
                joined: true,
            };
            self.push(at, kind);
        }
        Ok(id)
    }

    /// The message for `name(...)` that names no visible array where none is made for it
    /// ([`Self::implicit_array`]): after a declaration that was marked, for a name with one leading underscore
    /// and for the name of a SUB or FUNCTION.
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

    /// The number of dimensions of an array. One listed by `STATIC a()` that no `DIM` or `REDIM` has given bounds
    /// yet gets `first` here: the number of indexes of the use that names it first, 1 for `LBOUND` and `UBOUND`
    /// (measured, `verification\v24_x276`: a later use with another number is the compile error).
    ///
    /// An array parameter, and a listed array that was passed to one before it had a number, get theirs when
    /// every body is checked ([`Self::open_dims`], [`Checker::solve_dims`]): `first` is then one fact about it,
    /// written at `at` with the name `t`, and is what this use goes by.
    fn settle_dims(&mut self, array: VarId, first: usize, t: Tok, at: Span) -> usize {
        if let Some(n) = self.prog.var(array).dim_count() {
            return n;
        }
        if self.open_dims(array) {
            self.dim_facts.push(DimFact::Count {
                array,
                shown: show_bytes(self.text(t.span)),
                count: first,
                span: at,
                passed: false,
            });
            return first;
        }
        self.prog.vars[array.0 as usize].shape = Shape::Dynamic {
            dims: Some(first),
            at_start: None,
        };
        first
    }

    /// Whether an array's number of dimensions is found from everything that names it, after the bodies: an
    /// array parameter, and a listed array passed to one while it had no number.
    pub(super) fn open_dims(&self, array: VarId) -> bool {
        matches!(self.prog.var(array).storage, Storage::Param(_)) || self.dims_open.contains(&array)
    }

    /// The element `array(index, …)` named by a call node: one index per dimension, each converted to `_INTEGER64`
    /// as for an assignment.
    pub(super) fn element(&mut self, call: ast::CallExpr, array: VarId, t: Tok) -> R<Place> {
        let span = call.node().span();
        let args = self.present_args(call.arg_list())?;
        if args.is_empty() {
            // `x()`: a whole array (whole-array assignment, array arguments), deferred (`SOMEDAY.md`).
            let shown = show_bytes(self.text(t.span));
            return Err(self.unsupported(span, format!("the whole array `{shown}()`")));
        }
        self.element_at(array, args, t, span)
    }

    /// The element of `array` at the indexes `args` (at least one), written at `span` with the name `t`.
    fn element_at(&mut self, array: VarId, args: Vec<ast::Expr>, t: Tok, span: Span) -> R<Place> {
        let dims = self.settle_dims(array, args.len(), t, span);
        let index = self.indexes(args, dims, t, span)?;
        self.names.push((SymbolKind::Var(array), t.span));
        Ok(Place::Element {
            array: ArrayRef::Var(array),
            index,
        })
    }

    /// The indexes of an element of an array of `dims` dimensions, each converted to `_INTEGER64`.
    fn indexes(&mut self, args: Vec<ast::Expr>, dims: usize, t: Tok, span: Span) -> R<Vec<Expr>> {
        if args.len() != dims {
            // Measured: "Cannot change the number of elements an array has!" (`v18_d_index_count_*`; for a
            // member array `verification\v24_x192`).
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
                // Measured: "Illegal string-number conversion" (`v18_d_index_string`, `v24_x194`).
                return Err(self.error(e.span, "an array index must be a number"));
            }
            index.push(self.store(e, Ty::I64)?);
        }
        Ok(index)
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
        if let Some(place) = self.dotted_call(call, t, &name, suffix)? {
            return Ok(place);
        }
        let array = match self.find_array(&name, suffix) {
            Some(a) => a,
            None => {
                let dims = call.arg_list().map_or(0, |l| l.args().len());
                self.implicit_array(t, &name, suffix, dims, call.node())?
            }
        };
        self.element(call, array, t)
    }

    // ---- members ----

    /// A dotted name `a.b…` (design D3, measured M8 and `v18_h_dotted_before_dim`): member access when a scalar
    /// variable `a` of a user type is visible here, an error when only an array `a` of a user type is, else
    /// `None` (a plain variable named `a.b…`). A member array named without an index is the error (measured:
    /// "Expected array index after v", `verification\v24_x193`).
    pub(super) fn dotted(&mut self, t: Tok, name: &str, suffix: Option<Ty>) -> R<Option<Place>> {
        match self.dotted_any(t, name, suffix)? {
            Some(place) => self.member_element(place, None, t.span, t).map(Some),
            None => Ok(None),
        }
    }

    /// `a.b…(index, …)`, a call node whose name is a dotted name that [`Self::dotted`] reads as member access:
    /// an element of the member array `a.b…`. `None` for any other name.
    pub(super) fn dotted_call(
        &mut self,
        call: ast::CallExpr,
        t: Tok,
        name: &str,
        suffix: Option<Ty>,
    ) -> R<Option<Place>> {
        let Some(place) = self.dotted_any(t, name, suffix)? else {
            return Ok(None);
        };
        let span = call.node().span();
        self.member_element(place, Some(call.arg_list()), span, t).map(Some)
    }

    /// The member a place names: of a [`Place::Member`], its member.
    fn member_of(&self, place: &Place) -> Option<&Member> {
        match place {
            Place::Member { base, member } => Some(self.prog.member(self.prog.place_ty(base), *member)),
            Place::Var(_) | Place::Element { .. } => None,
        }
    }

    /// Whether a place that [`Self::members`] gave names a member array as a whole. Such a place is no place of
    /// the typed tree: it becomes an element ([`Self::member_element`]) or an [`ArrayRef::Member`].
    pub(super) fn is_member_array(&self, place: &Place) -> bool {
        self.member_of(place).is_some_and(Member::is_array)
    }

    /// What stands after a member that [`Self::members`] gave: for a member array its indexes (`args`: the
    /// parentheses written after it, `Some(None)` for a call node without any), which make an element; for any
    /// other member nothing. Measured (`verification\v24_x192`–`x194`, `x318`, `x326`): a member array without
    /// an index is "Expected array index after v", with empty parentheses "Expected array index", another
    /// number of indexes than its dimensions "Cannot change the number of elements an array has!", a string
    /// index "Illegal string-number conversion"; an index after a member that is no array "Element is not an
    /// array".
    fn member_element(&mut self, place: Place, args: Option<Option<ast::ArgList>>, span: Span, t: Tok) -> R<Place> {
        let shown = show_bytes(self.text(t.span));
        let Some(dims) = self.member_of(&place).filter(|m| m.is_array()).map(|m| m.bounds.len()) else {
            return match args {
                None => Ok(place),
                Some(_) => Err(self.error(span, format!("`{shown}` is a member that is no array: it has no index"))),
            };
        };
        let Some(list) = args else {
            return Err(self.error(
                span,
                format!("`{shown}` is a member array: an index is expected after it"),
            ));
        };
        let args = self.present_args(list)?;
        if args.is_empty() {
            // `t.v()`: the whole member array, where it is no array argument, no target of `ERASE` and no
            // argument of `LBOUND` or `UBOUND` ([`Self::member_array`]).
            let msg = format!("`{shown}()` is a member array: an index is expected between the parentheses");
            return Err(self.error(span, msg));
        }
        let index = self.indexes(args, dims, t, span)?;
        let Place::Member { base, member } = place else {
            unreachable!("a member array is a member");
        };
        Ok(Place::Element {
            array: ArrayRef::Member { base, member },
            index,
        })
    }

    /// The member array an expression names as a whole, where one may stand (the argument of `LBOUND` and
    /// `UBOUND`, a target of `ERASE`, an argument for an array parameter): `t.v`, `a(1).v`, `x(0).n(1).q`, each
    /// also with empty parentheses after it (the second value; measured for `LBOUND`, `verification\v24_x201`,
    /// `x311`, and for `ERASE`, `x320`). `None` for an expression of another shape and for a name that is no
    /// member access. A member that is no array is the error (measured for `LBOUND`: "Element is not an array",
    /// `x321`).
    pub(super) fn member_array(&mut self, node: ast::Expr) -> R<Option<(ArrayRef, bool, Tok)>> {
        let span = node.node().span();
        let (place, parens, t) = match node {
            ast::Expr::NameRef(n) => {
                let t = self.need(n.name(), span)?;
                let (name, suffix) = self.split_name(t)?;
                match self.dotted_any(t, &name, suffix)? {
                    Some(place) => (place, false, t),
                    None => return Ok(None),
                }
            }
            ast::Expr::Call(c) if c.arg_list().is_some_and(|l| l.args().is_empty()) => {
                let t = self.need(c.name(), span)?;
                let (name, suffix) = self.split_name(t)?;
                match self.dotted_any(t, &name, suffix)? {
                    Some(place) => (place, true, t),
                    None => return Ok(None),
                }
            }
            ast::Expr::Field(f) if f.arg_list().is_none_or(|l| l.args().is_empty()) => {
                let (place, t) = self.field_member(f)?;
                (place, f.arg_list().is_some(), t)
            }
            ast::Expr::Literal(_)
            | ast::Expr::Call(_)
            | ast::Expr::Field(_)
            | ast::Expr::Paren(_)
            | ast::Expr::Prefix(_)
            | ast::Expr::Bin(_) => return Ok(None),
        };
        if !self.is_member_array(&place) {
            let shown = show_bytes(self.text(t.span));
            return Err(self.error(span, format!("`{shown}` is a member that is no array")));
        }
        let Place::Member { base, member } = place else {
            unreachable!("a member array is a member");
        };
        Ok(Some((ArrayRef::Member { base, member }, parens, t)))
    }

    /// [`Self::dotted`] without its last step: the member the name reaches, which may be a member array named
    /// as a whole.
    pub(super) fn dotted_any(&mut self, t: Tok, name: &str, suffix: Option<Ty>) -> R<Option<Place>> {
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
    /// type's (measured: another suffix is "Incorrect symbol after element name"). The last one may be a member
    /// array, which the caller gives its index ([`Self::member_element`]); one before it is the error of a member
    /// array without an index.
    fn members(&mut self, mut base: Place, path: &str, suffix: Option<Ty>, t: Tok) -> R<Place> {
        let shown = show_bytes(self.text(t.span));
        for seg in path.split('.') {
            if let Some(m) = self.member_of(&base).filter(|m| m.is_array()) {
                let msg = format!("`{shown}`: an index is expected after the member array `{}`", m.name);
                return Err(self.error(t.span, msg));
            }
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

    /// `base(…).member…` (a `FieldExpr`), with the index of a member array after it (`a(1).v(2)`).
    pub(super) fn field_place(&mut self, f: ast::FieldExpr) -> R<Place> {
        let span = f.node().span();
        let (place, t) = self.field_member(f)?;
        self.member_element(place, f.arg_list().map(Some), span, t)
    }

    /// The member a `FieldExpr` names, without what stands in parentheses after it, and the member's token.
    pub(super) fn field_member(&mut self, f: ast::FieldExpr) -> R<(Place, Tok)> {
        let span = f.node().span();
        let base = match self.need(f.base(), span)? {
            // A member of a block a built-in gives (`_MEMNEW(4).SIZE`): measured, "Invalid expression"
            // (`verification\v32_x02`, `x114`).
            ast::Expr::Call(c)
                if c.name()
                    .is_some_and(|t| crate::builtins::MEM_RESULTS.contains(&self.word(t).as_str())) =>
            {
                return Err(self.error(
                    span,
                    "a member of a block a built-in gives cannot be read: store it first",
                ));
            }
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
        let place = self.members(base, &path, suffix, t)?;
        Ok((place, t))
    }

    /// `REDIM [_PRESERVE | _RETAIN] path(bounds) [AS type]` where the path names a member array (`t.v`, `a(1).v`,
    /// `x(0).n(1).q`; design D8 of `m2-defxxx-common-arrays`). Returns false for an item that names none: a
    /// plain array.
    ///
    /// The bounds of a member array never change (measured, `verification\v24_x184`–`x190`, `x218`–`x223`,
    /// `x312`): the statement is accepted only with constant bounds equal to the declared ones, written `lower
    /// TO upper`, or the upper bound alone with `AS` and the member's type. It then clears the elements
    /// ([`StmtKind::Clear`]); with `_PRESERVE` or `_RETAIN` (`keeps`) it does nothing at all, and the indexes of
    /// its path are not evaluated (measured, `verification\v34_x09`, `x10`). The compile errors: other bounds, also
    /// the same number of elements shifted ("Cannot change the number of elements an array has!"), a bound that is
    /// not constant ("TYPE member array bounds must be numeric constant"), the upper bound alone without `AS`
    /// ("REDIM of a TYPE member array must include AS <type> or explicit lower TO upper bounds"), another type
    /// ("REDIM type does not match the declared TYPE member array").
    ///
    /// `alone`: the item is the only one of its statement. `listed`: the type stands before the names.
    pub(super) fn redim_member(
        &mut self,
        item: ast::DimItem,
        keeps: bool,
        (listed, alone): (bool, bool),
        stmt: Node,
    ) -> R<bool> {
        let span = item.node().span();
        let parts = item.path();
        let Some(&(first, first_parens)) = parts.first() else {
            return Ok(false);
        };
        if parts.len() == 1 && !self.text(first.span).contains(&b'.') {
            return Ok(false);
        }
        let (name, suffix) = self.split_name(first)?;
        let mut index_used = false;
        let mut place = match self.dotted_any(first, &name, suffix)? {
            Some(place) => place,
            None if parts.len() == 1 => return Ok(false),
            // `a(i).member…`: an element of an array of a user type.
            None => {
                let shown = show_bytes(self.text(first.span));
                let Some(array) = self.find_array(&name, suffix) else {
                    return Err(self.unsupported(first.span, format!("a member of `{shown}`, which is no array here")));
                };
                let Some(index) = first_parens else {
                    return Err(self.error(first.span, format!("`{shown}` is an array; a member needs an index")));
                };
                index_used = true;
                let args = self.path_index(index)?;
                self.element_at(array, args, first, index.node().span())?
            }
        };
        let last = parts.len() - 1;
        let mut bounds = None;
        for (k, &(t, parens)) in parts.iter().enumerate() {
            if k > 0 {
                if !matches!(self.prog.place_ty(&place), Ty::User(_)) && !self.is_member_array(&place) {
                    return Err(self.error(t.span, "only a value of a `TYPE` has members"));
                }
                let (path, suffix) = self.split_name(t)?;
                place = self.members(place, &path, suffix, t)?;
            }
            let parens = parens.filter(|_| !(k == 0 && index_used));
            if k == last {
                bounds = parens;
                break;
            }
            // An index on the way: of a member array, which the next member is taken of.
            if let Some(index) = parens {
                let index_span = index.node().span();
                if !self.is_member_array(&place) {
                    let shown = show_bytes(self.text(t.span));
                    let msg = format!("`{shown}` is a member that is no array: it has no index");
                    return Err(self.error(index_span, msg));
                }
                let args = self.path_index(index)?;
                let dims = self.member_of(&place).map_or(0, |m| m.bounds.len());
                let index = self.indexes(args, dims, t, index_span)?;
                let Place::Member { base, member } = place else {
                    unreachable!("a member array is a member");
                };
                place = Place::Element {
                    array: ArrayRef::Member { base, member },
                    index,
                };
            }
        }
        let (last_tok, _) = parts[last];
        let shown = show_bytes(self.text(last_tok.span));
        let Some((declared, member_ty)) = self
            .member_of(&place)
            .filter(|m| m.is_array())
            .map(|m| (m.bounds.clone(), m.ty))
        else {
            // Not measured: a `REDIM` of a member that is a single value.
            return Err(self.unsupported(span, format!("`REDIM` of `{shown}`, a member that is no array")));
        };
        let Some(bounds) = bounds else {
            return Err(self.unsupported(span, format!("`REDIM` of the member array `{shown}` without bounds")));
        };
        if listed {
            // Measured: "Unknown type" (`verification\v24_x325`). `SHARED` changes nothing (`x324`).
            let msg = format!("the member array `{shown}` cannot stand in a `REDIM` with the type before the names");
            return Err(self.error(span, msg));
        }
        let as_ty = match item.as_clause() {
            Some(a) => Some((self.type_of(a)?, a.node().span())),
            None => None,
        };
        if let Some((ty, at)) = as_ty
            && ty != member_ty
        {
            let msg = format!(
                "the member array `{shown}` is declared {}: a `REDIM` cannot give it another type",
                self.prog.type_name(member_ty)
            );
            return Err(self.error(at, msg));
        }
        let bounds_span = bounds.node().span();
        let mut given = Vec::new();
        for (lower, upper) in bounds.ranges() {
            let upper = self.need(upper, bounds_span)?;
            let l = match lower {
                Some(l) => self.constant_bound(l)?,
                None if as_ty.is_none() => {
                    let msg = format!(
                        "a `REDIM` of the member array `{shown}` names its bounds as `lower TO upper`, or its type \
                         with `AS`"
                    );
                    return Err(self.error(upper.node().span(), msg));
                }
                // Not measured: whether the number 0 keeps its rule of a `DIM` here (0 to 0 under base 1).
                None if self.option_base != 0 && self.is_plain_zero(upper) => {
                    let msg = format!("`REDIM {shown}(0)` of a member array under `OPTION BASE 1`");
                    return Err(self.unsupported(upper.node().span(), msg));
                }
                // The upper bound alone goes by `OPTION BASE` as it stands at the `REDIM` (measured,
                // `verification\v24_x322`, `x323`).
                None => self.option_base,
            };
            let u = self.constant_bound(upper)?;
            given.push((l, u));
        }
        if given != declared {
            let text: Vec<String> = declared.iter().map(|(l, u)| format!("{l} TO {u}")).collect();
            let msg = format!(
                "the bounds of the member array `{shown}` are ({}): a `REDIM` cannot change them",
                text.join(", ")
            );
            return Err(self.error(bounds_span, msg));
        }
        if keeps {
            return Ok(true);
        }
        let Place::Member { base, member } = place else {
            unreachable!("a member array is a member");
        };
        let array = ArrayRef::Member { base, member };
        self.clear_member_array(array, alone, span, stmt)?;
        Ok(true)
    }

    /// The indexes written in the parentheses of a part of a `REDIM` path (`(1)` in `a(1).v(0 TO 3)`), which the
    /// parser reads as bounds.
    fn path_index<'t>(&mut self, index: ast::ArrayBounds<'t>) -> R<Vec<ast::Expr<'t>>> {
        let span = index.node().span();
        let mut out = Vec::new();
        for (lower, upper) in index.ranges() {
            if lower.is_some() {
                return Err(self.error(span, "an index cannot be a range (`TO`)"));
            }
            out.push(self.need(upper, span)?);
        }
        if out.is_empty() {
            return Err(self.error(span, "an index is expected between the parentheses"));
        }
        Ok(out)
    }

    /// The clear of a member array, by `ERASE` or `REDIM` (`alone`: it is the only target of its statement).
    /// Finding the array may raise when an index stands on its way, and where the old compiler's statement goes
    /// on with its other targets before the error is handled (measured, `verification\v24_x312`), each clear
    /// here is a statement of its own: such a target beside others is not supported yet.
    pub(super) fn clear_member_array(&mut self, array: ArrayRef, alone: bool, span: Span, stmt: Node) -> R<()> {
        if array.may_raise() && !alone {
            let msg = "a member array of an array element in a list of `ERASE` or `REDIM` targets";
            return Err(self.unsupported(span, msg));
        }
        self.push(stmt, StmtKind::Clear { array });
        Ok(())
    }

    /// The place of a user type that an expression names where a whole value is taken (the right side of `q = p`,
    /// the argument for a parameter of a user type): a variable, an element of an array of a user type, or a
    /// member. `None` for anything else (a number, a call, an expression in parentheses), which is then typed as
    /// an expression by the caller. A member of another type comes back as its place.
    pub(super) fn whole_place(&mut self, node: ast::Expr) -> R<Option<Place>> {
        let span = node.node().span();
        match node {
            ast::Expr::NameRef(n) => {
                let t = self.need(n.name(), span)?;
                let (name, suffix) = self.split_var_name(t)?;
                if let Some(place) = self.dotted(t, &name, suffix)? {
                    return Ok(Some(place));
                }
                let var = self
                    .lookup_var(name, suffix)
                    .filter(|&v| matches!(self.prog.var(v).ty, Ty::User(_)));
                Ok(var.map(|v| {
                    self.names.push((SymbolKind::Var(v), t.span));
                    Place::Var(v)
                }))
            }
            ast::Expr::Call(c) => {
                let t = self.need(c.name(), span)?;
                let (name, suffix) = self.split_name(t)?;
                if let Some(place) = self.dotted_call(c, t, &name, suffix)? {
                    return Ok(Some(place));
                }
                match self.find_array(&name, suffix) {
                    Some(a) if matches!(self.prog.var(a).ty, Ty::User(_)) => self.element(c, a, t).map(Some),
                    Some(_) | None => Ok(None),
                }
            }
            ast::Expr::Field(f) => self.field_place(f).map(Some),
            ast::Expr::Literal(_) | ast::Expr::Paren(_) | ast::Expr::Prefix(_) | ast::Expr::Bin(_) => Ok(None),
        }
    }

    /// The value of a place. A whole user-type value is no value (measured, "User defined types in expressions are
    /// invalid", also as an argument for a parameter that is no user type, `verification\v24_x214`). A `_BIT`
    /// place's value is held in its storage type and believed the place's `_BIT` type ([`Ty::held_value`]); a
    /// `STRING * n` place's value is a `STRING`.
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
            return Err(self.error(span, format!("a whole `TYPE` value cannot be used here: `{shown}`")));
        }
        Ok(Expr {
            span,
            ty: ty.held_value(),
            qb: ty.believed_value(),
            kind: ExprKind::Load(place),
        })
    }

    /// `LBOUND(array[, dimension])` and `UBOUND(…)` (design D4, measured `v18_e_bounds*`, `v18_f_*`). The array
    /// may be a member array, also written with empty parentheses (`verification\v24_h_member_arrays`,
    /// `v24_x201`, `x311`).
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
        let array = match (self.member_array(args[0])?, args[0]) {
            (Some((array, _, _)), _) => array,
            (None, ast::Expr::NameRef(n)) => {
                let t = self.need(n.name(), span)?;
                let (name, suffix) = self.split_name(t)?;
                let a = match self.find_array(&name, suffix) {
                    Some(a) => a,
                    // Measured: `LBOUND(x)` of a name that is no array makes an array of one dimension, used
                    // without `DIM` (`v18_f_*`, `v24_f_implicit`).
                    None => self.implicit_array(t, &name, suffix, 1, n.node())?,
                };
                // The bounds of an array parameter say nothing about its number of dimensions (measured,
                // `verification\v24_x153`, `x293`), nor do those of an array listed by `COMMON a()` (`x298`,
                // `x301`, `x302`); those of one listed by `STATIC a()` make it one (`x282`, `x297`).
                let common = self.prog.var(a).storage == Storage::Main;
                if !self.open_dims(a) && !common {
                    self.settle_dims(a, 1, t, span);
                }
                self.names.push((SymbolKind::Var(a), t.span));
                ArrayRef::Var(a)
            }
            (None, ast::Expr::Call(c)) if c.arg_list().is_some_and(|l| l.args().is_empty()) => {
                // Measured: "Expected ." (`v18_e_bounds_empty_parens`, `v18_e_bounds_type_array`).
                return Err(self.error(c.node().span(), format!("`{word}` takes the array's name without `()`")));
            }
            (
                None,
                other @ (ast::Expr::Literal(_)
                | ast::Expr::Call(_)
                | ast::Expr::Field(_)
                | ast::Expr::Paren(_)
                | ast::Expr::Prefix(_)
                | ast::Expr::Bin(_)),
            ) => {
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

    /// `VARPTR(a(i))` and `VARSEG(a(i))` (`segment`) (design D3 and D4 of `m3-get-put-bload`; measured,
    /// `verification\v28_b_bload`, `v28_d_varptr_index`, `v28_e_cmem_arrays`, `v28_x36`–`x41`). The argument is
    /// a variable or an element, never a value. An array of the main module with numeric elements, static or
    /// dynamic, named from any body, is put into conventional memory by being named here; every other kind of
    /// variable needs another allocation and is not supported yet.
    pub(super) fn var_address(&mut self, call: ast::CallExpr, segment: bool, span: Span) -> R<Expr> {
        let word = if segment { "VARSEG" } else { "VARPTR" };
        let args = self.present_args(call.arg_list())?;
        let &[arg] = &args[..] else {
            // Measured: "Incorrect number of arguments" (`v28_x39`).
            return Err(self.error(span, format!("`{word}` takes 1 argument")));
        };
        // An element or variable of a user type is a place here, not a value, as for `LEN` ([`Self::load`]).
        let outer = self.len_place.replace(arg.node().span());
        let e = self.expr(arg);
        self.len_place = outer;
        let e = e?;
        let ExprKind::Load(place) = e.kind else {
            // Measured: "Expected reference to a variable/array" for a literal, an expression, a function call
            // (`v28_x36`, `x37`, `x40`, `x41`).
            return Err(self.error(e.span, format!("`{word}` needs a variable or an array element")));
        };
        let Place::Element {
            array: ArrayRef::Var(array),
            index,
        } = place
        else {
            let what = match place {
                Place::Var(_) => "a variable that is no array element",
                Place::Element { .. } | Place::Member { .. } => "a member of a `TYPE`",
            };
            return Err(self.unsupported(e.span, format!("`{word}` of {what}")));
        };
        let v = self.prog.var(array);
        let what = match (v.storage, v.ty) {
            (Storage::Main, t) if (t.is_int() || t.is_float()) && !matches!(t, Ty::Bit { .. }) => None,
            (Storage::Main, Ty::Str | Ty::FixedStr(_)) => Some("a string array"),
            (Storage::Main, _) => Some("an array of a `TYPE` or of `_BIT`"),
            (Storage::Param(_), _) => Some("an array parameter"),
            (Storage::Static(_) | Storage::Local(_) | Storage::Result(_), _) => Some("an array of a SUB or FUNCTION"),
        };
        if let Some(what) = what {
            return Err(self.unsupported(e.span, format!("`{word}` of {what}")));
        }
        self.prog.vars[array.0 as usize].conventional = true;
        let kind = if segment {
            ExprKind::VarSeg { array }
        } else {
            ExprKind::VarPtr { array, index }
        };
        Ok(Expr {
            span,
            ty: Ty::I64,
            qb: Ty::I64,
            kind,
        })
    }
}

/// Whether an expression is written with numbers alone: no name, no call ([`Checker::member_bounds`]).
fn only_numbers(node: ast::Expr) -> bool {
    match node {
        ast::Expr::Literal(_) => true,
        ast::Expr::Paren(p) => p.inner().is_some_and(only_numbers),
        ast::Expr::Prefix(p) => p.operand().is_some_and(only_numbers),
        ast::Expr::Bin(b) => b.lhs().is_some_and(only_numbers) && b.rhs().is_some_and(only_numbers),
        ast::Expr::NameRef(_) | ast::Expr::Call(_) | ast::Expr::Field(_) => false,
    }
}

/// The bytes a value of a type takes in the emitter's layout, where a `STRING` is the pointer to its string:
/// what limits the size of a `TYPE`.
fn layout_bytes(types: &[UserType], t: Ty) -> u64 {
    match t {
        Ty::Str => 8,
        Ty::User(id) => types[id.0 as usize]
            .members
            .iter()
            .map(|m| layout_bytes(types, m.ty) * u64::from(m.count()))
            .sum(),
        Ty::I8
        | Ty::U8
        | Ty::I16
        | Ty::U16
        | Ty::I32
        | Ty::U32
        | Ty::I64
        | Ty::U64
        | Ty::Off
        | Ty::UOff
        | Ty::Bit { .. }
        | Ty::F32
        | Ty::F64
        | Ty::F80
        | Ty::FixedStr(_) => u64::from(crate::size_of(types, t)),
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
        ExprKind::VarPtr { .. }
        | ExprKind::VarSeg { .. }
        | ExprKind::Block { .. }
        | ExprKind::MemOf { .. }
        | ExprKind::MemAt { .. }
        | ExprKind::AddressOf(_)
        | ExprKind::MemRead { .. } => None,
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
        | ExprKind::Choice { .. }
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
