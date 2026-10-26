//! The language server's semantic index (`qb64rust_lsp::index`, design D1 of `m5b-language-features`): `sema`'s
//! symbol table turned into plain data, with each symbol's hover text, written from the same facts as
//! `qb64rust_sema::dump_symbols`.

use qb64rust_base::{Diagnostics, FileId, Severity, SourceMap, Span, show_bytes, to_u32};
use qb64rust_lsp::index::{Body, Entry, EntryKind, FirstError, SymbolIndex};
use qb64rust_sema::SymbolKind;
use qb64rust_sema::consteval::Value;
use qb64rust_sema::{ProcId, ProcKind, Program, Storage, Ty, TypeId, VarId};
use std::collections::HashMap;
use std::fmt::Write as _;

/// The index of a checked program (with or without errors): one entry per symbol of its table, in the table's order.
pub fn symbol_index(map: &SourceMap, p: &Program, diagnostics: &Diagnostics) -> SymbolIndex {
    let text = |s: Span| show_bytes(map.text(s));
    // The names as their definitions write them, for the texts that name another symbol.
    let mut procs: HashMap<ProcId, String> = HashMap::new();
    let mut vars: HashMap<VarId, String> = HashMap::new();
    let mut types: HashMap<TypeId, usize> = HashMap::new();
    let mut type_names: HashMap<TypeId, String> = HashMap::new();
    let mut members: HashMap<(TypeId, u32), String> = HashMap::new();
    for (i, (_, s)) in p.symbols.iter().enumerate() {
        match s.kind {
            SymbolKind::Proc(q) => {
                procs.insert(q, text(s.def));
            }
            SymbolKind::Var(v) => {
                vars.insert(v, text(s.def));
            }
            SymbolKind::Type(t) => {
                types.insert(t, i);
                type_names.insert(t, text(s.def));
            }
            SymbolKind::Member(t, m) => {
                members.insert((t, m.0), text(s.def));
            }
            SymbolKind::Label(_) | SymbolKind::Const(_) => {}
        }
    }
    let proc_name = |q: ProcId| procs.get(&q).cloned().unwrap_or_else(|| p.proc(q).name.clone());
    let proc_kind = |q: ProcId| match p.proc(q).kind {
        ProcKind::Sub => "SUB",
        ProcKind::Function(_) => "FUNCTION",
    };
    let in_proc = |q: ProcId| format!("`{} {}`", proc_kind(q), proc_name(q));
    let type_of = |t: Ty| match t {
        Ty::User(id) => types.get(&id).copied(),
        _ => None,
    };
    // A user type as its `TYPE` line writes it.
    let ty_name = |t: Ty| match t {
        Ty::User(id) => type_names.get(&id).cloned().unwrap_or_else(|| p.type_name(t)),
        _ => p.type_name(t),
    };

    let bodies: Vec<Body> = p
        .procs
        .iter()
        .enumerate()
        .map(|(i, q)| body(map, q, proc_name(ProcId(to_u32(i)))))
        .collect();
    let mut entries = Vec::new();
    // Each FUNCTION's entry, and each result variable's entry with its FUNCTION, to link the two.
    let mut proc_entries: HashMap<ProcId, usize> = HashMap::new();
    let mut results: Vec<(usize, ProcId)> = Vec::new();
    for (_, s) in p.symbols.iter() {
        let name = text(s.def);
        let (kind, detail, scope, shared, user_type) = match s.kind {
            SymbolKind::Var(v) => {
                let var = p.var(v);
                let shape = if var.is_array() { "()" } else { "" };
                let (scope, place) = match var.storage {
                    Storage::Main if p.shared.contains(&v) => (None, "Shared by the main module.".to_string()),
                    Storage::Main => (None, "In the main module.".to_string()),
                    Storage::Static(q) => (Some(q), format!("Static in {}.", in_proc(q))),
                    Storage::Local(q) => (Some(q), format!("Local to {}.", in_proc(q))),
                    Storage::Param(q) => (Some(q), format!("Parameter of {}.", in_proc(q))),
                    Storage::Result(q) => (Some(q), format!("Result of {}.", in_proc(q))),
                };
                let detail = format!("```qb64\n{name}{shape} AS {}\n```\n{place}", ty_name(var.ty));
                let shared = var.storage == Storage::Main && p.shared.contains(&v);
                (
                    EntryKind::Variable,
                    detail,
                    scope.map(|q| q.0 as usize),
                    shared,
                    type_of(var.ty),
                )
            }
            SymbolKind::Proc(q) => {
                let detail = format!("```qb64\n{}\n```", header(p, q, &name, &vars, &ty_name));
                (EntryKind::Procedure, detail, None, false, None)
            }
            SymbolKind::Label(l) => {
                let label = p.label(l);
                let (what, colon) = if label.line_number.is_some() {
                    ("Line number", "")
                } else {
                    ("Label", ":")
                };
                let place = label.proc.map_or("the main module".to_string(), in_proc);
                let detail = format!("```qb64\n{name}{colon}\n```\n{what} in {place}.");
                (EntryKind::Label, detail, label.proc.map(|q| q.0 as usize), false, None)
            }
            SymbolKind::Const(c) => {
                let c = p.constant(c);
                let value = match &c.value {
                    Value::Int(i) if c.ty.is_unsigned() => i.cast_unsigned().to_string(),
                    Value::Int(i) => i.to_string(),
                    Value::UInt(u) => u.to_string(),
                    Value::Float(r, _) => r.lo.to_g19(),
                    Value::Str(b) => format!("\"{}\"", show_bytes(b)),
                };
                let place = c.proc.map_or("the main module".to_string(), in_proc);
                let detail = format!(
                    "```qb64\nCONST {name} = {value}\n```\nConstant, `{}`, in {place}.",
                    c.ty.qb_name()
                );
                (EntryKind::Constant, detail, c.proc.map(|q| q.0 as usize), false, None)
            }
            SymbolKind::Type(t) => {
                let ut = p.user_type(t);
                let mut block = format!("```qb64\nTYPE {name}\n");
                for (i, m) in ut.members.iter().enumerate() {
                    let written = members.get(&(t, to_u32(i))).cloned().unwrap_or_else(|| m.name.clone());
                    let shape = if m.is_array() { "()" } else { "" };
                    writeln!(block, "    {written}{shape} AS {}", ty_name(m.ty)).unwrap();
                }
                block.push_str("END TYPE\n```");
                (EntryKind::Type, block, None, false, None)
            }
            SymbolKind::Member(t, m) => {
                let member = &p.user_type(t).members[m.0 as usize];
                let owner = type_names
                    .get(&t)
                    .cloned()
                    .unwrap_or_else(|| p.user_type(t).name.clone());
                let shape = if member.is_array() { "()" } else { "" };
                let detail = format!(
                    "```qb64\n{owner}.{name}{shape} AS {}\n```\nMember of `TYPE {owner}`.",
                    ty_name(member.ty)
                );
                // A member points at the type it belongs to; its own type is in its text.
                (EntryKind::Member, detail, None, false, types.get(&t).copied())
            }
        };
        let ty = match s.kind {
            SymbolKind::Var(v) => {
                let var = p.var(v);
                if let Storage::Result(q) = var.storage {
                    results.push((entries.len(), q));
                }
                format!("{}{}", ty_name(var.ty), if var.is_array() { "()" } else { "" })
            }
            SymbolKind::Const(c) => p.constant(c).ty.qb_name().to_string(),
            SymbolKind::Member(t, m) => ty_name(p.user_type(t).members[m.0 as usize].ty),
            SymbolKind::Proc(q) => {
                proc_entries.insert(q, entries.len());
                match p.proc(q).kind {
                    ProcKind::Function(t) => ty_name(t),
                    ProcKind::Sub => String::new(),
                }
            }
            SymbolKind::Label(_) | SymbolKind::Type(_) => String::new(),
        };
        entries.push(Entry {
            kind,
            name,
            detail,
            scope,
            shared,
            user_type,
            library: matches!(s.kind, SymbolKind::Proc(q) if p.proc(q).external.is_some()),
            twin: None,
            ty,
            def: s.def,
            refs: s.refs.clone(),
        });
    }
    for (var, q) in results {
        if let Some(&proc) = proc_entries.get(&q) {
            entries[var].twin = Some(proc);
            entries[proc].twin = Some(var);
        }
    }
    let mut errors: Vec<_> = diagnostics
        .list()
        .iter()
        .filter(|d| d.severity == Severity::Error)
        .collect();
    errors.sort_by_key(|d| (d.span.file.0, d.span.start));
    let error = errors.first().map(|d| FirstError {
        span: d.span,
        message: d.message.clone(),
    });
    SymbolIndex::new(entries, bodies, error)
}

