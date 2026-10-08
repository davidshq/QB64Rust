//! From the syntax tree to the typed [`Program`].
//!
//! Two passes (design D2 of `m2-procedures-and-errors`): pass 1 collects the procedure headers, so a call may come
//! before the definition; pass 2 checks every statement **in file order**, main-module statements and procedure
//! bodies as they come. File order is what the old compiler's scopes follow (measured, `verification\v14_*`): a
//! `DIM SHARED` after a procedure is not seen by it, and a `SHARED x AS T` in an earlier procedure acts like a
//! main-module `DIM x AS T` for the main-module code after it.

mod blocks;
mod builtins;
mod constants;
mod decl;
mod expr;
mod flow;
mod ops;
mod places;
mod proc;

use blocks::nested_statements;

use crate::{ConstId, LabelId, Place, PrintItem, ProcId, Program, Stmt, StmtKind, SymbolKind, Ty, TypeId, VarId};
use qb64rust_base::{Diagnostics, FileId, SourceMap, Span, show_bytes};
use qb64rust_syntax::ParsedProgram;
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::ast::{self, PrintPart};
use qb64rust_syntax::meta::{MemoryMode, comment_directives};
use qb64rust_syntax::tree::{Node, Tok, TreeId};
use std::collections::{HashMap, HashSet};

/// Checks a parsed program. Statements that already have a parse error are skipped (one error per statement). Text
/// is read through `map`, by each span's file.
pub fn check(map: &SourceMap, program: &ParsedProgram) -> (Program, Diagnostics) {
    check_with(map, program, true)
}

/// [`check`] with integer constant folding switched on or off. Off is for tests only: the slice programs must
/// print the same either way (design D5).
pub fn check_with(map: &SourceMap, program: &ParsedProgram, fold: bool) -> (Program, Diagnostics) {
    let root = program.main().root();
    let mut c = Checker {
        fold,
        map,
        program,
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
        labels_by_name: HashMap::new(),
        label_of_def: HashMap::new(),
        diags: Diagnostics::new(),
        stmt_error: false,
        console_only: false,
        names: Vec::new(),
        consts_main: HashMap::new(),
        consts_local: HashMap::new(),
        used_names: HashMap::new(),
        used_local: HashMap::new(),
        label_line: None,
        explicit: false,
        sinks: Vec::new(),
        bad_next: None,
        types_by_name: HashMap::new(),
        types_seen: HashSet::new(),
        type_defs: HashSet::new(),
        dim_shared_array_plain: HashMap::new(),
        whole_type_arg: false,
        len_place: None,
        types_marked: HashSet::new(),
        follow_on: false,
    };
    let skips = Skips::new(program);
    // Measured: an `OPTION _EXPLICIT` in an included file applies to the whole program, also before the include
    // (`verification\v19_explicit_*`).
    c.explicit = program.trees.iter().any(|t| has_option_explicit(map, t.root(), &skips));
    // The main file's statements, and the same with every include statement followed by what it includes (pass 1
    // reads the user types and procedures of included files from these; pass 2 and the label pre-pass descend into
    // includes as they meet them).
    let top: Vec<Node> = ast::SourceFile::cast(root)
        .into_iter()
        .flat_map(|f| f.statements())
        .collect();
    let statements = expand(program, top.iter().copied());

    // Pass 1: the user types (a parameter may name one), then procedure names, then their parameters (a parameter may not have the name of any procedure).
    // A header with an error (a parse error, a reserved name...) still enters its name as a broken procedure, so
    // calls of it fail without a second error.
    c.declare_types(&statements, &skips);
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
            Ok(id) => {
                let line_of = |n: Node| c.line(n.first_token().map_or(n.span(), |t| t.span));
                let line = line_of(header.node());
                let end_line = def.end().map_or(line, |e| line_of(e.node()));
                let proc = &mut c.prog.procs[id.0 as usize];
                (proc.line, proc.end_line) = (line, end_line);
                declared.push((def, id))
            }
            Err(Failed) => c.declare_broken(header),
        }
    }
    for (def, id) in declared {
        c.stmt_error = false;
        let ok = c.declare_params(def.header().unwrap(), id).is_ok();
        c.flush_names();
        if ok {
            c.proc_of_def.insert(def.node().key(), id);
        } else {
            c.broken.insert(id);
        }
    }

    // The labels of each body, also those inside blocks, so that a jump or `ON ERROR GOTO` may name one further
    // down (design D5).
    c.declare_labels(&top, &skips, None);
    for &def in &defs {
        if let Some(&id) = c.proc_of_def.get(&def.node().key()) {
            let body: Vec<Node> = def.body().collect();
            c.declare_labels(&body, &skips, Some(id));
        }
    }

    // Pass 2: everything in file order (procedures where they stand, included files at their include).
    for stmt in top {
        if skips.past_cap(stmt) {
            break;
        }
        c.check_statement(stmt, &skips);
    }
    if !c.console_only {
        let at = Span::new(root.file, 0, 0);
        c.diags.unsupported(at, "programs without `$CONSOLE:ONLY`");
    }
    (c.prog, c.diags)
}

