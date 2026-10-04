//! The symbol table (design D12 of `m2-procedures-and-errors`): every resolved name with its definition and
//! references, and a lookup from a byte position to the symbol named there. Spans are those of the name token,
//! suffix included. Variables of every storage class and procedures are recorded; labels follow.

use crate::{ProcId, ProcKind, Program, Storage, VarId};
use qb64rust_base::{FileId, SourceFile, Span};
use std::collections::HashMap;
use std::fmt::Write as _;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SymbolId(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SymbolKind {
    Var(VarId),
    Proc(ProcId),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub kind: SymbolKind,
    /// The name in the `DIM` or procedure header, or the first use of an implicit variable.
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
        self.list.iter().enumerate().map(|(i, s)| (SymbolId(i as u32), s))
    }

    /// The symbol whose name covers `offset` (from its first byte up to, not including, its end).
    pub fn at(&self, file: FileId, offset: u32) -> Option<SymbolId> {
        let i = self
            .by_pos
            .partition_point(|(s, _)| (s.file, s.start) <= (file, offset));
        let (span, id) = *self.by_pos.get(i.checked_sub(1)?)?;
        (span.file == file && offset < span.end).then_some(id)
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
                    let id = SymbolId(self.list.len() as u32);
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
pub fn dump_symbols(p: &Program, file: &SourceFile) -> String {
    let pos = |s: Span| {
        let (l, c) = file.line_col(s.start);
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
                write!(out, "Var {} : {}{storage} def {}", v.name, v.ty.qb_name(), pos(s.def)).unwrap();
            }
            SymbolKind::Proc(q) => {
                let q = p.proc(q);
                let kind = match q.kind {
                    ProcKind::Sub => "SUB".to_string(),
                    ProcKind::Function(t) => format!("FUNCTION : {}", t.qb_name()),
                };
                write!(out, "Proc {} {kind} def {}", q.name, pos(s.def)).unwrap();
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
