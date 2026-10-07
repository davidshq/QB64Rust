//! Blocks (design D3 of `m2-control-flow-slice`). `IF` (both forms), `FOR`, `DO` and `WHILE` become typed block
//! statements that keep the source structure; `EXIT FOR/DO/WHILE` leaves one. The other blocks are still marked as
//! a whole, with the statements inside checked one by one.

use super::{Checker, R, Skips, first_token_span};
use crate::{Branch, Expr, LoopKind, LoopTest, Stmt, StmtKind, SymbolKind, TestAt, Ty, VarId};
use qb64rust_base::show_bytes;
use qb64rust_syntax::ast;
use qb64rust_syntax::tree::{Node, Tok};

impl Checker<'_> {
    /// A block statement of any kind; false for any other statement.
    pub(super) fn block_statement(&mut self, node: Node, skips: &Skips) -> bool {
        if let Some(b) = ast::IfBlock::cast(node) {
            self.if_block(b, skips);
        } else if let Some(s) = ast::IfStmt::cast(node) {
            self.if_stmt(s, skips);
        } else if let Some(b) = ast::ForBlock::cast(node) {
            self.for_block(b, skips);
        } else if let Some(b) = ast::DoBlock::cast(node) {
            self.do_block(b, skips);
        } else if let Some(b) = ast::WhileBlock::cast(node) {
            self.while_block(b, skips);
        } else if let Some(block) = block_parts(node) {
            self.marked_block(node, block, skips);
        } else {
            return false;
        }
        true
    }

    /// Checks `nodes` as the statements of a block and returns them typed. Statements with errors are left out.
    fn block_body<'t>(&mut self, nodes: impl Iterator<Item = Node<'t>>, skips: &Skips) -> Vec<Stmt> {
        self.sinks.push(Vec::new());
        for s in nodes {
            if skips.past_cap(s) {
                break;
            }
            self.check_statement(s, skips);
        }
        self.sinks.pop().expect("pushed above")
    }

    /// Checks a block's header (`check`, which reports its own errors) as one statement: names are recorded only
    /// when it has no error. `None` after an error or when the header has a parse error.
    fn header<T>(&mut self, header: Option<Node>, skips: &Skips, check: impl FnOnce(&mut Self) -> R<T>) -> Option<T> {
        let header = header?;
        if !skips.usable(header) {
            return None;
        }
        self.stmt_error = false;
        let result = check(self).ok();
        self.flush_names();
        result
    }

    /// A condition of `IF`, `ELSEIF`, `WHILE`, `DO` or `LOOP`: any number (true when not zero). A string is an
    /// error, with a message per statement in the old compiler (measured, `verification\v17_c_err_*_string`).
    fn condition(&mut self, cond: Option<ast::Expr>, span: qb64rust_base::Span, what: &str) -> R<Expr> {
        let node = self.need(cond, span)?;
        let e = self.expr(node)?;
        if e.ty == Ty::Str {
            return Err(self.error(
                e.span,
                format!("the condition of `{what}` must be a number, not a string"),
            ));
        }
        Ok(e)
    }

    fn if_block(&mut self, b: ast::IfBlock, skips: &Skips) {
        let node = b.node();
        let mut branches = Some(Vec::new());
        let first = b.if_branch().map(|f| (f.header(), f.body().collect::<Vec<_>>()));
        let others = b.else_if_branches().map(|e| (e.header(), e.body().collect::<Vec<_>>()));
        for (header, body) in first.into_iter().chain(others) {
            let cond = self.header(header.map(|h| h.node()), skips, |c| {
                let h = header.expect("checked by `header`");
                let what = h.keyword().map_or("IF".to_string(), |k| c.word(k));
                c.condition(h.condition(), h.node().span(), &what)
            });
            let body = self.block_body(body.into_iter(), skips);
            match (cond, &mut branches) {
                (Some(cond), Some(list)) => list.push(Branch { cond, body }),
                _ => branches = None,
            }
        }
        let else_ = b.else_branch().map(|e| self.block_body(e.body(), skips));
        if let Some(branches) = branches.filter(|l| !l.is_empty()) {
            self.push(node, StmtKind::If { branches, else_ });
        }
    }

    /// A single-line `IF`: the same statement as the block form (`IF c GOTO x` has the `GOTO` as its branch).
    fn if_stmt(&mut self, s: ast::IfStmt, skips: &Skips) {
        let header = s.header();
        let cond = self.header(header.map(|h| h.node()), skips, |c| {
            let h = header.expect("checked by `header`");
            c.condition(h.condition(), h.node().span(), "IF")
        });
        let body = self.block_body(s.then_branch().into_iter().flat_map(|b| b.statements()), skips);
        let else_ = s.else_branch().map(|b| self.block_body(b.statements(), skips));
        if let Some(cond) = cond {
            let branches = vec![Branch { cond, body }];
            self.push(s.node(), StmtKind::If { branches, else_ });
        }
    }

    fn for_block(&mut self, b: ast::ForBlock, skips: &Skips) {
        let header = b.header();
        let parts = self.header(header.map(|h| h.node()), skips, |c| {
            c.for_header(header.expect("checked"))
        });
        let body = self.block_body(b.body(), skips);
        let next = b.next();
        let end_line = next.map_or(0, |n| self.line(first_token_span(n.node())));
        let Some(((var, name), temp, start, end, step)) = parts else {
            return;
        };
        // The variable `NEXT` names for this loop, checked as a statement of its own where `NEXT` stands. One
        // `NEXT j, i` closes two loops, inner first; once it had an error, the outer loop reports no second one.
        if let (Some(next), Some(next_name)) = (next, b.next_var()) {
            let key = next.node().key();
            if self.bad_next == Some(key) {
                return;
            }
            if self
                .header(Some(next.node()), skips, |c| c.next_var(next_name, var, name))
                .is_none()
            {
                self.bad_next = Some(key);
                return;
            }
        }
        let kind = StmtKind::For {
            var,
            temp,
            start,
            end,
            step,
            body,
            end_line,
        };
        self.push(b.node(), kind);
    }

    /// `FOR var = start TO end [STEP step]`: the variable (and its name token), the type the loop counts in, and
    /// the three values converted to it.
    #[expect(clippy::type_complexity, reason = "one use; the parts of `StmtKind::For`")]
    fn for_header(&mut self, h: ast::ForHeader) -> R<((VarId, Tok), Ty, Expr, Expr, Option<Expr>)> {
        let span = h.node().span();
        let name = self.need(h.var().and_then(|v| v.name()), span)?;
        let var = self.for_var(name)?;
        let temp = for_temp(self.prog.var(var).ty);
        let limit = |c: &mut Self, e: Option<ast::Expr>| -> R<Expr> {
            let node = c.need(e, span)?;
            let e = c.expr(node)?;
            if e.ty == Ty::Str {
                // Measured: "Illegal string-number conversion" (`v17_c_err_for_string_limit`).
                return Err(c.error(e.span, "a `FOR` start, limit or step must be a number, not a string"));
            }
            c.store(e, temp)
        };
        let start = limit(self, h.start())?;
        let end = limit(self, h.end())?;
        let step = match h.step() {
            Some(s) => Some(limit(self, Some(s))?),
            None => None,
        };
        Ok(((var, name), temp, start, end, step))
    }

    /// The `FOR` variable: a numeric scalar variable (measured, `v17_c_err_for_*`, `v17_c_for_array`).
    fn for_var(&mut self, t: Tok) -> R<VarId> {
        let shown = show_bytes(self.text(t.span));
        let (name, suffix) = self.split_name(t)?;
        if name.contains('.') {
            // A `TYPE` member, which the old compiler takes (`v17_c_for_type_member`).
            return Err(self.unsupported(t.span, format!("`TYPE` member `{shown}` as a `FOR` variable")));
        }
        if self.procs_by_name.contains_key(&name) {
            return Err(self.unsupported(
                t.span,
                format!("the name of a SUB or FUNCTION as a `FOR` variable: `{shown}`"),
            ));
        }
        if self.visible_const(&name).is_some() {
            return Err(self.error(t.span, format!("the constant `{shown}` cannot be a `FOR` variable")));
        }
        let var = self.variable(t, name, suffix)?;
        if self.prog.var(var).ty == Ty::Str {
            return Err(self.error(
                t.span,
                format!("the `FOR` variable `{shown}` must be a number, not a string"),
            ));
        }
        Ok(var)
    }

    /// A variable named by `NEXT` for the loop of `var` (named at `for_name`): the same variable, by name and type
    /// (measured: `NEXT i!` closes `FOR i`, `NEXT i` does not close `FOR i%`). It is looked up, never created.
    fn next_var(&mut self, next: ast::NameRef, var: VarId, for_name: Tok) -> R<()> {
        let t = self.need(next.name(), next.node().span())?;
        let (name, suffix) = self.split_name(t)?;
        if self.lookup_var(name, suffix) == Some(var) {
            self.names.push((SymbolKind::Var(var), t.span));
            return Ok(());
        }
        let msg = format!(
            "`NEXT {}` does not close `FOR {}`",
            show_bytes(self.text(t.span)),
            show_bytes(self.text(for_name.span))
        );
        Err(self.error(t.span, msg))
    }

    fn do_block(&mut self, b: ast::DoBlock, skips: &Skips) {
        let header = b.header();
        let top = self.header(header.map(|h| h.node()), skips, |c| {
            let h = header.expect("checked");
            c.loop_test(h.cond_word(), h.condition(), h.node().span(), TestAt::Top)
        });
        let body = self.block_body(b.body(), skips);
        let end = b.end();
        let bottom = self.header(end.map(|e| e.node()), skips, |c| {
            let e = end.expect("checked");
            c.loop_test(e.cond_word(), e.condition(), e.node().span(), TestAt::Bottom)
        });
        let end_line = end.map_or(0, |e| self.line(first_token_span(e.node())));
        if let (Some(top), Some(bottom)) = (top, bottom) {
            // The parser reports a condition at both ends.
            let test = top.or(bottom);
            self.push(b.node(), StmtKind::Do { test, body, end_line });
        }
    }

    /// The condition after `DO` or `LOOP`, if there is one.
    fn loop_test(
        &mut self,
        word: Option<Tok>,
        cond: Option<ast::Expr>,
        span: qb64rust_base::Span,
        at: TestAt,
    ) -> R<Option<LoopTest>> {
        let Some(word) = word else {
            return Ok(None);
        };
        let until = self.word(word) == "UNTIL";
        let what = if at == TestAt::Top { "DO" } else { "LOOP" };
        let cond = self.condition(cond, span, what)?;
        Ok(Some(LoopTest { at, until, cond }))
    }

    fn while_block(&mut self, b: ast::WhileBlock, skips: &Skips) {
        let header = b.header();
        let cond = self.header(header.map(|h| h.node()), skips, |c| {
            let h = header.expect("checked");
            c.condition(h.condition(), h.node().span(), "WHILE")
        });
        let body = self.block_body(b.body(), skips);
        let end_line = b.end().map_or(0, |e| self.line(first_token_span(e.node())));
        if let Some(cond) = cond {
            self.push(b.node(), StmtKind::While { cond, body, end_line });
        }
    }

    /// `EXIT FOR`, `EXIT DO`, `EXIT WHILE` (`word`); the parser checked that such a block is open.
    pub(super) fn exit_loop(&mut self, node: Node, word: &str) -> R<()> {
        let kind = match word {
            "FOR" => LoopKind::For,
            "DO" => LoopKind::Do,
            "WHILE" => LoopKind::While,
            _ => {
                let words = node.child_tokens().map(|t| self.word(t)).collect::<Vec<_>>().join(" ");
                return Err(self.unsupported(first_token_span(node), format!("`{words}`")));
            }
        };
        self.push(node, StmtKind::ExitLoop(kind));
        Ok(())
    }

    /// A block not compiled yet (design D10 of `m2-parser-breadth`): one "not supported yet" error at its first
    /// token, unless its header has a parse error; then the statements inside are checked one by one, as
    /// everywhere else.
    fn marked_block(&mut self, node: Node, block: BlockParts, skips: &Skips) {
        if block.header.is_none_or(|h| skips.usable(h)) && !skips.past_cap(node) {
            self.stmt_error = false;
            let span = first_token_span(node);
            let _ = match block.verdict {
                Verdict::Unsupported(what) => self.unsupported(span, what),
                Verdict::Error(msg) => self.error(span, msg),
            };
            self.flush_names();
        }
        for s in block.inner {
            if skips.past_cap(s) {
                break;
            }
            self.check_statement(s, skips);
        }
    }
}

