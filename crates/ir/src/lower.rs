//! Typed tree to IR. One IR statement per source statement.

use crate::{BinOp, Const, Conv, Op, PrintItem, Program, Stmt, Ty, Value, ValueKind, Var, VarId};
use qb64rust_sema as sema;

pub fn lower(p: &sema::Program) -> Program {
    let vars = p
        .vars
        .iter()
        .map(|v| Var {
            name: v.name.clone(),
            ty: ty(v.ty),
        })
        .collect();
    let main = p
        .stmts
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
    Program { vars, main }
}

fn op_may_raise(op: &Op) -> bool {
    match op {
        Op::SelectConsole | Op::End => false,
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

fn op(k: &sema::StmtKind) -> Op {
    match k {
        sema::StmtKind::ConsoleOnly => Op::SelectConsole,
        sema::StmtKind::End => Op::End,
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
        sema::StmtKind::Call { .. } | sema::StmtKind::Exit => unreachable!("{PROCS_NOT_LOWERED}"),
    }
}

/// Until the IR has procedures (task 4.1 of `m2-procedures-and-errors`), the driver rejects programs that define
/// any before lowering them, so neither calls nor `EXIT` reach this module.
const PROCS_NOT_LOWERED: &str = "procedures are rejected before lowering";

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
        sema::ExprKind::CallProc { .. } => unreachable!("{PROCS_NOT_LOWERED}"),
    };
    Value { ty: ty(e.ty), kind }
}
