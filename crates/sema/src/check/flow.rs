//! Labels, jumps and error handling: `GOTO`, `GOSUB`, `RETURN`, `ON ERROR GOTO`, `RESUME`, `ERROR`.
//!
//! Labels belong to a body, the main module or one procedure (design D5): each body's labels are entered in a pass
//! before the statements, so a jump may name a label further down, also one inside a block. A jump names a label
//! of its own body; `ON ERROR GOTO` names one of the main module's, from any body (all measured). A line number is
//! a label whose name is the number as the old compiler spells it (design D1 of `m4-line-numbers-whole-arrays`).

use super::walk::{Cursor, Event};
use super::{Checker, Failed, R, Skips, line_if_statements};
use crate::builtins::stmt_lookup;
use crate::{ConvKind, ErrorHandler, Label, LabelId, ProcId, ProcKind, Resume, RunTarget, StmtKind, SymbolKind, Ty};
use qb64rust_base::{Span, show_bytes, to_u32};
use qb64rust_builtins::{Kind, find_any};
use qb64rust_syntax::SyntaxKind::{
    GosubStmt, GotoStmt, Ident, Number, OnErrorStmt, OnEventStmt, OnJumpStmt, RestoreStmt, ResumeStmt, ReturnStmt,
};
use qb64rust_syntax::ast;
use qb64rust_syntax::tree::{Node, Tok};
use std::collections::HashSet;

impl<'a> Checker<'a> {
    /// Enters the labels of every body: the main module's (`top`, the walk's statements) first, then those of each
    /// procedure of `defs` in turn. A label belongs to the body it stands in by the walk, inside blocks and included
    /// files too; one in a procedure that has no body of its own (its header failed) is left out.
    pub(super) fn declare_labels(&mut self, top: &[Node<'a>], defs: &[ast::ProcDef<'a>], skips: &Skips) {
        let program = self.program;
        let mut found = Vec::new();
        let mut cur = Cursor::new(program, skips, top.iter().copied());
        self.find_labels(&mut cur, skips, &mut Vec::new(), &mut found);
        for &(_, stmt) in found.iter().filter(|(body, _)| body.is_none()) {
            self.declare_label_stmt(stmt, None);
        }
        for def in defs {
            let Some(&id) = self.proc_of_def.get(&def.node().key()) else {
                continue;
            };
            for &(_, stmt) in found.iter().filter(|(body, _)| *body == Some(Some(id))) {
                self.declare_label_stmt(stmt, Some(id));
            }
        }
    }