/// Which statements the checker skips: those with a parse error, and those past the parser's error cap. Kept per
/// tree, since one file included twice has two trees whose errors may differ.
struct Skips {
    /// Per tree: the start offsets of its parse errors.
    error_starts: HashMap<TreeId, Vec<u32>>,
    /// Per capped tree: where the parser stopped recording errors.
    cap_start: HashMap<TreeId, u32>,
}

impl Skips {
    fn new(program: &ParsedProgram) -> Skips {
        let mut skips = Skips {
            error_starts: HashMap::new(),
            cap_start: HashMap::new(),
        };
        for t in &program.trees {
            let list = t.diagnostics.list();
            skips
                .error_starts
                .insert(t.id, list.iter().map(|d| d.span.start).collect());
            // Past the error cap the parser records no more errors, so statements from the cap on may be malformed.
            if t.diagnostics.is_capped() {
                skips.cap_start.insert(t.id, list.last().map_or(0, |d| d.span.start));
            }
        }
        skips
    }

    fn past_cap(&self, node: Node) -> bool {
        self.cap_start.get(&node.tree).is_some_and(|&c| node.span().end >= c)
    }

    fn usable(&self, node: Node) -> bool {
        let s = node.span();
        let errors = self.error_starts.get(&node.tree).map_or(&[][..], Vec::as_slice);
        node.kind() != SyntaxKind::Error && !self.past_cap(node) && !errors.iter().any(|&o| o >= s.start && o <= s.end)
    }
}

/// The variables a name can mean in one scope: a name plus a type (design D5 of the last change). Arrays have a
/// name space of their own (design D4 of `m2-arrays-and-types`).
#[derive(Clone, Default)]
struct Scope {
    /// (name, type) -> variable.
    vars: HashMap<(String, Ty), VarId>,
    /// Type of the plain (suffix-less) name after a `DIM name AS type` (or a parameter or `SHARED` with `AS`).
    plain: HashMap<String, Ty>,
    /// (name, element type) -> array.
    arrays: HashMap<(String, Ty), VarId>,
    /// Element type of an array's plain name after `DIM name(…) AS type`.
    array_plain: HashMap<String, Ty>,
}

