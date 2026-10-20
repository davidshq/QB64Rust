//! Typed tree to IR. A plain statement becomes one IR statement; a label statement becomes a position in its body;
//! a block becomes statements, labels and jumps, as in the table of design D8 (`m2-control-flow-slice`):
//!
//! | Source | Lowered (`L…`: labels the lowering makes) |
//! |---|---|
//! | `IF c THEN a ELSE b` | `Branch(c, Zero → Lelse)`; `a`; `Jump(Lend)`; `Lelse:` `b`; `Lend:` |
//! | `ELSEIF c THEN` | `Branch(c, Zero → Lnext, UseValue)` |
//! | `WHILE c` … `WEND` | `Ltop:` `Branch(c, Zero → Lexit)`; body; `Jump(Ltop)`; `Lexit:` |
//! | `DO WHILE c` / `DO UNTIL c` | as `WHILE`, with `Zero` / `NonZero` |
//! | `LOOP WHILE c` / `LOOP UNTIL c` | `Ltop:` body; `Branch(c, NonZero / Zero → Ltop)`; `Lexit:` |
//! | `FOR v = a TO b STEP s` | see [`Lowerer::for_loop`] |
//! | `EXIT FOR/DO/WHILE` | `Jump(Lexit)` of the innermost loop of that kind |
//! | `EXIT SELECT` | `Jump(Lend)` of the `SELECT` whose `CASE` it stands in ([`Lowerer::select`]) |
//! | `EXIT CASE` | `Jump` to the end of its `CASE` body ([`Lowerer::case_end`]); in a `CASE ELSE` as `EXIT SELECT` |
//! | `GOTO x` / `GOSUB x` / `RETURN` | `Jump(x)` / `Gosub(x)` / `Return` |
//!
//! `IF c GOTO x` is an `IF` whose branch is `GOTO x`. Conditions of `IF`, `WHILE`, `DO` and `LOOP` are
//! [`OnError::Skip`] branches.

use crate::{
    Body, EventHandler, Expr, ExprKind, Label, LabelId, OnError, Op, Place, PrintItem, Proc, ProcId, ProcKind, Program,
    Resume, Stmt, Storage, Ty, Var, VarId, When,
};
use qb64rust_base::{Span, to_u32};
use qb64rust_sema as sema;
use qb64rust_sema::{BinOp, UnOp};

/// Lowers a checked program (one without errors).
pub fn lower(p: &sema::Program) -> Program {
    let ids = LabelIds::new(p);
    let mut vars: Vec<Var> = p
        .vars
        .iter()
        .map(|v| Var {
            name: v.name.clone(),
            ty: v.ty,
            storage: storage(v.storage),
            shape: v.shape.clone(),
            conventional: v.conventional,
        })
        .collect();
    // The main module first, so the temporaries are numbered in source order as far as possible.
    let mut counts = Counts::default();
    let main = Lowerer::new(p, &ids, None, &mut vars, &mut counts).body(&p.stmts);
    let mut procs = Vec::new();
    for (q, i) in p.procs.iter().zip(0..) {
        let body = Lowerer::new(p, &ids, Some(sema::ProcId(i)), &mut vars, &mut counts).body(&q.stmts);
        procs.push(Proc {
            name: q.name.clone(),
            kind: match q.kind {
                sema::ProcKind::Sub => ProcKind::Sub,
                sema::ProcKind::Function(t) => ProcKind::Function(t),
            },
            params: q.params.iter().map(|v| VarId(v.0)).collect(),
            result: q.result.map(|v| VarId(v.0)),
            body,
            line: q.line,
            end_line: q.end_line,
            external: q.external.clone(),
            checked: q.checked,
        });
    }
    let program = Program {
        vars,
        procs,
        main,
        types: p.types.clone(),
        data: p.data.clone(),
        libraries: p.libraries.clone(),
        settings: crate::Settings {
            console: p.settings.console,
            window: p.settings.window,
            window_hidden: p.settings.window_hidden,
            asserts: p.settings.asserts,
            runtime_parts: p
                .settings
                .runtime_parts
                .iter()
                .map(|part| match part {
                    sema::RuntimePart::Icon => crate::RuntimePart::Icon,
                    sema::RuntimePart::Audio => crate::RuntimePart::Audio,
                    sema::RuntimePart::Printer => crate::RuntimePart::Printer,
                    sema::RuntimePart::ImageCodec => crate::RuntimePart::ImageCodec,
                    sema::RuntimePart::Font => crate::RuntimePart::Font,
                    sema::RuntimePart::ScreenImage => crate::RuntimePart::ScreenImage,
                    sema::RuntimePart::DataProcessing => crate::RuntimePart::DataProcessing,
                    sema::RuntimePart::Sockets => crate::RuntimePart::Sockets,
                    sema::RuntimePart::Embed => crate::RuntimePart::Embed,
                })
                .collect(),
            exe_icon: p.settings.exe_icon.is_some(),
            version_info: p.settings.version_info.clone(),
        },
    };
    if cfg!(debug_assertions)
        && let Err(problems) = crate::validate(&program)
    {
        panic!("the lowering made an invalid IR:\n{}", problems.join("\n"));
    }
    program
}

