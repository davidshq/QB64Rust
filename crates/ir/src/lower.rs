//! Typed tree to IR. One IR statement per source statement.

use crate::{
    Arg, BinOp, Body, Const, Conv, Label, LabelId, Op, PrintItem, Proc, ProcId, ProcKind, Program, Resume, Stmt,
    Storage, Ty, Value, ValueKind, Var, VarId,
};
use qb64rust_sema as sema;

pub fn lower(p: &sema::Program) -> Program {
    let vars = p
        .vars
        .iter()
        .map(|v| Var {
            name: v.name.clone(),
            ty: ty(v.ty),
            storage: storage(v.storage),
        })
        .collect();
    let procs = p
        .procs
        .iter()
        .map(|q| Proc {
            name: q.name.clone(),
            kind: match q.kind {
                sema::ProcKind::Sub => ProcKind::Sub,
                sema::ProcKind::Function(t) => ProcKind::Function(ty(t)),
            },
            params: q.params.iter().map(|v| VarId(v.0)).collect(),
            result: q.result.map(|v| VarId(v.0)),
            body: body(&q.stmts, &[]),
            line: q.line,
            end_line: q.end_line,
        })
        .collect();
    Program {
        vars,
        procs,
        main: body(&p.stmts, &p.labels),
    }
}

fn body(stmts: &[sema::Stmt], labels: &[sema::Label]) -> Body {
    let labels = labels
        .iter()
        .map(|l| Label {
            name: l.name.clone(),
            line: l.line,
            at: l.at,
        })
        .collect();
    let stmts = stmts
        .iter()
        .map(|s| {
            let ops = vec![op(&s.kind)];
            let may_raise = ops.iter().any(op_may_raise);
            Stmt {
                span: s.span,
                line: s.line,
                ops,
                may_raise,
            }
        })
        .collect();
    Body { labels, stmts }
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

fn ty(t: sema::Ty) -> Ty {
    match t {
        sema::Ty::I16 => Ty::I16,
        sema::Ty::I32 => Ty::I32,
        sema::Ty::I64 => Ty::I64,
        sema::Ty::F32 => Ty::F32,
        sema::Ty::F64 => Ty::F64,
        sema::Ty::F80 => Ty::F80,
        sema::Ty::Str => Ty::Str,
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

fn op(k: &sema::StmtKind) -> Op {
    match k {
        sema::StmtKind::ConsoleOnly => Op::SelectConsole,
        sema::StmtKind::End => Op::End,
        sema::StmtKind::System => Op::System,
        sema::StmtKind::Exit => Op::Exit,
        sema::StmtKind::OnError(l) => Op::SetHandler(l.map(label)),
        sema::StmtKind::Resume(r) => Op::Resume(match r {
            sema::Resume::Retry => Resume::Retry,
            sema::Resume::Next => Resume::Next,
            sema::Resume::To(l) => Resume::To(label(*l)),
        }),
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

fn label(l: sema::LabelId) -> LabelId {
    LabelId(l.0)
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
            how: match how {
                sema::ConvKind::Widen => Conv::Widen,
                sema::ConvKind::Truncate => Conv::Truncate,
                sema::ConvKind::RoundEven => Conv::RoundEven,
                sema::ConvKind::Nearest => Conv::Nearest,
            },
            from: Box::new(value_of(from)),
        },
        sema::ExprKind::Binary { op, lhs, rhs } => ValueKind::Binary {
            op: match op {
                sema::BinOp::Add => BinOp::Add,
                sema::BinOp::Sub => BinOp::Sub,
                sema::BinOp::Mul => BinOp::Mul,
                sema::BinOp::Div => BinOp::Div,
            },
            lhs: Box::new(value_of(lhs)),
            rhs: Box::new(value_of(rhs)),
        },
        sema::ExprKind::Neg(x) => ValueKind::Neg(Box::new(value_of(x))),
        sema::ExprKind::Concat(a, b) => ValueKind::Concat(Box::new(value_of(a)), Box::new(value_of(b))),
        sema::ExprKind::Call { builtin, args } => ValueKind::CallBuiltin {
            id: *builtin,
            args: args.iter().map(|a| a.as_ref().map(value_of)).collect(),
        },
        sema::ExprKind::CallProc { proc, args } => ValueKind::CallProc {
            proc: ProcId(proc.0),
            args: args_of(args),
        },
    };
    Value { ty: ty(e.ty), kind }
}