struct Checker<'a> {
    map: &'a SourceMap,
    /// Every tree, for the included ones ([`Checker::expand`]).
    program: &'a ParsedProgram,
    prog: Program,
    main: Scope,
    /// The scope of the procedure being checked (only meaningful while `cur` is set).
    local: Scope,
    /// The procedure being checked; `None` in the main module.
    cur: Option<ProcId>,
    procs_by_name: HashMap<String, ProcId>,
    /// Key of a `ProcDef` node whose header was declared without errors -> its procedure.
    proc_of_def: HashMap<(TreeId, u32), ProcId>,
    /// Per procedure: the scope holding its parameters, which each check of the body starts from.
    param_scopes: Vec<Scope>,
    /// Procedures whose parameter list has an error: calls of them fail without a second error.
    broken: HashSet<ProcId>,
    /// Main-module variables declared with `DIM SHARED` so far (in file order), and the plain names they typed.
    dim_shared: HashSet<VarId>,
    dim_shared_plain: HashMap<String, Ty>,
    /// The labels by body (`None`: the main module) and name, and the label of each `LabelDef` node (by its key).
    labels_by_name: HashMap<(Option<ProcId>, String), LabelId>,
    label_of_def: HashMap<(TreeId, u32), LabelId>,
    diags: Diagnostics,
    stmt_error: bool,
    console_only: bool,
    fold: bool,
    /// Names resolved in the current statement, for the symbol table.
    names: Vec<(SymbolKind, Span)>,
    /// Constants by name (without suffix): the main module's so far, and those of the procedure being checked.
    consts_main: HashMap<String, ConstId>,
    consts_local: HashMap<String, ConstId>,
    /// The first use or declaration of each variable name so far, anywhere, and in the procedure being checked:
    /// a later `CONST` of the name is an error there (design D6).
    used_names: HashMap<String, Span>,
    used_local: HashMap<String, Span>,
    /// File and line of the last label, in any body (a `CONST` after it on that line is not supported yet).
    label_line: Option<(FileId, u32)>,
    /// `OPTION _EXPLICIT` stands somewhere in the program (it applies to the whole program, design D7).
    explicit: bool,
    /// The statement lists of the blocks being checked, innermost last; [`Self::push`] adds to the last one.
    sinks: Vec<Vec<Stmt>>,
    /// The last `NEXT` statement (by key) whose variable did not match its `FOR` (`check\blocks.rs`).
    bad_next: Option<(TreeId, u32)>,
    /// The user types by name.
    types_by_name: HashMap<String, TypeId>,
    /// The main module's `TYPE` blocks met in pass 1 (by key), and those of them that define a type.
    types_seen: HashSet<(TreeId, u32)>,
    type_defs: HashSet<(TreeId, u32)>,
    /// Element types of the plain names of main-module `DIM SHARED` arrays declared so far.
    dim_shared_array_plain: HashMap<String, Ty>,
    /// An argument is being typed: a whole user-type value there is "not supported yet", not an error.
    whole_type_arg: bool,
    /// The span of a `LEN` argument being typed: a whole user-type place loaded with exactly this span is taken as
    /// a place (its size), not rejected as a value.
    len_place: Option<Span>,
    /// Main-module `TYPE` blocks (by key) that pass 1 marked "not supported yet".
    types_marked: HashSet<(TreeId, u32)>,
    /// The follow-on rule is on (design D10): a declaration was marked "not supported yet", so real errors are
    /// dropped from here on.
    follow_on: bool,
}

/// An expression could not be typed; the error is already reported.
struct Failed;

type R<T> = Result<T, Failed>;

