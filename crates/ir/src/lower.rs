//! Typed tree to IR. One IR statement per source statement; a label statement becomes a position in its body.

use crate::{
    Arg, Body, Const, Label, LabelId, Op, PrintItem, Proc, ProcId, ProcKind, Program, Resume, Stmt, Storage, Value,
    ValueKind, Var, VarId,
};
use qb64rust_base::Diagnostics;
use qb64rust_sema as sema;

/// The statements of a checked program that the IR cannot express yet, each marked "not supported yet". The driver
/// reports them and stops before [`lower`], which must not see them. They are typed by `sema` (task 5 of
/// `m2-control-flow-slice`) and lowered from task 6.2 on. Only the bodies' own statements are looked at: a block is
/// reported as a whole.
pub fn not_lowered(p: &sema::Program) -> Diagnostics {
    let mut diags = Diagnostics::new();
    for s in p.stmts.iter().chain(p.procs.iter().flat_map(|q| &q.stmts)) {
        let what = match s.kind {
            sema::StmtKind::Goto(_) => "`GOTO`",
            sema::StmtKind::Gosub(_) => "`GOSUB`",
            sema::StmtKind::Return(_) => "`RETURN`",
            sema::StmtKind::If { .. } => "`IF`",
            sema::StmtKind::For { .. } => "`FOR`",
            sema::StmtKind::Do { .. } => "`DO`",
            sema::StmtKind::While { .. } => "`WHILE`",
            // Only inside a loop, which is reported itself.
            sema::StmtKind::ExitLoop(_) => "`EXIT`",
            sema::StmtKind::ConsoleOnly
            | sema::StmtKind::Assign { .. }
            | sema::StmtKind::Print { .. }
            | sema::StmtKind::End
            | sema::StmtKind::System
            | sema::StmtKind::Call { .. }
            | sema::StmtKind::Exit
            | sema::StmtKind::OnError(_)
            | sema::StmtKind::Resume(_)
            | sema::StmtKind::Error(_)
            | sema::StmtKind::Label(_) => continue,
        };
        diags.unsupported(s.span, format!("{what} in code generation"));
    }
    diags
}

/// Lowers a program for which [`not_lowered`] found nothing.
pub fn lower(p: &sema::Program) -> Program {
    let ids = LabelIds::new(p);
    let vars = p
        .vars
        .iter()
        .map(|v| Var {
            name: v.name.clone(),
            ty: v.ty,
            storage: storage(v.storage),
        })
        .collect();
    let procs = p
        .procs
        .iter()
        .zip(0..)
        .map(|(q, i)| Proc {
            name: q.name.clone(),
            kind: match q.kind {
                sema::ProcKind::Sub => ProcKind::Sub,
                sema::ProcKind::Function(t) => ProcKind::Function(t),
            },
            params: q.params.iter().map(|v| VarId(v.0)).collect(),
            result: q.result.map(|v| VarId(v.0)),
            body: body(p, &ids, &q.stmts, Some(sema::ProcId(i))),
            line: q.line,
            end_line: q.end_line,
        })
        .collect();
    Program {
        vars,
        procs,
        main: body(p, &ids, &p.stmts, None),
    }
}

/// The body `owner` (`None`: the main module). Its labels keep their order, so a label's IR id is its index among
/// its own body's labels ([`LabelIds`]).
fn body(p: &sema::Program, ids: &LabelIds, stmts: &[sema::Stmt], owner: Option<sema::ProcId>) -> Body {
    let mut labels: Vec<Label> = p
        .labels
        .iter()
        .filter(|l| l.proc == owner)
        .map(|l| Label {
            name: l.name.clone(),
            line: l.line,
            at: 0,
        })
        .collect();
    let mut out = Vec::new();
    for s in stmts {
        if let sema::StmtKind::Label(l) = s.kind {
            labels[ids.get(l).0 as usize].at = out.len();
            continue;
        }
        let ops = vec![op(ids, &s.kind)];
        let may_raise = ops.iter().any(op_may_raise);
        out.push(Stmt {
            span: s.span,
            line: s.line,
            ops,
            may_raise,
        });
    }
    Body { labels, stmts: out }
}

