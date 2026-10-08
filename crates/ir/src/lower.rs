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
//! | `GOTO x` / `GOSUB x` / `RETURN` | `Jump(x)` / `Gosub(x)` / `Return` |
//!
//! `IF c GOTO x` is an `IF` whose branch is `GOTO x`. Conditions of `IF`, `WHILE`, `DO` and `LOOP` are
//! [`OnError::Skip`] branches.

use crate::{
    Body, Conv, Expr, ExprKind, Label, LabelId, OnError, Op, Place, PrintItem, Proc, ProcId, ProcKind, Program, Resume,
    Stmt, Storage, Ty, Var, VarId, When,
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
            dims: v.dims.clone(),
        })
        .collect();
    // The main module first, so the temporaries are numbered in source order as far as possible.
    let mut fors = 0;
    let main = Lowerer::new(p, &ids, None, &mut vars, &mut fors).body(&p.stmts);
    let mut procs = Vec::new();
    for (q, i) in p.procs.iter().zip(0..) {
        let body = Lowerer::new(p, &ids, Some(sema::ProcId(i)), &mut vars, &mut fors).body(&q.stmts);
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
        });
    }
    let program = Program {
        vars,
        procs,
        main,
        types: p.types.clone(),
    };
    if cfg!(debug_assertions)
        && let Err(problems) = crate::validate(&program)
    {
        panic!("the lowering made an invalid IR:\n{}", problems.join("\n"));
    }
    program
}

/// The lowering of one body.
struct Lowerer<'a> {
    ids: &'a LabelIds,
    /// The body: `None` for the main module.
    owner: Option<sema::ProcId>,
    /// The program's variables; the lowering adds its temporaries.
    vars: &'a mut Vec<Var>,
    /// `FOR` loops lowered so far in the program, for the temporaries' names.
    fors: &'a mut u32,
    labels: Vec<Label>,
    stmts: Vec<Stmt>,
    /// The exit labels of the loops around the statement being lowered, innermost last.
    exits: Vec<(sema::LoopKind, LabelId)>,
}

impl<'a> Lowerer<'a> {
    fn new(
        p: &'a sema::Program,
        ids: &'a LabelIds,
        owner: Option<sema::ProcId>,
        vars: &'a mut Vec<Var>,
        fors: &'a mut u32,
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
                file: Some(l.file),
                at: 0,
            })
            .collect();
        Lowerer {
            ids,
            owner,
            vars,
            fors,
            labels,
            stmts: Vec::new(),
            exits: Vec::new(),
        }
    }

    fn body(mut self, stmts: &[sema::Stmt]) -> Body {
        self.stmts_of(stmts);
        Body {
            labels: self.labels,
            stmts: self.stmts,
        }
    }

    fn stmts_of(&mut self, stmts: &[sema::Stmt]) {
        for s in stmts {
            self.stmt(s);
        }
    }

    /// A new label of this body, placed later by [`Lowerer::place`].
    fn label(&mut self, line: u32) -> LabelId {
        self.labels.push(Label {
            name: None,
            line,
            file: None,
            at: usize::MAX,
        });
        LabelId(to_u32(self.labels.len() - 1))
    }

    /// Puts a label before the next statement.
    fn place(&mut self, l: LabelId) {
        self.labels[l.0 as usize].at = self.stmts.len();
    }

    fn emit(&mut self, span: Span, line: u32, ops: Vec<Op>) {
        let may_raise = ops.iter().any(Op::may_raise);
        self.stmts.push(Stmt {
            span,
            line,
            ops,
            may_raise,
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
            sema::StmtKind::While { cond, body, end_line } => {
                let test = sema::LoopTest {
                    at: sema::TestAt::Top,
                    until: false,
                    cond: cond.clone(),
                };
                return self.loop_block(s, sema::LoopKind::While, Some(&test), body, *end_line);
            }
            sema::StmtKind::Do { test, body, end_line } => {
                return self.loop_block(s, sema::LoopKind::Do, test.as_ref(), body, *end_line);
            }
            sema::StmtKind::For {
                var,
                temp,
                start,
                end,
                step,
                body,
                end_line,
            } => {
                let parts = ForParts {
                    var: VarId(var.0),
                    temp: *temp,
                    start,
                    end,
                    step: step.as_ref(),
                };
                return self.for_loop(s, parts, body, *end_line);
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
            sema::StmtKind::Goto(l) => Op::Jump(self.ids.get(*l)),
            sema::StmtKind::Gosub(l) => Op::Gosub(self.ids.get(*l)),
            sema::StmtKind::Return(l) => Op::Return(l.map(|l| self.ids.get(l))),
            sema::StmtKind::ConsoleOnly => Op::SelectConsole,
            sema::StmtKind::End => Op::End,
            sema::StmtKind::System => Op::System,
            sema::StmtKind::Exit => Op::Exit,
            sema::StmtKind::OnError(l) => Op::SetHandler(l.map(|l| self.ids.get(l))),
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
            sema::StmtKind::Print { items, newline } => Op::Print {
                items: items
                    .iter()
                    .map(|i| match i {
                        sema::PrintItem::Str(e) => PrintItem::Str(e.clone()),
                        sema::PrintItem::Num(e) => PrintItem::Num(e.clone()),
                        sema::PrintItem::Zone => PrintItem::Zone,
                    })
                    .collect(),
                newline: *newline,
            },
            sema::StmtKind::Call { proc, args } => Op::Call {
                proc: ProcId(proc.0),
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
                self.emit(s.span, end_line, vec![Op::Jump(end)]);
                self.place(next);
            }
        }
        if let Some(body) = else_ {
            self.stmts_of(body);
        }
        self.place(end);
    }

    /// `WHILE` and `DO`: a test at the top leaves the loop, a test at the bottom repeats it; without a test at the
    /// bottom the loop jumps back to its top. The bottom statement carries the `WEND` or `LOOP` line.
    fn loop_block(
        &mut self,
        s: &sema::Stmt,
        kind: sema::LoopKind,
        test: Option<&sema::LoopTest>,
        body: &[sema::Stmt],
        end_line: u32,
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
        self.emit(s.span, end_line, vec![back]);
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
    fn for_loop(&mut self, s: &sema::Stmt, f: ForParts, body: &[sema::Stmt], end_line: u32) {
        let lbody = self.label(s.line);
        let entry = self.label(end_line);
        let exit = self.label(end_line);
        *self.fors += 1;
        let n = *self.fors;
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
        self.emit(s.span, end_line, vec![next]);

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

    /// A new hidden variable of this body.
    fn temp(&mut self, name: String, ty: Ty) -> VarId {
        self.vars.push(Var {
            name,
            ty,
            storage: Storage::Temp(self.owner.map(|q| ProcId(q.0))),
            dims: Vec::new(),
        });
        VarId(to_u32(self.vars.len() - 1))
    }
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

    /// Between a `FOR` variable's type and the type its loop counts in: wider (exact), or back (an integer keeps
    /// its low bits, measured: `FOR i% = 32760 TO 32767 STEP 4` ends with -32768; a float rounds to nearest).
    fn convert(&self, v: Expr, to: Ty) -> Expr {
        if v.ty == to {
            return v;
        }
        let how = match (v.ty.is_int(), to > v.ty) {
            (_, true) => Conv::Widen,
            (true, false) => Conv::Truncate,
            (false, false) => Conv::Nearest,
        };
        self.expr(to, ExprKind::Convert { how, from: Box::new(v) })
    }
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