/// How many blocks of each kind with hidden variables the lowering has met in the program.
#[derive(Default)]
struct Counts {
    fors: u32,
    selects: u32,
    ons: u32,
}

/// The lowering of one body.
struct Lowerer<'a> {
    p: &'a sema::Program,
    ids: &'a LabelIds,
    /// The body: `None` for the main module.
    owner: Option<sema::ProcId>,
    /// The program's variables; the lowering adds its temporaries.
    vars: &'a mut Vec<Var>,
    /// Blocks lowered so far in the program, for the names of their hidden variables.
    counts: &'a mut Counts,
    labels: Vec<Label>,
    stmts: Vec<Stmt>,
    /// The exit labels of the loops around the statement being lowered, innermost last.
    exits: Vec<(sema::LoopKind, LabelId)>,
    /// The `CASE` bodies around the statement being lowered, innermost last: the end label of each one's
    /// `SELECT`, and whether it is a `CASE ELSE`. A loop inside a `CASE` does not hide it, and a `SELECT` inside
    /// a loop does not hide the loop from `EXIT FOR`.
    selects: Vec<(LabelId, bool)>,
    /// The label an `EXIT CASE` jumps to, waiting to be placed at the end of the next `CASE` body that closes
    /// ([`Lowerer::case_end`]).
    case_end: Option<LabelId>,
    /// Operations that belong to the next statement and run first in it (`joined` of
    /// [`sema::StmtKind::Allocate`]), each with the span and line it was written at.
    joined: Vec<(Span, u32, Op)>,
    /// The [`sema::Stmt::include_line`] of the statement being lowered.
    include_line: Option<u32>,
}

impl<'a> Lowerer<'a> {
    fn new(
        p: &'a sema::Program,
        ids: &'a LabelIds,
        owner: Option<sema::ProcId>,
        vars: &'a mut Vec<Var>,
        counts: &'a mut Counts,
    ) -> Self {
        // The body's own labels keep their order, so a label's IR id is its index among its own body's labels
        // ([`LabelIds`]); their positions are set where their statements stand.
        let labels = p
            .labels
            .iter()
            .filter(|l| l.proc == owner)
            .map(|l| Label {
                name: Some(l.name.clone()),
                line: l.line,
                include_line: l.include_line,
                file: Some(l.file),
                at: 0,
            })
            .collect();
        Lowerer {
            p,
            ids,
            owner,
            vars,
            counts,
            labels,
            stmts: Vec::new(),
            exits: Vec::new(),
            selects: Vec::new(),
            case_end: None,
            joined: Vec::new(),
            include_line: None,
        }
    }

    /// The label of an `EXIT CASE` in a `CASE`: **the end of the next `CASE` body that closes in the body's
    /// text**, before what ends that `CASE` (the jump out of a plain `SELECT`, the mark of a match of `SELECT
    /// EVERYCASE`). That is the `CASE` the statement stands in, unless a `SELECT` with a `CASE` follows it inside
    /// that `CASE`: then it is the first `CASE` of that inner `SELECT`, and the program goes on after the inner
    /// `END SELECT`. The old compiler names the label by a count of the `CASE`s closed so far (`qb64pe.bas`
    /// 7511, `sc_ec_<n + 1>_end`); kept as it is and listed for review (`SOMEDAY.md`).
    fn case_end(&mut self, line: u32) -> LabelId {
        match self.case_end {
            Some(l) => l,
            None => {
                let l = self.label(line);
                self.case_end = Some(l);
                l
            }
        }
    }

    fn body(mut self, stmts: &[sema::Stmt]) -> Body {
        self.stmts_of(stmts);
        self.flush_joined();
        Body {
            labels: self.labels,
            stmts: self.stmts,
        }
    }

    fn stmts_of(&mut self, stmts: &[sema::Stmt]) {
        for s in stmts {
            // Every statement lowered from `s` stands where `s` stands: a block does not cross a file.
            let outer = std::mem::replace(&mut self.include_line, s.include_line);
            self.stmt(s);
            self.include_line = outer;
        }
    }