/// The type a `FOR` loop counts in, for a variable of type `ty` (`study\02` §6.5, measured in task 1.1).
fn for_temp(ty: Ty) -> Ty {
    match ty {
        Ty::I16 => Ty::I32,
        Ty::I32 | Ty::I64 => Ty::I64,
        Ty::F32 => Ty::F64,
        Ty::F64 | Ty::F80 => Ty::F80,
        Ty::Str => Ty::Str,
    }
}

/// The statements directly inside a block statement, in every branch, in order; `None` for any other statement.
/// The label pre-pass reads them.
pub(super) fn nested_statements(node: Node) -> Option<Vec<Node>> {
    if let Some(b) = ast::IfBlock::cast(node) {
        let mut inner: Vec<Node> = b.if_branch().into_iter().flat_map(|f| f.body()).collect();
        for e in b.else_if_branches() {
            inner.extend(e.body());
        }
        inner.extend(b.else_branch().into_iter().flat_map(|e| e.body()));
        Some(inner)
    } else if let Some(s) = ast::IfStmt::cast(node) {
        let mut inner: Vec<Node> = s.then_branch().into_iter().flat_map(|b| b.statements()).collect();
        inner.extend(s.else_branch().into_iter().flat_map(|b| b.statements()));
        Some(inner)
    } else if let Some(b) = ast::ForBlock::cast(node) {
        Some(b.body().collect())
    } else if let Some(b) = ast::DoBlock::cast(node) {
        Some(b.body().collect())
    } else if let Some(b) = ast::WhileBlock::cast(node) {
        Some(b.body().collect())
    } else {
        block_parts(node).map(|p| p.inner)
    }
}

