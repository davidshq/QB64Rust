//! `--dump typed`: one node per line, indented, with its type (FreeBASIC lesson L8: assertions on types).

use crate::{Arg, Expr, ExprKind, Label, PrintItem, ProcKind, Program, Resume, Stmt, StmtKind, Storage, Ty, VarId};
use qb64rust_base::show_bytes;
use std::fmt::Write as _;

/// The main module's statements and labels, then each procedure: its header, its variables by storage class, its
/// statements. Variables other than main-module ones are shown with their storage (`A (param)`).
pub fn dump_typed(p: &Program) -> String {
    let mut out = String::new();
    stmts(p, &p.stmts, &p.labels, &mut out);
    for (i, proc) in p.procs.iter().enumerate() {
        match proc.kind {
            ProcKind::Sub => writeln!(out, "SUB {}", proc.name).unwrap(),
            ProcKind::Function(t) => writeln!(out, "FUNCTION {} : {}", proc.name, ty(t)).unwrap(),
        }
        for v in &p.vars {
            let class = match v.storage {
                Storage::Param(q) if q.0 as usize == i => "param",
                Storage::Result(q) if q.0 as usize == i => "result",
                Storage::Static(q) if q.0 as usize == i => "static",
                Storage::Local(q) if q.0 as usize == i => "local",
                // Variables of other procedures and of the main module.
                Storage::Main | Storage::Static(_) | Storage::Local(_) | Storage::Param(_) | Storage::Result(_) => {
                    continue;
                }
            };
            writeln!(out, "  {class} {}:{}", v.name, ty(v.ty)).unwrap();
        }
        stmts(p, &proc.stmts, &[], &mut out);
    }
    out
}

/// A variable's name, with its storage class unless it is a main-module variable.
fn var_name(p: &Program, id: VarId) -> String {
    let v = p.var(id);
    let class = match v.storage {
        Storage::Main => return v.name.clone(),
        Storage::Static(_) => "static",
        Storage::Local(_) => "local",
        Storage::Param(_) => "param",
        Storage::Result(_) => "result",
    };
    format!("{} ({class})", v.name)
}

fn args(p: &Program, args: &[Arg], depth: usize, out: &mut String) {
    let pad = "  ".repeat(depth);
    for a in args {
        match a {
            Arg::Ref(v) => writeln!(out, "{pad}ref {} : {}", var_name(p, *v), ty(p.var(*v).ty)).unwrap(),
            Arg::Temp(e) => {
                writeln!(out, "{pad}temp").unwrap();
                expr(p, e, depth + 1, out);
            }
        }
    }
}

/// The statements, each label before the statement it stands before (labels after the last statement at the end).
fn stmts(p: &Program, list: &[Stmt], labels: &[Label], out: &mut String) {
    let label_lines = |at: usize, out: &mut String| {
        for l in labels.iter().filter(|l| l.at == at) {
            writeln!(out, "line {}: Label {}", l.line, l.name).unwrap();
        }
    };
    for (i, s) in list.iter().enumerate() {
        label_lines(i, out);
        match &s.kind {
            StmtKind::ConsoleOnly => writeln!(out, "line {}: ConsoleOnly", s.line).unwrap(),
            StmtKind::End => writeln!(out, "line {}: End", s.line).unwrap(),
            StmtKind::System => writeln!(out, "line {}: System", s.line).unwrap(),
            StmtKind::Exit => writeln!(out, "line {}: Exit", s.line).unwrap(),
            StmtKind::OnError(Some(l)) => writeln!(out, "line {}: OnError {}", s.line, p.label(*l).name).unwrap(),
            StmtKind::OnError(None) => writeln!(out, "line {}: OnError 0", s.line).unwrap(),
            StmtKind::Resume(r) => {
                let to = match r {
                    Resume::Retry => "Retry".to_string(),
                    Resume::Next => "Next".to_string(),
                    Resume::To(l) => format!("To {}", p.label(*l).name),
                };
                writeln!(out, "line {}: Resume {to}", s.line).unwrap();
            }
            StmtKind::Error(code) => {
                writeln!(out, "line {}: Error", s.line).unwrap();
                expr(p, code, 1, out);
            }
            StmtKind::Call { proc, args: a } => {
                writeln!(out, "line {}: Call {}", s.line, p.proc(*proc).name).unwrap();
                args(p, a, 1, out);
            }
            StmtKind::Assign { var, value } => {
                let v = p.var(*var);
                writeln!(out, "line {}: Assign {}:{}", s.line, var_name(p, *var), ty(v.ty)).unwrap();
                expr(p, value, 1, out);
            }
            StmtKind::Print { items, newline } => {
                writeln!(out, "line {}: Print{}", s.line, if *newline { " newline" } else { "" }).unwrap();
                for i in items {
                    match i {
                        PrintItem::Str(e) => {
                            writeln!(out, "  Str").unwrap();
                            expr(p, e, 2, out);
                        }
                        PrintItem::Num(e) => {
                            writeln!(out, "  Num").unwrap();
                            expr(p, e, 2, out);
                        }
                        PrintItem::Zone => writeln!(out, "  Zone").unwrap(),
                    }
                }
            }
        }
    }
    label_lines(list.len(), out);
}

