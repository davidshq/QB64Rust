//! Blocks (design D3 of `m2-control-flow-slice`). `IF` (both forms), `FOR`, `DO` and `WHILE` become typed block
//! statements that keep the source structure; `EXIT FOR/DO/WHILE` leaves one. The other blocks are still marked as
//! a whole, with the statements inside checked one by one.

use super::{Checker, R, Skips, first_token_span};
use crate::{
    BIT_VALUE_UNREACHABLE, BinOp, Branch, Case, CaseItem, Expr, ExprKind, LATER_TYPE_UNREACHABLE, LoopKind, LoopTest,
    PLACE_ONLY_UNREACHABLE, Place, Stmt, StmtKind, SymbolKind, TestAt, Ty, VarId,
};
use qb64rust_base::show_bytes;
use qb64rust_syntax::SyntaxKind;
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
        } else if let Some(b) = ast::SelectBlock::cast(node) {
            self.select_block(b, skips);
        } else if let Some(b) = ast::TypeBlock::cast(node) {
            self.type_block(b, skips);
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
            let line = header.map_or(0, |h| self.line(first_token_span(h.node())));
            let cond = self.header(header.map(|h| h.node()), skips, |c| {
                let h = header.expect("checked by `header`");
                let what = h.keyword().map_or("IF".to_string(), |k| c.word(k));
                c.condition(h.condition(), h.node().span(), &what)
            });
            let body = self.block_body(body.into_iter(), skips);
            match (cond, &mut branches) {
                (Some(cond), Some(list)) => list.push(Branch { cond, body, line }),
                _ => branches = None,
            }
        }
        let else_ = b.else_branch().map(|e| self.block_body(e.body(), skips));
        let end_line = b.end().map_or(0, |e| self.line(first_token_span(e.node())));
        if let Some(branches) = branches.filter(|l| !l.is_empty()) {
            let kind = StmtKind::If {
                branches,
                else_,
                end_line,
            };
            self.push(node, kind);
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
            let line = self.line(first_token_span(s.node()));
            let branches = vec![Branch { cond, body, line }];
            let kind = StmtKind::If {
                branches,
                else_,
                end_line: line,
            };
            self.push(s.node(), kind);
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
        if let Ty::User(_) = self.prog.var(var).ty {
            return Err(self.unsupported(t.span, format!("the `TYPE` variable `{shown}` as a `FOR` variable")));
        }
        if self.prog.var(var).ty.is_string() {
            return Err(self.error(
                t.span,
                format!("the `FOR` variable `{shown}` must be a number, not a string"),
            ));
        }
        if let Ty::Bit { .. } = self.prog.var(var).ty {
            // Measured: "Unsupported variable used in FOR statement" (`verification\v21_x29`, `x30`).
            return Err(self.error(
                t.span,
                format!("the `_BIT` variable `{shown}` cannot be a `FOR` variable"),
            ));
        }
        // The new numeric types as `FOR` variables: task 8.3 of `m2-numeric-types`.
        self.later(self.prog.var(var).ty, t.span, "a `FOR` variable of type")?;
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

    /// `SELECT CASE` / `SELECT EVERYCASE` (design D7 of `m2-core-builtins`). The header and each `CASE` are checked
    /// as statements of their own; the items of a `CASE` can be typed only when the selector could.
    fn select_block(&mut self, b: ast::SelectBlock, skips: &Skips) {
        let header = b.header();
        let head = self.header(header.map(|h| h.node()), skips, |c| {
            c.select_header(header.expect("checked by `header`"))
        });
        // Statements before the first `CASE` are parse errors; checked for recovery only.
        let _ = self.block_body(b.before_cases(), skips);
        let mut cases = Some(Vec::new());
        let mut else_ = None;
        for clause in b.cases() {
            let h = clause.header();
            if else_.is_some() {
                // Measured: "Expected END SELECT" (`verification\v20_x31_case_after_else`); also a second `CASE ELSE`.
                self.header(h.map(|h| h.node()), skips, |c| -> R<()> {
                    let at = first_token_span(h.expect("checked by `header`").node());
                    Err(c.error(at, "a `CASE` after `CASE ELSE` (`CASE ELSE` comes last)"))
                });
                let _ = self.block_body(clause.body(), skips);
                cases = None;
                continue;
            }
            let line = h.map_or(0, |h| self.line(first_token_span(h.node())));
            let is_else = h.and_then(|h| h.else_word()).is_some();
            let items = match (&head, is_else) {
                (_, true) => None,
                (Some((selector, _, _)), false) => self.header(h.map(|h| h.node()), skips, |c| {
                    c.case_items(h.expect("checked by `header`"), selector)
                }),
                (None, false) => None,
            };
            let body = self.block_body(clause.body(), skips);
            match (is_else, items, &mut cases) {
                (true, _, _) => else_ = Some(body),
                (false, Some(items), Some(list)) => list.push(Case { items, body, line }),
                (false, _, _) => cases = None,
            }
        }
        let end_line = b.end().map_or(0, |e| self.line(first_token_span(e.node())));
        if let (Some((selector, copied, every)), Some(cases)) = (head, cases) {
            let kind = StmtKind::Select {
                selector,
                copied,
                every,
                cases,
                else_,
                end_line,
            };
            self.push(b.node(), kind);
        }
    }

    /// `SELECT CASE e` / `SELECT EVERYCASE e`: the selector, whether it is copied, and `EVERYCASE`. A plain scalar
    /// variable is read at each test; anything else is copied once into a variable of the old compiler's type for
    /// it (`qb64pe.bas` 6831–6884: a string, `int64`, `int32` for narrower integers, the float type it believes).
    fn select_header(&mut self, h: ast::SelectHeader) -> R<(Expr, bool, bool)> {
        let span = h.node().span();
        let every = h.kind_word().is_some_and(|w| self.word(w) == "EVERYCASE");
        let node = self.need(h.selector(), span)?;
        let e = self.expr(node)?;
        // Selectors of the new numeric types: task 8.3 of `m2-numeric-types`.
        let place_ty = if let ExprKind::Load(p) = &e.kind {
            self.prog.place_ty(p)
        } else {
            e.qb
        };
        for t in [place_ty, e.qb, e.ty] {
            self.later(t, e.span, "a `SELECT CASE` selector of type")?;
        }
        if matches!(e.kind, ExprKind::Load(Place::Var(_))) {
            return Ok((e, false, every));
        }
        let copy = match e.qb {
            Ty::Str => return Ok((e, true, every)),
            Ty::I16 | Ty::I32 => Ty::I32,
            t @ (Ty::I64 | Ty::F32 | Ty::F64 | Ty::F80) => t,
            Ty::User(_) => unreachable!("a whole `TYPE` value is no value"),
            crate::place_only_types!() => unreachable!("{PLACE_ONLY_UNREACHABLE}"),
            crate::later_types!() => unreachable!("{LATER_TYPE_UNREACHABLE}"),
            crate::Ty::Bit { .. } => unreachable!("{BIT_VALUE_UNREACHABLE}"),
        };
        Ok((self.convert_exact(e, copy), true, every))
    }

    /// The items of a `CASE`, each converted for its comparison with `selector` ([`CaseItem`]).
    fn case_items(&mut self, h: ast::CaseHeader, selector: &Expr) -> R<Vec<CaseItem>> {
        let mut items = Vec::new();
        for item in h.items() {
            let span = item.node().span();
            let op = match item.is_op().map(|t| t.kind) {
                None | Some(SyntaxKind::Eq) => BinOp::Eq,
                Some(SyntaxKind::Ne) => BinOp::Ne,
                Some(SyntaxKind::Lt) => BinOp::Lt,
                Some(SyntaxKind::Gt) => BinOp::Gt,
                Some(SyntaxKind::Le) => BinOp::Le,
                Some(SyntaxKind::Ge) => BinOp::Ge,
                Some(other) => unreachable!("`IS` with {other:?}"),
            };
            let low = self.need(item.value(), span)?;
            let low = self.case_value(low, selector)?;
            items.push(match item.upper() {
                Some(high) => CaseItem::Range(low, self.case_value(high, selector)?),
                None => CaseItem::Is(op, low),
            });
        }
        Ok(items)
    }

    /// A `CASE` value converted for the comparison (`qb64pe.bas` 7091–7107, 7167–7188): a string for a string
    /// selector; for an integer selector a float rounded half to even (to `_INTEGER64` for an `_INTEGER64` selector,
    /// else to LONG: measured, `CASE 2.4` matches 2), then both sides in C's common type; for a float selector the
    /// value converted to the selector's type.
    fn case_value(&mut self, node: ast::Expr, selector: &Expr) -> R<Expr> {
        let e = self.expr(node)?;
        match (selector.ty == Ty::Str, e.ty == Ty::Str) {
            (true, true) => return Ok(e),
            // Measured: "Expected string expression" / "Expected numeric expression" (`v20_x16`, `x17`, `x25`, `x26`).
            (true, false) => return Err(self.error(e.span, "a `CASE` of a string `SELECT CASE` needs a string")),
            (false, true) => return Err(self.error(e.span, "a `CASE` of a numeric `SELECT CASE` needs a number")),
            (false, false) => {}
        }
        let s = selector.ty;
        if s.is_float() {
            return Ok(self.convert_exact(e, s));
        }
        let e = if e.ty.is_float() {
            self.store(e, if s == Ty::I64 { Ty::I64 } else { Ty::I32 })?
        } else {
            e
        };
        // C++ compares the item as written (measured, `v21_d_select`).
        let common = super::ops::held(s, e.ty);
        Ok(self.convert_exact(e, common))
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
            if matches!(block.verdict, Verdict::Unsupported(_)) && ast::DeclareLibraryBlock::cast(node).is_some() {
                // A declaration: the follow-on rule starts (design D10).
                self.follow_on = true;
            }
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
        Ty::User(_) => unreachable!("a `TYPE` variable as a `FOR` variable is rejected first"),
        crate::place_only_types!() => unreachable!("{PLACE_ONLY_UNREACHABLE}"),
        crate::later_types!() => unreachable!("{LATER_TYPE_UNREACHABLE}"),
        crate::Ty::Bit { .. } => unreachable!("{BIT_VALUE_UNREACHABLE}"),
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
    } else if let Some(b) = ast::SelectBlock::cast(node) {
        let mut inner: Vec<Node> = b.before_cases().collect();
        for c in b.cases() {
            inner.extend(c.body());
        }
        Some(inner)
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
    if let Some(b) = ast::DeclareLibraryBlock::cast(node) {
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