    /// A new label of this body, placed later by [`Lowerer::place`].
    fn label(&mut self, line: u32) -> LabelId {
        self.labels.push(Label {
            name: None,
            line,
            include_line: self.include_line,
            file: None,
            at: usize::MAX,
        });
        LabelId(to_u32(self.labels.len() - 1))
    }

    /// Puts a label before the next statement.
    fn place(&mut self, l: LabelId) {
        self.flush_joined();
        self.labels[l.0 as usize].at = self.stmts.len();
    }

    /// The joined operations that no statement followed, as statements of their own (`sema` makes none: the
    /// statement that names an array first comes right after its allocation).
    fn flush_joined(&mut self) {
        for (span, line, op) in std::mem::take(&mut self.joined) {
            self.emit(span, line, vec![op]);
        }
    }

    /// A statement of `ops`, after the operations joined to it ([`Lowerer::joined`]).
    fn emit(&mut self, span: Span, line: u32, ops: Vec<Op>) {
        self.emit_as(span, line, ops, false);
    }

    /// [`Self::emit`] of a statement of the lowering's own, which stands for no source statement ([`Stmt::own`]);
    /// one that operations were joined to is not.
    fn emit_own(&mut self, span: Span, line: u32, ops: Vec<Op>) {
        self.emit_as(span, line, ops, true);
    }

    fn emit_as(&mut self, span: Span, line: u32, ops: Vec<Op>, own: bool) {
        let mut joined: Vec<Op> = std::mem::take(&mut self.joined).into_iter().map(|j| j.2).collect();
        let own = own && joined.is_empty();
        joined.extend(ops);
        let ops = joined;
        let may_raise = ops.iter().any(Op::may_raise);
        self.stmts.push(Stmt {
            span,
            line,
            include_line: self.include_line,
            ops,
            may_raise,
            own,
        });
    }

