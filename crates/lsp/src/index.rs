//! The semantic index (OpenSpec change `m5b-language-features`, design D1): what the whole front end knows of a
//! program's names, as plain data the driver's [`crate::Index`] hook makes, so that this crate needs no `sema`.
//! Hover, references, rename, completion and the definition of names other than procedures and labels are answered
//! from it.

use crate::analysis::Analysis;
use qb64rust_base::{FileId, SourceMap, Span};
use std::fmt::Write as _;

/// A program's semantic index, with the parse its spans belong to.
pub struct Indexed {
    pub analysis: Analysis,
    pub index: SymbolIndex,
}

/// What a name is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntryKind {
    Variable,
    Constant,
    Procedure,
    Label,
    Type,
    Member,
}

/// One symbol of the program: its definition and uses, as the front end resolved them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub kind: EntryKind,
    /// The name as the definition writes it, suffix included (`count%`, `bump`, `Point`).
    pub name: String,
    /// The hover's Markdown.
    pub detail: String,
    /// The body the symbol belongs to (an index into [`SymbolIndex::bodies`]); `None` for the main module, and for
    /// procedures, types and members, which every body sees.
    pub scope: Option<usize>,
    /// A main-module variable declared `DIM SHARED` or `COMMON SHARED`: every body sees it.
    pub shared: bool,
    /// For a variable or member of a user type: that type's entry (an index into [`SymbolIndex::entries`]). For a
    /// member: the entry of the type it belongs to.
    pub user_type: Option<usize>,
    /// A procedure of a `DECLARE LIBRARY` block: its name is the C function's unless an `ALIAS` names that, so it
    /// is not renamed.
    pub library: bool,
    /// A FUNCTION and its result variable: one name, renamed together (the other's entry).
    pub twin: Option<usize>,
    /// The type, as the hover writes it: of a variable, constant or member, and of a FUNCTION's result; empty for the
    /// rest. A rename must not change it (a `DEFtype` gives a name's first letter its type).
    pub ty: String,
    /// The name token of the definition: the name in its `DIM`, `CONST`, parameter list, header or `TYPE` block, the
    /// label, or the first use of a variable that is not declared.
    pub def: Span,
    /// The other uses, in source order.
    pub refs: Vec<Span>,
}

impl Entry {
    /// The definition and every use, in source order.
    pub fn uses(&self) -> impl Iterator<Item = Span> + '_ {
        std::iter::once(self.def).chain(self.refs.iter().copied())
    }
}

/// A procedure's body: from its header to its `END SUB` or `END FUNCTION` (to the end of the header's file when the
/// end stands in another file).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Body {
    /// The procedure's name as its header writes it.
    pub name: String,
    pub span: Span,
}

/// The first error the front end reported: where it is and its message.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirstError {
    pub span: Span,
    pub message: String,
}

/// Every symbol of a program, and a lookup from a position to the symbol named there.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SymbolIndex {
    pub entries: Vec<Entry>,
    pub bodies: Vec<Body>,
    pub error: Option<FirstError>,
    /// Every definition and use, sorted by (file, start), with its entry.
    by_pos: Vec<(Span, usize)>,
}

impl SymbolIndex {
    /// The index of `entries`, `bodies` and the first error, if any.
    pub fn new(entries: Vec<Entry>, bodies: Vec<Body>, error: Option<FirstError>) -> SymbolIndex {
        let mut by_pos: Vec<(Span, usize)> = entries
            .iter()
            .enumerate()
            .flat_map(|(i, e)| e.uses().map(move |s| (s, i)))
            .collect();
        by_pos.sort_by_key(|(s, _)| (s.file, s.start));
        SymbolIndex {
            entries,
            bodies,
            error,
            by_pos,
        }
    }

    /// The entry whose name covers `offset` (from its first byte up to, not including, its end), as
    /// `qb64rust_sema::Symbols::at` finds it.
    pub fn entry_at(&self, file: FileId, offset: u32) -> Option<(usize, Span)> {
        let i = self
            .by_pos
            .partition_point(|(s, _)| (s.file, s.start) <= (file, offset));
        let (span, id) = *self.by_pos.get(i.checked_sub(1)?)?;
        (span.file == file && offset < span.end).then_some((id, span))
    }

    /// The start of the first definition or use after `offset` in `file`: a member inside a dotted name's token.
    pub fn next_start(&self, file: FileId, offset: u32) -> Option<u32> {
        let i = self
            .by_pos
            .partition_point(|(s, _)| (s.file, s.start) <= (file, offset));
        self.by_pos.get(i).filter(|(s, _)| s.file == file).map(|(s, _)| s.start)
    }

    /// [`SymbolIndex::entry_at`] at `offset`, or just before it: the cursor just after a name names it.
    pub fn entry_near(&self, file: FileId, offset: u32) -> Option<(usize, Span)> {
        self.entry_at(file, offset)
            .or_else(|| self.entry_at(file, offset.checked_sub(1)?))
    }

    /// The body that holds `offset` of `file`, if any.
    pub fn body_at(&self, file: FileId, offset: u32) -> Option<usize> {
        self.bodies
            .iter()
            .position(|b| b.span.file == file && b.span.start <= offset && offset < b.span.end)
    }