fn ty(t: Ty) -> &'static str {
    match t {
        Ty::I16 => "I16",
        Ty::I32 => "I32",
        Ty::I64 => "I64",
        Ty::F32 => "F32",
        Ty::F64 => "F64",
        Ty::F80 => "F80",
        Ty::Str => "Str",
    }
}

fn expr(p: &Program, e: &Expr, depth: usize, out: &mut String) {
    let pad = "  ".repeat(depth);
    let types = if e.qb == e.ty {
        ty(e.ty).to_string()
    } else {
        format!("{} (qb {})", ty(e.ty), ty(e.qb))
    };
    match &e.kind {
        ExprKind::Int(v) => writeln!(out, "{pad}Int {v} : {types}").unwrap(),
        ExprKind::Float(t) => writeln!(out, "{pad}Float {t} : {types}").unwrap(),
        ExprKind::Str(s) => writeln!(out, "{pad}Str \"{}\" : {types}", show_bytes(s)).unwrap(),
        ExprKind::Var(id) => writeln!(out, "{pad}Var {} : {types}", var_name(p, *id)).unwrap(),
        ExprKind::Convert { how, from } => {
            writeln!(out, "{pad}Convert {how:?} : {types}").unwrap();
            expr(p, from, depth + 1, out);
        }
        ExprKind::Binary { op, lhs, rhs } => {
            writeln!(out, "{pad}Binary {op:?} : {types}").unwrap();
            expr(p, lhs, depth + 1, out);
            expr(p, rhs, depth + 1, out);
        }
        ExprKind::Unary { op, operand } => {
            writeln!(out, "{pad}{op:?} : {types}").unwrap();
            expr(p, operand, depth + 1, out);
        }
        ExprKind::Concat(a, b) => {
            writeln!(out, "{pad}Concat : {types}").unwrap();
            expr(p, a, depth + 1, out);
            expr(p, b, depth + 1, out);
        }
        ExprKind::StrCompare { op, lhs, rhs } => {
            writeln!(out, "{pad}StrCompare {op:?} : {types}").unwrap();
            expr(p, lhs, depth + 1, out);
            expr(p, rhs, depth + 1, out);
        }
        ExprKind::Call { builtin, args: slots } => {
            writeln!(out, "{pad}Call {} : {types}", builtin.get().name.to_ascii_uppercase()).unwrap();
            for a in slots {
                match a {
                    Some(a) => expr(p, a, depth + 1, out),
                    None => writeln!(out, "{pad}  (absent)").unwrap(),
                }
            }
        }
        ExprKind::CallProc { proc, args: a } => {
            writeln!(out, "{pad}CallProc {} : {types}", p.proc(*proc).name).unwrap();
            args(p, a, depth + 1, out);
        }
    }
}
