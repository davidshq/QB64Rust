//! Lint warnings (spec `compiler/lints`, OpenSpec change `lints`): code that compiles but is probably a mistake.
//!
//! They run once, on the finished program, and only when the program has no error (design D1). Our own lints are
//! given only for places in the file being compiled, not in its included files (design D3). Each warning carries
//! its lint's code (design D2).

use crate::consteval::Value;
use crate::{Place, ProcId, ProcKind, Program, RunTarget, Stmt, StmtKind, Storage, SymbolKind, Ty, VarId};
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
    used_once(prog, map, &inactive, &mut ours);
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
    // QB64pe's warnings, wherever QB64pe gives them, included files too.
    let mut theirs = unused_variables(prog, map);
    theirs.extend(prog.qb64pe_warnings.iter().cloned());
    for (span, code, msg) in theirs {
        diags.push(Diagnostic::lint(span, code, msg));
    }
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
/// item skipped name them once. So is a name of a skipped `$IF` branch.
fn used_once(prog: &Program, map: &SourceMap, inactive: &HashSet<String>, out: &mut Vec<Lint>) {
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