/// What `sema` says about a block it does not compile.
pub(super) enum Verdict {
    /// Not supported yet; names the construct.
    Unsupported(&'static str),
    /// A real error: the old compiler rejects the construct too.
    Error(&'static str),
}

/// A block statement `sema` does not compile yet: its header (whose parse errors suppress the verdict), the
/// verdict, and the statements inside it, in order.
pub(super) struct BlockParts<'a> {
    header: Option<Node<'a>>,
    verdict: Verdict,
    pub(super) inner: Vec<Node<'a>>,
}

fn unsupported<'a>(header: Option<Node<'a>>, what: &'static str, inner: Vec<Node<'a>>) -> Option<BlockParts<'a>> {
    Some(BlockParts {
        header,
        verdict: Verdict::Unsupported(what),
        inner,
    })
}

/// The parts of a block statement still marked as a whole; `None` for any other statement.
fn block_parts(node: Node) -> Option<BlockParts> {
    const DEF_FN: &str = "`DEF FN` is not available in QB64; use a FUNCTION";
    if let Some(b) = ast::SelectBlock::cast(node) {
        let mut inner: Vec<Node> = b.before_cases().collect();
        for c in b.cases() {
            inner.extend(c.body());
        }
        unsupported(b.header().map(|h| h.node()), "`SELECT CASE`", inner)
    } else if let Some(b) = ast::TypeBlock::cast(node) {
        unsupported(b.header().map(|h| h.node()), "`TYPE` blocks", Vec::new())
    } else if let Some(b) = ast::DeclareLibraryBlock::cast(node) {
        unsupported(b.header().map(|h| h.node()), "`DECLARE LIBRARY`", Vec::new())
    } else if let Some(b) = ast::DefFnBlock::cast(node) {
        Some(BlockParts {
            header: b.header().map(|h| h.node()),
            verdict: Verdict::Error(DEF_FN),
            inner: b.body().collect(),
        })
    } else if ast::DefFnStmt::cast(node).is_some() {
        Some(BlockParts {
            header: Some(node),
            verdict: Verdict::Error(DEF_FN),
            inner: Vec::new(),
        })
    } else {
        None
    }
}
