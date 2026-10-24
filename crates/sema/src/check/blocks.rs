//! Blocks (design D3 of `m2-control-flow-slice`). `IF` (both forms), `FOR`, `DO` and `WHILE` become typed block
//! statements that keep the source structure; `EXIT FOR/DO/WHILE` leaves one. The other blocks are still marked as
//! a whole, with the statements inside checked one by one.

use super::walk::{Cursor, Event};
use super::{Checker, NoJoin, R, Skips, first_token_span};
use crate::{
    BinOp, Branch, Case, CaseItem, Expr, ExprKind, LoopKind, LoopTest, PLACE_ONLY_UNREACHABLE, Place, Stmt, StmtKind,
    SymbolKind, TestAt, Ty,
};
use qb64rust_base::show_bytes;
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::ast;
use qb64rust_syntax::tree::{Node, Tok};

impl<'a> Checker<'a> {
    /// A block statement the walk gives as a statement (a single-line `IF`, `TYPE`, `DECLARE LIBRARY`, a
    /// single-line `DEF FN`); false for any other statement.
    pub(super) fn block_statement(&mut self, node: Node<'a>, skips: &Skips) -> bool {
        self.stamped(|c| {
            if let Some(s) = ast::IfStmt::cast(node) {
                c.if_stmt(s, skips);
            } else if let Some(b) = ast::TypeBlock::cast(node) {
                c.type_block(b);
            } else if ast::DeclareLibraryBlock::cast(node).is_some() {
                // Its declarations were entered in pass 1 (`library.rs`), where they stand in the file.
            } else if ast::DefFnStmt::cast(node).is_some() {
                c.marked_block(node, None, skips);
            } else {
                return false;
            }
            true
        })
    }

    /// A block the walk opened at `node`, other than a procedure: its events are taken up to its end.
    pub(super) fn open_block(&mut self, node: Node<'a>, cur: &mut Cursor<'a, '_>, skips: &Skips) {
        self.stamped(|c| {
            if let Some(b) = ast::IfBlock::cast(node) {
                c.if_block(b, cur, skips);
            } else if let Some(b) = ast::ForBlock::cast(node) {
                c.for_block(b, cur, skips);
            } else if let Some(b) = ast::DoBlock::cast(node) {
                c.do_block(b, cur, skips);
            } else if let Some(b) = ast::WhileBlock::cast(node) {
                c.while_block(b, cur, skips);
            } else if let Some(b) = ast::SelectBlock::cast(node) {
                c.select_block(b, cur, skips);
            } else {
                c.marked_block(node, Some(cur), skips);
            }
        });
    }

    /// Runs `check` on a block. A block's own statements check as `$CHECKING` stood at its header, whatever a line
    /// inside it sets (design D3 of `m4-handlers-logging-checking`); [`Self::push`] gave them the state at their
    /// end.
    fn stamped<T>(&mut self, check: impl FnOnce(&mut Self) -> T) -> T {
        let checked = self.at.checking;
        let before = self.sink().len();
        let result = check(self);
        for s in &mut self.sink()[before..] {
            s.checked = checked;
        }
        result
    }

    /// Checks the events up to the end or the next branch of the innermost open block as the statements of a
    /// block, and returns them typed, with the event that ended them (`None` at the end of the walk). Statements
    /// with errors are left out.
    fn block_body(&mut self, cur: &mut Cursor<'a, '_>, skips: &Skips) -> (Vec<Stmt>, Option<Event<'a>>) {
        self.sinks.push(Vec::new());
        let end = self.walk_to_end(cur, skips);
        (self.sinks.pop().expect("pushed above"), end)
    }

    /// [`Self::block_body`] over a list of statements of their own (a branch of a single-line `IF`).
    fn list_body(&mut self, nodes: impl Iterator<Item = Node<'a>>, skips: &Skips) -> Vec<Stmt> {
        let program = self.program;
        let mut cur = Cursor::new(program, skips, nodes);
        self.block_body(&mut cur, skips).0
    }

