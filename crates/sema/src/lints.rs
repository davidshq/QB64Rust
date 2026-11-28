//! Lint warnings (spec `compiler/lints`, OpenSpec change `lints`): code that compiles but is probably a mistake.
//!
//! They run once, on the finished program, and only when the program has no error (design D1). Our own lints are
//! given only for places in the file being compiled, not in its included files (design D3). Each warning carries
//! its lint's code (design D2).

use crate::consteval::Value;
use crate::{
    AbsArg, Arg, BinOp, CaseItem, ConvKind, Event, Expr, ExprKind, Handler, InputSource, KeyForm, MemValue, Place,
    PrintItem, PrintTo, ProcId, ProcKind, Program, RunTarget, Stmt, StmtArg, StmtKind, Storage, SymbolKind, Ty, UnOp,
    UsingValue, VarId,
};
use qb64rust_base::{Diagnostic, Diagnostics, FileId, SourceMap, Span, show_bytes, to_u32};
use qb64rust_syntax::tree::Node;
use qb64rust_syntax::{ParsedProgram, SyntaxKind};
use std::collections::{HashMap, HashSet};

/// A variable that no declaration made, named once in the whole program.
pub const USED_ONCE: &str = "used-once";
/// A SUB or FUNCTION that nothing calls.
pub const UNUSED_PROCEDURE: &str = "unused-procedure";
/// A named label that nothing names.
pub const UNUSED_LABEL: &str = "unused-label";
/// A `DIM` or `REDIM` in a procedure that hides a main-module `DIM SHARED` variable.
pub const HIDES_SHARED: &str = "hides-shared";
/// Statements after one that always leaves its statement list.
pub const UNREACHABLE: &str = "unreachable";
/// A file that declares no encoding, is UTF-8, and holds a byte above 127 in a string (OpenSpec change
/// `utf8-source`, design D7).
pub const UNDECLARED_UTF8: &str = "undeclared-utf8";
/// A BASIC PDS 7.1 function called with no argument, which QB64 reads as a variable that is never set.
pub const PDS_FUNCTION: &str = "pds-function";
/// A local variable of a SUB or FUNCTION that is read but never set.
pub const NEVER_SET: &str = "never-set";
/// A SINGLE variable set to a fraction that a SINGLE cannot hold exactly, in a product stored into an integer.
pub const SINGLE_FRACTION: &str = "single-fraction";

/// QB64pe's "Unused variable" (design D11).
pub const UNUSED_VARIABLE: &str = "unused-variable";
/// QB64pe's "Empty SELECT CASE block".
pub const EMPTY_SELECT: &str = "empty-select";
/// QB64pe's "Duplicate constant definition".
pub const DUPLICATE_CONST: &str = "duplicate-const";
/// QB64pe's "Feature incompatible with $DEBUG mode".
pub const DEBUG_INCOMPATIBLE: &str = "debug-incompatible";

/// Every lint code, in the order `--report warnings` lists them: ours, then QB64pe's.
pub const CODES: &[&str] = &[
    USED_ONCE,
    UNUSED_PROCEDURE,
    UNUSED_LABEL,
    HIDES_SHARED,
    UNREACHABLE,
    UNDECLARED_UTF8,
    PDS_FUNCTION,
    NEVER_SET,
    SINGLE_FRACTION,
    UNUSED_VARIABLE,
    EMPTY_SELECT,
    DUPLICATE_CONST,
    DEBUG_INCOMPATIBLE,
];

/// Gives the lint warnings of `prog`, whose main file is `main`, into `diags`. `parsed` is the program as parsed,
/// for the words of its skipped `$IF` branches ([`inactive_words`]).
pub(crate) fn run(prog: &Program, parsed: &ParsedProgram, map: &SourceMap, main: FileId, diags: &mut Diagnostics) {
    let inactive = inactive_words(parsed, map);
    let mut ours: Vec<Lint> = Vec::new();
    let set = set_vars(prog);
    let (pds, pds_vars) = pds_functions(prog, map, &set);
    used_once(prog, map, &inactive, &pds_vars, &mut ours);
    unused_procedures(prog, map, main, &inactive, &mut ours);
    unused_labels(prog, map, &inactive, &mut ours);
    hides_shared(prog, map, &mut ours);
    unreachable(prog, map, main, &mut ours);
    // The main-file filter (design D3).
    for (span, code, msg) in ours.drain(..) {
        if span.file == main {
            diags.push(Diagnostic::lint(span, code, msg));
        }
    }
    // For every file of the program (spec `compiler/lints`, "Only the file being compiled").
    for (span, code, msg) in undeclared_utf8(parsed, map) {
        diags.push(Diagnostic::lint(span, code, msg));
    }
    for (span, code, msg) in pds {
        diags.push(Diagnostic::lint(span, code, msg));
    }
    // Both point at a value the program likely did not mean, wherever it stands: a game's own code may be all in
    // included files.
    let mut likely_bugs = never_set(prog, map, &inactive, &pds_vars, &set);
    likely_bugs.extend(single_fractions(prog, map));
    for (span, code, msg) in likely_bugs {
        diags.push(Diagnostic::lint(span, code, msg));
    }
    // QB64pe's warnings, wherever QB64pe gives them, included files too.
    let mut theirs = unused_variables(prog, map);
    theirs.extend(prog.qb64pe_warnings.iter().cloned());
    for (span, code, msg) in theirs {
        diags.push(Diagnostic::lint(span, code, msg));
    }
}

/// `undeclared-utf8` (spec `compiler/lints`, "UTF-8 text in a file that declares nothing"; design D7 of
/// `utf8-source`): one warning for each file that has no `'$ENCODING` line and no settings file's `encoding`,
/// whose bytes are all UTF-8, at its first string literal or `DATA` text that holds a byte above 127. It names
/// how many characters the first such character is in the program, which reads it as code page 437.
fn undeclared_utf8(parsed: &ParsedProgram, map: &SourceMap) -> Vec<Lint> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    for tree in &parsed.trees {
        let file = tree.file;
        let declared = qb64rust_syntax::encoding::file_encoding(map, file, tree.encoding).by;
        if !seen.insert(file) || declared != qb64rust_syntax::encoding::DeclaredBy::Nothing {
            continue;
        }
        let bytes = &map.file(file).bytes;
        if bytes.is_ascii() || qb64rust_base::cp437::first_bad_utf8(bytes).is_some() {
            continue;
        }
        let first = tree.root().tokens().into_iter().find_map(|t| {
            if !matches!(t.kind, SyntaxKind::StringLit | SyntaxKind::DataText) {
                return None;
            }
            let text = map.text(t.span);
            let i = text.iter().position(|&b| b > 127)?;
            let (ch, len) = qb64rust_base::cp437::next_char(&text[i..])?;
            Some((t.span, ch, len))
        });
        let Some((span, ch, len)) = first else {
            continue;
        };
        out.push((
            span,
            UNDECLARED_UTF8,
            format!(
                "this file looks like UTF-8 but is read as code page 437, so `{ch}` is {len} characters in the \
                 program: declare it with a line `'$ENCODING:UTF8` or with `encoding = \"utf-8\"` under \
                 `[project]` in `qb64rust.toml`"
            ),
        ));
    }
    out
}

