//! Labels, jumps and error handling: `GOTO`, `GOSUB`, `RETURN`, `ON ERROR GOTO`, `RESUME`, `ERROR`.
//!
//! Labels belong to a body, the main module or one procedure (design D5): each body's labels are entered in a pass
//! before the statements, so a jump may name a label further down, also one inside a block. A jump names a label
//! of its own body; `ON ERROR GOTO` names one of the main module's, from any body (all measured).

use super::{Checker, R, Skips, nested_statements};
use crate::{Label, LabelId, ProcId, Resume, StmtKind, SymbolKind, Ty};
use qb64rust_base::{show_bytes, to_u32};
use qb64rust_builtins::find_any;
use qb64rust_syntax::SyntaxKind::Number;
use qb64rust_syntax::ast;
use qb64rust_syntax::tree::{Node, Tok};

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
                self.stmt_error = false;
                let _ = self.declare_label(l, body);
                self.flush_names();
            }
        }
    }

    // ---- labels and error handling ----

    /// Enters a label of `body` (before pass 2). The old compiler does not take a built-in name as a label
    /// (`CLS:` is a call of `CLS`), and may take a SUB name as a call; both are left unsupported here.
    fn declare_label(&mut self, l: ast::LabelDef, body: Option<ProcId>) -> R<()> {
        let node = l.node();
        let t = self.need(l.name(), node.span())?;
        let name = self.word(t);
        let shown = show_bytes(self.text(t.span));
        if find_any(name.as_bytes()).next().is_some() {
            return Err(self.unsupported(t.span, format!("the label `{shown}:` (`{shown}` is a built-in)")));
        }
        if self.procs_by_name.contains_key(&name) {
            let msg = format!("a label with the name of a SUB or FUNCTION: `{shown}`");
            return Err(self.unsupported(t.span, msg));
        }
        let key = (body, name);
        if self.labels_by_name.contains_key(&key) {
            return Err(self.error(t.span, format!("duplicate label: `{shown}`")));
        }
        let id = LabelId(to_u32(self.prog.labels.len()));
        let line = self.line(t.span);
        self.prog.labels.push(Label {
            name: key.1.clone(),
            line,
            file: t.span.file,
            proc: body,
        });
        self.labels_by_name.insert(key, id);
        self.label_of_def.insert(node.key(), id);
        self.names.push((SymbolKind::Label(id), t.span));
        Ok(())
    }

    /// A label in pass 2: it stands before the next statement of its body.
    pub(super) fn label_stmt(&mut self, l: ast::LabelDef) -> R<()> {
        let span = l.node().span();
        self.label_line = Some((span.file, self.line(span)));
        // Without an entry the label's declaration had an error, already reported.
        if let Some(&id) = self.label_of_def.get(&l.node().key()) {
            self.push(l.node(), StmtKind::Label(id));
        }
        Ok(())
    }

    /// A label of `body` named at `t`: `ON ERROR GOTO` and `RESUME` in the main module, and every jump.
    fn label_ref(&mut self, t: Tok, body: Option<ProcId>) -> R<LabelId> {
        let shown = show_bytes(self.text(t.span));
        let (name, suffix) = self.split_name(t)?;
        if suffix.is_some() {
            return Err(self.error(t.span, format!("`{shown}` is not a valid label")));
        }
        let Some(&id) = self.labels_by_name.get(&(body, name.clone())) else {
            // A label with such a name is "not supported yet" (`declare_label`), so it may exist.
            if self.procs_by_name.contains_key(&name) || find_any(name.as_bytes()).next().is_some() {
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
        self.names.push((SymbolKind::Label(id), t.span));
        Ok(id)
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

    /// `ON ERROR GOTO label|0`. Inside a procedure the label is one of the main module's (measured,
    /// `v14_on_error_sub_to_main`): the handler is the program's. A label of the procedure itself is an error, also
    /// when the main module has one of that name (measured, `v14_on_error_in_sub`, `v17_d_on_error_sub_label_both`).
    pub(super) fn on_error(&mut self, s: ast::OnErrorStmt) -> R<()> {
        let node = s.node();
        if let Some(w) = s.handler_word() {
            let msg = format!(
                "`ON ERROR GOTO {}`",
                show_bytes(&self.text(w.span).to_ascii_uppercase())
            );
            return Err(self.unsupported(w.span, msg));
        }
        let t = self.need(s.target(), node.span())?;
        if self.text(t.span).starts_with(b"_") {
            // `_LASTHANDLER`, or a label QB64 would reject; neither is a label of this program.
            let msg = format!(
                "`ON ERROR GOTO {}`",
                show_bytes(&self.text(t.span).to_ascii_uppercase())
            );
            return Err(self.unsupported(t.span, msg));
        }
        let handler = if t.kind == Number {
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