    fn stmt(&mut self, s: &sema::Stmt) {
        let op = match &s.kind {
            sema::StmtKind::Label(l) => {
                let l = self.ids.get(*l);
                self.place(l);
                return;
            }
            sema::StmtKind::If {
                branches,
                else_,
                end_line,
            } => return self.if_block(s, branches, else_.as_deref(), *end_line),
            sema::StmtKind::While {
                cond,
                body,
                end_line,
                end_at,
            } => {
                let test = sema::LoopTest {
                    at: sema::TestAt::Top,
                    until: false,
                    cond: cond.clone(),
                };
                let end = (*end_line, end_at.unwrap_or(s.span));
                return self.loop_block(s, sema::LoopKind::While, Some(&test), body, end);
            }
            sema::StmtKind::Do {
                test,
                body,
                end_line,
                end_at,
            } => {
                let end = (*end_line, end_at.unwrap_or(s.span));
                return self.loop_block(s, sema::LoopKind::Do, test.as_ref(), body, end);
            }
            sema::StmtKind::For {
                var,
                temp,
                start,
                end,
                step,
                body,
                end_line,
                end_at,
            } => {
                let parts = ForParts {
                    var: VarId(var.0),
                    temp: *temp,
                    start,
                    end,
                    step: step.as_ref(),
                };
                return self.for_loop(s, parts, body, (*end_line, end_at.unwrap_or(s.span)));
            }
            sema::StmtKind::OnJump { value, gosub, targets } => return self.on_jump(s, value, *gosub, targets),
            sema::StmtKind::Select {
                selector,
                copied,
                every,
                cases,
                else_,
                end_line,
            } => {
                let parts = SelectParts {
                    selector,
                    copied: *copied,
                    every: *every,
                    cases,
                    else_: else_.as_deref(),
                    end_line: *end_line,
                };
                return self.select(s, parts);
            }
            sema::StmtKind::ExitLoop(kind) => {
                let (_, exit) = *self
                    .exits
                    .iter()
                    .rev()
                    .find(|(k, _)| k == kind)
                    .expect("the parser checked that a loop of this kind is open");
                Op::Jump(exit)
            }
            sema::StmtKind::ExitSelect { case } => {
                let &(end, in_else) = self
                    .selects
                    .last()
                    .expect("the parser checked that a `CASE` of a `SELECT` is open");
                if *case && !in_else {
                    Op::Jump(self.case_end(s.line))
                } else {
                    Op::Jump(end)
                }
            }
            sema::StmtKind::Goto(l) => Op::Jump(self.ids.get(*l)),
            sema::StmtKind::Gosub(l) => Op::Gosub(self.ids.get(*l)),
            sema::StmtKind::Return(l) => Op::Return(l.map(|l| self.ids.get(l))),
            sema::StmtKind::ConsoleOnly => Op::SelectConsole,
            sema::StmtKind::End(code) => Op::End(code.clone()),
            sema::StmtKind::System(code) => Op::System(code.clone()),
            sema::StmtKind::Exit => Op::Exit,
            sema::StmtKind::OnError(l) => Op::SetHandler(l.map(|l| self.ids.get(l))),
            sema::StmtKind::OnEvent { event, handler } => Op::OnEvent {
                event: event.clone(),
                handler: match handler {
                    sema::Handler::Label(l) => EventHandler::Label(self.ids.get(*l)),
                    sema::Handler::Sub { proc, arg } => EventHandler::Sub {
                        proc: *proc,
                        arg: arg.clone(),
                    },
                },
            },
            sema::StmtKind::Key(form) => Op::Key(form.clone()),
            sema::StmtKind::Resume(r) => Op::Resume(match r {
                sema::Resume::Retry => Resume::Retry,
                sema::Resume::Next => Resume::Next,
                sema::Resume::To(l) => Resume::To(self.ids.get(*l)),
            }),
            sema::StmtKind::Error(code) => Op::Raise(code.clone()),
            sema::StmtKind::Assign { place, value } => Op::Assign {
                place: place.clone(),
                value: value.clone(),
            },
            sema::StmtKind::CopyWhole { from, to } => Op::CopyWhole {
                from: from.clone(),
                to: to.clone(),
            },
            sema::StmtKind::Allocate {
                array,
                bounds,
                mode,
                joined,
            } => {
                let op = Op::Allocate {
                    array: array.clone(),
                    bounds: bounds.clone(),
                    mode: *mode,
                };
                if *joined {
                    self.joined.push((s.span, s.line, op));
                    return;
                }
                op
            }
            sema::StmtKind::Clear { array } => Op::Clear { array: array.clone() },
            sema::StmtKind::Release { array } => Op::Release { array: array.clone() },
            sema::StmtKind::EraseParam { array } => Op::EraseParam { array: *array },
            sema::StmtKind::CopyArray { from, to } => Op::CopyArray {
                from: from.clone(),
                to: to.clone(),
            },
            sema::StmtKind::ArrayCopy {
                from,
                to,
                ranges,
                starts,
            } => Op::ArrayCopy {
                from: from.clone(),
                to: to.clone(),
                ranges: ranges.clone(),
                starts: starts.clone(),
            },
            sema::StmtKind::Print { to, items, newline } => Op::Print {
                to: to.clone(),
                items: print_items(items),
                newline: *newline,
            },
            sema::StmtKind::PrintUsing {
                to,
                before,
                format,
                values,
                newline,
            } => Op::PrintUsing {
                to: to.clone(),
                before: print_items(before),
                format: format.clone(),
                values: values.clone(),
                newline: *newline,
            },
            sema::StmtKind::MemGet { block, offset, target } => Op::MemGet {
                block: block.clone(),
                offset: offset.clone(),
                target: target.clone(),
            },
            sema::StmtKind::MemPut { block, offset, value } => Op::MemPut {
                block: block.clone(),
                offset: offset.clone(),
                value: value.clone(),
            },
            sema::StmtKind::MemFill {
                block,
                offset,
                bytes,
                value,
            } => Op::MemFill {
                block: block.clone(),
                offset: offset.clone(),
                bytes: bytes.clone(),
                value: value.clone(),
            },
            sema::StmtKind::Field { file, parts } => Op::Field {
                file: file.clone(),
                parts: parts.clone(),
            },
            sema::StmtKind::Lset { right, target, value } => Op::Lset {
                right: *right,
                target: target.clone(),
                value: value.clone(),
            },
            sema::StmtKind::Write { to, items, newline } => Op::Write {
                to: to.clone(),
                items: print_items(items),
                newline: *newline,
            },
            sema::StmtKind::Input { from, line, targets } => Op::Input {
                from: from.clone(),
                line: *line,
                targets: targets.clone(),
            },
            sema::StmtKind::Read(targets) => Op::Read(targets.clone()),
            sema::StmtKind::Restore(None) => Op::Restore { at: 0, label: None },
            sema::StmtKind::Restore(Some(l)) => {
                let l = self.p.label(*l);
                Op::Restore {
                    at: l.data_at,
                    label: Some(l.name.clone()),
                }
            }
            sema::StmtKind::Call { proc, args } => Op::Call {
                proc: ProcId(proc.0),
                args: args.clone(),
            },
            sema::StmtKind::Builtin { id, args } => Op::Builtin {
                id: *id,
                args: args.clone(),
            },
        };
        self.emit(s.span, s.line, vec![op]);
    }

