//! User types, arrays and places (design D3–D5 of `m2-arrays-and-types`): `TYPE` blocks (pass 1), `DIM` of static
//! arrays and their bounds, array elements, members and dotted names, `LBOUND`/`UBOUND`. Every rule is measured
//! (`verification\v18_*`, `study\00` §5); what was not measured is "not supported yet".

use super::decl::{AsPlace, Declarer, name_end};
use super::expr::{Use, is_builtin_function};
use super::{Checker, DimFact, Failed, NoJoin, Placing, R, Scope, Skips, split_unsigned};
use crate::{AddressWord, MEM_TYPE, ProcId, ProcKind, Program, Var, VarId, mem_type};
use crate::{AllocMode, BinOp, ConvKind, Join, StartBounds, StmtKind, UnOp};
use crate::{ArrayRef, Expr, ExprKind, Member, MemberId, Place, Shape, Storage, SymbolKind, Ty, TypeId, UserType};
use qb64rust_base::fixes::Fix;
use qb64rust_base::{Span, show_bytes, to_u32};
use qb64rust_builtins::{auto_constant, find_any, is_auto_include_function, is_auto_include_name};
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::ast;
use qb64rust_syntax::is_keyword;
use qb64rust_syntax::tree::{Node, Tok};
use std::collections::HashMap;

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

/// The member names the old compiler refuses, "Name already in use": the names of its reserved table that carry
/// `HASHFLAG_TYPE`, `HASHFLAG_OPERATOR`, `HASHFLAG_CUSTOMSYNTAX` or `HASHFLAG_XELEMENTNAME` (`qb64pe.bas` 2313–2320;
/// the table 1250–1395, in its order). Every other valid name is taken, keywords and built-ins among them (measured,
/// `verification\v61_m_*`; `AS`, `DATA`, `ELSE` and `REM` fail there in the old compiler's reading of the line, with
/// another message, and so does `END`, which is not in the table). A name with a single leading underscore is
/// refused before ([`Checker::declare_type`]).
const MEMBER_NAMES_REFUSED: &[&str] = &[
    // HASHFLAG_TYPE
    "_UNSIGNED",
    "_BIT",
    "_BYTE",
    "INTEGER",
    "LONG",
    "_INTEGER64",
    "_OFFSET",
    "SINGLE",
    "DOUBLE",
    "_FLOAT",
    "STRING",
    "ANY",
    // HASHFLAG_OPERATOR
    "NOT",
    "IMP",
    "EQV",
    "AND",
    "OR",
    "XOR",
    "MOD",
    "_NEGATE",
    "_ANDALSO",
    "_ORELSE",
    // HASHFLAG_CUSTOMSYNTAX
    "LIST",
    "BASE",
    "_EXPLICIT",
    "_EXPLICITARRAY",
    "AS",
    "IS",
    "OFF",
    "ON",
    "STOP",
    "TO",
    "USING",
    "_DYNAMIC",
    "_STATIC",
    "PRESET",
    "PSET",
    "FOR",
    "OUTPUT",
    "RANDOM",
    "BINARY",
    "APPEND",
    "SHARED",
    "ACCESS",
    "LOCK",
    "READ",
    "WRITE",
    "STEP",
    "LPRINT",
    "PRINT",
    // HASHFLAG_XELEMENTNAME (`CASE`, `DO`, `ERROR`, `EXIT`, `FIELD`, `INPUT`, `KEY`, `LET`, `LOOP`, `LEN`, `NEXT`,
    // `PLAY`, `PEN`, `RESTORE`, `RESUME`, `RETURN`, `RUN`, `SELECT`, `SCREEN`, `TIMER` and `TYPE` lack it)
    "COMMON",
    "CALL",
    "COM",
    "CONST",
    "DATA",
    "DECLARE",
    "DEF",
    "DEFDBL",
    "DEFINT",
    "DEFLNG",
    "DEFSNG",
    "DEFSTR",
    "DIM",
    "ELSE",
    "ELSEIF",
    "ENDIF",
    "FUNCTION",
    "GOSUB",
    "GOTO",
    "IF",
    "REDIM",
    "REM",
    "STATIC",
    "STRIG",
    "SEG",
    "SUB",
    "THEN",
    "UNTIL",
    "UEVENT",
    "WEND",
    "WHILE",
    // Not in the table: the old compiler reads `END AS …` as the block's end ("Expected END TYPE").
    "END",
];

/// When the bounds of a member array are read ([`Checker::member_bounds`]).
#[derive(Clone, Copy, Debug)]
enum Bounds {
    /// In pass 1, before any `CONST` is evaluated, with the number of `CONST`s above the block and whether it
    /// stands in a SUB or FUNCTION ([`Checker::const_names`]).
    Pass1((usize, bool)),
    /// In pass 2, where the constants above the block are known (the first bounds of a dynamic member array).
    Pass2,
}

/// The most elements an array may have here (none of the inputs comes near it; measured, 10,000,001 LONGs work).
const MAX_ELEMENTS: i64 = i32::MAX as i64;