/// A procedure's header: `SUB name (a AS LONG, b() AS STRING)`, a FUNCTION with its result type after.
fn header(p: &Program, q: ProcId, name: &str, vars: &HashMap<VarId, String>, ty_name: &dyn Fn(Ty) -> String) -> String {
    let proc = p.proc(q);
    let params: Vec<String> = proc
        .params
        .iter()
        .map(|&v| {
            let var = p.var(v);
            let written = vars.get(&v).cloned().unwrap_or_else(|| var.name.clone());
            let shape = if var.is_array() { "()" } else { "" };
            format!("{written}{shape} AS {}", ty_name(var.ty))
        })
        .collect();
    let params = if params.is_empty() {
        String::new()
    } else {
        format!(" ({})", params.join(", "))
    };
    let library = if proc.external.is_some() {
        "\n' DECLARE LIBRARY"
    } else {
        ""
    };
    match proc.kind {
        ProcKind::Sub => format!("SUB {name}{params}{library}"),
        ProcKind::Function(t) => format!("FUNCTION {name}{params} AS {}{library}", ty_name(t)),
    }
}

/// The span of a procedure's body: from the start of its header's line to the end of its `END` line, or to the end
/// of the header's file when the `END` stands in another file.
fn body(map: &SourceMap, q: &qb64rust_sema::Proc, name: String) -> Body {
    let file = map.file(q.file);
    let start = file.line_content(q.line.saturating_sub(1)).map_or(0, |(s, _)| s);
    let end = if q.end_file == q.file {
        file.line_content(q.end_line.saturating_sub(1))
            .map_or(to_u32(file.bytes.len()), |(_, e)| e)
    } else {
        to_u32(file.bytes.len())
    };
    Body {
        name,
        span: Span::new(FileId(q.file.0), start, end.max(start)),
    }
}