    /// `IF`, `ELSEIF`, `ELSE`: each branch tests its condition and jumps to the next branch when it is false; a
    /// branch that ran jumps to the end. The `IF` condition skips on a pending error (the body runs, measured); an
    /// `ELSEIF` condition uses the placeholder value (measured, design D8). The jumps to the end carry the
    /// `END IF` line.
    fn if_block(&mut self, s: &sema::Stmt, branches: &[sema::Branch], else_: Option<&[sema::Stmt]>, end_line: u32) {
        let end = self.label(end_line);
        for (k, b) in branches.iter().enumerate() {
            let last = k + 1 == branches.len();
            let next = if last && else_.is_none() {
                end
            } else {
                self.label(end_line)
            };
            let on_error = if k == 0 { OnError::Skip } else { OnError::UseValue };
            let branch = Op::Branch {
                cond: b.cond.clone(),
                when: When::Zero,
                to: next,
                on_error,
            };
            self.emit(b.cond.span, b.line, vec![branch]);
            self.stmts_of(&b.body);
            if next != end {
                self.emit_own(s.span, end_line, vec![Op::Jump(end)]);
                self.place(next);
            }
        }
        if let Some(body) = else_ {
            self.stmts_of(body);
        }
        self.place(end);
    }

    /// `WHILE` and `DO`: a test at the top leaves the loop, a test at the bottom repeats it; without a test at the
    /// bottom the loop jumps back to its top. The bottom statement carries the `WEND` or `LOOP` line and keyword
    /// (`end`), so that a coverage build counts it apart from the top on a one-line loop.
    fn loop_block(
        &mut self,
        s: &sema::Stmt,
        kind: sema::LoopKind,
        test: Option<&sema::LoopTest>,
        body: &[sema::Stmt],
        (end_line, end_at): (u32, Span),
    ) {
        let top = self.label(s.line);
        let exit = self.label(end_line);
        self.place(top);
        // `WHILE c` and `DO WHILE c` leave when `c` is zero, `DO UNTIL c` when it is not.
        let leave = |t: &sema::LoopTest| if t.until { When::NonZero } else { When::Zero };
        if let Some(t) = test.filter(|t| t.at == sema::TestAt::Top) {
            let branch = Op::Branch {
                cond: t.cond.clone(),
                when: leave(t),
                to: exit,
                on_error: OnError::Skip,
            };
            self.emit(s.span, s.line, vec![branch]);
        }
        self.exits.push((kind, exit));
        self.stmts_of(body);
        self.exits.pop();
        let back = match test.filter(|t| t.at == sema::TestAt::Bottom) {
            // `LOOP WHILE c` goes back when `c` is not zero, `LOOP UNTIL c` when it is.
            Some(t) => Op::Branch {
                cond: t.cond.clone(),
                when: if t.until { When::Zero } else { When::NonZero },
                to: top,
                on_error: OnError::Skip,
            },
            None => Op::Jump(top),
        };
        self.emit(end_at, end_line, vec![back]);
        self.place(exit);
    }