/// The BASIC PDS 7.1 functions that a program can call with no argument, as written. PDS 7.1 reads each bare
/// name as its function (`verification\qb45\q45_pds_bare_functions`). QB64pe has none of them, and builds each
/// bare name as a variable with no message (`verification\v75_b1_pds_bare_functions`).
const PDS_FUNCTIONS: &[&str] = &["CURDIR$", "DIR$", "STACK", "SAVEPOINT"];

/// The [`PDS_FUNCTIONS`] name at `span`, if it is one: the name as written, in any letter case.
fn pds_function(map: &SourceMap, span: Span) -> Option<&'static str> {
    let name = written(map, span).to_ascii_uppercase();
    PDS_FUNCTIONS.iter().copied().find(|n| *n == name)
}

/// The value a variable of type `ty` holds when nothing sets it, as the warning writes it: none for a fixed-length
/// string or a user type, which an implicit variable never is.
fn never_set_value(ty: Ty) -> Option<&'static str> {
    match ty {
        Ty::Str => Some("\"\""),
        Ty::FixedStr(_) | Ty::User(_) => None,
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
        | Ty::F80 => Some("0"),
    }
}

/// The variables a statement of the program may set: the place of an assignment, a whole-record copy, `FOR`,
/// `INPUT`, `READ`, `LSET`, `FIELD` or `_MEMGET`, a place a built-in statement stores into or exchanges (`SWAP`,
/// `MID$`), the variable of a file's `GET`, and a variable passed by reference to a SUB or FUNCTION that sets its
/// parameter, or to a `DECLARE LIBRARY` routine, which cannot be looked into. A parameter passed on is followed. A
/// variable whose memory the program takes (`VARPTR`, `_MEM`, `_OFFSET`, `CALL ABSOLUTE`) may be written through it,
/// so it counts as set.
fn set_vars(prog: &Program) -> HashSet<VarId> {
    let mut set = HashSet::new();
    // A variable passed by reference to a SUB or FUNCTION, and the parameter it is passed as.
    let mut passed: Vec<(VarId, VarId)> = Vec::new();
    let mut by_ref = |proc: ProcId, args: &[Arg], set: &mut HashSet<VarId>| {
        let p = prog.proc(proc);
        if p.external.is_some() {
            set.extend(args.iter().filter_map(Arg::root));
        } else {
            passed.extend(args.iter().zip(&p.params).filter_map(|(a, &q)| Some((a.root()?, q))));
        }
    };
    each_expr(prog, &mut |e| {
        if let ExprKind::CallProc { proc, args } = &e.kind {
            by_ref(*proc, args, &mut set);
        } else if let ExprKind::Address { of, .. } = &e.kind {
            set.insert(of.root());
        } else if let ExprKind::MemOf { of, .. } | ExprKind::AddressOf(of) = &e.kind {
            set.insert(of.root());
        }
    });
    each_list(prog, &mut |list| {
        for s in list {
            if let StmtKind::Assign { place, .. }
            | StmtKind::CopyWhole { to: place, .. }
            | StmtKind::For { var: place, .. }
            | StmtKind::Lset { target: place, .. } = &s.kind
            {
                set.insert(place.root());
            } else if let StmtKind::Input { targets, .. } | StmtKind::Read(targets) = &s.kind {
                set.extend(targets.iter().map(Place::root));
            } else if let StmtKind::Field { parts, .. } = &s.kind {
                set.extend(parts.iter().map(|(p, _)| p.root()));
            } else if let StmtKind::MemGet { target, .. } = &s.kind {
                set.insert(target.root());
            } else if let StmtKind::Call { proc, args } = &s.kind {
                by_ref(*proc, args, &mut set);
            } else if let StmtKind::Builtin { id, args } = &s.kind {
                // `PUT` takes its variable's bytes as `GET` does, but only reads them.
                let get = id.get().name.eq_ignore_ascii_case("GET");
                for a in args {
                    if let StmtArg::Place(p) = a {
                        set.insert(p.root());
                    } else if let StmtArg::Bytes(p) = a
                        && get
                    {
                        set.insert(p.root());
                    }
                }
            }
        }
    });
    // Each round adds the variables passed as a parameter that is set; the set only grows, so this ends.
    loop {
        let before = set.len();
        for &(var, param) in &passed {
            if set.contains(&param) {
                set.insert(var);
            }
        }
        if set.len() == before {
            return set;
        }
    }
}

/// `pds-function` (spec `compiler/lints`, "A BASIC PDS 7.1 function read as a variable"): each name of a scalar
/// variable that no declaration made, that no statement sets ([`set_vars`]), and that is written as one of
/// [`PDS_FUNCTIONS`]. A program ported from PDS that calls `CURDIR$` builds without it, so the warning goes at
/// every use, in every file: each is a call to replace. A variable the program sets is its own. Also gives the
/// variables reported, which get no `used-once`.
fn pds_functions(prog: &Program, map: &SourceMap, set: &HashSet<VarId>) -> (Vec<Lint>, HashSet<VarId>) {
    let mut out = Vec::new();
    let mut reported = HashSet::new();
    for (_, s) in prog.symbols.iter() {
        let SymbolKind::Var(v) = s.kind else { continue };
        let var = prog.var(v);
        if !var.implicit || var.is_array() {
            continue;
        }
        let Some(name) = pds_function(map, s.def) else {
            continue;
        };
        if set.contains(&v) {
            continue;
        }
        let msg = match never_set_value(var.ty) {
            Some(value) => format!(
                "`{name}` is a BASIC PDS 7.1 function that QB64 does not have: here it is a variable that is never \
                 set, so it is always {value}"
            ),
            None => format!(
                "`{name}` is a BASIC PDS 7.1 function that QB64 does not have: here it is a variable that is never \
                 set"
            ),
        };
        for &at in std::iter::once(&s.def).chain(&s.refs) {
            out.push((at, PDS_FUNCTION, msg.clone()));
        }
        reported.insert(v);
    }
    out.sort_by_key(|(at, _, _)| (at.file, at.start));
    (out, reported)
}

