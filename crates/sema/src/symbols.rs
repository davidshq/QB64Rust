//! The symbol table (design D12 of `m2-procedures-and-errors`): every resolved name with its definition and
//! references, and a lookup from a byte position to the symbol named there. Spans are those of the name token,
//! suffix included. Variables of every storage class, procedures, labels and constants are recorded.

use crate::consteval::Value;
use crate::{ConstId, LabelId, MemberId, ProcId, ProcKind, Program, Storage, TypeId, VarId};
use qb64rust_base::show_bytes;
use qb64rust_base::{FileId, SourceMap, Span, to_u32};
use std::collections::HashMap;
use std::fmt::Write as _;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SymbolId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Var(VarId),
    Proc(ProcId),
    Label(LabelId),
    Const(ConstId),
    /// A user type (design D3 of `m5b-formatter`): defined by the name of its `TYPE` line, referenced by each `AS`
    /// clause that names it. `_MEM`, which no `TYPE` line defines, is not recorded.
    Type(TypeId),
    /// A member of a user type: defined by its name in the `TYPE` block, referenced by each access that names it
    /// (the part of a dotted name after its dot, or the name after `.` of `a(1).m`).
    Member(TypeId, MemberId),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub kind: SymbolKind,
    /// The name in the `DIM`, `CONST` or procedure header, the label's definition, or the first use of an implicit
    /// variable.
    pub def: Span,
    /// The other uses, in source order.
    pub refs: Vec<Span>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Symbols {
    list: Vec<Symbol>,
    /// Every definition and reference, sorted by (file, start).
    by_pos: Vec<(Span, SymbolId)>,
    by_kind: HashMap<SymbolKind, SymbolId>,
}

impl Symbols {
    pub fn get(&self, id: SymbolId) -> &Symbol {
        &self.list[id.0 as usize]
    }

    pub fn iter(&self) -> impl Iterator<Item = (SymbolId, &Symbol)> {
        self.list.iter().enumerate().map(|(i, s)| (SymbolId(to_u32(i)), s))
    }

    /// The symbol whose name covers `offset` (from its first byte up to, not including, its end).
    pub fn at(&self, file: FileId, offset: u32) -> Option<SymbolId> {
        let i = self
            .by_pos
            .partition_point(|(s, _)| (s.file, s.start) <= (file, offset));
        let (span, id) = *self.by_pos.get(i.checked_sub(1)?)?;
        (span.file == file && offset < span.end).then_some(id)
    }

    /// Every definition and reference that starts inside `span`, in order: the name of a dotted token and its
    /// members (design D3 of `m5b-formatter`).
    pub fn within(&self, span: Span) -> impl Iterator<Item = (Span, SymbolId)> + '_ {
        let from = self
            .by_pos
            .partition_point(|(s, _)| (s.file, s.start) < (span.file, span.start));
        self.by_pos[from..]
            .iter()
            .take_while(move |(s, _)| s.file == span.file && s.start < span.end)
            .copied()
    }

    /// Records the names one statement resolved. The first name of a symbol not seen before becomes its
    /// definition; names are taken in source order, so for `x = x + 1` the target is the definition although the
    /// value is resolved first.
    pub(crate) fn record(&mut self, mut names: Vec<(SymbolKind, Span)>) {
        names.sort_by_key(|(_, s)| (s.file, s.start));
        for (kind, span) in names {
            let id = match self.by_kind.get(&kind) {
                Some(&id) => {
                    self.list[id.0 as usize].refs.push(span);
                    id
                }
                None => {
                    let id = SymbolId(to_u32(self.list.len()));
                    self.list.push(Symbol {
                        kind,
                        def: span,
                        refs: Vec::new(),
                    });
                    self.by_kind.insert(kind, id);
                    id
                }
            };
            let at = self
                .by_pos
                .partition_point(|(s, _)| (s.file, s.start) <= (span.file, span.start));
            self.by_pos.insert(at, (span, id));
        }
    }
}

/// One line per symbol: kind, name and type, then definition and references as `line:column`.
pub fn dump_symbols(p: &Program, map: &SourceMap) -> String {
    let pos = |s: Span| {
        let (l, c) = map.file(s.file).line_col(s.start);
        format!("{l}:{c}")
    };
    let mut out = String::new();
    for (_, s) in p.symbols.iter() {
        match s.kind {
            SymbolKind::Var(v) => {
                let v = p.var(v);
                let storage = match v.storage {
                    Storage::Main => String::new(),
                    Storage::Static(q) => format!(" (static in {})", p.proc(q).name),
                    Storage::Local(q) => format!(" (local in {})", p.proc(q).name),
                    Storage::Param(q) => format!(" (param of {})", p.proc(q).name),
                    Storage::Result(q) => format!(" (result of {})", p.proc(q).name),
                };
                let shape = if v.is_array() { "()" } else { "" };
                let t = p.type_name(v.ty);
                write!(out, "Var {}{shape} : {t}{storage} def {}", v.name, pos(s.def)).unwrap();
            }
            SymbolKind::Proc(q) => {
                let q = p.proc(q);
                let kind = match q.kind {
                    ProcKind::Sub => "SUB".to_string(),
                    ProcKind::Function(t) => format!("FUNCTION : {}", t.qb_name()),
                };
                write!(out, "Proc {} {kind} def {}", q.name, pos(s.def)).unwrap();
            }
            SymbolKind::Label(l) => {
                let l = p.label(l);
                let owner = l.proc.map_or(String::new(), |q| format!(" (in {})", p.proc(q).name));
                write!(out, "Label {}{owner} def {}", l.name, pos(s.def)).unwrap()
            }
            SymbolKind::Const(c) => {
                let c = p.constant(c);
                let value = match &c.value {
                    Value::Int(i) if c.ty.is_unsigned() => i.cast_unsigned().to_string(),
                    Value::Int(i) => i.to_string(),
                    Value::UInt(u) => u.to_string(),
                    // `_TOSTR$`'s 19 digits.
                    Value::Float(r, _) => r.lo.to_g19(),
                    Value::Str(b) => format!("\"{}\"", show_bytes(b)),
                };
                let owner = c.proc.map_or(String::new(), |q| format!(" (in {})", p.proc(q).name));
                let (name, ty) = (&c.name, c.ty.qb_name());
                write!(out, "Const {name} : {ty} = {value}{owner} def {}", pos(s.def)).unwrap();
            }
            SymbolKind::Type(t) => write!(out, "Type {} def {}", p.user_type(t).name, pos(s.def)).unwrap(),
            SymbolKind::Member(t, m) => {
                let ut = p.user_type(t);
                let member = &ut.members[m.0 as usize];
                let shape = if member.is_array() { "()" } else { "" };
                let ty = p.type_name(member.ty);
                write!(
                    out,
                    "Member {}.{}{shape} : {ty} def {}",
                    ut.name,
                    member.name,
                    pos(s.def)
                )
                .unwrap();
            }
        }
        let refs: Vec<String> = s.refs.iter().map(|&r| pos(r)).collect();
        if !refs.is_empty() {
            write!(out, " refs {}", refs.join(" ")).unwrap();
        }
        out.push('\n');
    }
    out
}