    /// `FOR v = a TO b STEP s` … `NEXT`, as the old compiler emits it (`study\02` §6.5), with four temporaries:
    /// the count `t`, the limit `f`, the step `st` and the step's sign `neg`, taken once at the header.
    ///
    /// ```text
    ///         AssignAll(t = a, f = b, st = s, neg = st < 0); Jump(Lentry)      FOR line, one statement
    /// Lbody:  body
    ///         t = st + v                                                      NEXT line
    /// Lentry: v = t; Branch((neg AND t < f) OR (NOT neg AND t > f), NonZero → Lexit); Jump(Lbody)
    /// Lexit:
    /// ```
    ///
    /// The header is one statement, so `RESUME` re-runs all of it (measured). With an error pending its jump is
    /// not taken: the body runs with `v` unassigned, after the limits were stored from the placeholder values.
    /// The step statement carries the `NEXT` line and keyword (`end`).
    fn for_loop(&mut self, s: &sema::Stmt, f: ForParts, body: &[sema::Stmt], (end_line, end_at): (u32, Span)) {
        let lbody = self.label(s.line);
        let entry = self.label(end_line);
        let exit = self.label(end_line);
        self.counts.fors += 1;
        let n = self.counts.fors;
        let t = self.temp(format!("for{n}.value"), f.temp);
        let lim = self.temp(format!("for{n}.limit"), f.temp);
        let st = self.temp(format!("for{n}.step"), f.temp);
        let neg = self.temp(format!("for{n}.negative"), Ty::I32);
        let var_ty = self.vars[f.var.0 as usize].ty;

        // The made values carry the `FOR` statement's span.
        let m = Make(s.span);
        let step = match f.step {
            Some(e) => e.clone(),
            None => m.number(1, f.temp),
        };
        let header = vec![
            Op::AssignAll(vec![
                (t, f.start.clone()),
                (lim, f.end.clone()),
                (st, step),
                (
                    neg,
                    m.binary(BinOp::Lt, Ty::I32, m.var(st, f.temp), m.number(0, f.temp)),
                ),
            ]),
            Op::Jump(entry),
        ];
        self.emit(s.span, s.line, header);

        self.place(lbody);
        self.exits.push((sema::LoopKind::For, exit));
        self.stmts_of(body);
        self.exits.pop();

        // The step is added to the variable's current value (measured: `k = k + 1` in the body counts on from it).
        let current = m.convert(m.var(f.var, var_ty), f.temp);
        let next = Op::Assign {
            place: Place::Var(t),
            value: m.binary(BinOp::Add, f.temp, m.var(st, f.temp), current),
        };
        self.emit(end_at, end_line, vec![next]);

        self.place(entry);
        let store = Op::Assign {
            place: Place::Var(f.var),
            value: m.convert(m.var(t, f.temp), var_ty),
        };
        let neg_v = || m.var(neg, Ty::I32);
        let below = m.binary(BinOp::Lt, Ty::I32, m.var(t, f.temp), m.var(lim, f.temp));
        let above = m.binary(BinOp::Gt, Ty::I32, m.var(t, f.temp), m.var(lim, f.temp));
        let not_neg = m.expr(
            Ty::I32,
            ExprKind::Unary {
                op: UnOp::Not,
                operand: Box::new(neg_v()),
            },
        );
        let past = m.binary(
            BinOp::Or,
            Ty::I32,
            m.binary(BinOp::And, Ty::I32, neg_v(), below),
            m.binary(BinOp::And, Ty::I32, not_neg, above),
        );
        let test = Op::Branch {
            cond: past,
            when: When::NonZero,
            to: exit,
            on_error: OnError::Skip,
        };
        self.emit(s.span, end_line, vec![store, test, Op::Jump(lbody)]);
        self.place(exit);
    }

    /// `SELECT CASE` / `SELECT EVERYCASE` (design D7 of `m2-core-builtins`, as the old compiler emits it,
    /// `qb64pe.bas` 6790–7209):
    ///
    /// ```text
    ///         copy = selector; matched = 0                            SELECT line, one statement (each part if any)
    ///         Branch(items, Zero → Lnext1, Skip)                      CASE line
    ///         body; Jump(Lend)   (EVERYCASE: matched = -1)
    /// Lnext1: Branch(items, Zero → Lnext2, Skip)                      next CASE line
    ///         …
    /// Lnextk: (EVERYCASE: Branch(matched, NonZero → Lend))  CASE ELSE body
    /// Lend:
    /// ```
    ///
    /// The items of a `CASE` are joined by `_ORELSE` and a range is `>=` `_ANDALSO` `<=`, as the old compiler's
    /// `||` and `&&` (a later item is not evaluated once one matched). A test that raises takes its `CASE`, as `IF`
    /// does (measured). The copy of a selector that is not a plain variable is one per `SELECT` statement: a global
    /// variable in the main module, a `STATIC` one in a procedure, so a recursive call overwrites it
    /// (`DIVERGENCES-QB45.md` Q-002); its name starts with a digit, so no BASIC name meets it. The `EVERYCASE` flag
    /// is a temporary of the body (per call in a procedure, measured from the C++).
    fn select(&mut self, s: &sema::Stmt, sel: SelectParts) {
        self.counts.selects += 1;
        let n = self.counts.selects;
        let m = Make(s.span);
        let end = self.label(sel.end_line);
        let mut head = Vec::new();
        let value = if sel.copied {
            let storage = match self.owner {
                None => Storage::Global,
                Some(q) => Storage::Static(ProcId(q.0)),
            };
            let copy = self.hidden(format!("{n}SELECT"), sel.selector.ty, storage);
            head.push(Op::Assign {
                place: Place::Var(copy),
                value: sel.selector.clone(),
            });
            m.var(copy, sel.selector.ty)
        } else {
            sel.selector.clone()
        };
        let matched = sel.every.then(|| {
            let f = self.temp(format!("select{n}.matched"), Ty::I32);
            head.push(Op::Assign {
                place: Place::Var(f),
                value: m.number(0, Ty::I32),
            });
            f
        });
        if !head.is_empty() {
            self.emit(s.span, s.line, head);
        }
        for c in sel.cases {
            let next = self.label(sel.end_line);
            let mut tests = c.items.iter().map(|item| match item {
                sema::CaseItem::Is(op, v) => m.compare(*op, &value, v),
                sema::CaseItem::Range(low, high) => m.binary(
                    BinOp::AndAlso,
                    Ty::I32,
                    m.compare(BinOp::Ge, &value, low),
                    m.compare(BinOp::Le, &value, high),
                ),
            });
            let first = tests.next().expect("a `CASE` has an item");
            let cond = tests.fold(first, |acc, t| m.binary(BinOp::OrElse, Ty::I32, acc, t));
            let branch = Op::Branch {
                cond,
                when: When::Zero,
                to: next,
                on_error: OnError::Skip,
            };
            self.emit(s.span, c.line, vec![branch]);
            self.selects.push((end, false));
            self.stmts_of(&c.body);
            self.selects.pop();
            // Where an `EXIT CASE` met since the last `CASE` closed continues.
            if let Some(l) = self.case_end.take() {
                self.place(l);
            }
            let leave = match matched {
                Some(f) => Op::Assign {
                    place: Place::Var(f),
                    value: m.number(-1, Ty::I32),
                },
                None => Op::Jump(end),
            };
            self.emit_own(s.span, sel.end_line, vec![leave]);
            self.place(next);
        }
        if let Some(body) = sel.else_ {
            if let Some(f) = matched {
                let skip = Op::Branch {
                    cond: m.var(f, Ty::I32),
                    when: When::NonZero,
                    to: end,
                    on_error: OnError::UseValue,
                };
                self.emit_own(s.span, sel.end_line, vec![skip]);
            }
            self.selects.push((end, true));
            self.stmts_of(body);
            self.selects.pop();
        }
        self.place(end);
    }