    /// Checks the events up to the end or the next branch of the innermost open block into the current body, and
    /// returns the event that ended them.
    pub(super) fn walk_to_end(&mut self, cur: &mut Cursor<'a, '_>, skips: &Skips) -> Option<Event<'a>> {
        loop {
            let event = cur.next()?;
            if self.pp_dropped(event, cur) {
                continue;
            }
            match event {
                Event::Stmt(n) => self.check_statement(n, skips),
                Event::Open(n) => self.check_open(n, cur, skips),
                e @ (Event::Branch(_) | Event::Close(..)) => return Some(e),
            }
        }
    }

    /// Checks a block's header (`check`, which reports its own errors) as one statement: names are recorded only
    /// when it has no error. `None` after an error or when the header has a parse error.
    fn header<T>(&mut self, header: Option<Node>, skips: &Skips, check: impl FnOnce(&mut Self) -> R<T>) -> Option<T> {
        let header = header?;
        if !skips.usable(header) {
            return None;
        }
        self.stmt.stmt_error = false;
        let result = self.unjoined(NoJoin::Header, check).ok();
        self.flush_names();
        result
    }

    /// [`Self::header`] for a header tested each time its part of the block runs (`ELSEIF`, a test at `LOOP`, a
    /// `CASE`): the allocations its check makes are its own, run first in its test, not statements before the block
    /// (design D2 of `m4-names-implicit-arrays`; measured, `verification\v24_x262`, `v55_a_implicit_headers`).
    fn part_header<T>(
        &mut self,
        header: Option<Node>,
        skips: &Skips,
        check: impl FnOnce(&mut Self) -> R<T>,
    ) -> (Option<T>, Vec<Stmt>) {
        let before = self.sink().len();
        let result = self.header(header, skips, check);
        let allocs = self.sink().split_off(before);
        debug_assert!(allocs.iter().all(|s| matches!(s.kind, StmtKind::Allocate { .. })));
        (result, allocs)
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

    fn if_block(&mut self, b: ast::IfBlock<'a>, cur: &mut Cursor<'a, '_>, skips: &Skips) {
        let node = b.node();
        let mut branches = Some(Vec::new());
        // The header of the branch to check next; `None` for a branch after `ELSE`.
        let mut header = b.if_branch().map(|f| f.header());
        let mut else_ = None;
        let mut k = 0;
        let end = loop {
            let Some(head) = header else {
                // A branch after `ELSE` (a parse error): its header is not checked, its statements are, for their
                // own errors, as any statement after a parse error is; they are then thrown away, since the program
                // is rejected anyway.
                let (_, ev) = self.block_body(cur, skips);
                match ev {
                    Some(Event::Branch(_)) => continue,
                    ev => break ev,
                }
            };
            let line = head.map_or(0, |h| self.line(first_token_span(h.node())));
            let check = |c: &mut Self| {
                let h = head.expect("checked by `header`");
                let what = h.keyword().map_or("IF".to_string(), |k| c.word(k));
                c.condition(h.condition(), h.node().span(), &what)
            };
            // The `IF`'s allocations stand before the statement, an `ELSEIF`'s run with its test.
            let (cond, allocs) = if k == 0 {
                (self.header(head.map(|h| h.node()), skips, check), Vec::new())
            } else {
                self.part_header(head.map(|h| h.node()), skips, check)
            };
            let (body, ev) = self.block_body(cur, skips);
            match (cond, &mut branches) {
                (Some(cond), Some(list)) => list.push(Branch {
                    cond,
                    body,
                    line,
                    allocs,
                }),
                _ => branches = None,
            }
            k += 1;
            match ev {
                Some(Event::Branch(n)) => match ast::ElseIfBranch::cast(n) {
                    Some(e) => header = Some(e.header()),
                    None => {
                        let (body, ev) = self.block_body(cur, skips);
                        else_ = Some(body);
                        match ev {
                            Some(Event::Branch(_)) => header = None,
                            ev => break ev,
                        }
                    }
                },
                ev => break ev,
            }
        };
        let end_line = match end {
            Some(Event::Close(Some(e), _)) => self.line(first_token_span(e)),
            _ => 0,
        };
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
    fn if_stmt(&mut self, s: ast::IfStmt<'a>, skips: &Skips) {
        let header = s.header();
        let cond = self.header(header.map(|h| h.node()), skips, |c| {
            let h = header.expect("checked by `header`");
            c.condition(h.condition(), h.node().span(), "IF")
        });
        let body = self.list_body(s.then_branch().into_iter().flat_map(|b| b.statements()), skips);
        let else_ = s.else_branch().map(|b| self.list_body(b.statements(), skips));
        if let Some(cond) = cond {
            let line = self.line(first_token_span(s.node()));
            let branches = vec![Branch {
                cond,
                body,
                line,
                allocs: Vec::new(),
            }];
            let kind = StmtKind::If {
                branches,
                else_,
                end_line: line,
            };
            self.push(s.node(), kind);
        }
    }

    fn for_block(&mut self, b: ast::ForBlock<'a>, cur: &mut Cursor<'a, '_>, skips: &Skips) {
        let header = b.header();
        let parts = self.header(header.map(|h| h.node()), skips, |c| {
            c.for_header(header.expect("checked"))
        });
        let (body, end) = self.block_body(cur, skips);
        // The `NEXT` that ends this loop, and this loop's place among the variables it names.
        let (next, depth) = match end {
            Some(Event::Close(Some(n), depth)) => (ast::NextStmt::cast(n), depth),
            _ => (None, 0),
        };
        let end_at = next.map(|n| first_token_span(n.node()));
        let end_line = end_at.map_or(0, |s| self.line(s));
        let Some(((var, name), temp, start, end, step)) = parts else {
            return;
        };
        // The variable `NEXT` names for this loop, checked as a statement of its own where `NEXT` stands. One
        // `NEXT j, i` closes two loops, inner first; once it had an error, the outer loop reports no second one.
        if let Some(next) = next
            && let Some(next_name) = next.vars().nth(depth)
        {
            let key = next.node().key();
            if self.bad_next == Some(key) {
                return;
            }
            if self
                .header(Some(next.node()), skips, |c| c.next_var(next_name, &var, name))
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
            end_at,
        };
        self.push(b.node(), kind);
    }

    /// `FOR var = start TO end [STEP step]`: the variable (and its name token), the type the loop counts in, and
    /// the three values converted to it.
    #[expect(clippy::type_complexity, reason = "one use; the parts of `StmtKind::For`")]
    fn for_header(&mut self, h: ast::ForHeader) -> R<((Place, Tok), Ty, Expr, Expr, Option<Expr>)> {
        let span = h.node().span();
        let name = self.need(h.var().and_then(|v| v.name()), span)?;
        let var = self.for_var(name, h.var_args())?;
        let temp = for_temp(self.prog.place_ty(&var));
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

    /// The `FOR` variable: a numeric scalar variable (measured, `v17_c_err_for_*`, `v17_c_for_array`), or a
    /// numeric member of a `TYPE` variable reached through members (design D8 of `m4-memory-records`; measured,
    /// `v17_c_for_type_member`, `v56_f1`, `f4`). Its indexes, when written (`FOR r.m(i) = …`), are those of an
    /// element of a member array, which the old compiler finds again at every pass (`v56_f3`): not supported yet;
    /// of anything else they are an error (`v17_c_for_array`).
    fn for_var(&mut self, t: Tok, args: Option<ast::ArgList>) -> R<Place> {
        let shown = show_bytes(self.text(t.span));
        let (name, suffix) = self.split_name(t)?;
        if let Some(args) = args {
            if name.contains('.')
                && let Some(Place::Member { base, member }) = self.dotted_any(t, &name, suffix)?
                && self.prog.member(self.prog.place_ty(&base), member).is_array()
            {
                let msg = format!("an element of the member array `{shown}` as a `FOR` variable");
                return Err(self.unsupported(args.node().span(), msg));
            }
            return Err(self.error(args.node().span(), "a `FOR` variable cannot be an array element"));
        }
        if name.contains('.')
            && let Some(place) = self.dotted(t, &name, suffix)?
        {
            return match self.prog.place_ty(&place) {
                crate::numeric_value_types!() => Ok(place),
                Ty::Str | Ty::FixedStr(_) => {
                    // Measured: "Unsupported variable used in FOR statement" (`verification\v56_x13`).
                    Err(self.error(
                        t.span,
                        format!("the `FOR` variable `{shown}` must be a number, not a string"),
                    ))
                }
                Ty::Bit { .. } | Ty::User(_) => {
                    let msg = format!("the member `{shown}` of a `TYPE` as a `FOR` variable");
                    Err(self.unsupported(t.span, msg))
                }
            };
        }
        if self.procs_by_name.contains_key(&name) {
            return Err(self.unsupported(
                t.span,
                format!("the name of a SUB or FUNCTION as a `FOR` variable: `{shown}`"),
            ));
        }
        // A constant of the auto-included file is one whether the program named it before or not.
        if self.visible_const(&name).is_some() || qb64rust_builtins::auto_constant(name.as_bytes()).is_some() {
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
        Ok(Place::Var(var))
    }

    /// A variable named by `NEXT` for the loop of `var` (named at `for_name`): the same variable, by name and type
    /// (measured: `NEXT i!` closes `FOR i`, `NEXT i` does not close `FOR i%`). It is looked up, never created. A
    /// member is named as the `FOR` names it (`NEXT r.v`, `verification\v56_f1`).
    fn next_var(&mut self, next: ast::NameRef, var: &Place, for_name: Tok) -> R<()> {
        let t = self.need(next.name(), next.node().span())?;
        let (name, suffix) = self.split_name(t)?;
        if let Place::Var(var) = *var {
            if self.lookup_var(name, suffix) == Some(var) {
                self.stmt.names.push((SymbolKind::Var(var), t.span));
                return Ok(());
            }
        } else if (name, suffix) == self.split_name(for_name)? {
            return Ok(());
        }
        let msg = format!(
            "`NEXT {}` does not close `FOR {}`",
            show_bytes(self.text(t.span)),
            show_bytes(self.text(for_name.span))
        );
        Err(self.error(t.span, msg))
    }

    fn do_block(&mut self, b: ast::DoBlock<'a>, cur: &mut Cursor<'a, '_>, skips: &Skips) {
        let header = b.header();
        let top = self.header(header.map(|h| h.node()), skips, |c| {
            let h = header.expect("checked");
            c.loop_test(h.cond_word(), h.condition(), h.node().span(), TestAt::Top)
        });
        let (body, end) = self.block_body(cur, skips);
        let end = match end {
            Some(Event::Close(Some(n), _)) => ast::LoopStmt::cast(n),
            _ => None,
        };
        let (bottom, allocs) = self.part_header(end.map(|e| e.node()), skips, |c| {
            let e = end.expect("checked");
            c.loop_test(e.cond_word(), e.condition(), e.node().span(), TestAt::Bottom)
        });
        let bottom = bottom.map(|t| t.map(|t| LoopTest { allocs, ..t }));
        let end_at = end.map(|e| first_token_span(e.node()));
        let end_line = end_at.map_or(0, |s| self.line(s));
        if let (Some(top), Some(bottom)) = (top, bottom) {
            // The parser reports a condition at both ends.
            let test = top.or(bottom);
            let kind = StmtKind::Do {
                test,
                body,
                end_line,
                end_at,
            };
            self.push(b.node(), kind);
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
        Ok(Some(LoopTest {
            at,
            until,
            cond,
            allocs: Vec::new(),
        }))
    }

    fn while_block(&mut self, b: ast::WhileBlock<'a>, cur: &mut Cursor<'a, '_>, skips: &Skips) {
        let header = b.header();
        let cond = self.header(header.map(|h| h.node()), skips, |c| {
            let h = header.expect("checked");
            c.condition(h.condition(), h.node().span(), "WHILE")
        });
        let (body, end) = self.block_body(cur, skips);
        let end_at = match end {
            Some(Event::Close(Some(n), _)) => Some(first_token_span(n)),
            _ => None,
        };
        let end_line = end_at.map_or(0, |s| self.line(s));
        if let Some(cond) = cond {
            let kind = StmtKind::While {
                cond,
                body,
                end_line,
                end_at,
            };
            self.push(b.node(), kind);
        }
    }

    /// `SELECT CASE` / `SELECT EVERYCASE` (design D7 of `m2-core-builtins`). The header and each `CASE` are checked
    /// as statements of their own; the items of a `CASE` can be typed only when the selector could.
    fn select_block(&mut self, b: ast::SelectBlock<'a>, cur: &mut Cursor<'a, '_>, skips: &Skips) {
        let header = b.header();
        let head = self.header(header.map(|h| h.node()), skips, |c| {
            c.select_header(header.expect("checked by `header`"))
        });
        // Statements before the first `CASE` are parse errors; checked for recovery only.
        let (_, mut end) = self.block_body(cur, skips);
        let mut cases = Some(Vec::new());
        let mut else_ = None;
        while let Some(Event::Branch(n)) = end {
            let h = ast::CaseClause::cast(n).and_then(|c| c.header());
            if else_.is_some() {
                // Measured: "Expected END SELECT" (`verification\v20_x31_case_after_else`); also a second `CASE ELSE`.
                self.header(h.map(|h| h.node()), skips, |c| -> R<()> {
                    let at = first_token_span(h.expect("checked by `header`").node());
                    Err(c.error(at, "a `CASE` after `CASE ELSE` (`CASE ELSE` comes last)"))
                });
                end = self.block_body(cur, skips).1;
                cases = None;
                continue;
            }
            let line = h.map_or(0, |h| self.line(first_token_span(h.node())));
            let is_else = h.and_then(|h| h.else_word()).is_some();
            let (items, allocs) = match (&head, is_else) {
                (_, true) => (None, Vec::new()),
                (Some((selector, _, _)), false) => self.part_header(h.map(|h| h.node()), skips, |c| {
                    c.case_items(h.expect("checked by `header`"), selector)
                }),
                (None, false) => (None, Vec::new()),
            };
            let (body, next) = self.block_body(cur, skips);
            end = next;
            match (is_else, items, &mut cases) {
                (true, _, _) => else_ = Some(body),
                (false, Some(items), Some(list)) => list.push(Case {
                    items,
                    body,
                    line,
                    allocs,
                }),
                (false, _, _) => cases = None,
            }
        }
        let end_line = match end {
            Some(Event::Close(Some(e), _)) => self.line(first_token_span(e)),
            _ => 0,
        };
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
        if matches!(e.kind, ExprKind::Load(Place::Var(_))) {
            return Ok((e, false, every));
        }
        // Measured (`v21_d_select`, C++): signed up to 32 bits `int32`, unsigned up to 32 `uint32`, the 64-bit types
        // and `_OFFSET` themselves (`ptrszint` is `int64`), a `_BIT * n` as its storage.
        let copy = match e.qb.storage() {
            Ty::Str => return Ok((e, true, every)),
            Ty::I8 | Ty::I16 | Ty::I32 => Ty::I32,
            Ty::U8 | Ty::U16 | Ty::U32 => Ty::U32,
            Ty::Off => Ty::I64,
            Ty::UOff => Ty::U64,
            t @ (Ty::I64 | Ty::U64 | Ty::F32 | Ty::F64 | Ty::F80) => t,
            Ty::User(_) => unreachable!("a whole `TYPE` value is no value"),
            crate::place_only_types!() => unreachable!("{PLACE_ONLY_UNREACHABLE}"),
            crate::Ty::Bit { .. } => unreachable!("`storage` gives no `_BIT`"),
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
            // Rounded to a 64-bit selector's own type, to LONG for any narrower one (measured, `v21_d_select`).
            self.store(e, if s.int_bits() == Some(64) { s } else { Ty::I32 })?
        } else {
            e
        };
        // C++ compares the item as written (measured, `v21_d_select`).
        let common = super::ops::held(s, e.ty);
        Ok(self.convert_exact(e, common))
    }

    /// `EXIT FOR`, `EXIT DO`, `EXIT WHILE`, `EXIT SELECT`, `EXIT CASE` (`word`); the parser checked that such a
    /// block is open in this body (`EXIT SELECT without SELECT CASE`: measured, `verification\v30_x60`, `x63`,
    /// `x64`), and a statement between `SELECT` and its first `CASE` is its error too (`x61`).
    pub(super) fn exit_loop(&mut self, node: Node, word: &str) -> R<()> {
        let kind = match word {
            "FOR" => LoopKind::For,
            "DO" => LoopKind::Do,
            "WHILE" => LoopKind::While,
            "SELECT" | "CASE" => {
                self.push(node, StmtKind::ExitSelect { case: word == "CASE" });
                return Ok(());
            }
            _ => {
                let words = node.child_tokens().map(|t| self.word(t)).collect::<Vec<_>>().join(" ");
                return Err(self.unsupported(first_token_span(node), format!("`{words}`")));
            }
        };
        self.push(node, StmtKind::ExitLoop(kind));
        Ok(())
    }

    /// A block the old compiler rejects (design D10 of `m2-parser-breadth`): one error at its first token, unless
    /// its header has a parse error; then the statements inside (the events up to its end, `cur`) are checked one
    /// by one, as everywhere else. `DEF FN` in both its forms.
    fn marked_block(&mut self, node: Node<'a>, cur: Option<&mut Cursor<'a, '_>>, skips: &Skips) {
        const DEF_FN: &str = "`DEF FN` is not available in QB64; use a FUNCTION";
        let header = match ast::DefFnBlock::cast(node) {
            Some(b) => b.header().map(|h| h.node()),
            None => Some(node),
        };
        if header.is_none_or(|h| skips.usable(h)) && !skips.past_cap(node) {
            self.stmt.stmt_error = false;
            let _ = self.error(first_token_span(node), DEF_FN);
            self.flush_names();
        }
        if let Some(cur) = cur {
            while let Some(Event::Branch(_)) = self.walk_to_end(cur, skips) {}
        }
    }
}

/// The type a `FOR` loop counts in, for a variable of type `ty` (`study\02` §6.5, measured in task 1.1).
fn for_temp(ty: Ty) -> Ty {
    match ty {
        // Measured (`v21_d_for`, C++): signed or not, 8 bits count in INTEGER, 16 in LONG, 32 and 64 and `_OFFSET` in
        // `_INTEGER64`.
        Ty::I8 | Ty::U8 => Ty::I16,
        Ty::I16 | Ty::U16 => Ty::I32,
        Ty::I32 | Ty::U32 | Ty::I64 | Ty::U64 | Ty::Off | Ty::UOff => Ty::I64,
        Ty::F32 => Ty::F64,
        Ty::F64 | Ty::F80 => Ty::F80,
        Ty::Str => Ty::Str,
        Ty::User(_) => unreachable!("a `TYPE` variable as a `FOR` variable is rejected first"),
        crate::place_only_types!() => unreachable!("{PLACE_ONLY_UNREACHABLE}"),
        crate::Ty::Bit { .. } => unreachable!("a `_BIT` variable as a `FOR` variable is rejected first"),
    }
}

/// The statements of a single-line `IF`, in both branches, in order; `None` for any other statement. The label
/// pre-pass reads them: the walk gives the `IF` as one statement.
pub(super) fn line_if_statements(node: Node) -> Option<Vec<Node>> {
    let s = ast::IfStmt::cast(node)?;
    let mut inner: Vec<Node> = s.then_branch().into_iter().flat_map(|b| b.statements()).collect();
    inner.extend(s.else_branch().into_iter().flat_map(|b| b.statements()));
    Some(inner)
}