/// `never-set` (spec `compiler/lints`, "A local variable that is never set"): a scalar `DIM`, `STATIC` or implicit
/// local of a SUB or FUNCTION that is read but that no statement sets ([`set_vars`]), so it always holds 0 or `""`.
/// One warning per variable, at its first read. When a `DIM SHARED` array has its name, the warning names it: a read
/// of `side_cash` where `side_cash(side)` was meant is the case Civil War Strategy's port met (`ai_fortify`). Left
/// out: a fixed-length string and a record, which a program may keep blank on purpose to write a blank record; a
/// variable whose memory the program takes; a name of a skipped `$IF` branch; and a variable in `pds`, which gets
/// `pds-function`. An implicit one named once gets `used-once` instead, a declared one never named again QB64pe's
/// "Unused variable".
fn never_set(
    prog: &Program,
    map: &SourceMap,
    inactive: &HashSet<String>,
    pds: &HashSet<VarId>,
    set: &HashSet<VarId>,
) -> Vec<Lint> {
    let shared_arrays: HashSet<&str> = prog
        .shared
        .iter()
        .map(|&v| prog.var(v))
        .filter(|v| v.is_array())
        .map(|v| v.name.as_str())
        .collect();
    let mut out = Vec::new();
    for (_, s) in prog.symbols.iter() {
        let SymbolKind::Var(v) = s.kind else { continue };
        let var = prog.var(v);
        if !matches!(var.storage, Storage::Local(_) | Storage::Static(_))
            || var.is_array()
            || var.conventional
            || set.contains(&v)
            || pds.contains(&v)
            || in_inactive(map, s.def, inactive)
        {
            continue;
        }
        let Some(value) = never_set_value(var.ty) else {
            continue;
        };
        // A declared variable's first name is its declaration; an implicit one's is its first read.
        let reads = std::iter::once(&s.def)
            .filter(|_| var.implicit)
            .chain(&s.refs)
            .copied()
            .filter(|&sp| !prog.symbols.is_declaring(sp));
        if var.implicit && s.refs.is_empty() {
            continue;
        }
        let Some(at) = reads.min_by_key(|sp| (sp.file, sp.start)) else {
            continue;
        };
        let shown = written(map, at);
        let mut msg = format!("`{shown}` is never set, so it is always {value}");
        if shared_arrays.contains(var.name.as_str()) {
            msg.push_str(&format!(": is the shared array `{}()` meant?", without_suffix(&shown)));
        }
        out.push((at, NEVER_SET, msg));
    }
    out.sort_by_key(|(at, _, _)| (at.file, at.start));
    out
}

/// `single-fraction` (spec `compiler/lints`, "A SINGLE that holds a fraction"): a scalar SINGLE variable that an
/// assignment sets to a fraction a SINGLE cannot hold exactly (`.3`, not `.5`), named as a factor of a product that
/// is then rounded to an integer by a store. A fraction written out is a DOUBLE, so `r% = .3 * 95` gives 28, but
/// `s! = .3: r% = s! * 95` gives 29: the SINGLE holds 0.300000012. Civil War Strategy's port met this when its
/// numbers became tuning knobs (its `docs\qb64-friction.md`, section 7). One warning per such factor. A SINGLE that
/// an assignment computes with an operator, or that a `FOR` counts, is left out: its value is no number written
/// out that it stands for, as `pct! = .005 * n` in the original game is not. Only a product directly under the
/// rounding is looked at, through `+`, `-`, unary minus, `_IIF` and conversions, not one inside a function's
/// argument.
fn single_fractions(prog: &Program, map: &SourceMap) -> Vec<Lint> {
    // Each such SINGLE, with the first fraction assigned to it, and the SINGLEs that an assignment computes with
    // arithmetic or a `FOR` counts: their value is no number given to them.
    let mut holds: HashMap<VarId, (&Expr, bool)> = HashMap::new();
    let mut computed = HashSet::new();
    each_list(prog, &mut |list| {
        for s in list {
            if let StmtKind::Assign {
                place: Place::Var(v),
                value,
            } = &s.kind
                && prog.var(*v).ty == Ty::F32
            {
                if let Some(lit) = inexact_fraction(value) {
                    holds.entry(*v).or_insert(lit);
                } else if arithmetic(value) {
                    computed.insert(*v);
                }
            } else if let StmtKind::For { var: Place::Var(v), .. } = &s.kind {
                computed.insert(*v);
            }
        }
    });
    holds.retain(|v, _| !computed.contains(v));
    let mut out = Vec::new();
    if holds.is_empty() {
        return out;
    }
    each_list(prog, &mut |list| {
        for s in list {
            if let StmtKind::Assign { value, .. } = &s.kind
                && let Some(rounded) = rounded_to_integer(value)
            {
                products(rounded, &mut |factor| {
                    let ExprKind::Load(Place::Var(v)) = &factor.kind else {
                        return;
                    };
                    let Some(&(lit, negated)) = holds.get(v) else {
                        return;
                    };
                    let name = written(map, factor.span);
                    let (line, _) = map.file(lit.span.file).line_col(lit.span.start);
                    let place = if lit.span.file == factor.span.file {
                        format!("line {line}")
                    } else {
                        format!("line {line} of {}", map.file(lit.span.file).name)
                    };
                    let sign = if negated { "-" } else { "" };
                    let written_text = written(map, lit.span);
                    let text = format!("{sign}{written_text}");
                    // A `CONST`'s name is followed by its value: `RATE (0.3)`, `-RATE (-0.3)`.
                    let named = written_text.starts_with(|ch: char| ch.is_ascii_alphabetic() || ch == '_');
                    let set_to = if let ExprKind::Float(c) = &lit.kind
                        && named
                    {
                        let value = c
                            .parse::<f64>()
                            .map_or_else(|_| format!("{sign}{c}"), |x| (if negated { -x } else { x }).to_string());
                        format!("{text} ({value})")
                    } else {
                        text.clone()
                    };
                    let msg = format!(
                        "`{name}` is a SINGLE set to {set_to} on {place}, which a SINGLE cannot hold exactly: this \
                         product, rounded to an integer, can differ from the same product with {text} written out, \
                         which is a DOUBLE; a DOUBLE variable gives the same result as the number"
                    );
                    out.push((factor.span, SINGLE_FRACTION, msg));
                });
            }
        }
    });
    out.sort_by_key(|(at, _, _)| (at.file, at.start));
    out
}

