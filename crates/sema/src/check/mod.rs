//! From the syntax tree to the typed [`Program`].
//!
//! Two passes (design D2 of `m2-procedures-and-errors`): pass 1 collects the procedure headers, so a call may come
//! before the definition; pass 2 checks every statement **in file order**, main-module statements and procedure
//! bodies as they come. File order is what the old compiler's scopes follow (measured, `verification\v14_*`): a
//! `DIM SHARED` after a procedure is not seen by it, and a `SHARED x AS T` in an earlier procedure acts like a
//! main-module `DIM x AS T` for the main-module code after it.

mod blocks;
mod constants;
mod decl;
mod expr;
mod flow;
mod ops;
mod proc;

use blocks::nested_statements;

use crate::{ConstId, LabelId, PrintItem, ProcId, Program, Stmt, StmtKind, SymbolKind, Ty, VarId};
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
        has_include: false,
        sinks: Vec::new(),
        bad_next: None,
        parse_marks: program
            .trees
            .iter()
            .flat_map(|t| t.diagnostics.list())
            .filter(|d| d.unsupported)
            .map(|d| d.span)
            .collect(),
    };
    let skips = Skips::new(program);
    c.explicit = has_option_explicit(map, root, &skips);
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
    c.has_include = has_include(map, root);
    c.declare_labels(&statements, &skips, None);
    for &def in &defs {
        if let Some(&id) = c.proc_of_def.get(&def.node().key()) {
            let body: Vec<Node> = def.body().collect();
            c.declare_labels(&body, &skips, Some(id));
        }
    }

    // Pass 2: everything in file order.
    for stmt in statements {
        if skips.past_cap(stmt) {
            break;
        }
        match ast::ProcDef::cast(stmt) {
            Some(def) => {
                let Some(&id) = c.proc_of_def.get(&def.node().key()) else {
                    continue;
                };
                c.cur = Some(id);
                c.local = c.param_scopes[id.0 as usize].clone();
                c.consts_local.clear();
                c.used_local.clear();
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

/// The variables a name can mean in one scope: a name plus a type (design D5 of the last change).
#[derive(Clone, Default)]
struct Scope {
    /// (name, type) -> variable.
    vars: HashMap<(String, Ty), VarId>,
    /// Type of the plain (suffix-less) name after a `DIM name AS type` (or a parameter or `SHARED` with `AS`).
    plain: HashMap<String, Ty>,
}

struct Checker<'a> {
    map: &'a SourceMap,
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
    /// An `$INCLUDE` stands somewhere in the program: what it would bring in is not seen yet.
    has_include: bool,
    /// `OPTION _EXPLICIT` stands somewhere in the program (it applies to the whole program, design D7).
    explicit: bool,
    /// Where the parser marked something "not supported yet".
    parse_marks: Vec<Span>,
    /// The statement lists of the blocks being checked, innermost last; [`Self::push`] adds to the last one.
    sinks: Vec<Vec<Stmt>>,
    /// The last `NEXT` statement (by key) whose variable did not match its `FOR` (`check\blocks.rs`).
    bad_next: Option<(TreeId, u32)>,
}

/// An expression could not be typed; the error is already reported.
struct Failed;

type R<T> = Result<T, Failed>;

impl Checker<'_> {
    fn error(&mut self, span: Span, msg: impl Into<String>) -> Failed {
        if !self.stmt_error {
            self.stmt_error = true;
            self.diags.error(span, msg);
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
        if self.block_statement(stmt, skips) {
            return;
        }
        if !skips.usable(stmt) {
            return;
        }
        self.stmt_error = false;
        self.statement(stmt);
        self.flush_names();
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
        } else if let Some(s) = ast::PrintStmt::cast(node) {
            self.print(s)
        } else if let Some(s) = ast::DimStmt::cast(node) {
            self.dim(s)
        } else if let Some(s) = ast::AssignStmt::cast(node) {
            self.assign(s)
        } else if ast::EndStmt::cast(node).is_some() {
            self.push(node, StmtKind::End);
            Ok(())
        } else if ast::SystemStmt::cast(node).is_some() {
            self.push(node, StmtKind::System);
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
            Err(self.unsupported(tok.span, format!("metacommand `{shown}`")))
        }
    }

    /// A metacommand comment. Its `$INCLUDE`, `$STATIC` and `$DYNAMIC` are never ignored (that gave wrong code):
    /// they are not supported yet; a malformed `$INCLUDE` is an error, as in the old compiler.
    fn meta_comment(&mut self, stmt: ast::MetaCommentStmt) -> R<()> {
        let tok = self.need(stmt.token(), stmt.node().span())?;
        match comment_directives(self.text(tok.span)) {
            Err(msg) => Err(self.error(tok.span, msg)),
            Ok(d) if d.include.is_some() => Err(self.unsupported(tok.span, "metacommand `$INCLUDE` in a comment")),
            Ok(d) => match d.memory {
                Some(MemoryMode::Static) => Err(self.unsupported(tok.span, "metacommand `$STATIC` in a comment")),
                Some(MemoryMode::Dynamic) => Err(self.unsupported(tok.span, "metacommand `$DYNAMIC` in a comment")),
                None => Ok(()),
            },
        }
    }

    // ---- statements ----

    fn assign(&mut self, stmt: ast::AssignStmt) -> R<()> {
        let node = stmt.node();
        let target = match self.need(stmt.target(), node.span())? {
            ast::Expr::NameRef(n) => self.need(n.name(), node.span())?,
            ast::Expr::Call(c) => {
                let span = c.node().span();
                return Err(self.unsupported(span, "arrays"));
            }
            ast::Expr::Field(f) => return Err(self.field(f)),
            e @ (ast::Expr::Literal(_) | ast::Expr::Paren(_) | ast::Expr::Prefix(_) | ast::Expr::Bin(_)) => {
                return Err(self.error(e.node().span(), "cannot assign to this expression"));
            }
        };
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

/// Whether the tree holds an `$INCLUDE` (only the comment form exists, `study\00` §5), wherever it stands.
fn has_include(map: &SourceMap, node: Node) -> bool {
    if let Some(s) = ast::MetaCommentStmt::cast(node) {
        return s
            .token()
            .is_some_and(|t| comment_directives(map.text(t.span)).is_ok_and(|d| d.include.is_some()));
    }
    node.child_nodes().any(|n| has_include(map, n))
}

/// The span of a statement's first token (where a "not supported yet" mark goes, design D10).
fn first_token_span(node: Node) -> Span {
    node.first_token().map_or(node.span(), |t| t.span)
}