    /// The labels and line numbers the walk `cur` gives, each with its body (`None` the main module, `Some(None)`
    /// a procedure without one), in text order. `open` holds the body of each block open in the walk.
    fn find_labels(
        &self,
        cur: &mut Cursor<'a, '_>,
        skips: &Skips,
        open: &mut Vec<Option<Option<ProcId>>>,
        found: &mut Vec<(Option<Option<ProcId>>, Node<'a>)>,
    ) {
        let body = |open: &Vec<Option<Option<ProcId>>>| open.last().copied().flatten();
        // The statement walk drops a disagreeing `$IF`'s branches, so their labels are not entered: a jump to one
        // is "Label not defined" (measured, `verification\v57_j01`–`j04`).
        let mut drop = None;
        while let Some(event) = cur.next() {
            if self.pp_drop_event(&mut drop, event, cur) {
                continue;
            }
            match event {
                Event::Open(n) => {
                    let inner = match ast::ProcDef::cast(n) {
                        Some(def) => Some(self.proc_of_def.get(&def.node().key()).copied()),
                        None => body(open),
                    };
                    open.push(inner);
                }
                Event::Close(..) => {
                    open.pop();
                }
                Event::Branch(_) => {}
                Event::Stmt(n) => {
                    if let Some(inner) = line_if_statements(n) {
                        let mut sub = Cursor::new(self.program, skips, inner);
                        let mut depth = vec![body(open)];
                        self.find_labels(&mut sub, skips, &mut depth, found);
                    } else if (ast::LabelDef::cast(n).is_some() || ast::LineNumber::cast(n).is_some())
                        && skips.usable(n)
                    {
                        found.push((body(open), n));
                    }
                }
            }
        }
    }

    /// Enters one label or line number of `body`.
    fn declare_label_stmt(&mut self, stmt: Node, body: Option<ProcId>) {
        self.stmt.stmt_error = false;
        let _ = match (ast::LabelDef::cast(stmt), ast::LineNumber::cast(stmt)) {
            (Some(l), _) => self.declare_label(l, body),
            (_, Some(l)) => self.declare_line_number(l, body),
            (None, None) => unreachable!("only labels and line numbers are found"),
        };
        self.flush_names();
    }

    // ---- labels and error handling ----

    /// Collects the words the jumps of the whole program name as their targets, before the labels are entered
    /// (design D1 of `m3-get-put-bload`). A line that is a SUB's name and a colon is a label when one of them is
    /// that name and a call of the SUB when none is, whichever body the jump stands in (measured,
    /// `verification\v28_c_labels`, `v28_x71`–`x92`: `GOTO`, `GOSUB`, `RETURN`, `RESTORE`, `RESUME`, `ON ERROR
    /// GOTO`, `ON n GOTO/GOSUB` and `ON <event> GOSUB` make the label; `IF c THEN name`, `CALL name` and `ON
    /// <event> name` do not). The statements are taken from every tree, compiled by `sema` or not, so no jump is
    /// missed; the words of a line the parser could not read are kept apart ([`Self::declare_label`]).
    pub(super) fn collect_jump_words(&mut self, skips: &Skips) {
        let program = self.program;
        for tree in &program.trees {
            let root = tree.root();
            self.jump_words_in(root);
            // The lines with a parse error, and everything after the point where the parser stopped recording.
            let lines: HashSet<u32> = skips
                .error_starts
                .get(&root.tree)
                .into_iter()
                .flatten()
                .map(|&o| self.line(Span::new(root.file, o, o)))
                .collect();
            let cap = skips.cap_start.get(&root.tree).copied();
            if lines.is_empty() && cap.is_none() {
                continue;
            }
            for t in root.tokens() {
                if t.kind == Ident && (cap.is_some_and(|c| t.span.end >= c) || lines.contains(&self.line(t.span))) {
                    let word = self.word(t);
                    self.doubt_words.insert(word);
                }
            }
        }
    }

    /// The targets of the jumps in `node` and below it.
    fn jump_words_in(&mut self, node: Node) {
        // The words of the statement that can be a target: all of them where only the target is one (`GOTO a`,
        // `RESUME NEXT`: a keyword in the set does no harm), those after `GOTO` or `GOSUB` where an expression's
        // words stand before it.
        let kind = node.kind();
        let after_jump_word = if matches!(
            kind,
            GotoStmt | GosubStmt | ReturnStmt | RestoreStmt | ResumeStmt | OnErrorStmt
        ) {
            Some(false)
        } else if matches!(kind, OnJumpStmt | OnEventStmt) {
            Some(true)
        } else {
            None
        };
        if let Some(after) = after_jump_word {
            let mut seen = !after;
            for t in node.child_tokens() {
                if t.kind != Ident {
                    continue;
                }
                let word = self.word(t);
                if seen {
                    self.jump_words.insert(word);
                } else if word == "GOTO" || word == "GOSUB" {
                    seen = true;
                }
            }
        }
        for c in node.children() {
            if let Some(n) = c.as_node() {
                self.jump_words_in(n);
            }
        }
    }

    /// Enters a label of `body` (before pass 2). The old compiler does not take a built-in name as a label
    /// (`CLS:` is a call of `CLS`). A built-in statement `sema` compiles is that statement, checked in pass 2
    /// ([`Self::label_stmt`]); any other built-in's name is left unsupported here. The name of a SUB that can be
    /// called without an argument is a call of the SUB unless a jump of the program names it
    /// ([`Self::collect_jump_words`]); the name of any other SUB and of a FUNCTION is a label.
    fn declare_label(&mut self, l: ast::LabelDef, body: Option<ProcId>) -> R<()> {
        let node = l.node();
        let t = self.need(l.name(), node.span())?;
        let name = self.word(t);
        let shown = show_bytes(self.text(t.span));
        if !stmt_lookup(&name, false).is_empty() {
            return Ok(());
        }
        if find_any(name.as_bytes()).next().is_some() {
            if find_any(name.as_bytes()).all(|b| b.kind == Kind::Function) {
                // `validlabel` refuses the name, and the line is then a statement that is a function: "Syntax
                // error" (measured, `verification\v55_x35`, `x44`: `abs:`, `LEN:`).
                return Err(self.error(
                    t.span,
                    format!("`{shown}:`: `{shown}` is a built-in function, not a label"),
                ));
            }
            return Err(self.unsupported(t.span, format!("the label `{shown}:` (`{shown}` is a built-in)")));
        }
        // `validname` refuses a single leading `_` for a label too (measured, "Invalid name",
        // `verification\v50_x28_newhandler_underscore_defined`), as [`Self::reserved`] does for a variable. The
        // label is still entered, so that a jump to it gives no error of its own.
        let reserved = name.starts_with('_') && !name.starts_with("__") && !self.in_after_last_line(t.span);
        if let Some(&p) = self.procs_by_name.get(&name) {
            if self.unclear_proc_label(p, &name) {
                let msg = format!("a label with the name of a SUB or FUNCTION: `{shown}`");
                return Err(self.unsupported(t.span, msg));
            }
            if self.callable_bare(p) && !self.jump_words.contains(&name) {
                self.sub_call_lines.insert(node.key(), p);
                return Ok(());
            }
        }
        self.enter_label(node, t, name, None, body)?;
        if reserved {
            return Err(self.error(t.span, format!("names starting with `_` are reserved: `{shown}`")));
        }
        Ok(())
    }

    /// Enters a line number of `body` as a label (design D1 of `m4-line-numbers-whole-arrays`).
    fn declare_line_number(&mut self, l: ast::LineNumber, body: Option<ProcId>) -> R<()> {
        let node = l.node();
        let t = self.need(l.number(), node.span())?;
        let Some((name, last_line)) = line_label_name(self.text(t.span)) else {
            let msg = format!("`{}` is not a valid line number", show_bytes(self.text(t.span)));
            return Err(self.error(t.span, msg));
        };
        let why = cpp_line_problem(&last_line);
        self.enter_label(node, t, name, Some(last_line.clone()), body)?;
        if let Some(why) = why {
            let shown = show_bytes(self.text(t.span));
            let msg = format!("`{shown}` cannot be built as a line number: C++ reads `{last_line}` {why}");
            return Err(self.error(t.span, msg));
        }
        Ok(())
    }

    /// Enters the label `name` of `body` defined by `node` at `t`; "duplicate label" when the body has one, with a
    /// whole line number by its name, as the old compiler shows it (`010` as `10`, `v54_x04`).
    fn enter_label(
        &mut self,
        node: Node,
        t: Tok,
        name: String,
        line_number: Option<String>,
        body: Option<ProcId>,
    ) -> R<()> {
        let key = (body, name);
        if self.labels_by_name.contains_key(&key) {
            let shown = if line_number.is_some() && key.1.bytes().all(|b| b.is_ascii_digit()) {
                key.1.clone()
            } else {
                show_bytes(self.text(t.span))
            };
            let msg = format!("duplicate label: `{shown}`");
            return Err(self.error(t.span, msg));
        }
        let id = LabelId(to_u32(self.prog.labels.len()));
        let line = self.line(t.span);
        let include_line = self.include_line(node);
        self.prog.labels.push(Label {
            name: key.1.clone(),
            line,
            include_line,
            file: t.span.file,
            proc: body,
            line_number,
            data_at: 0,
        });
        self.labels_by_name.insert(key, id);
        self.label_of_def.insert(node.key(), id);
        self.stmt.names.push((SymbolKind::Label(id), t.span));
        Ok(())
    }

    /// A label or line number in pass 2, entered by [`Self::declare_labels`]: it stands before the next statement
    /// of its body. `false` when `node` has no entry.
    fn placed_label(&mut self, node: Node) -> bool {
        let Some(&id) = self.label_of_def.get(&node.key()) else {
            return false;
        };
        // Pass 2 is in file order, so the items collected so far are those before the label.
        self.prog.labels[id.0 as usize].data_at = self.prog.data.len();
        self.push(node, StmtKind::Label(id));
        true
    }

    /// A line number in pass 2. One without an entry had an error in its declaration, already reported.
    pub(super) fn line_number_stmt(&mut self, l: ast::LineNumber) -> R<()> {
        self.placed_label(l.node());
        Ok(())
    }

    /// A label in pass 2: it stands before the next statement of its body.
    pub(super) fn label_stmt(&mut self, l: ast::LabelDef) -> R<()> {
        if self.placed_label(l.node()) {
            return Ok(());
        }
        // The name of a SUB and a colon, with no jump to the name: a call of the SUB.
        if let Some(&proc) = self.sub_call_lines.get(&l.node().key()) {
            if let Some(t) = l.name() {
                self.stmt.names.push((SymbolKind::Proc(proc), t.span));
            }
            self.push(l.node(), StmtKind::Call { proc, args: Vec::new() });
            return Ok(());
        }
        // The name of a built-in statement and a colon: the statement with no argument, as in the old compiler.
        // Any other label without an entry had an error in its declaration, already reported.
        match l.name() {
            Some(t) if !stmt_lookup(&self.word(t), false).is_empty() => self.bare_stmt(l.node(), t),
            Some(_) | None => Ok(()),
        }
    }

    /// Whether a label of this name gets a mark in place of an entry ([`Self::declare_label`]): the name of a SUB
    /// or FUNCTION where the label could not be told from a call.
    fn marked_label_name(&self, name: &str) -> bool {
        self.procs_by_name
            .get(name)
            .is_some_and(|&p| self.unclear_proc_label(p, name))
    }

    /// The error for a jump to a label that is not defined, named `shown` at `t`, when `name` is a built-in's: the
    /// old compiler's `validlabel` refuses every name of a built-in, so no line defines such a label and every
    /// jump to it is an error: "Invalid label!" after `GOTO` and `ON … GOTO` (`goto`), "Invalid label" after
    /// `GOSUB`, `RESTORE` and `ON ERROR GOTO` (`qb64pe.bas` 27121–27166; measured, `verification\v27_x63`, `v29_x78`,
    /// `x79`, `v55_x36`–`x38`, `x42`, `x43`, `x45`).
    fn builtin_label(&mut self, t: Tok, name: &str, shown: &str, goto: bool) -> Option<Failed> {
        find_any(name.as_bytes()).next()?;
        let msg = if goto {
            format!("`{shown}` is a built-in's name, not a label to go to")
        } else {
            format!("`{shown}` is a built-in's name, not a label")
        };
        Some(self.error(t.span, msg))
    }

    /// Whether the procedure is a SUB that can be called without an argument.
    fn callable_bare(&self, p: ProcId) -> bool {
        let proc = self.prog.proc(p);
        proc.kind == ProcKind::Sub && proc.params.is_empty()
    }

    /// Whether a line that is the procedure's name and a colon cannot be told to be a call or a label: the
    /// procedure's header, or a line that may hold a jump to the name of a SUB callable without an argument, could
    /// not be read.
    fn unclear_proc_label(&self, p: ProcId, name: &str) -> bool {
        self.broken.contains(&p) || (self.callable_bare(p) && self.doubt_words.contains(name))
    }

    /// The name a jump's target `t` names: a label's name in upper case, or a line number's name
    /// ([`line_label_name`]).
    fn target_name(&mut self, t: Tok) -> R<String> {
        let shown = show_bytes(self.text(t.span));
        let name = if t.kind == Number {
            line_label_name(self.text(t.span)).map(|(name, _)| name)
        } else {
            let (name, suffix) = self.split_name(t)?;
            suffix.is_none().then_some(name)
        };
        // Measured: a number of more than 40 characters is "Invalid label!" (`verification\v54_x20`).
        match name {
            Some(name) => Ok(name),
            None => Err(self.error(t.span, format!("`{shown}` is not a valid label"))),
        }
    }

    /// Whether `t` is the `0` of `ON ERROR GOTO 0` or `RESUME 0` (any number of zeros, as the old compiler's line
    /// formatter writes them), which names no line.
    fn is_zero(&self, t: Tok) -> bool {
        let text = self.text(t.span);
        t.kind == Number && !text.is_empty() && text.iter().all(|&b| b == b'0')
    }

    /// A label of `body` named at `t`: `ON ERROR GOTO` and `RESUME` in the main module, and every jump.
    pub(super) fn label_ref(&mut self, t: Tok, body: Option<ProcId>) -> R<LabelId> {
        self.label_for(t, body, false)
    }

    /// [`Self::label_ref`] for `GOTO` and `ON … GOTO` (`goto`), whose error for a built-in's name is worded apart.
    fn label_for(&mut self, t: Tok, body: Option<ProcId>, goto: bool) -> R<LabelId> {
        let shown = show_bytes(self.text(t.span));
        let name = self.target_name(t)?;
        let Some(&id) = self.labels_by_name.get(&(body, name.clone())) else {
            if let Some(e) = self.builtin_label(t, &name, &shown, goto) {
                return Err(e);
            }
            // A label with such a name is "not supported yet" (`declare_label`), so it may exist.
            if self.marked_label_name(&name) {
                let msg = format!("a label with the name of a SUB or FUNCTION: `{shown}`");
                return Err(self.unsupported(t.span, msg));
            }
            // Nothing hides a label: one is entered wherever its line parses, also inside a block, an included
            // file or a construct not supported yet, so the follow-on rule of `OPTION _EXPLICIT` (`check\decl.rs`
            // `undeclared`) is not needed here.
            let msg = if body.is_some() {
                // Measured: labels of the main module and of other procedures are not visible
                // (`verification\v17_d_err_goto_main_from_sub`, `v17_d_err_gosub_main_from_sub`).
                format!("label `{shown}` is not defined in this SUB or FUNCTION")
            } else {
                // Measured: labels of procedures are not visible (`v17_d_goto_sub_label_from_main`,
                // `v17_d_on_error_main_to_sub_label`).
                format!("label `{shown}` is not defined in the main module")
            };
            return Err(self.error(t.span, msg));
        };
        self.stmt.names.push((SymbolKind::Label(id), t.span));
        Ok(id)
    }

    /// `RESTORE` or `RESTORE label`. The label has no scope (`qb64pe.bas`: "a RESTORE label has no scope, therefore,
    /// only one instance of that label may exist"; measured, `verification\v22_c_restore`, `v22_x65`, `x72`, `x83`):
    /// it may stand in any body, and a name that two bodies have is "Ambiguous DATA label". A line number is a
    /// label as any other (`v22_c_restore`, `v22_x86`).
    pub(super) fn restore(&mut self, s: ast::RestoreStmt) -> R<()> {
        let node = s.node();
        let Some(t) = s.target() else {
            self.push(node, StmtKind::Restore(None));
            return Ok(());
        };
        let shown = show_bytes(self.text(t.span));
        let name = self.target_name(t)?;
        let mut found = self
            .labels_by_name
            .iter()
            .filter(|((_, n), _)| *n == name)
            .map(|(_, &id)| id);
        let (first, second) = (found.next(), found.next());
        let Some(id) = first else {
            if let Some(e) = self.builtin_label(t, &name, &shown, false) {
                return Err(e);
            }
            // A label with such a name is "not supported yet" (`declare_label`), so it may exist.
            if self.marked_label_name(&name) {
                let msg = format!("a label with the name of a SUB or FUNCTION: `{shown}`");
                return Err(self.unsupported(t.span, msg));
            }
            return Err(self.error(t.span, format!("label `{shown}` is not defined")));
        };
        if second.is_some() {
            let msg =
                format!("`RESTORE {shown}`: more than one SUB, FUNCTION or the main module has a label `{shown}`");
            return Err(self.error(t.span, msg));
        }
        self.stmt.names.push((SymbolKind::Label(id), t.span));
        self.push(node, StmtKind::Restore(Some(id)));
        Ok(())
    }

    /// The target of `GOTO` (`goto`), `GOSUB` or `RETURN`: a label or line number of the current body.
    fn jump_target(&mut self, t: Tok, goto: bool) -> R<LabelId> {
        self.label_for(t, self.cur, goto)
    }

    /// `GOTO label`.
    pub(super) fn goto(&mut self, s: ast::GotoStmt) -> R<()> {
        let node = s.node();
        let t = self.need(s.target(), node.span())?;
        let l = self.jump_target(t, true)?;
        self.push(node, StmtKind::Goto(l));
        Ok(())
    }

    /// `THEN 10` or `ELSE 20` of a single-line `IF`: a `GOTO` (`verification\v17_b_if_then_line`).
    pub(super) fn implicit_goto(&mut self, s: ast::ImplicitGoto) -> R<()> {
        let node = s.node();
        let t = self.need(s.number(), node.span())?;
        let l = self.jump_target(t, true)?;
        self.push(node, StmtKind::Goto(l));
        Ok(())
    }

    /// `GOSUB label`, in any body (measured: a `GOSUB` in a SUB or FUNCTION works within it).
    pub(super) fn gosub(&mut self, s: ast::GosubStmt) -> R<()> {
        let node = s.node();
        let t = self.need(s.target(), node.span())?;
        let l = self.jump_target(t, false)?;
        self.push(node, StmtKind::Gosub(l));
        Ok(())
    }

    /// `ON n GOTO l1, l2, …` / `ON n GOSUB …` (design D8 of `m2-core-builtins`, `qb64pe.bas` 27620): `n` converted
    /// to LONG as the old compiler does, a float through `qbr_float_to_long` (narrowed to SINGLE, then rounded half
    /// to even), an integer keeping its low 32 bits; each target a label or line number of this body.
    pub(super) fn on_jump(&mut self, s: ast::OnJumpStmt) -> R<()> {
        let node = s.node();
        let span = node.span();
        let n = self.need(s.value(), span)?;
        let kw = self.need(s.keyword(), span)?;
        let gosub = self.word(kw) == "GOSUB";
        let e = self.expr(n)?;
        if e.ty == Ty::Str {
            // Measured: "Expected numeric expression" (`verification\v20_x18_on_string`).
            return Err(self.error(
                e.span,
                format!("the value of `ON … {}` must be a number", self.word(kw)),
            ));
        }
        let value = if e.qb.is_float() {
            let single = self.convert_exact(e, Ty::F32);
            super::expr::conv(single, Ty::I32, ConvKind::RoundEven)
        } else {
            self.convert_exact(e, Ty::I32)
        };
        let mut targets = Vec::new();
        for t in s.targets() {
            let Some(t) = t else {
                return Err(self.unsupported(span, "an omitted target of `ON … GOTO`/`ON … GOSUB`"));
            };
            targets.push(self.jump_target(t, !gosub)?);
        }
        self.push(node, StmtKind::OnJump { value, gosub, targets });
        Ok(())
    }

    /// `RETURN` or `RETURN label`. The second form is an error in a procedure (measured,
    /// `verification\v17_d_err_return_label_sub`).
    pub(super) fn return_stmt(&mut self, s: ast::ReturnStmt) -> R<()> {
        let node = s.node();
        let target = match s.target() {
            None => None,
            Some(t) if self.cur.is_some() => {
                let msg = "`RETURN` with a label or line number is not allowed in a SUB or FUNCTION";
                return Err(self.error(t.span, msg));
            }
            Some(t) => Some(self.jump_target(t, false)?),
        };
        self.push(node, StmtKind::Return(target));
        Ok(())
    }

    /// `ON ERROR GOTO label`, `0`, `_NEWHANDLER label` or `_LASTHANDLER`, with the chain's errors as the old compiler
    /// gives them (`qb64pe.bas` 10049–10053; design D1 of `m4-handlers-logging-checking`). Inside a procedure the
    /// label is one of the main module's (measured, `v14_on_error_sub_to_main`): the handler is the program's. A
    /// label of the procedure itself is an error, also when the main module has one of that name (measured,
    /// `v14_on_error_in_sub`, `v17_d_on_error_sub_label_both`).
    pub(super) fn on_error(&mut self, s: ast::OnErrorStmt) -> R<()> {
        let node = s.node();
        let t = self.need(s.target(), node.span())?;
        let new = match s.handler_word() {
            Some(w) if self.word(w) == "_LASTHANDLER" => {
                return Err(self.error(t.span, "No label allowed after _LASTHANDLER"));
            }
            Some(_) if self.is_zero(t) => {
                return Err(self.error(t.span, "Zero not allowed after _NEWHANDLER"));
            }
            Some(_) => true,
            None if t.kind == Number => false,
            None => match self.word(t).as_str() {
                "_LASTHANDLER" => {
                    self.push(node, StmtKind::OnError(ErrorHandler::Last));
                    return Ok(());
                }
                "_NEWHANDLER" => {
                    return Err(self.error(t.span, "Expected: ON ERROR GOTO [_NEWHANDLER] label"));
                }
                _ => false,
            },
        };
        if !new && self.text(t.span).starts_with(b"_") {
            // A label QB64 would reject; not a label of this program.
            let msg = format!(
                "`ON ERROR GOTO {}`",
                show_bytes(&self.text(t.span).to_ascii_uppercase())
            );
            return Err(self.unsupported(t.span, msg));
        }
        let handler = if !new && self.is_zero(t) {
            None
        } else {
            if self.cur.is_some() {
                let name = self.target_name(t)?;
                if self.labels_by_name.contains_key(&(self.cur, name)) {
                    let msg = format!(
                        "`ON ERROR GOTO` needs a label of the main module, not one of this SUB or FUNCTION: `{}`",
                        show_bytes(self.text(t.span))
                    );
                    return Err(self.error(t.span, msg));
                }
            }
            Some(self.label_ref(t, None)?)
        };
        let handler = match handler {
            Some(l) if new => ErrorHandler::New(l),
            h => ErrorHandler::Set(h),
        };
        self.push(node, StmtKind::OnError(handler));
        Ok(())
    }

    /// `RESUME`, `RESUME 0`, `RESUME NEXT`, `RESUME label`. Inside a procedure the label is one of the procedure's:
    /// one of the main module's is an error (measured, `v14_err_label_in_main_from_sub`, `v57_x05`, `x10`–`x12`).
    /// In a SUB the statement is the main module's (design D7 of `m4-statements-metacommands`; measured,
    /// `v17_d_resume_sub_label`, `v57_x01`–`x08`): the old compiler writes the same C++ line there, whose `return`
    /// leaves the SUB. In a FUNCTION that line does not build in the old compiler (`v57_x09`).
    pub(super) fn resume(&mut self, s: ast::ResumeStmt) -> R<()> {
        let node = s.node();
        let target = s.target();
        let is_next = target.is_some_and(|t| t.kind != Number && self.word(t) == "NEXT");
        if let Some(p) = self.cur
            && matches!(self.prog.proc(p).kind, ProcKind::Function(_))
        {
            if let Some(t) = target.filter(|&t| !is_next && !self.is_zero(t)) {
                self.label_ref(t, self.cur)?;
            }
            let msg = "`RESUME` cannot stand in a FUNCTION: QB64pe's C++ for it does not build";
            return Err(self.error(super::first_token_span(node), msg));
        }
        let resume = match target {
            None => Resume::Retry,
            Some(t) if self.is_zero(t) => Resume::Retry,
            Some(_) if is_next => Resume::Next,
            Some(t) => Resume::To(self.label_ref(t, self.cur)?),
        };
        self.push(node, StmtKind::Resume(resume));
        Ok(())
    }

    /// `RUN`, `RUN label`, `RUN line`, `RUN file$` (design D5 of `m4-statements-metacommands`; `qb64pe.bas`
    /// 9764–9836; measured, `verification\v57_r01`–`r19`, `v28_x86`). A name or a number alone after `RUN` is a label
    /// of the main module, from any body: one that is a SUB's name is "Name already in use", any other name that is
    /// no label of the main module "Label not defined" (a numeric variable too). Anything else is a file name: a
    /// number there is "Illegal string-number conversion".
    pub(super) fn run(&mut self, s: ast::RunStmt) -> R<()> {
        let node = s.node();
        // A name that means a string is a file name, as the old compiler types the element before it takes it for a
        // label: by its suffix, a string `CONST` of the name, or the type a variable of the name has here (a
        // `DIM … AS STRING`, or `DEFSTR` for its first letter, assigned or not) (`v57_r16`, `r20`–`r24`).
        let alone = match s.operand() {
            Some(ast::Expr::Literal(l)) => l.token().filter(|t| t.kind == Number),
            Some(ast::Expr::NameRef(n)) => match n.name() {
                Some(t) => {
                    let (name, suffix) = self.split_name(t)?;
                    let string = match (suffix, self.visible_const(&name)) {
                        (Some(ty), _) => ty.is_string(),
                        (None, Some(c)) => self.prog.constant(c).ty.is_string(),
                        (None, None) => self.var_key(name, None).1.is_string(),
                    };
                    (!string).then_some(t)
                }
                None => None,
            },
            Some(_) | None => None,
        };
        let target = if let Some(t) = alone {
            if t.kind == Ident && self.procs_by_name.contains_key(&self.word(t)) {
                let msg = format!(
                    "`RUN {}`: `{0}` is the name of a SUB, not a label",
                    show_bytes(self.text(t.span))
                );
                return Err(self.error(t.span, msg));
            }
            RunTarget::Label(self.label_ref(t, None)?)
        } else if let Some(e) = s.operand() {
            let file = self.expr(e)?;
            if file.ty != Ty::Str {
                return Err(self.error(file.span, "`RUN` takes a label, a line number or a file name"));
            }
            RunTarget::File(file)
        } else {
            RunTarget::Restart
        };
        self.push(node, StmtKind::Run(target));
        Ok(())
    }

    /// `ERROR n`: `n` is stored as a LONG (the old compiler emits `error(qbr(x))`: half to even).
    pub(super) fn error_stmt(&mut self, s: ast::ErrorStmt) -> R<()> {
        let node = s.node();
        let value = self.need(s.value(), node.span())?;
        let e = self.expr(value)?;
        if e.ty == Ty::Str {
            return Err(self.error(e.span, "`ERROR` needs a number"));
        }
        let code = self.store(e, Ty::I32)?;
        self.push(node, StmtKind::Error(code));
        Ok(())
    }
}

/// The name a line number gives its line, and the text the old compiler sets `last_line` to after it, as it spells
/// them (`validlabel`, `qb64pe.bas` 27121, on the text its line formatter keeps, 24688 and 24847; measured,
/// `verification\v54_x03`, `x04`, `x11`, `x12`, `x20`, `x23`–`x25`): a whole number without a suffix by its digits
/// without leading zeros (`010` is `10`); one with a decimal point or a suffix by its text as written, `.` as `p`,
/// `#` as a trailing `d` and `!` as `s`, any other suffix left off (`10&` is `10`; `10.0` and `30#` are their own).
/// `last_line` gets the digits and the point (`010&` writes `010`, which C++ reads as octal: `ERL` 8, as in the old
/// compiler). `None` where the old compiler takes the text for no label: an exponent, a `$`, two points, a point
/// alone, a name of more than 40 characters.
pub(crate) fn line_label_name(text: &[u8]) -> Option<(String, String)> {
    let end = text
        .iter()
        .position(|&b| !(b.is_ascii_digit() || b == b'.'))
        .unwrap_or(text.len());
    let (number, suffix) = text.split_at(end);
    // Only digits and points: one character each.
    let number: String = number.iter().map(|&b| char::from(b)).collect();
    let points = number.matches('.').count();
    if number.is_empty() || number == "." || points > 1 {
        return None;
    }
    if suffix.iter().any(|&b| b.is_ascii_alphabetic() || b == b'$') {
        return None;
    }
    let (name, last_line) = if points == 0 && suffix.is_empty() {
        let digits = number.trim_start_matches('0');
        let digits = if digits.is_empty() { "0" } else { digits };
        (digits.to_string(), digits.to_string())
    } else {
        let mut name = number.replace('.', "p");
        match suffix {
            b"#" => name.push('d'),
            b"!" => name.push('s'),
            _ => {}
        }
        (name, number)
    };
    (name.len() <= 40).then_some((name, last_line))
}

/// Why the C++ build rejects `last_line=<text>;` for a line number's `last_line` text, or `None` when it takes
/// it: a whole number with a leading 0 is octal there, so an 8 or 9 is no digit of it, and an integer above
/// `u64::MAX` has no C++ type. The old compiler writes the text as it is, and its C++ build fails (measured,
/// `verification\v54_x40`, `x41`). A text with a point is a floating literal and always taken.
fn cpp_line_problem(text: &str) -> Option<&'static str> {
    if text.contains('.') {
        return None;
    }
    let (digits, radix) = match text.strip_prefix('0') {
        Some(rest) if !rest.is_empty() => (rest, 8),
        _ => (text, 10),
    };
    if radix == 8 && digits.bytes().any(|b| b == b'8' || b == b'9') {
        return Some("as an octal number, which has no digit 8 or 9");
    }
    u64::from_str_radix(digits, radix)
        .is_err()
        .then_some("as an integer above 18446744073709551615, which no C++ type holds")
}