/// The literal of a value that is a fraction a SINGLE cannot hold exactly, written out or through a `CONST`, after
/// any conversion and unary minus, and whether an odd number of minus signs stood before it (`-KNOB`).
fn inexact_fraction(e: &Expr) -> Option<(&Expr, bool)> {
    if let ExprKind::Convert { from, .. } = &e.kind {
        return inexact_fraction(from);
    }
    if let ExprKind::Unary { op: UnOp::Neg, operand } = &e.kind {
        return inexact_fraction(operand).map(|(lit, negated)| (lit, !negated));
    }
    let ExprKind::Float(text) = &e.kind else {
        return None;
    };
    // C form, `0.3E+0` (`literal::NumLit::Float`).
    let x: f64 = text.parse().ok()?;
    #[expect(clippy::cast_possible_truncation, reason = "the SINGLE the value becomes")]
    let single = x as f32;
    (x.fract() != 0.0 && f64::from(single) != x).then_some((e, false))
}

/// Whether a value is computed by an operator, after any conversion: `.005 * n`, not `.3`, `VAL(v$)` or `t!`.
fn arithmetic(e: &Expr) -> bool {
    if let ExprKind::Convert { from, .. } = &e.kind {
        return arithmetic(from);
    }
    if let ExprKind::Unary { operand, .. } = &e.kind {
        return !matches!(operand.kind, ExprKind::Float(_) | ExprKind::Int(_));
    }
    matches!(
        e.kind,
        ExprKind::Binary { .. } | ExprKind::Concat(..) | ExprKind::StrCompare { .. } | ExprKind::Choice { .. }
    )
}

/// The float value that a store rounds to an integer, half to even, under any narrowing of the integer.
fn rounded_to_integer(e: &Expr) -> Option<&Expr> {
    let ExprKind::Convert { how, from } = &e.kind else {
        return None;
    };
    match how {
        ConvKind::RoundEven => Some(from),
        ConvKind::Truncate => rounded_to_integer(from),
        ConvKind::Widen | ConvKind::Nearest | ConvKind::Cast => None,
    }
}

/// Calls `f` on each factor of each product in `e`, through `+`, `-`, unary minus, `_IIF`'s parts and conversions,
/// each factor without its conversions.
fn products<'e>(e: &'e Expr, f: &mut impl FnMut(&'e Expr)) {
    fn bare(e: &Expr) -> &Expr {
        if let ExprKind::Convert { from, .. } = &e.kind {
            bare(from)
        } else {
            e
        }
    }
    if let ExprKind::Binary {
        op: BinOp::Mul,
        lhs,
        rhs,
    } = &e.kind
    {
        for side in [lhs, rhs] {
            f(bare(side));
            products(side, f);
        }
    } else if let ExprKind::Binary {
        op: BinOp::Add | BinOp::Sub,
        lhs,
        rhs,
    }
    | ExprKind::Choice {
        then: lhs, else_: rhs, ..
    } = &e.kind
    {
        products(lhs, f);
        products(rhs, f);
    } else if let ExprKind::Convert { from, .. }
    | ExprKind::Unary {
        op: UnOp::Neg,
        operand: from,
    } = &e.kind
    {
        products(from, f);
    }
}

/// Calls `f` on every expression of the program, and on each part of one, an expression before its parts.
fn each_expr<'p>(prog: &'p Program, f: &mut impl FnMut(&'p Expr)) {
    each_list(prog, &mut |list| {
        for s in list {
            stmt_exprs(s, &mut |e| expr_tree(e, f));
        }
    });
}

/// Calls `f` on `e` and on each of its parts, `e` first.
fn expr_tree<'p>(e: &'p Expr, f: &mut impl FnMut(&'p Expr)) {
    f(e);
    let mut each = |x: &'p Expr| expr_tree(x, &mut *f);
    match &e.kind {
        ExprKind::Int(_) | ExprKind::Float(_) | ExprKind::Str(_) | ExprKind::ArrayBytes(_) => {}
        ExprKind::Load(p) | ExprKind::Address { of: p, .. } => p.indexes().into_iter().for_each(each),
        ExprKind::Bound { array, dim, .. } => array.indexes().into_iter().chain(dim.as_deref()).for_each(each),
        ExprKind::Block { index, .. } => index.iter().for_each(each),
        ExprKind::Convert { from, .. } | ExprKind::Unary { operand: from, .. } => each(from),
        ExprKind::Binary { lhs, rhs, .. } | ExprKind::Concat(lhs, rhs) | ExprKind::StrCompare { lhs, rhs, .. } => {
            each(lhs);
            each(rhs);
        }
        ExprKind::Choice { condition, then, else_ } => {
            each(condition);
            each(then);
            each(else_);
        }
        ExprKind::Call { args, .. } => args.iter().flatten().for_each(each),
        ExprKind::CallProc { args, .. } => arg_exprs(args).into_iter().for_each(each),
        ExprKind::MemOf { of, .. } | ExprKind::AddressOf(of) => of.indexes().into_iter().for_each(each),
        ExprKind::MemAt { offset, size } => {
            each(offset);
            each(size);
        }
        ExprKind::MemRead { block, offset, .. } => block.indexes().into_iter().chain([&**offset]).for_each(each),
    }
}

/// The expressions of arguments: a value, and the indexes of a place or array passed.
fn arg_exprs(args: &[Arg]) -> Vec<&Expr> {
    let mut out = Vec::new();
    for a in args {
        match a {
            Arg::Ref(p) => out.extend(p.indexes()),
            Arg::Temp(e, _) | Arg::Value(e) => out.push(e),
            Arg::Array(a) => out.extend(a.indexes()),
            Arg::Null => {}
        }
    }
    out
}