impl Checker<'_> {
    // ---- user types ----

    /// Pass 1: every `TYPE` block in file order, in the main module, a procedure or a block, each a type of the
    /// whole program, so a type may be used above its block (measured, `v18_h_type_before_block`,
    /// `v48_c_type_anywhere`; the old compiler's prepass, `qb64pe.bas` 2483). A block with an error is not entered;
    /// its uses are "not supported yet".
    pub(super) fn declare_types(&mut self, blocks: &[ast::TypeBlock], skips: &Skips) {
        for (k, &b) in blocks.iter().enumerate() {
            let s = b.node();
            if skips.past_cap(s) {
                break;
            }
            if !skips.usable(s) {
                continue;
            }
            self.stmt.stmt_error = false;
            let marks = self.diags.unsupported_count();
            if let Ok(id) = self.declare_type(b) {
                self.type_defs.insert(s.key());
                self.type_order.insert(id, k);
            } else {
                if self.diags.unsupported_count() > marks {
                    // Pass 2 starts the follow-on rule when it reaches this block (design D10).
                    self.types_marked.insert(s.key());
                }
                if let Some(t) = b.header().and_then(|h| h.name()) {
                    let word = self.text(t.span);
                    let name = show_bytes(&word[..name_end(word)].to_ascii_uppercase());
                    self.types_failed.insert(name);
                }
            }
            self.flush_names();
        }
    }

    fn declare_type(&mut self, b: ast::TypeBlock) -> R<TypeId> {
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
            // Measured: "Name already in use", wherever the two blocks stand (`verification\v48_x05`).
            return Err(self.in_use(name_tok));
        }
        // `OPTION BASE` as it stands at the block: the lower bound of a member array's dimension without `TO`.
        let base = self.type_bases.get(&b.node().key()).copied().unwrap_or(0);
        let above = self.consts_above.get(&b.node().key()).copied().unwrap_or((0, false));
        let mut members: Vec<Member> = Vec::new();
        // The name of each member, for its symbol.
        let mut member_toks: Vec<Tok> = Vec::new();
        let mut const_bounds = false;
        let mut pending = false;
        for field in b.fields() {
            let fspan = field.node().span();
            let a = self.need(field.as_clause(), fspan)?;
            let list = field
                .names()
                .next()
                .is_some_and(|f| a.node().span().start < f.node().span().start);
            // `STRING * n` with `n` a `CONST` above the block: its length is read in pass 2, as a bound naming one is
            // (design D11 of `m4-statements-metacommands`); until then 1.
            let ty = if self.member_len_const(a, above)? {
                const_bounds = true;
                pending = true;
                Ty::FixedStr(1)
            } else {
                self.member_type(a, list)?
            };
            for f in field.names() {
                let t = self.need(f.name(), f.node().span())?;
                let shown = show_bytes(self.text(t.span));
                let dynamic = self.member_markers(b, field, f)?;
                let bounds = match f.bounds() {
                    // The first bounds of a dynamic member array may name a `CONST` (measured,
                    // `verification\v35_x17`): they are read in pass 2, where the constants above the block are
                    // known, in a procedure its own (`v48_x27`, `x28`); until then each dimension is 0 TO 0.
                    Some(bounds) if dynamic && names_constants(bounds) => {
                        const_bounds = true;
                        vec![(0, 0); bounds.ranges().len()]
                    }
                    // A static member array's bound that names a `CONST` above the block, a constant of the
                    // auto-included file or a function of the constant evaluator (design D7 of
                    // `m4-memory-records`; `verification\v24_x196`, `v51_x09`–`x11`, `x14`): read in pass 2 as
                    // well, where it changes the type's layout; until then 0 TO 0 ([`Checker::waits_for_layout`]).
                    Some(bounds) => match self.member_bounds(bounds, base, &shown, Bounds::Pass1(above))? {
                        Some(read) => read,
                        None => {
                            const_bounds = true;
                            pending = true;
                            vec![(0, 0); bounds.ranges().len()]
                        }
                    },
                    None => Vec::new(),
                };
                if super::proc::fixed_name(&self.name_text(t)).is_some() {
                    // Measured: "Invalid name" (design D11 of `m4-memory-records`; `verification\v21_x57`,
                    // `v56_x21`, `x22`).
                    return Err(self.error(t.span, format!("a `TYPE` member cannot have a type suffix: `{shown}`")));
                }
                let (mname, msuffix) = self.split_name(t)?;
                if msuffix.is_some() {
                    // Measured: "Invalid name" and "Expected element-name AS type" (`v18_h_member_decl_suffix*`).
                    return Err(self.error(t.span, format!("a `TYPE` member cannot have a type suffix: `{shown}`")));
                }
                // A single leading underscore: "Invalid name" (`validname`, `qb64pe.bas` 28273; measured,
                // `verification\v61_m__width`, `v61_m__mine`); the reserved names among them too.
                if mname.len() >= 2 && mname.starts_with('_') && !mname[1..].starts_with('_') {
                    return Err(self.error(t.span, format!("a `TYPE` member cannot start with one `_`: `{shown}`")));
                }
                // The old compiler's table (measured, `v18_h_member_names*`, `v61_m_*`).
                if MEMBER_NAMES_REFUSED.contains(&mname.as_str()) {
                    return Err(self.in_use(t));
                }
                if mname.contains('.') {
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
                member_toks.push(t);
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
        self.stmt.names.push((SymbolKind::Type(id), name_tok.span));
        for (i, t) in member_toks.iter().enumerate() {
            self.stmt
                .names
                .push((SymbolKind::Member(id, MemberId(to_u32(i))), t.span));
        }
        self.types_by_name.insert(name, id);
        if const_bounds {
            self.type_const_bounds.insert(b.node().key(), id);
        }
        if pending {
            self.layout_pending.insert(id);
        }
        Ok(id)
    }

    /// Pass 2 at a `TYPE` block of [`Checker::type_const_bounds`]: the bounds of its member arrays that name a
    /// `CONST`, now that the constants above the block are known: the first bounds of a dynamic one, and the bounds
    /// of a static one, which lay the type out (design D7 of `m4-memory-records`).
    fn type_const_bounds(&mut self, b: ast::TypeBlock, id: TypeId) {
        self.stmt.stmt_error = false;
        let base = self.type_bases.get(&b.node().key()).copied().unwrap_or(0);
        let names = b
            .fields()
            .flat_map(|field| field.names().map(move |f| (field, f)).collect::<Vec<_>>());
        for (i, (field, f)) in names.enumerate() {
            if let Some(size) = field.as_clause().and_then(|a| a.size())
                && size.kind == SyntaxKind::Ident
                && let Ok(n) = self.fixed_len_of(size, true)
            {
                self.prog.types[id.0 as usize].members[i].ty = Ty::FixedStr(n);
            }
            let Some(bounds) = f.bounds().filter(|&bounds| names_constants(bounds)) else {
                continue;
            };
            let shown = show_bytes(self.text(f.name().map_or(f.node().span(), |t| t.span)));
            if let Ok(Some(read)) = self.member_bounds(bounds, base, &shown, Bounds::Pass2) {
                self.prog.types[id.0 as usize].members[i].bounds = read;
            }
        }
        self.layout_pending.remove(&id);
        let bytes: u64 = self.prog.types[id.0 as usize]
            .members
            .iter()
            .map(|m| layout_bytes(&self.prog.types, m.ty) * u64::from(m.count()))
            .fold(0, u64::saturating_add);
        if bytes > MAX_ELEMENTS as u64 {
            let at = b.header().map_or(b.node().span(), |h| h.node().span());
            let _ = self.unsupported(at, "a `TYPE` of more than 2^31 bytes");
        }
        self.flush_names();
    }

    /// Whether a value of type `t` is laid out by member array bounds that pass 2 has not read yet: a use of the
    /// type above its block, which reads its size or bounds before they are known, is not supported yet (design D7
    /// of `m4-memory-records`).
    pub(super) fn waits_for_layout(&self, t: Ty) -> bool {
        match t {
            Ty::User(id) => {
                self.layout_pending.contains(&id)
                    || self
                        .prog
                        .user_type(id)
                        .members
                        .iter()
                        .any(|m| self.waits_for_layout(m.ty))
            }
            crate::numeric_types!() | Ty::Str | Ty::FixedStr(_) => false,
        }
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
    /// A bound is a constant expression, read where the block stands: a name that is no `CONST` above it is "TYPE
    /// member array bounds must be numeric constant" (`v24_x195`; `v51_x01`–`x08`: a variable, an unknown name, a
    /// `CONST` below the block or in a SUB below it, `LEN`). The blocks are read in pass 1, before the program's
    /// `CONST`s are evaluated, so bounds that name one above, or may be one ([`Self::may_be_constant`]; accepted by
    /// the old compiler, `v24_x196`, `v51_x09`–`x11`, `x14`), are `None` there and read in pass 2.
    fn member_bounds(
        &mut self,
        bounds: ast::ArrayBounds,
        base: i64,
        shown: &str,
        when: Bounds,
    ) -> R<Option<Vec<(i64, i64)>>> {
        let span = bounds.node().span();
        let ranges = bounds.ranges();
        if ranges.is_empty() {
            return Err(self.error(span, format!("the member array `{shown}` needs bounds")));
        }
        if let Bounds::Pass1(above) = when {
            let mut deferred = false;
            for (lower, upper) in &ranges {
                for &bound in lower.iter().chain(upper) {
                    if only_numbers(bound) {
                        continue;
                    }
                    if !self.may_be_constant(bound, above) {
                        return Err(self.error(bound.node().span(), "a bound of a member array must be a constant"));
                    }
                    deferred = true;
                }
            }
            if deferred {
                return Ok(None);
            }
        }
        let mut out = Vec::new();
        let mut count: i64 = 1;
        // In pass 2 a bound may name constants and the constant evaluator's functions (`_PI`, `ABS(…)`).
        let read = |c: &mut Self, node: ast::Expr| match when {
            Bounds::Pass1(_) => c.constant_bound(node),
            Bounds::Pass2 => c.const_integer(node),
        };
        for (lower, upper) in ranges {
            let upper = self.need(upper, span)?;
            let l = match lower {
                Some(l) => read(self, l)?,
                None => base,
            };
            let u = read(self, upper)?;
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
        Ok(Some(out))
    }

    /// Whether a bound of a member array names what may be a constant where its block stands: a `CONST` above
    /// the block in file order (of [`Checker::const_names`]; for a block of the main module one of the main
    /// module, measured: a SUB's above it is the error, `verification\v51_x13`), a constant or function of
    /// QB64pe's auto-included file, a function of the constant evaluator (`_PI` alone, `ABS(…)`: accepted,
    /// `v51_x10`, `x11`, `x14`), or a dotted name (not measured). An array's element, a FUNCTION of the program
    /// and `LEN` are none (`x05`, `x06`, `x12`, `x15`).
    /// Whether a member's type is `STRING * n` with `n` the name of a `CONST` above the block (`above`, as
    /// [`Self::may_be_constant`] takes it), whose length pass 2 reads. Any other name there is the old compiler's
    /// "Invalid number/constant after STRING * type": a constant below the block, a suffixed name, a variable
    /// (measured, `verification\v56_x30`, `v57_t01`–`t03`).
    fn member_len_const(&mut self, a: ast::AsClause, above: (usize, bool)) -> R<bool> {
        let Some(size) = a.size().filter(|t| t.kind == SyntaxKind::Ident) else {
            return Ok(false);
        };
        let words = self.as_words(a);
        if split_unsigned(&words).1 != ["STRING"] {
            return Ok(false);
        }
        let shown = show_bytes(self.text(size.span));
        let (name, suffix) = self.split_name(size)?;
        let (count, in_proc) = above;
        let above = self.const_names[..count]
            .iter()
            .any(|(n, proc)| *n == name && (in_proc || !proc));
        if suffix.is_none() && above {
            return Ok(true);
        }
        if auto_constant(name.as_bytes()).is_some() {
            return Err(self.unsupported(
                size.span,
                format!("a constant of QB64pe as the length of a member: `{shown}`"),
            ));
        }
        let msg =
            format!("a member's `STRING * n` needs a number or a `CONST` above the block without a suffix: `{shown}`");
        Err(self.error(size.span, msg))
    }

    fn may_be_constant(&self, node: ast::Expr, above: (usize, bool)) -> bool {
        let (count, in_proc) = above;
        let known = |c: &Self, t: Tok| {
            let word = c.text(t.span);
            let name = show_bytes(&word[..name_end(word)].to_ascii_uppercase());
            c.const_names[..count]
                .iter()
                .any(|(n, proc)| *n == name && (in_proc || !proc))
                || auto_constant(name.as_bytes()).is_some()
                || is_auto_include_name(name.as_bytes())
                || crate::consteval::function_arity(&name).is_some()
        };
        match node {
            ast::Expr::Literal(_) => false,
            ast::Expr::Paren(p) => p.inner().is_some_and(|e| self.may_be_constant(e, above)),
            ast::Expr::Prefix(p) => p.operand().is_some_and(|e| self.may_be_constant(e, above)),
            ast::Expr::Bin(b) => {
                b.lhs().is_some_and(|e| self.may_be_constant(e, above))
                    || b.rhs().is_some_and(|e| self.may_be_constant(e, above))
            }
            ast::Expr::NameRef(n) => n.name().is_none_or(|t| known(self, t)),
            ast::Expr::Call(c) => c.name().is_none_or(|t| known(self, t)),
            ast::Expr::Field(_) => true,
        }
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
    ///
    /// Any other name is "Undefined type", a type defined in a later block too (measured, `verification\v51_x20`–
    /// `x25`, `x27`, upstream `arrays/40_err_undefined_type`), and before the names of an `AS type` list "Expected
    /// element-name AS type, AS type element-list, or END TYPE" (`x26`). `list`: the clause stands before the
    /// names.
    fn member_type(&mut self, a: ast::AsClause, list: bool) -> R<Ty> {
        let span = a.node().span();
        let words = self.as_words(a);
        let text = words.join(" ");
        if let Some(t) = self.user_type_of(&text) {
            self.type_reference(a, t);
            return Ok(t);
        }
        let (unsigned, rest) = split_unsigned(&words);
        let base = rest.join(" ");
        let sized = a.size().is_none() || base == "STRING" || base == "_BIT";
        let built_in = numeric_type(&base).is_some()
            || base == "STRING"
            || (unsigned && (base.is_empty() || self.user_type_of(&base).is_some()));
        if !(built_in && sized) {
            // A block above that was not entered may define it.
            if self.types_failed.contains(&base) {
                return Err(self.unsupported(span, format!("the member type `{text}`")));
            }
            let shown = if a.size().is_some() {
                format!("{text} * …")
            } else {
                text
            };
            let msg = if list {
                format!("the type `{shown}` of a member list is no type")
            } else {
                format!("the member type `{shown}` is no type")
            };
            return Err(self.error(span, msg));
        }
        let ty = self.type_of_in(a, false, AsPlace::Decl)?;
        if let Ty::Bit { .. } = ty {
            Err(self.error(span, "a `TYPE` member cannot be a `_BIT`"))
        } else {
            Ok(ty)
        }
    }

    /// A `TYPE` block met in pass 2, wherever it stands: pass 1 declared it ([`Self::declare_types`]). A block
    /// pass 1 marked starts the follow-on rule here, in file order.
    pub(super) fn type_block(&mut self, b: ast::TypeBlock) {
        let node = b.node();
        if self.types_marked.contains(&node.key()) {
            self.at.follow_on = true;
        }
        if let Some(&id) = self.type_const_bounds.get(&node.key()) {
            self.type_const_bounds(b, id);
        }
    }

    /// Records the user type an `AS` clause names as a reference of its symbol, at the clause's type words (design D3
    /// of `m5b-formatter`); `_MEM`, which no `TYPE` line defines, is left out.
    pub(super) fn type_reference(&mut self, a: ast::AsClause, t: Ty) {
        if let Ty::User(id) = t
            && self.prog.user_type(id).name != MEM_TYPE
        {
            let mut words = a.type_words();
            if let Some(first) = words.next() {
                let last = words.last().unwrap_or(first);
                self.stmt
                    .names
                    .push((SymbolKind::Type(id), first.span.cover(last.span)));
            }
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
        if let Ty::Bit { width, .. } = ty
            && width > 63
        {
            // Measured: "Cannot create a _BIT array of size > 63 bits" (`verification\v21_x27`); in `dim2` for every
            // array, by `AS` or by suffix (`qb64pe.bas`).
            let msg = format!("an array cannot be of a `_BIT` type of more than 63 bits (`_BIT * {width}`)");
            return Err(self.error(name_tok.span, msg));
        }
        // Measured (`v09` test 3, `v24_x48`): a `REDIM` without a type takes the type of an array of the name
        // declared with `AS`, by `REDIM` or by `DIM`. With `AS` and another type than that array's it is "Name
        // already in use", between `STRING` and `STRING * n` or two lengths "Type mismatch" (`v24_x39`, `x41`,
        // `x42`).
        let untyped = suffix.is_none() && !typed;
        let mut plain_array = self.array_plain_ty(&name);
        // `a$(…)` beside a `STRING * n` array `a` means that array ([`Self::array_key_ty`]); a `DIM` of it is "Type
        // mismatch" (measured, `verification\v52_x45`), the other declarers are not measured.
        if suffix == Some(Ty::Str)
            && let Some(Ty::FixedStr(_)) = plain_array
            && self.array_of(&(name.clone(), Ty::Str)).is_none()
        {
            if by == Declarer::Dim {
                let msg = format!("`{shown}`: an array of variable-length strings beside the `STRING * n` one");
                return Err(self.error(name_tok.span, msg));
            }
            let msg = format!("`{shown}` declared beside a `STRING * n` array of the name");
            return Err(self.unsupported(name_tok.span, msg));
        }
        // Between two arrays of one name the plain name stays with the one typed by `AS` (measured,
        // `verification\v48_x16`–`x24`), but for an array listed by `STATIC a() AS type` that nothing has reached:
        // the old compiler makes no array for the list until then (`qb64pe.bas` 9260), so an array without `AS`
        // is the only one of the name (`v24_x126`, `v48_a_scalar_array`).
        let listed_typed = plain_array
            .filter(|&t| untyped && t != ty)
            .and_then(|t| self.scope().arrays.get(&(name.clone(), t)).copied())
            .map(|v| self.prog.var(v))
            .filter(|v| v.dim_count().is_none());
        let takes_plain = match listed_typed.map(|v| v.storage) {
            Some(Storage::Static(_)) => {
                plain_array = None;
                true
            }
            Some(Storage::Main) => {
                // Listed by `COMMON a() AS type`: not measured.
                let msg = format!("an array `{shown}` without `AS` beside one listed by `COMMON … AS`");
                return Err(self.unsupported(name_tok.span, msg));
            }
            // In a procedure, its own array without `AS` takes the plain name from a shared typed one of the
            // main module (measured, `v48_x20`).
            _ => {
                untyped
                    && !redim
                    && self.cur.is_some()
                    && !self.local.array_plain.contains_key(&name)
                    && plain_array.is_some_and(|t| t != ty)
            }
        };
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
        let mut existing = self.array_of(&key);
        let bounds_span = bounds.node().span();
        let mut beside_shared = by == Declarer::Static && self.static_array_free(existing, name_tok, &shown, listed)?;
        // Beside a shared static array, `STATIC a(n)` declares an array of the procedure's own, as QuickBASIC 4.5 does
        // (`verification\qb45\q45_static_local`), where the old compiler ends the program at the start with
        // "Duplicate definition" (`QB64PE-DIVERGENCES.md` D-041, `study\39` row 4, always); beside a dynamic one only
        // with the fix `static-local-dynamic`. Every later use of the name in the procedure finds that array.
        if beside_shared
            && let Some(id) = existing
            && (matches!(self.prog.var(id).shape, Shape::Static { .. })
                || self.settings.fixes.is_on(Fix::StaticLocalDynamic))
        {
            existing = None;
            beside_shared = false;
        }
        if common && existing.is_some() {
            // Not measured: `COMMON a()` twice, or below a `DIM` or a use of the array.
            let msg = format!("`COMMON {shown}()` of an array that exists already");
            return Err(self.unsupported(name_tok.span, msg));
        }
        let dims = if common {
            Vec::new()
        } else if redim {
            let vars = self.prog.vars.len();
            let dims = self.unjoined(NoJoin::RedimBound(mode), |c| c.dim_bounds(bounds, &shown))?;
            let shared = existing.is_some_and(|v| self.prog.var(v).storage == Storage::Main);
            let new_array = self.prog.vars[vars..].iter().any(|v| !matches!(v.shape, Shape::Scalar));
            if let (Some(proc), true, true) = (self.cur, shared, new_array) {
                // Measured: in a procedure, a `REDIM` of a shared array whose bound first names an array used
                // without `DIM` makes the name an array of the procedure in the whole procedure, which it gives its
                // bounds, and the shared one keeps its own (`verification\v24_x268` case 5, `v55_x84`,
                // `v60_b1`–`b10`). Noted for the second check, which makes that array where the procedure starts
                // (design D2 of `m4-left-marked`); this check stops here.
                let plain = suffix.is_none() && (typed || self.array_plain_ty(&name) == Some(ty));
                self.redim_found.push(super::RedimLocal {
                    proc,
                    name: name.clone(),
                    ty,
                    dims: dims.len(),
                    plain,
                });
                let msg = format!(
                    "`REDIM {shown}(...)` of a shared array with a bound that names an array used without `DIM`"
                );
                return Err(self.unsupported(bounds_span, msg));
            }
            dims
        } else {
            self.dim_bounds(bounds, &shown)?
        };
        let constant: Option<Vec<(i64, i64)>> = dims.iter().map(|d| d.constant).collect();
        // An array that lives for the program may be static: the main module's, a `STATIC` one and every array
        // of a procedure with `STATIC` after its header. A procedure's other arrays are made on each call.
        let lives = matches!(storage, Storage::Main | Storage::Static(_));
        let static_bounds = constant
            .clone()
            .filter(|_| by == Declarer::Dim && lives && !self.at.dynamic_mode);
        // `STATIC a(bounds)` is no static array: a dynamic one that is allocated when the program starts, below
        // `$DYNAMIC` or not ([`Shape::Dynamic`], `at_start`; read from the old compiler's C++ and measured,
        // `verification\v24_x278`, `v24_b_metacommand_sub`, slice `s56_proc_arrays`).
        let by_static = by == Declarer::Static && !listed;
        // That allocation stands in the main module's code, where the old compiler writes the bounds as they are
        // (design D4 of `m4-names-implicit-arrays`): a bound whose value is known when compiling is taken with it
        // (`2 + 3`, a `CONST` of the procedure, `LEN("abc")`; measured, `verification\v24_x280`,
        // `v55_c_static_bounds`, `v55_x73`).
        let values: Option<Vec<(i64, i64)>> = dims
            .iter()
            .map(|d| Some((constant_int(&d.lower)?, constant_int(&d.upper)?)))
            .collect();
        let mut start_bounds = constant.or(values).filter(|_| by_static).map(StartBounds::Values);
        if by_static && start_bounds.is_none() {
            let local = |c: &Self, t: Tok, scope: &HashMap<(String, Ty), VarId>| {
                let name = c.word(t);
                let name = &name[..name_end(name.as_bytes())];
                let own = |v: VarId| c.prog.var(v).storage != Storage::Main;
                c.cur.is_some() && scope.iter().any(|((n, _), &v)| n == name && own(v))
            };
            let tokens = bounds.node().tokens();
            let names = |c: &Self, scope| {
                tokens
                    .iter()
                    .any(|&t| t.kind == SyntaxKind::Ident && local(c, t, scope))
            };
            if names(self, &self.local.vars) {
                // A parameter or a variable of the procedure: the old compiler's C++ build fails (measured,
                // `verification\v24_x121`).
                let msg = format!(
                    "`STATIC {shown}(...)`: a bound names a variable of the procedure, which the code that allocates \
                     the array when the program starts cannot see (the old compiler's C++ build fails)"
                );
                return Err(self.error(bounds_span, msg));
            }
            if names(self, &self.local.arrays) {
                // An array of the procedure (a parameter array, a local or `STATIC` one: `UBOUND(b)`, `c(1)`): not
                // measured, the start code cannot see it either.
                let msg = format!("`STATIC {shown}(...)` with a bound that names an array of the procedure");
                return Err(self.unsupported(bounds_span, msg));
            }
            if beside_shared {
                // Not measured.
                let msg = format!("`STATIC {shown}(...)` beside a shared array, with a bound not known when compiling");
                return Err(self.unsupported(bounds_span, msg));
            }
            // A shared variable or a FUNCTION: read when the start code allocates the array, at the start and when
            // a handler is entered while it is released (measured, `v55_x74`, `x75`, `v60_a1`–`a7`; design D1 of
            // `m4-left-marked`).
            let read = dims.iter().map(|d| (d.lower.clone(), d.upper.clone())).collect();
            start_bounds = Some(StartBounds::Read(read));
        }
        if let (true, Some(id), Some(bounds)) = (beside_shared, existing, start_bounds.clone()) {
            // `STATIC a(n)` beside a shared main-module array of the name and type names that array, and its
            // allocation when the program starts is a second one of it, after its own (measured,
            // `verification\v24_x272`, `v55_d1`, `d3`, `d4`; design D4 of `m4-names-implicit-arrays`).
            if self.prog.var(id).dim_count() != Some(bounds.dims()) {
                // Not measured.
                let msg = format!("`STATIC {shown}(...)` beside a shared array of another number of dimensions");
                return Err(self.unsupported(bounds_span, msg));
            }
            self.prog.start_allocations.push((id, bounds));
            self.stmt.names.push((SymbolKind::Var(id), name_tok.span));
            return Ok(());
        }
        let allocate = |array| StmtKind::Allocate {
            array: ArrayRef::Var(array),
            bounds: dims.iter().map(|d| (d.lower.clone(), d.upper.clone())).collect(),
            mode,
            join: Join::None,
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
            self.stmt.names.push((SymbolKind::Var(id), name_tok.span));
            self.push(stmt, allocate(id));
            return Ok(());
        }
        // A scalar of the name, typed by `AS` or not, is another variable (measured, `v24_x56`,
        // `v24_b_dim_empty_then_dim`, `v48_a_scalar_array`).
        let known = match (&static_bounds, &start_bounds) {
            (Some(bounds), _) | (None, Some(StartBounds::Values(bounds))) => Some(bounds),
            _ => None,
        };
        if let Some(bounds) = known {
            let mut count: i64 = 1;
            for &(l, u) in bounds {
                count = match u
                    .checked_sub(l)
                    .and_then(|n| n.checked_add(1))
                    .and_then(|n| count.checked_mul(n))
                {
                    Some(n) if n <= MAX_ELEMENTS => n,
                    _ => return Err(self.unsupported(bounds_span, "an array of more than 2^31 elements")),
                };
            }
        }
        let shape = match (static_bounds, start_bounds) {
            (Some(bounds), _) => Shape::Static { bounds },
            (None, Some(bounds)) => Shape::Dynamic {
                dims: Some(bounds.dims()),
                at_start: Some(bounds),
            },
            // `STATIC a()` (measured, `verification\v24_g_local`, `v24_f_implicit_sub`) and `COMMON a()`
            // (`v24_g_common`, `v24_x167`): a dynamic array that lives for the program and is not allocated;
            // what gives it bounds or indexes first gives it its number of dimensions.
            (None, None) if listed => Shape::Dynamic {
                dims: None,
                at_start: None,
            },
            (None, None) => Shape::Dynamic {
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
        } else if takes_plain {
            self.scope().array_plain.insert(name, ty);
        }
        if shared {
            self.dim_shared.insert(id);
        }
        self.stmt.names.push((SymbolKind::Var(id), name_tok.span));
        if dynamic {
            self.push(stmt, allocate(id));
        }
        Ok(())
    }

    /// Whether `STATIC name(…)` may declare its array, given the array of that name and type visible here
    /// (`listed`: nothing stands between the parentheses). Measured: "Name already in use" after a `STATIC`, a
    /// `DIM` or a use without `DIM` of it in the procedure (`verification\v24_x122`, `x123`, `x125`, `x269`),
    /// "Array already listed as STATIC" for bounds after `STATIC a()` (`x124`). True when it names a shared array
    /// of the main module, which `STATIC a(bounds)` allocates again when the program starts.
    fn static_array_free(&mut self, existing: Option<VarId>, name_tok: Tok, shown: &str, listed: bool) -> R<bool> {
        if let Some(id) = existing {
            let var = self.prog.var(id);
            if matches!(var.storage, Storage::Param(_)) {
                return Err(self.in_use(name_tok));
            }
            if var.storage == Storage::Main {
                // A shared array of the main module (measured, `v24_x272`: the old compiler accepts it, and a static
                // shared array then ends the program when it starts, "Duplicate definition"). `STATIC a()` beside
                // one is not measured.
                if listed {
                    let msg = format!("`STATIC {shown}()` beside a shared array of the main module");
                    return Err(self.unsupported(name_tok.span, msg));
                }
                return Ok(true);
            }
            if var.dim_count().is_none() && !listed {
                let msg = format!("`{shown}` is already listed by `STATIC {shown}()`: `DIM` gives it its bounds");
                return Err(self.error(name_tok.span, msg));
            }
            return Err(self.in_use(name_tok));
        }
        Ok(false)
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
                    let base = if self.is_plain_zero(upper) {
                        0
                    } else {
                        self.at.option_base
                    };
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

    /// The type an array's plain (suffix-less) name means here. `name$` where no array of the name and `$` is
    /// visible means the `STRING * n` array of the name, if the plain name means one (design D7 of
    /// `m4-qb64pe-sources`; measured, `verification\v52_x40`–`x42`, `x47`, `x48`).
    fn array_key_ty(&mut self, name: &str, suffix: Option<Ty>) -> Ty {
        if suffix == Some(Ty::Str)
            && self.array_of(&(name.to_string(), Ty::Str)).is_none()
            && let Some(t @ Ty::FixedStr(_)) = self.array_plain_ty(name)
        {
            return t;
        }
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
    ///   [`StmtKind::Allocate`] that runs as part of the statement that names it first: its first part, or just
    ///   before the item that names it ([`Join`]);
    /// - in a procedure a local dynamic array that nothing allocates, so that every use raises error 9 (as the
    ///   old compiler does; `DIVERGENCES-QB45.md` Q-009) until a `DIM` or `REDIM` of it in the procedure runs;
    /// - in a procedure with `STATIC` after its header a static array like the main module's that lives for the
    ///   program, and below `$DYNAMIC` a dynamic one that lives for the program and that nothing allocates.
    ///
    /// Where the old compiler's allocation would be another kind of allocation, nothing is made and the use is
    /// marked "not supported yet" ([`NoJoin`]).
    ///
    /// After a declaration that was marked "not supported yet" (which may have declared this array) nothing is
    /// made: the name keeps the mark of [`Self::not_an_array`].
    pub(super) fn implicit_array(&mut self, t: Tok, name: &str, suffix: Option<Ty>, dims: usize, at: Node) -> R<VarId> {
        let shown = show_bytes(self.text(t.span));
        let underscore = name.starts_with('_') && !name.starts_with("__");
        // The name of a supported built-in statement that is no function is that error for certain (`v39_x11`:
        // `l = _SNDPLAY(h)`).
        if underscore
            && !self.at.follow_on
            && !is_builtin_function(name, suffix)
            && !crate::builtins::stmt_lookup(name, suffix == Some(Ty::Str)).is_empty()
        {
            self.implicit_names.insert(name.to_string());
            return Err(self.error(t.span, format!("`{shown}(...)`: `{shown}` is a built-in statement")));
        }
        // A function whose every entry requires `$`, named without it: "Invalid array name" for certain (measured,
        // `verification\v52_x55`, `x59`: `_READFILE("a")`, `_FULLPATH(".")`).
        let needs_dollar =
            |b: &qb64rust_builtins::Builtin| b.kind == qb64rust_builtins::Kind::Function && b.musthave == Some("$");
        if underscore
            && !self.at.follow_on
            && suffix.is_none()
            && find_any(name.as_bytes()).next().is_some()
            && find_any(name.as_bytes()).all(needs_dollar)
        {
            self.implicit_names.insert(name.to_string());
            return Err(self.error(t.span, format!("`{shown}(...)`: `{shown}` is written with `$`")));
        }
        // The functions of the auto-included files (`_ENCODEURL$`) as a place: not measured.
        let auto = is_auto_include_function(name.as_bytes());
        if self.at.follow_on || auto {
            return Err(self.not_an_array(t, name));
        }
        if underscore && !is_builtin_function(name, suffix) {
            // Measured: "Invalid array name" for a name with one leading underscore that calls no built-in
            // function with this spelling, as a place, a value or in `UBOUND` (`v24_x112`, `v26_x120`, `v33_x29`,
            // `v44_x46`, `v49_x16`, `v55_x54`, `x55`, `x80`); the table knows every spelling of its functions
            // (`qb64rust_builtins::calls_function`).
            self.implicit_names.insert(name.to_string());
            let msg = format!("`{shown}(...)`: `{shown}` calls no built-in function and may not be an array");
            return Err(self.error(t.span, msg));
        }
        if let Some(&p) = self.procs_by_name.get(name) {
            self.implicit_names.insert(name.to_string());
            return Err(self.not_a_variable(t, suffix, Some(p)));
        }
        if is_builtin_function(name, suffix) {
            // The statements written like a store to a function, `MID$` and `ASC`, have their own arm of the
            // assignment (`check\io.rs`).
            self.implicit_names.insert(name.to_string());
            return Err(self.not_a_variable(t, suffix, None));
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
        // `x()`, a whole array, where it is no argument for an array parameter ([`Self::array_arg`]): as a value the
        // implicit array of one dimension, as for `LBOUND(x)`, whose whole value is its first element (measured,
        // `verification\v48_x04`, `x14`, `v55_b_implicit_whole`; design D5 of `m4-names-implicit-arrays`). As a
        // place (`x() = 1`, `READ x()`) not measured.
        if dims == 0 && self.stmt.placing != Placing::Value {
            self.implicit_names.insert(name.to_string());
            return Err(self.unsupported(at.span(), format!("the whole array `{shown}()` as a place")));
        }
        let dims = dims.max(1);
        self.reserved(t, name, suffix)?;
        let ty = self.array_key_ty(name, suffix);
        if self.settings.explicit_array {
            // Measured: "Array 'z' (SINGLE) not defined" (`v24_x105`, `x256`, `x257`).
            let msg = format!(
                "array `{shown}` ({}) is not declared (`OPTION _EXPLICIT` or `OPTION _EXPLICITARRAY`)",
                ty.qb_name()
            );
            return Err(self.error(t.span, msg));
        }
        // First named in a bound of `REDIM`: in the main module dynamic wherever it stands, and allocated as the
        // `REDIM` allocates, so a second run makes it anew or keeps it (measured, `verification\v24_x266`, `x268`;
        // design D3 of `m4-names-implicit-arrays`); in a procedure its implicit array, which nothing allocates.
        let redim = match self.stmt.no_join {
            Some(NoJoin::RedimBound(mode)) => Some(mode),
            Some(NoJoin::Item(_) | NoJoin::Header) | None => None,
        };
        let base = self.at.option_base;
        // In a procedure with `STATIC` after its header the array lives for the program: a static one that
        // works, or below `$DYNAMIC` a dynamic one that nothing allocates (measured, `v24_f_implicit_sub`,
        // `v24_x120`).
        // First named in a bound of `REDIM` there, it is a dynamic one that nothing allocates, as below `$DYNAMIC`
        // (measured, `verification\v60_b5`: error 9 at each call).
        let static_proc = self.cur.filter(|_| self.at.static_scope);
        let (storage, shape) = match (self.cur, static_proc) {
            (None, _) if redim.is_some() => (
                Storage::Main,
                Shape::Dynamic {
                    dims: Some(dims),
                    at_start: None,
                },
            ),
            (Some(p), None) => (
                Storage::Local(p),
                Shape::Dynamic {
                    dims: Some(dims),
                    at_start: None,
                },
            ),
            (_, Some(p)) if self.at.dynamic_mode || redim.is_some() => (
                Storage::Static(p),
                Shape::Dynamic {
                    dims: Some(dims),
                    at_start: None,
                },
            ),
            (None, _) if self.at.dynamic_mode => (
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
        let allocated_here = self.cur.is_none() && (self.at.dynamic_mode || redim.is_some());
        // The allocation runs where the name is evaluated ([`NoJoin`]): the start of the statement, or just before
        // the part of a statement of items that names it (measured, `verification\v24_x264`, `x265`; design D1 of
        // `m4-names-implicit-arrays`). Measured for the line of a block (`v24_x262`, `v55_a_implicit_headers`):
        // before an `IF`, a `FOR`, a `WHILE`, a `DO WHILE` and a `SELECT CASE`, inside the block for an `ELSEIF`
        // (not reached: not allocated) and a `LOOP UNTIL` (on every round).
        // In the line of a block the block's checker takes the allocation where it runs (`check\blocks.rs`).
        let join = match self.stmt.no_join {
            Some(NoJoin::Item(n)) => Join::Before(n),
            _ => Join::First,
        };
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
                array: ArrayRef::Var(id),
                bounds: vec![(bound(base), bound(10)); dims],
                mode: redim.unwrap_or(AllocMode::Dim),
                join,
            };
            self.push(at, kind);
        }
        Ok(id)
    }

    /// The error for `name(...)` at `t` where `name` is a SUB's or FUNCTION's (`proc`) or a built-in function's
    /// name and no array, worded by what it stands for ([`Placing`]). Measured: a store is "Expected variable ="
    /// (`verification\v24_x106`, `x119`, `v28_x53`, `v55_x10`, `x11`, `x20`; a SUB's name "SUB does not require any
    /// arguments", `x76`), a target "Expected variable" (`v22_x79`, `v55_x13`–`x18`), a value "Name already in use"
    /// for a SUB's or FUNCTION's name (`x77`, `x83`) and "Expected (...)" for a built-in's (`x78`); `_RGB(1) = 2` is
    /// "Incorrect number of arguments" with `_RGB`'s reference (`x81`, `x82`).
    pub(super) fn not_a_variable(&mut self, t: Tok, suffix: Option<Ty>, proc: Option<ProcId>) -> Failed {
        let shown = show_bytes(self.text(t.span));
        let kind = proc.map(|p| self.prog.proc(p).kind);
        let what = match kind {
            Some(ProcKind::Sub) => "a SUB",
            Some(ProcKind::Function(_)) => "a FUNCTION",
            None => "a built-in function",
        };
        let msg = match (self.stmt.placing, kind) {
            (Placing::Store, Some(ProcKind::Sub)) => format!("`{shown}(...) =`: the SUB `{shown}` takes no index"),
            // A suffix that does not fit the FUNCTION's type: "Name already in use" (`v55_x19`: `f&(3) = 5`).
            (_, Some(ProcKind::Function(ty))) if suffix.is_some_and(|s| !super::proc::suffix_fits(s, ty)) => {
                format!("`{shown}(...)`: a FUNCTION of this name has another type")
            }
            (Placing::Store, _) => format!("`{shown}(...) =`: `{shown}` is {what}, not a variable"),
            (Placing::Target, _) => format!("`{shown}(...)`: `{shown}` is {what}; a variable is needed here"),
            (Placing::Value, Some(_)) => format!("`{shown}(...)`: `{shown}` is {what}, not an array"),
            (Placing::Value, None) => format!("`{shown}`: a built-in function's name, not an array"),
        };
        self.error(t.span, msg)
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
    pub(super) fn settle_dims(&mut self, array: VarId, first: usize, t: Tok, at: Span) -> usize {
        if let Some(n) = self.prog.var(array).dim_count() {
            return n;
        }
        if self.open_dims(array) {
            self.dims.dim_facts.push(DimFact::Count {
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
        matches!(self.prog.var(array).storage, Storage::Param(_)) || self.dims.dims_open.contains(&array)
    }

    /// The element `array(index, …)` named by a call node: one index per dimension, each converted to `_INTEGER64`
    /// as for an assignment.
    pub(super) fn element(&mut self, call: ast::CallExpr, array: VarId, t: Tok) -> R<Place> {
        let span = call.node().span();
        let args = self.present_args(call.arg_list())?;
        if args.is_empty() {
            // `a()` where a value or a variable is taken: the array's first element in memory, with no index check,
            // as the old compiler writes it (design D4 of `m4-line-numbers-whole-arrays`; measured,
            // `verification\v54_c_whole_values`, `v54_d_whole_erased`, `v54_x05`–`x34`): an element with no indexes.
            // The forms the old compiler rejects are refused before here (the whole value of an assignment,
            // `check\whole.rs`; a scalar parameter, `check\proc.rs`; `SWAP` of two types, `check\io.rs`).
            self.stmt.names.push((SymbolKind::Var(array), t.span));
            return Ok(Place::Element {
                array: ArrayRef::Var(array),
                index: Vec::new(),
            });
        }
        self.element_at(array, args, t, span)
    }

    /// The element of `array` at the indexes `args` (at least one), written at `span` with the name `t`.
    fn element_at(&mut self, array: VarId, args: Vec<ast::Expr>, t: Tok, span: Span) -> R<Place> {
        let dims = self.settle_dims(array, args.len(), t, span);
        let index = self.indexes(args, dims, t, span)?;
        self.stmt.names.push((SymbolKind::Var(array), t.span));
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

    /// The place an element-shaped call names, for a store or the base of a member: an array element. `what` is
    /// what the name stands for ([`Placing`]); the indexes are values.
    pub(super) fn call_place(&mut self, call: ast::CallExpr, what: Placing) -> R<Place> {
        let span = call.node().span();
        let t = self.need(call.name(), span)?;
        let (name, suffix) = self.split_name(t)?;
        if let Some(&p) = self.procs_by_name.get(&name) {
            if is_auto_include_function(name.as_bytes()) {
                // A function of the auto-included files (`_ENCODEURL$`) as a place: not measured.
                let shown = show_bytes(self.text(t.span));
                return Err(self.unsupported(t.span, format!("`{shown}(...)` here (a FUNCTION of this name exists)")));
            }
            let e = self.placing(what, |c| Ok(c.not_a_variable(t, suffix, Some(p))))?;
            return Err(e);
        }
        if let Some(place) = self.dotted_call(call, t, &name, suffix)? {
            return Ok(place);
        }
        let array = match self.find_array(&name, suffix) {
            Some(a) => a,
            None => {
                let dims = call.arg_list().map_or(0, |l| l.args().len());
                self.placing(what, |c| c.implicit_array(t, &name, suffix, dims, call.node()))?
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

    /// Whether an array is a dynamic member array (design D4 of `m3-typefields`).
    pub(super) fn is_dynamic_member(&self, array: &ArrayRef) -> bool {
        match array {
            ArrayRef::Var(_) => false,
            ArrayRef::Member { base, member } => self.prog.member(self.prog.place_ty(base), *member).dynamic,
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
    pub(super) fn member_element(
        &mut self,
        place: Place,
        args: Option<Option<ast::ArgList>>,
        span: Span,
        t: Tok,
    ) -> R<Place> {
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
        // A dynamic member array has the number of dimensions of its last `REDIM`, checked when the element is
        // used (design D4 of `m3-typefields`; measured, `verification\v35_g_dyn_errors` 9.).
        let dynamic = self.member_of(&place).is_some_and(|m| m.dynamic);
        let dims = if dynamic { args.len() } else { dims };
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
                if self.plain_spelled(f)?.is_some() {
                    return Ok(None);
                }
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
        if !self.user_head(head) {
            return Ok(None);
        }
        if let Some(v) = self.lookup_var(head.to_string(), None)
            && matches!(self.prog.var(v).ty, Ty::User(_))
        {
            self.stmt.names.push((SymbolKind::Var(v), t.span));
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
        unreachable!("`user_head` holds for `{head}` only by a `TYPE` variable or array")
    }

    /// Whether `head`, the part of a dotted name before its first dot, names a `TYPE` variable or array where the
    /// name stands. When it does not, the dotted name is a plain variable's ([`Self::dotted_any`],
    /// [`Self::plain_spelled`]), also in a procedure beside a main-module `TYPE` variable it does not share: a
    /// variable of the procedure's own, of the default type (measured, `verification\v52_x65`, `x66`; QB64pe's
    /// `const_eval.bas` reads `num.typ` so in `LogicalNot&`).
    pub(super) fn user_head(&mut self, head: &str) -> bool {
        self.lookup_var(head.to_string(), None)
            .is_some_and(|v| matches!(self.prog.var(v).ty, Ty::User(_)))
            || matches!(self.array_key_ty(head, None), Ty::User(_)) && self.find_array(head, None).is_some()
    }

    /// The members `path` (`b` or `b.c`, upper case) of `base`; `suffix` is on the last one, which must be its own
    /// type's (measured: another suffix is "Incorrect symbol after element name"). The last one may be a member
    /// array, which the caller gives its index ([`Self::member_element`]); one before it is the error of a member
    /// array without an index.
    fn members(&mut self, mut base: Place, path: &str, suffix: Option<Ty>, t: Tok) -> R<Place> {
        let shown = show_bytes(self.text(t.span));
        // Where each member's name stands, for its symbol: the parts of the token's text after the name before the
        // first dot when the token holds it (`p.xpos`), else all of them (the member token of `a(1).m`).
        let parts = self.dotted_parts(t.span);
        let skip = parts.len().saturating_sub(path.split('.').count());
        for (k, seg) in path.split('.').enumerate() {
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
            if ut.name != MEM_TYPE
                && let Some(&at) = parts.get(skip + k)
            {
                self.stmt.names.push((SymbolKind::Member(tid, MemberId(to_u32(i))), at));
            }
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

    /// The parts of the text at `span` split at its dots, each without the blanks around it and without a type suffix
    /// at its end (`a . b%` gives `a` and `b`).
    fn dotted_parts(&self, span: Span) -> Vec<Span> {
        let text = self.text(span);
        let mut parts = Vec::new();
        let mut start = 0;
        for end in text
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == b'.')
            .map(|(i, _)| i)
            .chain([text.len()])
        {
            let part = &text[start..end];
            let lead = part.iter().take_while(|c| c.is_ascii_whitespace()).count();
            let name = part
                .iter()
                .skip(lead)
                .take_while(|c| c.is_ascii_alphanumeric() || **c == b'_')
                .count();
            let at = span.start + to_u32(start + lead);
            parts.push(Span::new(span.file, at, at + to_u32(name)));
            start = end + 1;
        }
        parts
    }

    /// The dotted name a chain of member accesses on a plain name spells (design D2 of
    /// `m4-devices-continue-paths`): `a . s` and `a . w . v` are `A.S` and `A.W.V`, as `a.s` and `a.w.v` are,
    /// since the old compiler drops the blanks around such a dot (`verification\v19_dot_blanks_*`,
    /// `v47_c_dot_blanks`). The suffix is the last part's; one before a dot is an error, as to the old compiler
    /// (`s$ . x`, "Invalid symbol", `v47_x29`). The token is made to cover the whole name, for spans and the text
    /// shown. `None` when the chain's base is no plain name, or a part before the last has parentheses after it
    /// (`a(1) . s`, `a . s(1) . v`): those keep the path of a member of a place.
    pub(super) fn spelled_name(&mut self, f: ast::FieldExpr) -> R<Option<(String, Option<Ty>, Tok)>> {
        let span = f.node().span();
        let mut parts = vec![self.need(f.member(), span)?];
        let mut cur = f;
        let base = loop {
            match self.need(cur.base(), span)? {
                ast::Expr::NameRef(n) => break self.need(n.name(), span)?,
                ast::Expr::Field(inner) if inner.arg_list().is_none() => {
                    parts.push(self.need(inner.member(), span)?);
                    cur = inner;
                }
                ast::Expr::Literal(_)
                | ast::Expr::Call(_)
                | ast::Expr::Field(_)
                | ast::Expr::Paren(_)
                | ast::Expr::Prefix(_)
                | ast::Expr::Bin(_) => return Ok(None),
            }
        };
        parts.push(base);
        parts.reverse();
        let last = *parts.last().unwrap();
        let t = Tok {
            kind: base.kind,
            span: base.span.cover(last.span),
        };
        let mut name = String::new();
        let mut suffix = None;
        for (i, &p) in parts.iter().enumerate() {
            let (part, s) = self.split_name(p)?;
            if s.is_some() && i + 1 < parts.len() {
                let shown = show_bytes(self.text(t.span));
                return Err(self.error(t.span, format!("`{shown}`: a type suffix before `.`")));
            }
            if i > 0 {
                name.push('.');
            }
            name.push_str(&part);
            suffix = s;
        }
        Ok(Some((name, suffix, t)))
    }

    /// The plain dotted name a member access on a plain name spells when no `TYPE` variable of its first part is
    /// visible ([`Self::spelled_name`], [`Self::dotted_any`]): `p . q` with no `TYPE` variable `p` is the variable
    /// or array `P.Q`. `None` for anything else; a member it names is left to [`Self::field_member`]. A name of a
    /// procedure or a visible `CONST` is "not supported yet": the old compiler drops the blanks before it reads the
    /// statement (`qb64pe.bas` 4196–4241), so `a . b` would be that procedure or constant, which only the reading
    /// of a whole name (`a.b`) resolves here; not measured.
    pub(super) fn plain_spelled(&mut self, f: ast::FieldExpr) -> R<Option<(String, Option<Ty>, Tok)>> {
        let Some((name, suffix, t)) = self.spelled_name(f)? else {
            return Ok(None);
        };
        let head = &name[..name.find('.').expect("a spelled name has a dot")];
        if self.user_head(head) {
            return Ok(None);
        }
        let what = if self.procs_by_name.contains_key(&name) {
            Some("SUB or FUNCTION")
        } else if self.visible_const(&name).is_some() {
            Some("`CONST`")
        } else {
            None
        };
        if let Some(what) = what {
            let shown = show_bytes(self.text(t.span));
            let msg = format!("`{shown}`, the name of a {what} written with blanks around `.`");
            return Err(self.unsupported(t.span, msg));
        }
        Ok(Some((name, suffix, t)))
    }

    /// The FUNCTION a member access on a plain name spells when no `TYPE` variable of its first part is visible:
    /// `a . b(4)` calls the FUNCTION `a.b`, as the old compiler drops the blanks (measured, `verification\v55_x60`;
    /// design D7 of `m4-names-implicit-arrays`). `None` for anything else.
    pub(super) fn spelled_function(&mut self, f: ast::FieldExpr) -> R<Option<(ProcId, Option<Ty>, Tok)>> {
        let Some((name, suffix, t)) = self.spelled_name(f)? else {
            return Ok(None);
        };
        let head = &name[..name.find('.').expect("a spelled name has a dot")];
        if self.user_head(head) {
            return Ok(None);
        }
        let function = self
            .procs_by_name
            .get(&name)
            .filter(|&&p| matches!(self.prog.proc(p).kind, ProcKind::Function(_)));
        Ok(function.map(|&p| (p, suffix, t)))
    }

    /// `base(…).member…` (a `FieldExpr`), with the index of a member array after it (`a(1).v(2)`). On a plain
    /// name with no `TYPE` variable of that name, the plain dotted variable or array element it spells (design D2
    /// of `m4-devices-continue-paths`).
    pub(super) fn field_place(&mut self, f: ast::FieldExpr) -> R<Place> {
        let span = f.node().span();
        if let Some((name, suffix, t)) = self.plain_spelled(f)? {
            let Some(list) = f.arg_list() else {
                return Ok(Place::Var(self.variable(t, name, suffix)?));
            };
            let args = self.present_args(Some(list))?;
            let Some(array) = self.find_array(&name, suffix).filter(|_| !args.is_empty()) else {
                let shown = show_bytes(self.text(t.span));
                return Err(self.unsupported(span, format!("`{shown}(…)`, written with blanks around `.`, here")));
            };
            return self.element_at(array, args, t, span);
        }
        let (place, t) = self.field_member(f)?;
        self.member_element(place, f.arg_list().map(Some), span, t)
    }

    /// The member a `FieldExpr` names, without what stands in parentheses after it, and the member's token (for a
    /// member access on a plain name, one covering the whole name).
    pub(super) fn field_member(&mut self, f: ast::FieldExpr) -> R<(Place, Tok)> {
        let span = f.node().span();
        if let Some((name, suffix, t)) = self.spelled_name(f)? {
            // `a . s`: the member `S` of the `TYPE` variable `a` (design D2 of `m4-devices-continue-paths`).
            return match self.dotted_any(t, &name, suffix)? {
                Some(place) => Ok((place, t)),
                // A plain dotted name written with blanks where only a member was looked for (the callers that
                // take a plain name look with [`Self::plain_spelled`] first).
                None => {
                    let shown = show_bytes(self.text(t.span));
                    Err(self.unsupported(t.span, format!("`{shown}`, written with blanks around `.`, here")))
                }
            };
        }
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
            ast::Expr::Call(c) => self.call_place(c, Placing::Value)?,
            ast::Expr::Field(inner) => self.field_place(inner)?,
            ast::Expr::NameRef(_) => unreachable!("a member access on a plain name is spelled above"),
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

    /// The member a `DimItem`'s path names (`t.v`, `a(1).v`, `x(0).n(1).q`), with the parentheses after its last
    /// part and that part's token: the place before those parentheses, each index on the way evaluated once and
    /// in order, as a `REDIM` (design D8 of `m2-defxxx-common-arrays`) and an `_ARRAYCOPY` (design D1 of
    /// `m3-arraycopy`) name a member array. `None` for an item that names no member: a plain array.
    pub(super) fn member_path_array<'t>(
        &mut self,
        item: ast::DimItem<'t>,
    ) -> R<Option<(Place, Option<ast::ArrayBounds<'t>>, Tok)>> {
        let parts = item.path();
        let Some(&(first, first_parens)) = parts.first() else {
            return Ok(None);
        };
        if parts.len() == 1 && !self.text(first.span).contains(&b'.') {
            return Ok(None);
        }
        let (name, suffix) = self.split_name(first)?;
        let mut index_used = false;
        let mut place = match self.dotted_any(first, &name, suffix)? {
            Some(place) => place,
            None if parts.len() == 1 => return Ok(None),
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
                let dims = match self.member_of(&place) {
                    Some(m) if m.dynamic => args.len(),
                    Some(m) => m.bounds.len(),
                    None => 0,
                };
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
        Ok(Some((place, bounds, parts[last].0)))
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
    ///
    /// A dynamic member array (design D4 of `m3-typefields`) takes new bounds as a dynamic array does
    /// ([`Self::redim_dynamic_member`]).
    pub(super) fn redim_member(
        &mut self,
        item: ast::DimItem,
        mode: AllocMode,
        (listed, alone): (bool, bool),
        stmt: Node,
    ) -> R<bool> {
        let keeps = mode.keeps_elements();
        let span = item.node().span();
        let Some((place, bounds, last_tok)) = self.member_path_array(item)? else {
            return Ok(false);
        };
        let shown = show_bytes(self.text(last_tok.span));
        if self.waits_for_layout(self.prog.var(place.root()).ty) {
            let msg = format!("`REDIM` of `{shown}` above its `TYPE` block, whose bounds name a `CONST`");
            return Err(self.unsupported(span, msg));
        }
        let Some((declared, member_ty, dynamic)) = self
            .member_of(&place)
            .filter(|m| m.is_array())
            .map(|m| (m.bounds.clone(), m.ty, m.dynamic))
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
            Some(a) => Some((self.type_of_at(a, AsPlace::Other)?, a.node().span())),
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
        if dynamic {
            // A type suffix on the member's name stands for `AS` its type (measured, `verification\v35_x32`).
            let typed = as_ty.is_some() || self.split_name(last_tok)?.1.is_some();
            let Place::Member { base, member } = place else {
                unreachable!("a member array is a member");
            };
            let array = ArrayRef::Member { base, member };
            self.redim_dynamic_member(array, bounds, typed, mode, (&shown, alone, span), stmt)?;
            return Ok(true);
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
                // The upper bound alone goes by `OPTION BASE` as it stands at the `REDIM` (measured,
                // `verification\v24_x322`, `x323`). The number 0 has no rule of its own, as in the `TYPE` block:
                // `REDIM t.v(0) AS LONG` under base 1 is 1 TO 0, "Invalid array bounds" (`v64_b1`–`b3`).
                None => self.at.option_base,
            };
            let u = self.constant_bound(upper)?;
            if u < l {
                let msg = format!("invalid bounds for `{shown}`: {l} TO {u}");
                return Err(self.error(upper.node().span(), msg));
            }
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
        self.clear_member_array(array, stmt)?;
        Ok(true)
    }

    /// `REDIM` of a dynamic member array (design D4 and D5 of `m3-typefields`; measured, `verification\v35_a`,
    /// `x29`–`x33`): bounds as a dynamic array's, any number of dimensions, the first one with `lower TO upper`
    /// unless the type is written (`AS` or a suffix; else "REDIM of a TYPE member array must include AS <type> or
    /// explicit lower TO upper bounds"). An [`StmtKind::Allocate`] of the member, which finds the place it is part
    /// of once; beside other targets of its statement as for a clear ([`Self::clear_member_array`]).
    fn redim_dynamic_member(
        &mut self,
        array: ArrayRef,
        bounds: ast::ArrayBounds,
        typed: bool,
        mode: AllocMode,
        (shown, alone, span): (&str, bool, Span),
        stmt: Node,
    ) -> R<()> {
        let ranges = bounds.ranges();
        let Some((first_lower, first_upper)) = ranges.first() else {
            return Err(self.unsupported(span, format!("`REDIM` of the member array `{shown}` without bounds")));
        };
        if first_lower.is_none() && !typed {
            let msg = format!(
                "a `REDIM` of the member array `{shown}` names its bounds as `lower TO upper`, or its type with `AS`"
            );
            let at = first_upper.map_or(bounds.node().span(), |u| u.node().span());
            return Err(self.error(at, msg));
        }
        if array.may_raise() && !alone {
            let msg = "a member array of an array element in a list of `REDIM` targets";
            return Err(self.unsupported(span, msg));
        }
        let dims = self.dim_bounds(bounds, shown)?;
        let kind = StmtKind::Allocate {
            array,
            bounds: dims.into_iter().map(|d| (d.lower, d.upper)).collect(),
            mode,
            join: Join::None,
        };
        self.push(stmt, kind);
        Ok(())
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

    /// The clear of a member array, by `ERASE` or `REDIM`. Finding the array may raise when an index stands on its
    /// way; the old compiler's statement then goes on with its other targets before the error is handled
    /// (measured, `verification\v24_x312`, `v64_a1`), which [`Checker::join_targets`] makes of the targets.
    pub(super) fn clear_member_array(&mut self, array: ArrayRef, stmt: Node) -> R<()> {
        // `ERASE` of a dynamic member array releases it (design D4 of `m3-typefields`). The old compiler evaluates
        // its index twice; here once (`QB64PE-DIVERGENCES.md` D-055, `verification\v64_a1`, `a2`).
        let kind = if self.is_dynamic_member(&array) {
            StmtKind::Release {
                array,
                join: Join::None,
            }
        } else {
            StmtKind::Clear {
                array,
                join: Join::None,
            }
        };
        self.push(stmt, kind);
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
                    self.stmt.names.push((SymbolKind::Var(v), t.span));
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
    pub(super) fn load(&mut self, place: Place, span: Span, use_: Use) -> R<Expr> {
        let ty = self.prog.place_ty(&place);
        if use_ == Use::Place && matches!(ty, Ty::User(_)) {
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

    /// The array `LBOUND` or `UBOUND` names by `name` (written `t`, in `node`).
    fn bound_array(&mut self, t: Tok, name: &str, suffix: Option<Ty>, node: Node, span: Span) -> R<ArrayRef> {
        let a = match self.find_array(name, suffix) {
            Some(a) => a,
            // Measured: `LBOUND(x)` of a name that is no array makes an array of one dimension, used
            // without `DIM` (`v18_f_*`, `v24_f_implicit`).
            None => self.implicit_array(t, name, suffix, 1, node)?,
        };
        // The bounds of an array parameter say nothing about its number of dimensions (measured,
        // `verification\v24_x153`, `x293`), nor do those of an array listed by `COMMON a()` (`x298`,
        // `x301`, `x302`); those of one listed by `STATIC a()` make it one (`x282`, `x297`).
        let common = self.prog.var(a).storage == Storage::Main;
        if !self.open_dims(a) && !common {
            self.settle_dims(a, 1, t, span);
        }
        self.stmt.names.push((SymbolKind::Var(a), t.span));
        Ok(ArrayRef::Var(a))
    }

    /// The diagnostic of `LBOUND` or `UBOUND` (`word`) whose array argument is no array (design D4 of
    /// `m4-upstream-rejections`). Measured (`verification\v51_x28`–`x39`, upstream
    /// `arrays/42_err_solved_bug_Incorrect_nr_of_args`, `43_…`): an element of a member array, of a single value
    /// or of an array, is "Incorrect number of arguments"; an element of an array of a `TYPE` "Expected ."; of
    /// another array, a call of a FUNCTION of the program, a literal and an operation "Syntax error". Any other
    /// expression is not supported yet (`LBOUND((a))` is the array `a`, `x34`).
    fn bound_of_other(&mut self, node: ast::Expr, word: &str) -> R<Failed> {
        let span = node.node().span();
        let msg = match node {
            ast::Expr::Field(f) if f.arg_list().is_some() => {
                let (place, _) = self.field_member(f)?;
                self.is_member_array(&place)
                    .then(|| format!("`{word}` takes a member array without its indexes"))
            }
            ast::Expr::Call(c) => {
                let t = self.need(c.name(), span)?;
                let (name, suffix) = self.split_name(t)?;
                if let Some(place) = self.dotted_any(t, &name, suffix)? {
                    self.is_member_array(&place)
                        .then(|| format!("`{word}` takes a member array without its indexes"))
                } else {
                    match self.find_array(&name, suffix).map(|a| self.prog.var(a).ty) {
                        Some(Ty::User(_)) => Some(format!("`{word}` takes the array's name, not an element of it")),
                        Some(_) => Some(format!("`{word}` takes an array's name, not one of its elements")),
                        None if self.procs_by_name.contains_key(&name) => {
                            Some(format!("`{word}` takes an array, not a value"))
                        }
                        None => None,
                    }
                }
            }
            ast::Expr::Literal(_) | ast::Expr::Bin(_) => Some(format!("`{word}` takes an array, not a value")),
            ast::Expr::NameRef(_) | ast::Expr::Field(_) | ast::Expr::Paren(_) | ast::Expr::Prefix(_) => None,
        };
        Ok(match msg {
            Some(msg) => self.error(span, msg),
            None => self.unsupported(span, format!("`{word}` of this expression")),
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
        let mut args = self.present_args(call.arg_list())?;
        if !(required..=slots.len()).contains(&args.len()) {
            return Err(self.unsupported(span, format!("`{word}` with {} arguments", args.len())));
        }
        // An array's name in parentheses, at any depth, is the array (design D6 of `m4-statements-metacommands`;
        // measured, `verification\v51_x34`, `v57_p08`–`p10`, `p14`, `p15`); any other expression in them keeps
        // its error or mark.
        if let inner @ ast::Expr::NameRef(_) = super::whole::without_parens(args[0]) {
            args[0] = inner;
        }
        let plain = match args[0] {
            ast::Expr::Field(f) if f.arg_list().is_none() => self.plain_spelled(f)?,
            ast::Expr::Literal(_)
            | ast::Expr::NameRef(_)
            | ast::Expr::Call(_)
            | ast::Expr::Field(_)
            | ast::Expr::Paren(_)
            | ast::Expr::Prefix(_)
            | ast::Expr::Bin(_) => None,
        };
        let array = match (self.member_array(args[0])?, args[0]) {
            (Some((array, _, _)), _) => array,
            (None, ast::Expr::NameRef(n)) => {
                let t = self.need(n.name(), span)?;
                let (name, suffix) = self.split_name(t)?;
                self.bound_array(t, &name, suffix, n.node(), span)?
            }
            // `LBOUND(p . q)`: the plain dotted array `P.Q` (design D2 of `m4-devices-continue-paths`).
            (None, ast::Expr::Field(f)) if plain.is_some() => {
                let (name, suffix, t) = plain.unwrap();
                self.bound_array(t, &name, suffix, f.node(), span)?
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
            ) => return Err(self.bound_of_other(other, word)?),
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

    /// `VARPTR`, `VARSEG`, `SADD` and `VARPTR$` (design D3 and D4 of `m3-get-put-bload`, D1–D3 of
    /// `m4-memory-records`; measured, `verification\v28_b_bload`, `v28_d_varptr_index`, `v28_e_cmem_arrays`,
    /// `v28_x36`–`x41`, `x55`–`x57`, `v56_*`). The argument is a variable, an element or a member, never a value.
    /// The variable it is part of, of the main module or of a procedure (design D1 of `qbasic-testcases`;
    /// `verification\v61_a*`), is put into conventional memory by being named here; a parameter's arguments are, after
    /// every body is checked ([`Checker::conventional_args`]). The forms the families did not measure are not
    /// supported yet.
    pub(super) fn var_address(&mut self, call: ast::CallExpr, word: AddressWord, span: Span) -> R<Expr> {
        let name = word.name();
        let args = self.present_args(call.arg_list())?;
        let &[arg] = &args[..] else {
            // Measured: "Incorrect number of arguments" (`v28_x39`).
            return Err(self.error(span, format!("`{name}` takes 1 argument")));
        };
        // An element or variable of a user type is a place here, not a value, as for `LEN` ([`Self::load`]).
        let e = self.expr_as(arg, Use::Place)?;
        self.address_of(e, word, span)
    }

    /// The address `word` gives of an argument already read as a place ([`Self::var_address`]); also a variable
    /// argument of `CALL ABSOLUTE`, as the old compiler takes `VARPTR` of it (`qb64pe.bas` 10704).
    pub(super) fn address_of(&mut self, e: Expr, word: AddressWord, span: Span) -> R<Expr> {
        let name = word.name();
        let ExprKind::Load(place) = e.kind else {
            if word == AddressWord::Sadd {
                // Measured: the old compiler's message for a literal (`v28_x56`).
                return Err(self.error(e.span, "`SADD` needs a variable-length string variable"));
            }
            // Measured: "Expected reference to a variable/array" for a literal, an expression, a function call
            // (`v28_x36`, `x37`, `x40`, `x41`).
            return Err(self.error(e.span, format!("`{name}` needs a variable or an array element")));
        };
        let ty = self.prog.place_ty(&place);
        let whole = matches!(&place, Place::Element { index, .. } if index.is_empty());
        let on_member = place.in_member();
        // The forms each function takes, by the old compiler's rules (`qb64pe.bas` 21306–21560).
        match word {
            AddressWord::Sadd => {
                if ty == Ty::Str && !on_member {
                    // A variable, an element, or `s$()`: its first element (`v56_x11`).
                } else if let Ty::User(_) = ty {
                    // Measured: "User defined types in expressions are invalid" (`v56_x09`).
                    return Err(self.error(e.span, "`SADD` of a `TYPE` variable"));
                } else if matches!(ty, Ty::FixedStr(_)) && !on_member {
                    // Measured: the old compiler's message for a fixed-length string (`v28_x55`, `v56_x05`).
                    return Err(self.error(e.span, "`SADD` needs a variable-length string variable"));
                } else {
                    // A number or a member fails the old compiler's C++ build (`v56_x03`, `x04`).
                    return Err(self.error(e.span, "`SADD` of a number or of a member"));
                }
            }
            AddressWord::VarPtrStr => {
                if let Place::Element {
                    array: ArrayRef::Var(_),
                    ..
                } = &place
                    && ty != Ty::Str
                {
                    // Measured: the old compiler's message (`v28_x57`, `v56_x01`, `x02`).
                    return Err(self.error(e.span, "`VARPTR$` of an element of an array that is not of strings"));
                }
                if on_member && ty == Ty::Str && !place.has_element() {
                    // Measured: the old compiler's C++ build fails (`verification\v60_d8`).
                    return Err(self.error(e.span, "`VARPTR$` of a `STRING` member"));
                }
                if on_member && (place.has_element() || !(ty.is_int() || ty.is_float())) {
                    let msg = "`VARPTR$` of a member of an array element or of a member that is no number";
                    return Err(self.unsupported(e.span, msg));
                }
            }
            AddressWord::VarPtr | AddressWord::VarSeg => {
                if let (AddressWord::VarPtr, Place::Var(_), Ty::User(_)) = (word, &place, ty) {
                    // Measured: the old compiler's internal error (`v56_t02`); its `VARSEG` is 80 (`t03`).
                    return Err(self.error(e.span, "`VARPTR` of a whole `TYPE` variable"));
                }
                if let (Place::Element { .. }, Ty::Bit { .. }) = (&place, ty) {
                    if word == AddressWord::VarPtr {
                        // Measured: "VARPTR cannot reference _BIT type arrays" (`v56_x07`).
                        return Err(self.error(e.span, "`VARPTR` of an element of a `_BIT` array"));
                    }
                    return Err(self.unsupported(e.span, "`VARSEG` of a `_BIT` array"));
                }
                if whole && !(ty.is_int() || ty.is_float()) {
                    let msg = format!("`{name}` of a whole array of strings or of a `TYPE`");
                    return Err(self.unsupported(e.span, msg));
                }
                if (word, on_member, ty) == (AddressWord::VarPtr, true, Ty::Str) {
                    if place.has_element() {
                        // Measured: the old compiler's C++ build fails (`verification\v60_d7`).
                        return Err(self.error(e.span, "`VARPTR` of the `STRING` member of an array element"));
                    }
                    // The offset of the member's string pointer (measured, `verification\v56_x08`).
                } else if on_member && !(ty.is_int() || ty.is_float() || matches!(ty, Ty::FixedStr(_))) {
                    // A fixed-length string member is at its offset, as a number is (measured, `v61_a4`).
                    let msg = format!("`{name}` of a member that is no number");
                    return Err(self.unsupported(e.span, msg));
                }
            }
        }
        // A variable of a procedure is taken as one of the main module (design D1 of `qbasic-testcases`); a
        // parameter's arguments move there after every body is checked ([`Checker::conventional_args`]).
        let root = place.root();
        let v = self.prog.var(root);
        // A `TYPE` that holds a `STRING` is taken, its strings made where the old compiler leaves them out
        // (`QB64PE-DIVERGENCES.md` D-037; design D3 of `m4-left-marked`); one that owns a dynamic member array, and `_MEM`,
        // were not measured.
        if crate::owns_desc(&self.prog.types, v.ty) || self.prog.is_mem(v.ty) {
            let msg =
                format!("`{name}` of a `TYPE` with a dynamic member array or of a `_MEM`, in conventional memory");
            return Err(self.unsupported(e.span, msg));
        }
        if through_dynamic_member(&self.prog, &place) {
            return Err(self.unsupported(e.span, format!("`{name}` of a dynamic member array")));
        }
        self.prog.vars[root.0 as usize].conventional = true;
        // `VARSEG` finds no element: of an array (not of strings), the whole array; of anything else, the variable,
        // but for a parameter of a user type, which may lie outside `dblock`: the place, whose segment libqb finds
        // (`qb64pe.bas` 21550).
        let param_udt =
            matches!(self.prog.var(root).storage, Storage::Param(_)) && matches!(self.prog.var(root).ty, Ty::User(_));
        let of = match (word, &self.prog.var(root).shape) {
            (AddressWord::VarSeg, Shape::Scalar) if !param_udt => Place::Var(root),
            (AddressWord::VarSeg, Shape::Scalar) => place,
            (AddressWord::VarSeg, _) => Place::Element {
                array: ArrayRef::Var(root),
                index: Vec::new(),
            },
            _ => place,
        };
        let ty = if word == AddressWord::VarPtrStr {
            Ty::Str
        } else {
            Ty::I64
        };
        Ok(Expr {
            span,
            ty,
            qb: ty,
            kind: ExprKind::Address { word, of },
        })
    }
}

/// Whether a dynamic member array lies on the way to a place.
fn through_dynamic_member(prog: &Program, place: &Place) -> bool {
    match place {
        Place::Var(_) => false,
        Place::Element {
            array: ArrayRef::Var(_),
            ..
        } => false,
        Place::Element {
            array: ArrayRef::Member { base, member },
            ..
        } => prog.member(prog.place_ty(base), *member).dynamic || through_dynamic_member(prog, base),
        Place::Member { base, .. } => through_dynamic_member(prog, base),
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

/// Whether the bounds of a member array name more than numbers: a `CONST`, or a variable, which is an error.
fn names_constants(bounds: ast::ArrayBounds) -> bool {
    bounds
        .ranges()
        .into_iter()
        .flat_map(|(lower, upper)| lower.into_iter().chain(upper))
        .any(|b| !only_numbers(b))
}

/// The bytes a value of a type takes in the emitter's layout, where a `STRING` is the pointer to its string:
/// what limits the size of a `TYPE`.
fn layout_bytes(types: &[UserType], t: Ty) -> u64 {
    match t {
        Ty::Str => 8,
        // A dynamic member array is the pointer to its descriptor, before alignment.
        Ty::User(id) => types[id.0 as usize]
            .members
            .iter()
            .map(|m| {
                if m.dynamic {
                    8
                } else {
                    layout_bytes(types, m.ty) * u64::from(m.count())
                }
            })
            .sum(),
        crate::numeric_types!() | Ty::FixedStr(_) => u64::from(crate::size_of(types, t)),
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

/// The value of a `STATIC` array's bound (converted to `_INTEGER64`) when it is known when compiling: [`constant`]
/// as a whole number, or `LEN` of a string literal (measured, `verification\v55_x73`), which the old compiler's
/// start code computes from the text it writes.
fn constant_int(e: &Expr) -> Option<i64> {
    let inner = if let ExprKind::Convert {
        how: ConvKind::Widen,
        from,
    } = &e.kind
    {
        from
    } else {
        e
    };
    if let ExprKind::Call { builtin, args } = &inner.kind
        && builtin.get().name.eq_ignore_ascii_case("LEN")
        && let [
            Some(Expr {
                kind: ExprKind::Str(s), ..
            }),
        ] = args.as_slice()
    {
        return i64::try_from(s.len()).ok();
    }
    match constant(e)? {
        Num::Int(v) => Some(v),
        Num::Float(_) => None,
    }
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
                (ConvKind::RoundEven, Num::Int(_)) | (ConvKind::Truncate, Num::Float(_)) | (ConvKind::Cast, _) => None,
            }
        }
        ExprKind::Address { .. }
        | ExprKind::Block { .. }
        | ExprKind::MemOf { .. }
        | ExprKind::MemAt { .. }
        | ExprKind::AddressOf(_)
        | ExprKind::MemRead { .. }
        | ExprKind::ArrayBytes(_) => None,
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