    /// One block per entry for tests: kind, name, body, `shared`, user type, definition and uses as `line:column`
    /// (with the file's name when it is not the main file), then the hover text indented; then the bodies and the
    /// first error.
    pub fn dump(&self, map: &SourceMap) -> String {
        let pos = |s: Span| {
            let f = map.file(s.file);
            let (l, c) = f.line_col(s.start);
            if s.file.0 == 0 {
                format!("{l}:{c}")
            } else {
                format!("{}:{l}:{c}", f.name.replace('\\', "/"))
            }
        };
        let mut out = String::new();
        for e in &self.entries {
            let mut line = format!("{:?} {}", e.kind, e.name);
            if let Some(b) = e.scope {
                write!(line, " in {}", self.bodies[b].name).unwrap();
            }
            if e.shared {
                line.push_str(" shared");
            }
            if let Some(t) = e.user_type {
                write!(line, " type {}", self.entries[t].name).unwrap();
            }
            write!(line, " def {}", pos(e.def)).unwrap();
            if !e.refs.is_empty() {
                let refs: Vec<String> = e.refs.iter().map(|&r| pos(r)).collect();
                write!(line, " refs {}", refs.join(" ")).unwrap();
            }
            out.push_str(&line);
            out.push('\n');
            for l in e.detail.lines() {
                writeln!(out, "    {l}").unwrap();
            }
        }
        for b in &self.bodies {
            let end = map.file(b.span.file).line_col(b.span.end).0;
            writeln!(out, "Body {} {}-{end}", b.name, pos(b.span)).unwrap();
        }
        if let Some(e) = &self.error {
            writeln!(out, "Error {}: {}", pos(e.span), e.message).unwrap();
        }
        out
    }

    /// The entry `entries[id]` is seen from body `body` (`None`: the main module): one of that body, a shared one,
    /// one every body sees (a procedure, a type, a member, a constant of the main module), or a variable of the main
    /// module that a use inside the body names (a `SHARED` statement).
    pub fn visible_in(&self, id: usize, body: Option<usize>) -> bool {
        let e = &self.entries[id];
        let global = match e.kind {
            EntryKind::Procedure | EntryKind::Type | EntryKind::Member => true,
            EntryKind::Constant => e.scope.is_none(),
            EntryKind::Variable | EntryKind::Label => false,
        };
        if e.scope == body || e.shared || global {
            return true;
        }
        match (e.scope, body) {
            (None, Some(b)) if e.kind == EntryKind::Variable => {
                let span = self.bodies[b].span;
                e.uses()
                    .any(|s| s.file == span.file && span.start <= s.start && s.start < span.end)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span(file: u32, start: u32, end: u32) -> Span {
        Span {
            file: FileId(file),
            start,
            end,
        }
    }

    fn var(name: &str, scope: Option<usize>, def: Span, refs: Vec<Span>) -> Entry {
        Entry {
            kind: EntryKind::Variable,
            name: name.into(),
            detail: String::new(),
            scope,
            shared: false,
            user_type: None,
            library: false,
            twin: None,
            ty: "SINGLE".into(),
            def,
            refs,
        }
    }

    #[test]
    fn lookup_by_position() {
        let ix = SymbolIndex::new(
            vec![
                var("a", None, span(0, 10, 11), vec![span(1, 4, 6), span(0, 30, 31)]),
                var("bb", None, span(0, 20, 22), vec![]),
            ],
            Vec::new(),
            None,
        );
        assert_eq!(ix.entry_at(FileId(0), 10), Some((0, span(0, 10, 11))));
        assert_eq!(ix.entry_at(FileId(0), 11), None);
        assert_eq!(ix.entry_near(FileId(0), 11), Some((0, span(0, 10, 11))));
        assert_eq!(ix.entry_at(FileId(0), 21), Some((1, span(0, 20, 22))));
        assert_eq!(ix.entry_at(FileId(1), 5), Some((0, span(1, 4, 6))));
        assert_eq!(ix.entry_at(FileId(1), 3), None);
    }

    #[test]
    fn what_a_body_sees() {
        let body = Body {
            name: "a".into(),
            span: span(0, 100, 200),
        };
        let mut shared = var("s", None, span(0, 0, 1), vec![]);
        shared.shared = true;
        let ix = SymbolIndex::new(
            vec![
                var("main", None, span(0, 2, 3), vec![]),
                var("named", None, span(0, 4, 5), vec![span(0, 150, 155)]),
                shared,
                var("local", Some(0), span(0, 120, 125), vec![]),
            ],
            vec![body],
            None,
        );
        assert_eq!(ix.body_at(FileId(0), 150), Some(0));
        assert_eq!(ix.body_at(FileId(0), 50), None);
        let seen: Vec<bool> = (0..4).map(|i| ix.visible_in(i, Some(0))).collect();
        assert_eq!(seen, [false, true, true, true]);
        let seen: Vec<bool> = (0..4).map(|i| ix.visible_in(i, None)).collect();
        assert_eq!(seen, [true, true, true, false]);
    }
}