/// Calls `f` on each expression a statement holds itself, not those of the statement lists inside it, which
/// [`each_list`] gives: its values, conditions, arguments and the indexes of its places.
fn stmt_exprs<'p>(s: &'p Stmt, f: &mut impl FnMut(&'p Expr)) {
    let mut out: Vec<&'p Expr> = Vec::new();
    let print = |items: &'p [PrintItem], out: &mut Vec<&'p Expr>| {
        for item in items {
            match item {
                PrintItem::Str(e) | PrintItem::Num(e) => out.push(e),
                PrintItem::Zone => {}
            }
        }
    };
    let print_to = |to: &'p PrintTo, out: &mut Vec<&'p Expr>| match to {
        PrintTo::File(e) => out.push(e),
        PrintTo::Screen | PrintTo::Printer => {}
    };
    let mem_value = |v: &'p MemValue, out: &mut Vec<&'p Expr>| match v {
        MemValue::Typed { value, .. } | MemValue::Str(value) => out.push(value),
        MemValue::Bytes(r) => out.extend(r.indexes()),
    };
    match &s.kind {
        StmtKind::Builtin { args, .. } => {
            for a in args {
                match a {
                    StmtArg::Value(e) => out.push(e),
                    StmtArg::Place(p) | StmtArg::Rest(p) | StmtArg::Bytes(p) => out.extend(p.indexes()),
                    StmtArg::Block { index, .. } => out.extend(index),
                    StmtArg::Word(_) | StmtArg::Absent => {}
                }
            }
        }
        StmtKind::MemGet { block, offset, target } => {
            out.extend(block.indexes());
            out.push(offset);
            out.extend(target.indexes());
        }
        StmtKind::MemPut { block, offset, value } => {
            out.extend(block.indexes());
            out.push(offset);
            mem_value(value, &mut out);
        }
        StmtKind::MemFill {
            block,
            offset,
            bytes,
            value,
        } => {
            out.extend(block.indexes());
            out.extend([offset, bytes]);
            mem_value(value, &mut out);
        }
        StmtKind::Run(RunTarget::File(e)) | StmtKind::Log { msg: e, .. } | StmtKind::Error(e) => out.push(e),
        StmtKind::Assign { place, value }
        | StmtKind::Lset {
            target: place, value, ..
        } => {
            out.extend(place.indexes());
            out.push(value);
        }
        StmtKind::CopyWhole { from, to } => {
            out.extend(from.indexes());
            out.extend(to.indexes());
        }
        StmtKind::Allocate { array, bounds, .. } => {
            out.extend(array.indexes());
            out.extend(bounds.iter().flat_map(|(a, b)| [a, b]));
        }
        StmtKind::Clear { array, .. } | StmtKind::Release { array, .. } => out.extend(array.indexes()),
        StmtKind::CopyArray { from, to } => {
            out.extend(from.indexes());
            out.extend(to.indexes());
        }
        StmtKind::ArrayCopy {
            from,
            to,
            ranges,
            starts,
        } => {
            out.extend(from.indexes());
            out.extend(to.indexes());
            out.extend(ranges.iter().flat_map(|(a, b)| [a, b]));
            out.extend(starts);
        }
        StmtKind::Print { to, items, .. } => {
            print_to(to, &mut out);
            print(items, &mut out);
        }
        StmtKind::PrintUsing {
            to,
            before,
            format,
            values,
            ..
        } => {
            print_to(to, &mut out);
            print(before, &mut out);
            out.push(format);
            for v in values {
                match v {
                    UsingValue::Str(e) | UsingValue::Num { value: e, .. } | UsingValue::Blanks(e) => out.push(e),
                }
            }
        }
        StmtKind::Field { file, parts } => {
            out.push(file);
            for (place, width) in parts {
                out.extend(place.indexes());
                out.push(width);
            }
        }
        StmtKind::Write { to, items, .. } => {
            out.extend(to);
            print(items, &mut out);
        }
        StmtKind::Input { from, targets, .. } => {
            if let InputSource::File(e) = from {
                out.push(e);
            }
            out.extend(targets.iter().flat_map(Place::indexes));
        }
        StmtKind::Read(places) => out.extend(places.iter().flat_map(Place::indexes)),
        StmtKind::End(e) | StmtKind::System(e) => out.extend(e),
        StmtKind::Call { args, .. } => out.extend(arg_exprs(args)),
        StmtKind::CallAbsolute { args, offset } => {
            for a in args {
                match a {
                    AbsArg::Address(e) | AbsArg::Value(e, _) => out.push(e),
                }
            }
            out.push(offset);
        }
        StmtKind::PaletteUsing { index, .. } => out.extend(index),
        StmtKind::If { branches, .. } => out.extend(branches.iter().map(|b| &b.cond)),
        StmtKind::For {
            var, start, end, step, ..
        } => {
            out.extend(var.indexes());
            out.extend([start, end]);
            out.extend(step);
        }
        StmtKind::Do { test, .. } => out.extend(test.iter().map(|t| &t.cond)),
        StmtKind::While { cond, .. } => out.push(cond),
        StmtKind::OnJump { value, .. } => out.push(value),
        StmtKind::OnEvent { event, handler } => {
            match event {
                Event::Timer { number, seconds } => {
                    out.extend(number);
                    out.push(seconds);
                }
                Event::Key { key } => out.push(key),
                Event::Strig { button, controller } => {
                    out.push(button);
                    out.extend(controller);
                }
            }
            if let Handler::Sub { arg: Some(e), .. } = handler {
                out.push(e);
            }
        }
        StmtKind::Key(KeyForm::Assign { number, text }) => out.extend([number, text]),
        StmtKind::Select { selector, cases, .. } => {
            out.push(selector);
            for c in cases {
                for item in &c.items {
                    match item {
                        CaseItem::Is(_, e) => out.push(e),
                        CaseItem::Range(a, b) => out.extend([a, b]),
                    }
                }
            }
        }
        // No expression of its own.
        StmtKind::Run(RunTarget::Restart | RunTarget::Label(_))
        | StmtKind::Key(KeyForm::On | KeyForm::Off | KeyForm::List)
        | StmtKind::ConsoleOnly
        | StmtKind::EraseParam { .. }
        | StmtKind::Restore(_)
        | StmtKind::Stop
        | StmtKind::Exit
        | StmtKind::OnError(_)
        | StmtKind::Resume(_)
        | StmtKind::Label(_)
        | StmtKind::Goto(_)
        | StmtKind::Gosub(_)
        | StmtKind::Return(_)
        | StmtKind::ExitLoop(_)
        | StmtKind::Continue
        | StmtKind::ExitSelect { .. } => {}
    }
    out.into_iter().for_each(f);
}

/// A constant's value as QB64pe's "Duplicate constant definition" shows it (`qb64pe.bas` 2662–2679): an integer
/// in decimal, a float by `_TOSTR$`, a string as its quoted text, a comma and its length (measured,
/// `verification\v68_f1_const_same`: `s = "x",1`).
pub(crate) fn const_value_text(ty: Ty, value: &Value) -> String {
    match value {
        Value::Int(i) if ty.is_unsigned() => i.cast_unsigned().to_string(),
        Value::Int(i) => i.to_string(),
        Value::UInt(u) => u.to_string(),
        Value::Float(r, _) => r.lo.to_g19(),
        Value::Str(b) => format!("\"{}\",{}", show_bytes(b), b.len()),
    }
}

