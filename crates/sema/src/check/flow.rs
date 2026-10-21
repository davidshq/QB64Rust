//! Labels, jumps and error handling: `GOTO`, `GOSUB`, `RETURN`, `ON ERROR GOTO`, `RESUME`, `ERROR`.
//!
//! Labels belong to a body, the main module or one procedure (design D5): each body's labels are entered in a pass
//! before the statements, so a jump may name a label further down, also one inside a block. A jump names a label
//! of its own body; `ON ERROR GOTO` names one of the main module's, from any body (all measured).

use super::{Checker, R, Skips, nested_statements};
use crate::builtins::stmt_lookup;
use crate::{ConvKind, ErrorHandler, Label, LabelId, ProcId, ProcKind, Resume, StmtKind, SymbolKind, Ty};
use qb64rust_base::{Span, show_bytes, to_u32};
use qb64rust_builtins::find_any;
use qb64rust_syntax::SyntaxKind::{
    GosubStmt, GotoStmt, Ident, Number, OnErrorStmt, OnEventStmt, OnJumpStmt, RestoreStmt, ResumeStmt, ReturnStmt,
};
use qb64rust_syntax::ast;
use qb64rust_syntax::tree::{Node, Tok};
use std::collections::HashSet;

impl Checker<'_> {
    /// Enters the labels of one body (`body`: `None` for the main module) among `statements` and inside their
    /// blocks; a procedure inside them is another body and is left out.
    pub(super) fn declare_labels(&mut self, statements: &[Node], skips: &Skips, body: Option<ProcId>) {
        for &stmt in statements {
            if skips.past_cap(stmt) {
                break;
            }
            if let Some(tree) = self.program.included(stmt) {
                // The labels of an included file belong to the body it is included in.
                let inner: Vec<Node> = ast::SourceFile::cast(tree.root())
                    .into_iter()
                    .flat_map(|f| f.statements())
                    .collect();
                self.declare_labels(&inner, skips, body);
            } else if let Some(inner) = nested_statements(stmt) {
                self.declare_labels(&inner, skips, body);
            } else if let Some(l) = ast::LabelDef::cast(stmt)
                && skips.usable(stmt)
            {
                self.stmt.stmt_error = false;
                let _ = self.declare_label(l, body);
                self.flush_names();
            }
        }
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
        let key = (body, name);
        if self.labels_by_name.contains_key(&key) {
            return Err(self.error(t.span, format!("duplicate label: `{shown}`")));
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
            data_at: 0,
        });
        self.labels_by_name.insert(key, id);
        self.label_of_def.insert(node.key(), id);
        self.stmt.names.push((SymbolKind::Label(id), t.span));
        if reserved {
            return Err(self.error(t.span, format!("names starting with `_` are reserved: `{shown}`")));
        }
        Ok(())
    }

    /// A label in pass 2: it stands before the next statement of its body.
    pub(super) fn label_stmt(&mut self, l: ast::LabelDef) -> R<()> {
        if let Some(&id) = self.label_of_def.get(&l.node().key()) {
            // Pass 2 is in file order, so the items collected so far are those before the label.
            self.prog.labels[id.0 as usize].data_at = self.prog.data.len();
            self.push(l.node(), StmtKind::Label(id));
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

    /// Whether a label of this name gets a mark in place of an entry ([`Self::declare_label`]): the name of a
    /// built-in, or of a SUB or FUNCTION where the label could not be told from a call.
    fn marked_label_name(&self, name: &str) -> bool {
        find_any(name.as_bytes()).next().is_some()
            || self
                .procs_by_name
                .get(name)
                .is_some_and(|&p| self.unclear_proc_label(p, name))
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

    /// A label of `body` named at `t`: `ON ERROR GOTO` and `RESUME` in the main module, and every jump.
    pub(super) fn label_ref(&mut self, t: Tok, body: Option<ProcId>) -> R<LabelId> {
        let shown = show_bytes(self.text(t.span));
        let (name, suffix) = self.split_name(t)?;
        if suffix.is_some() {
            return Err(self.error(t.span, format!("`{shown}` is not a valid label")));
        }
        let Some(&id) = self.labels_by_name.get(&(body, name.clone())) else {
            // A label with such a name is "not supported yet" (`declare_label`), so it may exist.
            if self.marked_label_name(&name) {
                let msg = format!("a label with the name of a SUB, FUNCTION or built-in: `{shown}`");
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
    /// it may stand in any body, and a name that two bodies have is "Ambiguous DATA label". A line number is not
    /// supported yet.
    pub(super) fn restore(&mut self, s: ast::RestoreStmt) -> R<()> {
        let node = s.node();
        let Some(t) = s.target() else {
            self.push(node, StmtKind::Restore(None));
            return Ok(());
        };
        if t.kind == Number {
            return Err(self.unsupported(t.span, "line numbers"));
        }
        let shown = show_bytes(self.text(t.span));
        let (name, suffix) = self.split_name(t)?;
        if suffix.is_some() {
            return Err(self.error(t.span, format!("`{shown}` is not a valid label")));
        }
        let mut found = self
            .labels_by_name
            .iter()
            .filter(|((_, n), _)| *n == name)
            .map(|(_, &id)| id);
        let (first, second) = (found.next(), found.next());
        let Some(id) = first else {
            // A label with such a name is "not supported yet" (`declare_label`), so it may exist.
            if self.marked_label_name(&name) {
                let msg = format!("a label with the name of a SUB, FUNCTION or built-in: `{shown}`");
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

    /// The target of `GOTO`, `GOSUB` or `RETURN`: a label of the current body. A number is a line number.
    fn jump_target(&mut self, t: Tok) -> R<LabelId> {
        if t.kind == Number {
            return Err(self.unsupported(t.span, "line numbers"));
        }
        self.label_ref(t, self.cur)
    }

    /// `GOTO label`.
    pub(super) fn goto(&mut self, s: ast::GotoStmt) -> R<()> {
        let node = s.node();
        let t = self.need(s.target(), node.span())?;
        let l = self.jump_target(t)?;
        self.push(node, StmtKind::Goto(l));
        Ok(())
    }

    /// `GOSUB label`, in any body (measured: a `GOSUB` in a SUB or FUNCTION works within it).
    pub(super) fn gosub(&mut self, s: ast::GosubStmt) -> R<()> {
        let node = s.node();
        let t = self.need(s.target(), node.span())?;
        let l = self.jump_target(t)?;
        self.push(node, StmtKind::Gosub(l));
        Ok(())
    }

    /// `ON n GOTO l1, l2, …` / `ON n GOSUB …` (design D8 of `m2-core-builtins`, `qb64pe.bas` 27620): `n` converted
    /// to LONG as the old compiler does, a float through `qbr_float_to_long` (narrowed to SINGLE, then rounded half
    /// to even), an integer keeping its low 32 bits; each target a label of this body (line numbers are not
    /// supported yet).
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
            targets.push(self.jump_target(t)?);
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
            Some(t) => Some(self.jump_target(t)?),
        };
        self.push(node, StmtKind::Return(target));
        Ok(())
    }

    /// The number after `GOTO` or `RESUME`: only `0` is supported (other numbers are line numbers).
    fn zero(&mut self, t: Tok) -> R<()> {
        if self.text(t.span) == b"0" {
            Ok(())
        } else {
            Err(self.unsupported(t.span, "line numbers"))
        }
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
            Some(_) if t.kind == Number && self.text(t.span) == b"0" => {
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
        let handler = if t.kind == Number && !new {
            self.zero(t)?;
            None
        } else {
            if self.cur.is_some() {
                let (name, _) = self.split_name(t)?;
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
    /// one of the main module's is an error (measured, `v14_err_label_in_main_from_sub`), one of the procedure's
    /// compiles (`v17_d_resume_sub_label`); `RESUME` itself is not supported yet there.
    pub(super) fn resume(&mut self, s: ast::ResumeStmt) -> R<()> {
        let node = s.node();
        let target = s.target();
        let is_next = target.is_some_and(|t| t.kind != Number && self.word(t) == "NEXT");
        if self.cur.is_some() {
            if let Some(t) = target.filter(|t| t.kind != Number && !is_next) {
                self.label_ref(t, self.cur)?;
            }
            return Err(self.unsupported(node.span(), "`RESUME` inside a SUB or FUNCTION"));
        }
        let resume = match target {
            None => Resume::Retry,
            Some(t) if t.kind == Number => {
                self.zero(t)?;
                Resume::Retry
            }
            Some(_) if is_next => Resume::Next,
            Some(t) => Resume::To(self.label_ref(t, None)?),
        };
        self.push(node, StmtKind::Resume(resume));
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