impl Checker<'_> {
    /// A real error, at most one per statement. After the follow-on rule started (design D10: a declaration was
    /// marked "not supported yet" earlier in file order) it is dropped: the statement fails without a diagnostic,
    /// and the program is rejected by that mark already.
    fn error(&mut self, span: Span, msg: impl Into<String>) -> Failed {
        if !self.stmt_error {
            self.stmt_error = true;
            if !self.follow_on {
                self.diags.error(span, msg);
            }
        }
        Failed
    }

    /// Like [`Self::error`], for a construct not handled yet: marked "not supported yet", `msg` names it.
    fn unsupported(&mut self, span: Span, msg: impl Into<String>) -> Failed {
        if !self.stmt_error {
            self.stmt_error = true;
            self.diags.unsupported(span, msg);
        }
        Failed
    }

    fn in_use(&mut self, t: Tok) -> Failed {
        let msg = format!("name already in use: `{}`", show_bytes(self.text(t.span)));
        self.error(t.span, msg)
    }

    fn text(&self, span: Span) -> &[u8] {
        self.map.text(span)
    }

    /// The 1-based line a span starts on, in its own file.
    fn line(&self, span: Span) -> u32 {
        self.map.file(span.file).line_col(span.start).0
    }

    /// The token in upper case, other than printable ASCII escaped (`\xC9`).
    fn word(&self, t: Tok) -> String {
        show_bytes(&self.text(t.span).to_ascii_uppercase())
    }

    /// Records the names of the statement just checked, unless it had an error (one error per statement).
    fn flush_names(&mut self) {
        let names = std::mem::take(&mut self.names);
        if !self.stmt_error {
            self.prog.symbols.record(names);
        }
    }

    fn check_statement(&mut self, stmt: Node, skips: &Skips) {
        if let Some(def) = ast::ProcDef::cast(stmt) {
            self.proc_def(def, skips);
            return;
        }
        if self.block_statement(stmt, skips) {
            return;
        }
        if ast::MetaCommentStmt::cast(stmt).is_some() {
            self.check_one(stmt, skips);
            // The included file's statements follow the include statement, in its place (design D10).
            let program = self.program;
            if let Some(tree) = program.included(stmt) {
                for s in ast::SourceFile::cast(tree.root())
                    .into_iter()
                    .flat_map(|f| f.statements())
                {
                    if skips.past_cap(s) {
                        break;
                    }
                    self.check_statement(s, skips);
                }
            }
            return;
        }
        self.check_one(stmt, skips);
    }

    /// A procedure where it stands: in the main module (or a file included there), its body is checked in its own
    /// scope; anywhere else it came from a file included inside a SUB, FUNCTION or block (the parser keeps every
    /// other `SUB` at file level), which the old compiler rejects (measured M7: "Expected END SUB/FUNCTION before
    /// SUB").
    fn proc_def(&mut self, def: ast::ProcDef, skips: &Skips) {
        if self.cur.is_some() || !self.sinks.is_empty() {
            if let Some(h) = def.header().filter(|h| skips.usable(h.node())) {
                self.stmt_error = false;
                let _ = self.error(
                    first_token_span(h.node()),
                    "a SUB or FUNCTION from an included file cannot stand inside a SUB, FUNCTION or block",
                );
            }
            return;
        }
        let Some(&id) = self.proc_of_def.get(&def.node().key()) else {
            return;
        };
        self.cur = Some(id);
        self.local = self.param_scopes[id.0 as usize].clone();
        self.consts_local.clear();
        self.used_local.clear();
        for s in def.body() {
            if skips.past_cap(s) {
                break;
            }
            self.check_statement(s, skips);
        }
        self.cur = None;
    }

    /// One statement that is neither a block nor an include. A declaration marked "not supported yet" starts the
    /// follow-on rule (design D10).
    fn check_one(&mut self, stmt: Node, skips: &Skips) {
        if !skips.usable(stmt) {
            return;
        }
        self.stmt_error = false;
        let marks = self.diags.unsupported_count();
        self.statement(stmt);
        self.flush_names();
        if self.diags.unsupported_count() > marks && is_declaration(stmt) {
            self.follow_on = true;
        }
    }

    fn push(&mut self, node: Node, kind: StmtKind) {
        let span = node.span();
        let line = self.line(node.first_token().map_or(span, |t| t.span));
        let stmt = Stmt { span, line, kind };
        if let Some(sink) = self.sinks.last_mut() {
            sink.push(stmt);
            return;
        }
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
        } else if let Some(s) = ast::MetaCommentStmt::cast(node) {
            self.meta_comment(s)
        } else if ast::InactiveCode::cast(node).is_some() {
            // A `$IF` branch not taken: not compiled.
            Ok(())
        } else if let Some(s) = ast::PrintStmt::cast(node) {
            self.print(s)
        } else if let Some(s) = ast::DimStmt::cast(node) {
            self.dim(s)
        } else if let Some(s) = ast::AssignStmt::cast(node) {
            self.assign(s)
        } else if ast::EndStmt::cast(node).is_some() {
            self.end_or_system(node, StmtKind::End)
        } else if ast::SystemStmt::cast(node).is_some() {
            self.end_or_system(node, StmtKind::System)
        } else if let Some(s) = ast::OnJumpStmt::cast(node) {
            self.on_jump(s)
        } else if ast::OnEventStmt::cast(node).is_some() {
            Err(self.unsupported(first_token_span(node), "event handlers (`ON TIMER`, `ON KEY`, …)"))
        } else if ast::EventSwitchStmt::cast(node).is_some() {
            Err(self.unsupported(first_token_span(node), "event switches (`TIMER ON`, `KEY(n) OFF`, …)"))
        } else if ast::LprintStmt::cast(node).is_some()
            || ast::WriteStmt::cast(node).is_some()
            || ast::InputStmt::cast(node).is_some()
            || ast::LineInputStmt::cast(node).is_some()
            || ast::CloseStmt::cast(node).is_some()
            || ast::FieldStmt::cast(node).is_some()
            || ast::LsetStmt::cast(node).is_some()
            || ast::SwapStmt::cast(node).is_some()
            || ast::MemStmt::cast(node).is_some()
            || ast::ArrayCopyStmt::cast(node).is_some()
        {
            // Task 7.4: parsed, compiled later (file I/O and the other built-in statements, step 8).
            let t = first_token_span(node);
            let word = show_bytes(&self.text(t).to_ascii_uppercase());
            let word = if ast::LineInputStmt::cast(node).is_some() {
                "LINE INPUT".to_string()
            } else {
                word
            };
            Err(self.unsupported(t, format!("`{word}`")))
        } else if ast::BuiltinStmt::cast(node).is_some() {
            // Task 7.5: read by its template; the built-in statements are compiled later (step 8).
            let t = first_token_span(node);
            let word = show_bytes(&self.text(t).to_ascii_uppercase());
            Err(self.unsupported(t, format!("`{word}`")))
        } else if ast::StopStmt::cast(node).is_some() {
            Err(self.unsupported(first_token_span(node), "`STOP`"))
        } else if ast::RunStmt::cast(node).is_some() {
            Err(self.unsupported(first_token_span(node), "`RUN`"))
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
        } else if let Some(s) = ast::LabelDef::cast(node) {
            self.label_stmt(s)
        } else if let Some(s) = ast::OnErrorStmt::cast(node) {
            self.on_error(s)
        } else if let Some(s) = ast::ResumeStmt::cast(node) {
            self.resume(s)
        } else if let Some(s) = ast::ErrorStmt::cast(node) {
            self.error_stmt(s)
        } else if ast::LineNumber::cast(node).is_some() || ast::ImplicitGoto::cast(node).is_some() {
            Err(self.unsupported(first_token_span(node), "line numbers"))
        } else if let Some(s) = ast::GotoStmt::cast(node) {
            self.goto(s)
        } else if let Some(s) = ast::GosubStmt::cast(node) {
            self.gosub(s)
        } else if let Some(s) = ast::ReturnStmt::cast(node) {
            self.return_stmt(s)
        } else if ast::DataStmt::cast(node).is_some() {
            Err(self.unsupported(first_token_span(node), "`DATA`"))
        } else if ast::ReadStmt::cast(node).is_some() {
            Err(self.unsupported(first_token_span(node), "`READ`"))
        } else if ast::RestoreStmt::cast(node).is_some() {
            Err(self.unsupported(first_token_span(node), "`RESTORE`"))
        } else if let Some(s) = ast::ConstStmt::cast(node) {
            self.const_stmt(s)
        } else if let Some(s) = ast::OptionStmt::cast(node) {
            self.option_stmt(s)
        } else if ast::RedimStmt::cast(node).is_some() {
            Err(self.unsupported(first_token_span(node), "`REDIM`"))
        } else if ast::CommonStmt::cast(node).is_some() {
            Err(self.unsupported(first_token_span(node), "`COMMON`"))
        } else if ast::EraseStmt::cast(node).is_some() {
            Err(self.unsupported(first_token_span(node), "`ERASE`"))
        } else if ast::DefTypeStmt::cast(node).is_some() {
            let t = first_token_span(node);
            let word = show_bytes(&self.text(t).to_ascii_uppercase());
            Err(self.unsupported(t, format!("`{word}`")))
        } else {
            Err(self.error(
                node.span(),
                format!("internal error: unexpected {:?} node", node.kind()),
            ))
        };
    }

    /// `END` or `SYSTEM` (`kind`); with an exit code not supported yet.
    fn end_or_system(&mut self, node: Node, kind: StmtKind) -> R<()> {
        if let Some(code) = node.child_nodes().find_map(ast::Expr::cast) {
            let word = show_bytes(&self.text(first_token_span(node)).to_ascii_uppercase());
            return Err(self.unsupported(code.node().span(), format!("`{word}` with an exit code")));
        }
        self.push(node, kind);
        Ok(())
    }

    fn meta(&mut self, stmt: ast::MetaStmt) -> R<()> {
        let node = stmt.node();
        let tok = self.need(stmt.token(), node.span())?;
        let raw = self.text(tok.span);
        let trimmed: Vec<u8> = raw.iter().copied().filter(|b| !b.is_ascii_whitespace()).collect();
        if let Some(flag) = stmt.precompiler_flag(raw) {
            // The old compiler sets these from the whole program; not known here yet (`syntax::pp`).
            Err(self.unsupported(tok.span, format!("the precompiler flag `{flag}` in `$IF`")))
        } else if stmt.is_preprocessor(raw) || trimmed.eq_ignore_ascii_case(b"$INCLUDEONCE") {
            // Evaluated by the parser (`$INCLUDEONCE` too); an error in it is a parse error, and the statement is
            // skipped then.
            Ok(())
        } else if trimmed.eq_ignore_ascii_case(b"$CONSOLE:ONLY") {
            if !self.console_only {
                self.console_only = true;
                self.push(node, StmtKind::ConsoleOnly);
            }
            Ok(())
        } else {
            let shown = show_bytes(raw.split(|&b| b == b':' || b == b' ').next().unwrap_or(raw));
            Err(self.unsupported(tok.span, format!("metacommand `{shown}`")))
        }
    }

    /// A metacommand comment. Its `$STATIC` and `$DYNAMIC` are never ignored (that gave wrong code): they are not
    /// supported yet; a malformed `$INCLUDE` is an error, as in the old compiler. A well-formed `$INCLUDE` was
    /// followed by the parser (its statements come next, [`Self::expand`]; a missing file is a parse error).
    fn meta_comment(&mut self, stmt: ast::MetaCommentStmt) -> R<()> {
        let tok = self.need(stmt.token(), stmt.node().span())?;
        match comment_directives(self.text(tok.span)) {
            Err(msg) => Err(self.error(tok.span, msg)),
            Ok(d) => match d.memory {
                Some(MemoryMode::Static) => Err(self.unsupported(tok.span, "metacommand `$STATIC` in a comment")),
                Some(MemoryMode::Dynamic) => Err(self.unsupported(tok.span, "metacommand `$DYNAMIC` in a comment")),
                None => Ok(()),
            },
        }
    }

    // ---- statements ----

    /// `place = value`. A plain variable is resolved after the value, as before places existed (an implicit
    /// variable in the value is created first); an element or a member before it.
    fn assign(&mut self, stmt: ast::AssignStmt) -> R<()> {
        let node = stmt.node();
        let target_node = self.need(stmt.target(), node.span())?;
        let value_node = self.need(stmt.value(), node.span())?;
        let place = match target_node {
            ast::Expr::NameRef(n) => {
                let t = self.need(n.name(), node.span())?;
                let (name, suffix) = self.split_name(t)?;
                match self.dotted(t, &name, suffix)? {
                    Some(place) => place,
                    None => {
                        let whole = self.lookup_var(name, suffix).filter(|&v| self.is_whole_type(v));
                        match whole {
                            Some(v) => {
                                self.names.push((SymbolKind::Var(v), t.span));
                                Place::Var(v)
                            }
                            None => {
                                let value = self.expr(value_node)?;
                                let var = self.target(t)?;
                                let ty = self.prog.var(var).ty;
                                let value = self.store(value, ty)?;
                                let place = Place::Var(var);
                                self.push(node, StmtKind::Assign { place, value });
                                return Ok(());
                            }
                        }
                    }
                }
            }
            ast::Expr::Call(c) => self.call_place(c)?,
            ast::Expr::Field(f) => self.field_place(f)?,
            e @ (ast::Expr::Literal(_) | ast::Expr::Paren(_) | ast::Expr::Prefix(_) | ast::Expr::Bin(_)) => {
                return Err(self.error(e.node().span(), "cannot assign to this expression"));
            }
        };
        let ty = self.prog.place_ty(&place);
        if let Ty::User(_) = ty {
            // Measured: `q = p` is accepted (a copy), `p = 5` is "Expected = similar user defined type".
            return Err(match value_node {
                ast::Expr::NameRef(_) | ast::Expr::Call(_) | ast::Expr::Field(_) => {
                    self.unsupported(node.span(), "assigning a whole `TYPE` value")
                }
                ast::Expr::Literal(_) | ast::Expr::Paren(_) | ast::Expr::Prefix(_) | ast::Expr::Bin(_) => self.error(
                    value_node.node().span(),
                    "a `TYPE` variable takes only a value of its `TYPE`",
                ),
            });
        }
        let value = self.expr(value_node)?;
        let value = self.store(value, ty)?;
        self.push(node, StmtKind::Assign { place, value });
        Ok(())
    }

    /// Whether a variable holds a whole user-type value.
    fn is_whole_type(&self, v: VarId) -> bool {
        matches!(self.prog.var(v).ty, Ty::User(_))
    }

    fn print(&mut self, stmt: ast::PrintStmt) -> R<()> {
        if let Some(f) = stmt.file() {
            return Err(self.unsupported(f.node().span(), "`PRINT #`"));
        }
        if let Some(u) = stmt.using() {
            return Err(self.unsupported(u.node().span(), "`PRINT USING`"));
        }
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
}