/// The words of every `$IF` branch the program skips, in capitals: each run of letters, digits, `_` and `.`, and
/// also each part of it between dots. A name in such a branch may be used on another platform or with other flags:
/// a `$IF WIN THEN` branch that calls `WinSetup` and a `$ELSE` branch that calls `LinuxSetup`. So `used-once`,
/// `unused-procedure` and `unused-label` say nothing about it (found in the review of `lints`). The words are taken from the text, comments and strings
/// included: a word too many only keeps a warning back.
fn inactive_words(parsed: &ParsedProgram, map: &SourceMap) -> HashSet<String> {
    fn walk(node: Node, map: &SourceMap, out: &mut HashSet<String>) {
        for child in node.child_nodes() {
            if child.kind() != SyntaxKind::InactiveCode {
                walk(child, map, out);
                continue;
            }
            let text = map.text(child.span()).to_ascii_uppercase();
            for word in text.split(|&b| !(b.is_ascii_alphanumeric() || b == b'_' || b == b'.')) {
                if word.is_empty() {
                    continue;
                }
                out.insert(show_bytes(word));
                for part in word.split(|&b| b == b'.').filter(|p| !p.is_empty()) {
                    out.insert(show_bytes(part));
                }
            }
        }
    }
    let mut out = HashSet::new();
    for tree in &parsed.trees {
        walk(tree.root(), map, &mut out);
    }
    out
}

/// Whether the name at `span` is one of the `inactive` words ([`inactive_words`]): its letters before any type
/// suffix, in capitals.
fn in_inactive(map: &SourceMap, span: Span, inactive: &HashSet<String>) -> bool {
    !inactive.is_empty() && inactive.contains(&without_suffix(&written(map, span)).to_ascii_uppercase())
}

/// The suffix QB64pe writes after a variable's name in "Unused variable" (measured, `verification\v68_a2_types`):
/// the type's own symbol, none for a user type and `_MEM`.
fn type_suffix(ty: Ty) -> String {
    match ty {
        Ty::I8 => "%%".into(),
        Ty::U8 => "~%%".into(),
        Ty::I16 => "%".into(),
        Ty::U16 => "~%".into(),
        Ty::I32 => "&".into(),
        Ty::U32 => "~&".into(),
        Ty::I64 => "&&".into(),
        Ty::U64 => "~&&".into(),
        Ty::Off => "%&".into(),
        Ty::UOff => "~%&".into(),
        Ty::Bit { width, signed } => format!("{}`{width}", if signed { "" } else { "~" }),
        Ty::F32 => "!".into(),
        Ty::F64 => "#".into(),
        Ty::F80 => "##".into(),
        Ty::Str => "$".into(),
        Ty::FixedStr(n) => format!("${n}"),
        Ty::User(_) => String::new(),
    }
}

/// The name part of a name as written: up to its type suffix.
fn without_suffix(name: &str) -> &str {
    let end = name.find(['%', '&', '!', '#', '$', '~', '`']).unwrap_or(name.len());
    &name[..end]
}

/// QB64pe's "Unused variable" (design D11; measured, `verification\v68_a_unused`, `v68_a2_types`, `v68_a3_more`,
/// `v68_b_shared`, `v68_c_input`, `v68_d_include`). A variable is created by a declaration (`DIM`, `DIM SHARED`,
/// `REDIM`, `STATIC`, a parameter, a FUNCTION's result) or by its first use, and each later lookup of it marks it
/// used, an assignment too. So a variable made by its first use is never reported, nor is a `COMMON` one. A
/// declared one is reported when no name of it follows its declaration; a FUNCTION's result when the FUNCTION never
/// assigns it, at its header. The detail is the name as declared with QB64pe's suffix for its type, `()` for an
/// array, padded to the longest name reported plus one, two blanks, and the type's name.
fn unused_variables(prog: &Program, map: &SourceMap) -> Vec<Lint> {
    let mut symbol_of = HashMap::new();
    let mut type_written = HashMap::new();
    for (_, s) in prog.symbols.iter() {
        if let SymbolKind::Var(v) = s.kind {
            symbol_of.insert(v, s);
        } else if let SymbolKind::Type(t) = s.kind {
            type_written.insert(t, written(map, s.def));
        }
    }
    let mut found = Vec::new();
    for (i, v) in prog.vars.iter().enumerate() {
        let id = VarId(to_u32(i));
        if v.implicit || prog.not_reported_unused.contains(&id) {
            continue;
        }
        let at = match v.storage {
            // A FUNCTION's result has a symbol once the FUNCTION assigns it.
            Storage::Result(q) if !symbol_of.contains_key(&id) => header_span(prog, map, q),
            Storage::Result(_) => None,
            // A `DECLARE LIBRARY` routine's parameters are no variables of the program.
            Storage::Param(q) if prog.proc(q).external.is_some() => None,
            Storage::Main | Storage::Static(_) | Storage::Local(_) | Storage::Param(_) => {
                symbol_of.get(&id).filter(|s| s.refs.is_empty()).map(|s| s.def)
            }
        };
        let Some(at) = at else { continue };
        let shown = written(map, at);
        let shape = if v.is_array() { "()" } else { "" };
        let name = format!("{}{}{shape}", without_suffix(&shown), type_suffix(v.ty));
        let ty = if let Ty::User(t) = v.ty {
            type_written.get(&t).cloned().unwrap_or_else(|| prog.type_name(v.ty))
        } else {
            prog.type_name(v.ty)
        };
        found.push((at, name, ty));
    }
    let width = found.iter().map(|(_, n, _)| n.len()).max().unwrap_or(0) + 1;
    found
        .into_iter()
        .map(|(at, name, ty)| (at, UNUSED_VARIABLE, format!("Unused variable\n    {name:width$}  {ty}")))
        .collect()
}

/// The span of a procedure's name in its header ([`crate::Symbols::is_declaring`] on the header's line).
fn header_span(prog: &Program, map: &SourceMap, q: ProcId) -> Option<Span> {
    let p = prog.proc(q);
    let (_, s) = prog.symbols.iter().find(|(_, s)| s.kind == SymbolKind::Proc(q))?;
    std::iter::once(&s.def).chain(&s.refs).copied().find(|&sp| {
        prog.symbols.is_declaring(sp) && sp.file == p.file && map.file(sp.file).line_col(sp.start).0 == p.line
    })
}