fn op_may_raise(op: &Op) -> bool {
    match op {
        Op::SelectConsole | Op::End | Op::System | Op::Exit | Op::SetHandler(_) => false,
        // `RESUME` outside a handler raises error 20.
        Op::Call { .. } | Op::Raise(_) | Op::Resume(_) => true,
        Op::Assign { value, .. } => value.may_raise(),
        Op::Print { items, .. } => items.iter().any(|i| match i {
            PrintItem::Str(v) | PrintItem::Num(v) => v.may_raise(),
            PrintItem::Zone => false,
        }),
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

fn op(ids: &LabelIds, k: &sema::StmtKind) -> Op {
    match k {
        sema::StmtKind::ConsoleOnly => Op::SelectConsole,
        sema::StmtKind::End => Op::End,
        sema::StmtKind::System => Op::System,
        sema::StmtKind::Exit => Op::Exit,
        sema::StmtKind::OnError(l) => Op::SetHandler(l.map(|l| ids.get(l))),
        sema::StmtKind::Resume(r) => Op::Resume(match r {
            sema::Resume::Retry => Resume::Retry,
            sema::Resume::Next => Resume::Next,
            sema::Resume::To(l) => Resume::To(ids.get(*l)),
        }),
        sema::StmtKind::Label(_) => unreachable!("a label is a position (`body`)"),
        sema::StmtKind::Goto(_)
        | sema::StmtKind::Gosub(_)
        | sema::StmtKind::Return(_)
        | sema::StmtKind::If { .. }
        | sema::StmtKind::For { .. }
        | sema::StmtKind::Do { .. }
        | sema::StmtKind::While { .. }
        | sema::StmtKind::ExitLoop(_) => unreachable!("reported by `not_lowered`"),
        sema::StmtKind::Error(code) => Op::Raise(value_of(code)),
        sema::StmtKind::Assign { var, value } => Op::Assign {
            place: VarId(var.0),
            value: value_of(value),
        },
        sema::StmtKind::Print { items, newline } => Op::Print {
            items: items
                .iter()
                .map(|i| match i {
                    sema::PrintItem::Str(e) => PrintItem::Str(value_of(e)),
                    sema::PrintItem::Num(e) => PrintItem::Num(value_of(e)),
                    sema::PrintItem::Zone => PrintItem::Zone,
                })
                .collect(),
            newline: *newline,
        },
        sema::StmtKind::Call { proc, args } => Op::Call {
            proc: ProcId(proc.0),
            args: args_of(args),
        },
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

fn args_of(args: &[sema::Arg]) -> Vec<Arg> {
    args.iter()
        .map(|a| match a {
            sema::Arg::Ref(v) => Arg::Ref(VarId(v.0)),
            sema::Arg::Temp(e) => Arg::Temp(value_of(e)),
        })
        .collect()
}

fn value_of(e: &sema::Expr) -> Value {
    let kind = match &e.kind {
        sema::ExprKind::Int(v) => ValueKind::Const(Const::Int(*v)),
        sema::ExprKind::Float(t) => ValueKind::Const(Const::Float(t.clone())),
        sema::ExprKind::Str(s) => ValueKind::Const(Const::Str(s.clone())),
        sema::ExprKind::Var(id) => ValueKind::Var(VarId(id.0)),
        sema::ExprKind::Convert { how, from } => ValueKind::Convert {
            how: *how,
            from: Box::new(value_of(from)),
        },
        sema::ExprKind::Binary { op, lhs, rhs } => ValueKind::Binary {
            op: *op,
            lhs: Box::new(value_of(lhs)),
            rhs: Box::new(value_of(rhs)),
        },
        sema::ExprKind::Unary { op, operand } => ValueKind::Unary {
            op: *op,
            operand: Box::new(value_of(operand)),
        },
        sema::ExprKind::Concat(a, b) => ValueKind::Concat(Box::new(value_of(a)), Box::new(value_of(b))),
        sema::ExprKind::StrCompare { op, lhs, rhs } => ValueKind::StrCompare {
            op: *op,
            lhs: Box::new(value_of(lhs)),
            rhs: Box::new(value_of(rhs)),
        },
        sema::ExprKind::Call { builtin, args } => ValueKind::CallBuiltin {
            id: *builtin,
            args: args.iter().map(|a| a.as_ref().map(value_of)).collect(),
        },
        sema::ExprKind::CallProc { proc, args } => ValueKind::CallProc {
            proc: ProcId(proc.0),
            args: args_of(args),
        },
    };
    Value { ty: e.ty, kind }
}
