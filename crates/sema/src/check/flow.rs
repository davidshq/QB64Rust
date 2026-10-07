//! Labels and error handling: `ON ERROR GOTO`, `RESUME`, `ERROR`.

use super::{Checker, R, Skips, block_parts};
use crate::{Label, LabelId, Resume, StmtKind, SymbolKind, Ty};
use qb64rust_base::{show_bytes, to_u32};
use qb64rust_builtins::find_any;
use qb64rust_syntax::SyntaxKind::Number;
use qb64rust_syntax::ast;
use qb64rust_syntax::tree::{Node, Tok};

impl Checker<'_> {
    /// Enters the main module's labels among `statements` and inside their blocks (not inside procedures).
    pub(super) fn declare_labels(&mut self, statements: &[Node], skips: &Skips) {
        for &stmt in statements {
            if skips.past_cap(stmt) {
                break;
            }
            if let Some(block) = block_parts(stmt) {
                self.declare_labels(&block.inner, skips);
            } else if let Some(l) = ast::LabelDef::cast(stmt)
                && skips.usable(stmt)
            {
                self.stmt_error = false;
                let _ = self.declare_label(l);
                self.flush_names();
            }
        }
    }

    // ---- labels and error handling ----

    /// Enters a main-module label (before pass 2). The old compiler does not take a built-in name as a label
    /// (`CLS:` is a call of `CLS`), and may take a SUB name as a call; both are left unsupported here.
    fn declare_label(&mut self, l: ast::LabelDef) -> R<()> {
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
        if self.labels_by_name.contains_key(&name) {
            return Err(self.error(t.span, format!("duplicate label: `{shown}`")));
        }
        let id = LabelId(to_u32(self.prog.labels.len()));
        let line = self.line(t.span);
        self.prog.labels.push(Label {
            name: name.clone(),
            line,
            at: 0,
        });
        self.labels_by_name.insert(name, id);
        self.label_of_def.insert(node.key(), id);
        self.names.push((SymbolKind::Label(id), t.span));
        Ok(())
    }

    /// A label in pass 2: it stands before the next statement of the main module.
    pub(super) fn label_stmt(&mut self, l: ast::LabelDef) -> R<()> {
        if self.cur.is_some() {
            let span = l.node().span();
            return Err(self.unsupported(span, "labels inside a SUB or FUNCTION"));
        }
        if let Some(&id) = self.label_of_def.get(&l.node().key()) {
            self.prog.labels[id.0 as usize].at = self.prog.stmts.len();
        }
        let span = l.node().span();
        self.label_line = Some((span.file, self.line(span)));
        Ok(())
    }

    /// A label named by `ON ERROR GOTO` or `RESUME`: one of the main module's.
    fn label_ref(&mut self, t: Tok) -> R<LabelId> {
        let shown = show_bytes(self.text(t.span));
        let (name, suffix) = self.split_name(t)?;
        if suffix.is_some() {
            return Err(self.error(t.span, format!("`{shown}` is not a valid label")));
        }
        let Some(&id) = self.labels_by_name.get(&name) else {
            // A label with such a name is "not supported yet" (`declare_label`), so it may exist.
            if self.procs_by_name.contains_key(&name) || find_any(name.as_bytes()).next().is_some() {
                let msg = format!("a label with the name of a SUB, FUNCTION or built-in: `{shown}`");
                return Err(self.unsupported(t.span, msg));
            }
            return Err(self.error(t.span, format!("label `{shown}` is not defined")));
        };
        self.names.push((SymbolKind::Label(id), t.span));
        Ok(id)
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
    /// `v14_on_error_sub_to_main`): the handler is the program's.
    pub(super) fn on_error(&mut self, s: ast::OnErrorStmt) -> R<()> {
        let node = s.node();
        let t = self.need(s.target(), node.span())?;
        let handler = if t.kind == Number {
            self.zero(t)?;
            None
        } else {
            Some(self.label_ref(t)?)
        };
        self.push(node, StmtKind::OnError(handler));
        Ok(())
    }

    /// `RESUME`, `RESUME 0`, `RESUME NEXT`, `RESUME label`. Inside a procedure a label is an error, as with the old
    /// compiler (a procedure has no labels here, and the main module's are not visible); the other forms are not
    /// measured there and not supported yet.
    pub(super) fn resume(&mut self, s: ast::ResumeStmt) -> R<()> {
        let node = s.node();
        let target = s.target();
        let is_next = target.is_some_and(|t| t.kind != Number && self.word(t) == "NEXT");
        if self.cur.is_some() {
            if let Some(t) = target.filter(|t| t.kind != Number && !is_next) {
                let msg = format!(
                    "label `{}` is not defined in this SUB or FUNCTION (the main module's labels are not visible here)",
                    show_bytes(self.text(t.span))
                );
                return Err(self.error(t.span, msg));
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
            Some(t) => Resume::To(self.label_ref(t)?),
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
