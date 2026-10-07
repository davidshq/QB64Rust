//! `--dump typed`: one node per line, indented, with its type (FreeBASIC lesson L8: assertions on types).

use crate::{Arg, Expr, ExprKind, PrintItem, ProcKind, Program, Resume, Stmt, StmtKind, Storage, Ty, VarId};
use qb64rust_base::show_bytes;
use std::fmt::Write as _;

/// The main module's statements and labels, then each procedure: its header, its variables by storage class, its
/// statements. Variables other than main-module ones are shown with their storage (`A (param)`).
pub fn dump_typed(p: &Program) -> String {
    let mut out = String::new();
    stmts(p, &p.stmts, 0, &mut out);
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
        stmts(p, &proc.stmts, 0, &mut out);
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

/// The statements at nesting depth `d`: each is `line N: …` indented by `d` steps, its parts one step deeper, and
/// the statements of a block's bodies at `d + 1` under a heading (`Then`, `ElseIf`, `Else`, `Body`).
fn stmts(p: &Program, list: &[Stmt], d: usize, out: &mut String) {
    let pad = "  ".repeat(d);
    for s in list {
        let line = |out: &mut String, text: &str| writeln!(out, "{pad}line {}: {text}", s.line).unwrap();
        let heading = |out: &mut String, text: &str| writeln!(out, "{pad}  {text}").unwrap();
        match &s.kind {
            StmtKind::Label(l) => line(out, &format!("Label {}", p.label(*l).name)),
            StmtKind::Goto(l) => line(out, &format!("Goto {}", p.label(*l).name)),
            StmtKind::Gosub(l) => line(out, &format!("Gosub {}", p.label(*l).name)),
            StmtKind::Return(None) => line(out, "Return"),
            StmtKind::Return(Some(l)) => line(out, &format!("Return {}", p.label(*l).name)),
            StmtKind::ConsoleOnly => line(out, "ConsoleOnly"),
            StmtKind::End => line(out, "End"),
            StmtKind::System => line(out, "System"),
            StmtKind::Exit => line(out, "Exit"),
            StmtKind::ExitLoop(k) => line(out, &format!("Exit {k:?}")),
            StmtKind::OnError(Some(l)) => line(out, &format!("OnError {}", p.label(*l).name)),
            StmtKind::OnError(None) => line(out, "OnError 0"),
            StmtKind::Resume(r) => {
                let to = match r {
                    Resume::Retry => "Retry".to_string(),
                    Resume::Next => "Next".to_string(),
                    Resume::To(l) => format!("To {}", p.label(*l).name),
                };
                line(out, &format!("Resume {to}"));
            }
            StmtKind::Error(code) => {
                line(out, "Error");
                expr(p, code, d + 1, out);
            }
            StmtKind::Call { proc, args: a } => {
                line(out, &format!("Call {}", p.proc(*proc).name));
                args(p, a, d + 1, out);
            }
            StmtKind::Assign { var, value } => {
                let v = p.var(*var);
                line(out, &format!("Assign {}:{}", var_name(p, *var), ty(v.ty)));
                expr(p, value, d + 1, out);
            }
            StmtKind::Print { items, newline } => {
                line(out, &format!("Print{}", if *newline { " newline" } else { "" }));
                for i in items {
                    match i {
                        PrintItem::Str(e) => {
                            heading(out, "Str");
                            expr(p, e, d + 2, out);
                        }
                        PrintItem::Num(e) => {
                            heading(out, "Num");
                            expr(p, e, d + 2, out);
                        }
                        PrintItem::Zone => heading(out, "Zone"),
                    }
                }
            }
            StmtKind::If { branches, else_ } => {
                line(out, "If");
                for (i, b) in branches.iter().enumerate() {
                    heading(out, if i == 0 { "If" } else { "ElseIf" });
                    expr(p, &b.cond, d + 2, out);
                    stmts(p, &b.body, d + 2, out);
                }
                if let Some(body) = else_ {
                    heading(out, "Else");
                    stmts(p, body, d + 2, out);
                }
            }
            StmtKind::For {
                var,
                temp,
                start,
                end,
                step,
                body,
                end_line,
            } => {
                let v = p.var(*var);
                line(
                    out,
                    &format!(
                        "For {}:{} counting in {} (NEXT line {end_line})",
                        var_name(p, *var),
                        ty(v.ty),
                        ty(*temp)
                    ),
                );
                heading(out, "From");
                expr(p, start, d + 2, out);
                heading(out, "To");
                expr(p, end, d + 2, out);
                if let Some(step) = step {
                    heading(out, "Step");
                    expr(p, step, d + 2, out);
                }
                heading(out, "Body");
                stmts(p, body, d + 2, out);
            }
            StmtKind::Do { test, body, end_line } => {
                line(out, &format!("Do (LOOP line {end_line})"));
                if let Some(t) = test {
                    let word = if t.until { "Until" } else { "While" };
                    heading(out, &format!("{word} at {:?}", t.at));
                    expr(p, &t.cond, d + 2, out);
                }
                heading(out, "Body");
                stmts(p, body, d + 2, out);
            }
            StmtKind::While { cond, body, end_line } => {
                line(out, &format!("While (WEND line {end_line})"));
                heading(out, "Cond");
                expr(p, cond, d + 2, out);
                heading(out, "Body");
                stmts(p, body, d + 2, out);
            }
        }
    }
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