/// One warning of our own lints: where, its code, its message.
type Lint = (Span, &'static str, String);

/// The name at `span` as written.
fn written(map: &SourceMap, span: Span) -> String {
    show_bytes(map.text(span))
}

/// Calls `f` on every statement list of the program: the main module's, each procedure's, and inside them each
/// block's (the branches of an `IF`, the bodies of loops, the cases of a `SELECT`, and the allocations a block's
/// tests make), outer lists first.
fn each_list<'p>(prog: &'p Program, f: &mut impl FnMut(&'p [Stmt])) {
    fn walk<'p>(list: &'p [Stmt], f: &mut impl FnMut(&'p [Stmt])) {
        f(list);
        for s in list {
            match &s.kind {
                StmtKind::If { branches, else_, .. } => {
                    for b in branches {
                        walk(&b.allocs, f);
                        walk(&b.body, f);
                    }
                    if let Some(e) = else_ {
                        walk(e, f);
                    }
                }
                StmtKind::For { body, .. } | StmtKind::While { body, .. } => walk(body, f),
                StmtKind::Do { test, body, .. } => {
                    walk(body, f);
                    if let Some(t) = test {
                        walk(&t.allocs, f);
                    }
                }
                StmtKind::Select { cases, else_, .. } => {
                    for c in cases {
                        walk(&c.allocs, f);
                        walk(&c.body, f);
                    }
                    if let Some(e) = else_ {
                        walk(e, f);
                    }
                }
                // No statement list inside: a new kind that holds one must be walked above.
                StmtKind::Builtin { .. }
                | StmtKind::ConsoleOnly
                | StmtKind::MemGet { .. }
                | StmtKind::MemPut { .. }
                | StmtKind::MemFill { .. }
                | StmtKind::Assign { .. }
                | StmtKind::CopyWhole { .. }
                | StmtKind::Allocate { .. }
                | StmtKind::Clear { .. }
                | StmtKind::Release { .. }
                | StmtKind::EraseParam { .. }
                | StmtKind::CopyArray { .. }
                | StmtKind::ArrayCopy { .. }
                | StmtKind::Print { .. }
                | StmtKind::PrintUsing { .. }
                | StmtKind::Field { .. }
                | StmtKind::Lset { .. }
                | StmtKind::Write { .. }
                | StmtKind::Input { .. }
                | StmtKind::Read(_)
                | StmtKind::Restore(_)
                | StmtKind::End(_)
                | StmtKind::System(_)
                | StmtKind::Stop
                | StmtKind::Call { .. }
                | StmtKind::CallAbsolute { .. }
                | StmtKind::Exit
                | StmtKind::OnError(_)
                | StmtKind::Log { .. }
                | StmtKind::Resume(_)
                | StmtKind::Error(_)
                | StmtKind::Run(_)
                | StmtKind::PaletteUsing { .. }
                | StmtKind::Label(_)
                | StmtKind::Goto(_)
                | StmtKind::Gosub(_)
                | StmtKind::Return(_)
                | StmtKind::ExitLoop(_)
                | StmtKind::Continue
                | StmtKind::ExitSelect { .. }
                | StmtKind::OnJump { .. }
                | StmtKind::OnEvent { .. }
                | StmtKind::Key(_) => {}
            }
        }
    }
    walk(&prog.stmts, f);
    for p in &prog.procs {
        walk(&p.stmts, f);
    }
}

/// `used-once` (design D4): a variable made by its first use whose name occurs nowhere else. A `FOR` counter and an
/// `INPUT`, `LINE INPUT` or `READ` target are left out: a loop with a bare `NEXT`, a wait for Enter and a `DATA`
/// item skipped name them once. So is a name of a skipped `$IF` branch, and a variable in `pds`, which gets
/// `pds-function` instead.
fn used_once(prog: &Program, map: &SourceMap, inactive: &HashSet<String>, pds: &HashSet<VarId>, out: &mut Vec<Lint>) {
    let mut exempt = HashSet::new();
    each_list(prog, &mut |list| {
        for s in list {
            if let StmtKind::For { var: Place::Var(v), .. } = &s.kind {
                exempt.insert(*v);
            } else if let StmtKind::Input { targets, .. } | StmtKind::Read(targets) = &s.kind {
                exempt.extend(targets.iter().map(Place::root));
            }
        }
    });
    for (_, s) in prog.symbols.iter() {
        if let SymbolKind::Var(v) = s.kind
            && prog.var(v).implicit
            && s.refs.is_empty()
            && !exempt.contains(&v)
            && !in_inactive(map, s.def, inactive)
            && !pds.contains(&v)
        {
            let msg = format!("`{}` is used only once: is it misspelled?", written(map, s.def));
            out.push((s.def, USED_ONCE, msg));
        }
    }
}

/// `unused-procedure` (design D5): a procedure none of whose names is a call. The header's name and `DECLARE`
/// lines declare it ([`crate::Symbols::is_declaring`]); any other name of it calls it or names it as a handler,
/// also from within itself. The warning goes on the header's name. `SUB _GL`, which the runtime calls, a
/// `DECLARE LIBRARY` routine, a test SUB of a `_test.bas` main file (spec `compiler/unit-tests`) and a name of a
/// skipped `$IF` branch are left out.
fn unused_procedures(prog: &Program, map: &SourceMap, main: FileId, inactive: &HashSet<String>, out: &mut Vec<Lint>) {
    let test_file = map.file(main).name.to_ascii_lowercase().ends_with("_test.bas");
    for (_, s) in prog.symbols.iter() {
        let SymbolKind::Proc(id) = s.kind else { continue };
        let p = prog.proc(id);
        let spans = || std::iter::once(&s.def).chain(&s.refs).copied();
        if p.external.is_some() || p.name == "_GL" || spans().any(|sp| !prog.symbols.is_declaring(sp)) {
            continue;
        }
        let test_sub = test_file && p.kind == ProcKind::Sub && p.params.is_empty() && p.name.starts_with("TEST");
        if test_sub {
            continue;
        }
        let Some(header) = header_span(prog, map, id) else {
            continue;
        };
        if in_inactive(map, header, inactive) {
            continue;
        }
        let kind = match p.kind {
            ProcKind::Sub => "SUB",
            ProcKind::Function(_) => "FUNCTION",
        };
        let msg = format!("{kind} `{}` is never called", written(map, header));
        out.push((header, UNUSED_PROCEDURE, msg));
    }
}

/// `hides-shared` (design D7): the locals `check\decl.rs` found hiding a main-module `DIM SHARED` variable.
fn hides_shared(prog: &Program, map: &SourceMap, out: &mut Vec<Lint>) {
    for &(span, _) in &prog.hidden_shared {
        let msg = format!(
            "this `{}` is a new local variable, which hides the `DIM SHARED` one of the main module",
            written(map, span)
        );
        out.push((span, HIDES_SHARED, msg));
    }
}

/// Whether a statement always leaves its statement list (design D8). `ERROR n`, `GOSUB`, `ON … GOTO` and
/// `RUN file$` are not on the list: each can go on with the next statement (`RUN file$` raises error 53 when the
/// program cannot be started, and a handler may `RESUME NEXT`).
fn leaves(kind: &StmtKind) -> bool {
    match kind {
        StmtKind::Goto(_)
        | StmtKind::End(_)
        | StmtKind::System(_)
        | StmtKind::Stop
        | StmtKind::Exit
        | StmtKind::ExitLoop(_)
        | StmtKind::Return(_)
        | StmtKind::Resume(_) => true,
        StmtKind::Run(target) => !matches!(target, RunTarget::File(_)),
        // A new kind that always leaves its list must be listed above.
        StmtKind::Builtin { .. }
        | StmtKind::ConsoleOnly
        | StmtKind::MemGet { .. }
        | StmtKind::MemPut { .. }
        | StmtKind::MemFill { .. }
        | StmtKind::Assign { .. }
        | StmtKind::CopyWhole { .. }
        | StmtKind::Allocate { .. }
        | StmtKind::Clear { .. }
        | StmtKind::Release { .. }
        | StmtKind::EraseParam { .. }
        | StmtKind::CopyArray { .. }
        | StmtKind::ArrayCopy { .. }
        | StmtKind::Print { .. }
        | StmtKind::PrintUsing { .. }
        | StmtKind::Field { .. }
        | StmtKind::Lset { .. }
        | StmtKind::Write { .. }
        | StmtKind::Input { .. }
        | StmtKind::Read(_)
        | StmtKind::Restore(_)
        | StmtKind::Call { .. }
        | StmtKind::CallAbsolute { .. }
        | StmtKind::OnError(_)
        | StmtKind::Log { .. }
        | StmtKind::Error(_)
        | StmtKind::PaletteUsing { .. }
        | StmtKind::Label(_)
        | StmtKind::Gosub(_)
        | StmtKind::Continue
        | StmtKind::ExitSelect { .. }
        | StmtKind::OnJump { .. }
        | StmtKind::OnEvent { .. }
        | StmtKind::Key(_)
        | StmtKind::If { .. }
        | StmtKind::For { .. }
        | StmtKind::Do { .. }
        | StmtKind::While { .. }
        | StmtKind::Select { .. } => false,
    }
}

/// The keyword a leaving statement is named by in the warning: its first word, two for `EXIT SUB` and the like.
fn keyword(map: &SourceMap, s: &Stmt) -> String {
    // Only ASCII words are kept; `show_bytes` writes any other byte as `\xNN`, which splits words as before.
    let text = show_bytes(map.text(s.span)).to_ascii_uppercase();
    let mut words = text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_' || c == '$'));
    let words: Vec<&str> = words.by_ref().filter(|w| !w.is_empty()).take(2).collect();
    match words.as_slice() {
        ["EXIT", second] => format!("EXIT {second}"),
        [first, ..] => first.to_string(),
        [] => String::new(),
    }
}

/// Whether a line of a precompiler block (`$IF`, `$ELSEIF`, `$ELSE`, `$END IF`) stands between `from` and `to`,
/// statements of one file. The code after it may run on another platform or with other flags: `$IF WIN` with a
/// `SYSTEM` and then the code for Linux (`tests\upstream\compile_tests\glut\*`). It ends a dead stretch as a label
/// does (found by the corpus run, task 6.1 of `lints`).
fn pp_boundary(map: &SourceMap, from: &Stmt, to: &Stmt) -> bool {
    if from.span.file != to.span.file {
        return false;
    }
    let file = map.file(from.span.file);
    // The lines strictly between the two, 0-based for `line_content` (`Stmt::line` is 1-based).
    (from.line..to.line.saturating_sub(1)).any(|line| {
        let Some((start, end)) = file.line_content(line) else {
            return false;
        };
        let text = file.bytes[start as usize..end as usize]
            .trim_ascii_start()
            .to_ascii_uppercase();
        [&b"$IF "[..], b"$ELSEIF ", b"$ELSE", b"$END IF", b"$ENDIF"]
            .iter()
            .any(|p| text.starts_with(p))
    })
}

/// `unreachable` (design D8): in each statement list, the statements after one that always leaves it, up to the
/// next label or line number. One warning per stretch, at its first statement in the main file. `$CONSOLE:ONLY`,
/// the only statement of the typed tree that the user does not write as a statement, does not start a stretch;
/// `DATA`, `DECLARE`, `CONST`, `TYPE` and a `DIM` with no allocation are no statements there at all.
fn unreachable(prog: &Program, map: &SourceMap, main: FileId, out: &mut Vec<Lint>) {
    each_list(prog, &mut |list| {
        // The statement that left, and whether its stretch has had its warning.
        let mut left: Option<(&Stmt, bool)> = None;
        for s in list {
            match (&s.kind, &mut left) {
                (StmtKind::Label(_), _) => left = None,
                (StmtKind::ConsoleOnly, _) => {}
                (kind, Some((from, _))) if pp_boundary(map, from, s) => {
                    left = leaves(kind).then_some((s, false));
                }
                (_, Some((from, given))) => {
                    if !*given && s.span.file == main {
                        let msg = format!(
                            "this code is never run: no label follows the {} on line {}",
                            keyword(map, from),
                            from.line
                        );
                        out.push((s.span, UNREACHABLE, msg));
                        *given = true;
                    }
                }
                (kind, None) => {
                    if leaves(kind) {
                        left = Some((s, false));
                    }
                }
            }
        }
    });
}

/// `unused-label` (design D6): a named label whose symbol has no use. A label is entered in pass 1, so its
/// definition is the symbol's `def` and every use is a `ref`. Line numbers are never reported, nor a label named
/// in a skipped `$IF` branch.
fn unused_labels(prog: &Program, map: &SourceMap, inactive: &HashSet<String>, out: &mut Vec<Lint>) {
    for (_, s) in prog.symbols.iter() {
        if let SymbolKind::Label(l) = s.kind
            && prog.label(l).line_number.is_none()
            && s.refs.is_empty()
            && !in_inactive(map, s.def, inactive)
        {
            let msg = format!("label `{}` is never used", written(map, s.def));
            out.push((s.def, UNUSED_LABEL, msg));
        }
    }
}