    /// `ON n GOTO …` / `ON n GOSUB …` (design D8 of `m2-core-builtins`, as the old compiler emits it, `qb64pe.bas`
    /// 27620–27709), all statements on the `ON` line:
    ///
    /// ```text
    ///         n = value; Branch(n = 1 → l1, UseValue); …; Branch(n = k → lk, UseValue)     GOTO: one statement
    ///         n = value                                                                     GOSUB: the store, then
    ///         Branch(n <> 1 → Lskip1, UseValue); Gosub(l1); Jump(Lend)                      per target
    /// Lskip1: …
    ///         Branch(n >= 0 → Lend, UseValue)
    ///         Raise 5
    /// Lend:
    /// ```
    ///
    /// The tests use the value even when computing it raised (measured: `ON ASC("") + 1 GOTO` goes to the first
    /// label); such an error is serviced at the first event check reached, the target label's or the store's
    /// statement's, as in the old compiler.
    fn on_jump(&mut self, s: &sema::Stmt, value: &Expr, gosub: bool, targets: &[sema::LabelId]) {
        self.counts.ons += 1;
        let n = self.counts.ons;
        let m = Make(s.span);
        let v = self.temp(format!("on{n}.value"), Ty::I32);
        let end = self.label(s.line);
        let targets: Vec<LabelId> = targets.iter().map(|l| self.ids.get(*l)).collect();
        let test = |op: BinOp, k: i64| m.binary(op, Ty::I32, m.var(v, Ty::I32), m.number(k, Ty::I32));
        let store = Op::Assign {
            place: Place::Var(v),
            value: value.clone(),
        };
        if gosub {
            self.emit(s.span, s.line, vec![store]);
            for (k, &l) in (1..).zip(&targets) {
                let skip = self.label(s.line);
                let other = Op::Branch {
                    cond: test(BinOp::Ne, k),
                    when: When::NonZero,
                    to: skip,
                    on_error: OnError::UseValue,
                };
                self.emit(s.span, s.line, vec![other, Op::Gosub(l), Op::Jump(end)]);
                self.place(skip);
            }
        } else {
            let mut ops = vec![store];
            for (k, &l) in (1..).zip(&targets) {
                ops.push(Op::Branch {
                    cond: test(BinOp::Eq, k),
                    when: When::NonZero,
                    to: l,
                    on_error: OnError::UseValue,
                });
            }
            self.emit(s.span, s.line, ops);
        }
        let not_negative = Op::Branch {
            cond: test(BinOp::Ge, 0),
            when: When::NonZero,
            to: end,
            on_error: OnError::UseValue,
        };
        self.emit(s.span, s.line, vec![not_negative]);
        self.emit(s.span, s.line, vec![Op::Raise(m.number(5, Ty::I32))]);
        self.place(end);
    }