/// Whether an `OPTION _EXPLICIT` without a parse error stands anywhere in the tree: in the main module, a
/// procedure or a block, before or after the variables it concerns (measured, `verification\v17_f_*`).
fn has_option_explicit(map: &SourceMap, node: Node, skips: &Skips) -> bool {
    if let Some(s) = ast::OptionStmt::cast(node) {
        return skips.usable(node)
            && s.word()
                .is_some_and(|w| map.text(w.span).eq_ignore_ascii_case(b"_EXPLICIT"));
    }
    node.child_nodes().any(|n| has_option_explicit(map, n, skips))
}

/// The declaration statements of the follow-on rule (design D10; `TYPE` and `DECLARE LIBRARY` blocks are handled
/// where they are checked).
fn is_declaration(stmt: Node) -> bool {
    ast::DimStmt::cast(stmt).is_some()
        || ast::RedimStmt::cast(stmt).is_some()
        || ast::CommonStmt::cast(stmt).is_some()
        || ast::SharedStmt::cast(stmt).is_some()
        || ast::StaticStmt::cast(stmt).is_some()
        || ast::ConstStmt::cast(stmt).is_some()
        || ast::DefTypeStmt::cast(stmt).is_some()
}

/// A statement list with every include statement followed by the statements of the tree it includes (design D10:
/// the walk descends into an included tree at its include statement), recursively.
fn expand<'t>(program: &'t ParsedProgram, nodes: impl IntoIterator<Item = Node<'t>>) -> Vec<Node<'t>> {
    let mut out = Vec::new();
    for n in nodes {
        out.push(n);
        if ast::MetaCommentStmt::cast(n).is_some()
            && let Some(tree) = program.included(n)
        {
            let inner: Vec<Node<'t>> = ast::SourceFile::cast(tree.root())
                .into_iter()
                .flat_map(|f| f.statements())
                .collect();
            out.extend(expand(program, inner));
        }
    }
    out
}

/// The span of a statement's first token (where a "not supported yet" mark goes, design D10).
fn first_token_span(node: Node) -> Span {
    node.first_token().map_or(node.span(), |t| t.span)
}