    /// A new hidden variable of this body.
    fn temp(&mut self, name: String, ty: Ty) -> VarId {
        let storage = Storage::Temp(self.owner.map(|q| ProcId(q.0)));
        self.hidden(name, ty, storage)
    }

    /// A new variable the lowering makes, of any storage class.
    fn hidden(&mut self, name: String, ty: Ty, storage: Storage) -> VarId {
        self.vars.push(Var {
            name,
            ty,
            storage,
            shape: crate::Shape::Scalar,
            conventional: false,
        });
        VarId(to_u32(self.vars.len() - 1))
    }
}

/// A `SELECT`, with its parts still typed-tree values and statements.
struct SelectParts<'e> {
    selector: &'e sema::Expr,
    copied: bool,
    every: bool,
    cases: &'e [sema::Case],
    else_: Option<&'e [sema::Stmt]>,
    end_line: u32,
}

/// The header of a `FOR`, with its values still typed-tree expressions.
struct ForParts<'e> {
    var: VarId,
    /// The type the loop counts in: start, end and step are already of this type.
    temp: Ty,
    start: &'e sema::Expr,
    end: &'e sema::Expr,
    step: Option<&'e sema::Expr>,
}

/// Makes the values of a lowered statement, all with its span (`qb` is the type: the emitter does not read it).
struct Make(Span);

impl Make {
    fn expr(&self, ty: Ty, kind: ExprKind) -> Expr {
        Expr {
            span: self.0,
            ty,
            qb: ty,
            kind,
        }
    }

    fn var(&self, id: VarId, ty: Ty) -> Expr {
        self.expr(ty, ExprKind::Load(Place::Var(id)))
    }

    /// A whole number of a numeric type.
    fn number(&self, n: i64, ty: Ty) -> Expr {
        let kind = if ty.is_float() {
            ExprKind::Float(n.to_string())
        } else {
            ExprKind::Int(n)
        };
        self.expr(ty, kind)
    }

    fn binary(&self, op: BinOp, ty: Ty, lhs: Expr, rhs: Expr) -> Expr {
        let kind = ExprKind::Binary {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        };
        self.expr(ty, kind)
    }

    /// A comparison of a `SELECT`'s value with a `CASE` value, which is already of the comparison's type (the value
    /// is converted to it, never narrower): -1 or 0, LONG.
    fn compare(&self, op: BinOp, value: &Expr, item: &Expr) -> Expr {
        if item.ty == Ty::Str {
            let kind = ExprKind::StrCompare {
                op,
                lhs: Box::new(value.clone()),
                rhs: Box::new(item.clone()),
            };
            return self.expr(Ty::I32, kind);
        }
        self.binary(op, Ty::I32, self.convert(value.clone(), item.ty), item.clone())
    }

    /// Between a `FOR` variable's type and the type its loop counts in: wider (exact), or back (an integer keeps
    /// its low bits, measured: `FOR i% = 32760 TO 32767 STEP 4` ends with -32768; a float rounds to nearest).
    fn convert(&self, v: Expr, to: Ty) -> Expr {
        if v.ty == to {
            return v;
        }
        let how = sema::conversion(v.ty, to);
        self.expr(to, ExprKind::Convert { how, from: Box::new(v) })
    }
}

fn print_items(items: &[sema::PrintItem]) -> Vec<PrintItem> {
    items
        .iter()
        .map(|i| match i {
            sema::PrintItem::Str(e) => PrintItem::Str(e.clone()),
            sema::PrintItem::Num(e) => PrintItem::Num(e.clone()),
            sema::PrintItem::Zone => PrintItem::Zone,
        })
        .collect()
}

fn storage(s: sema::Storage) -> Storage {
    let id = |p: sema::ProcId| ProcId(p.0);
    match s {
        sema::Storage::Main => Storage::Global,
        sema::Storage::Static(p) => Storage::Static(id(p)),
        sema::Storage::Local(p) => Storage::Local(id(p)),
        sema::Storage::Param(p) => Storage::Param(id(p)),
        sema::Storage::Result(p) => Storage::Result(id(p)),
    }
}

/// The IR id of each label (by its `sema` id): its index among the labels of its own body.
struct LabelIds(Vec<LabelId>);

impl LabelIds {
    fn new(p: &sema::Program) -> LabelIds {
        let mut counts: std::collections::HashMap<Option<sema::ProcId>, u32> = std::collections::HashMap::new();
        LabelIds(
            p.labels
                .iter()
                .map(|l| {
                    let n = counts.entry(l.proc).or_default();
                    *n += 1;
                    LabelId(*n - 1)
                })
                .collect(),
        )
    }

    fn get(&self, l: sema::LabelId) -> LabelId {
        self.0[l.0 as usize]
    }
}
