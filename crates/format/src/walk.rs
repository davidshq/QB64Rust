//! The lines of a file and what QB64pe's main pass makes of each (design D2): the statements on it in order, the
//! comment after them, and the indent level, then the line in QB64pe's form for [`crate::apply`].
//!
//! The level follows `qb64pe.bas` 12643-12657: a line is indented by `min(lhscontrollevel, controllevel)`, the
//! level when the line starts (less one for each `ELSE`, `ELSEIF`, `CASE` after a `CASE`, `$ELSE` and `$ELSEIF` on
//! it) against the level after it; one more inside `TYPE` and `DECLARE LIBRARY`, but not on their first and last
//! lines. A line on which a statement's handler sets no layout (`layoutok = 0`) is its own text trimmed, at that
//! indent; a line joined to the next by `_`, with the lines it joins, and a line of a `$IF` branch that is not taken,
//! are written as read.

use crate::apply::Apply;
use crate::fmt::Fmt;
use crate::lay::Lay;
use qb64rust_syntax::SyntaxKind::{self, *};
use qb64rust_syntax::tree::{Element, Node, Tok};

/// What a line becomes.
pub(crate) enum Plan {
    /// Written as read.
    AsRead,
    /// QB64pe's `layout$` for the line, its indent level as leading spaces, with the settings at the line.
    Layout(Vec<u8>, Apply),
}

/// One thing a line holds, in source order.
enum Event<'a> {
    /// A statement and the kind of the container it stands in.
    Stmt(Node<'a>, SyntaxKind),
    /// A `:` between statements.
    Colon,
    /// `ELSE` of a block (`block`), or of a single-line `IF`.
    Else { block: bool },
}

/// The kinds that hold statements; every other node met where a statement may stand is a statement.
fn is_container(k: SyntaxKind) -> bool {
    matches!(
        k,
        SourceFile
            | IfBlock
            | IfBranch
            | ElseIfBranch
            | ElseBranch
            | IfStmt
            | LineBranch
            | ForBlock
            | DoBlock
            | WhileBlock
            | SelectBlock
            | CaseClause
            | ProcDef
            | TypeBlock
            | DeclareLibraryBlock
            | DefFnBlock
    )
}

/// The state of the main pass that carries from line to line.
pub(crate) struct Pass {
    /// `controllevel`.
    pub level: i32,
    /// `definingtype`: 1 on the `TYPE` line, 2 after it, 0 outside.
    pub defining_type: u8,
    /// `declaringlibrary`, likewise.
    pub declaring_library: u8,
    /// For each open `SELECT CASE`: whether a `CASE` of it is open (`controltype` 18 or 19).
    pub selects: Vec<bool>,
    /// `IDEAutoIndent` and `IDEAutoLayout` as `'$FORMAT:OFF` and `'$FORMAT:ON` leave them.
    pub format_off: bool,
}

/// What the statements of one line do to the level.
#[derive(Default)]
pub(crate) struct LineLevel {
    /// Change of `lhscontrollevel`.
    pub lhs: i32,
}

impl Fmt<'_> {
    /// The plan of every line of the file, from its tree (tree 0 of the program).
    pub(crate) fn plan(&self, root: Node<'_>, lines: &[(u32, u32)]) -> Vec<Plan> {
        let line_of = |offset: u32| lines.partition_point(|&(start, _)| start <= offset).saturating_sub(1);
        let n = lines.len();
        // Trivia and node spans that decide a line before its statements: a comment, a continuation, an inactive
        // branch.
        let mut comment: Vec<Option<Tok>> = vec![None; n];
        let mut continued = vec![false; n];
        for t in root.tokens() {
            let line = line_of(t.span.start);
            match t.kind {
                Comment | MetaComment if comment[line].is_none() => comment[line] = Some(t),
                LineContinuation => continued[line] = true,
                _ => {}
            }
        }
        let mut as_read = vec![false; n];
        // A line joined to the next, and the lines it joins.
        for line in 0..n {
            if continued[line] {
                as_read[line] = true;
                if line + 1 < n {
                    as_read[line + 1] = true;
                }
            }
        }
        let mut inactive = vec![false; n];
        let mut events: Vec<Vec<Event<'_>>> = (0..n).map(|_| Vec::new()).collect();
        self.collect(root, &line_of, &mut events, &mut inactive);

        let mut pass = Pass {
            level: 0,
            defining_type: 0,
            declaring_library: 0,
            selects: Vec::new(),
            format_off: false,
        };
        let mut plans = Vec::with_capacity(n);
        // `linecontinuation`, which only `lineformat$` clears: after a line joined by `_`, a line that does not go
        // through `lineformat$` (an empty line, a metacommand) is written as read too (`qb64pe.bas` 12684).
        let mut stale = false;
        for (i, &(start, end)) in lines.iter().enumerate() {
            let original = &self.map.file(self.file).bytes[start as usize..end as usize];
            let trimmed = trim_spaces(original);
            let upper = trimmed.to_ascii_uppercase();
            // A line of a branch that is not taken is skipped (`InvalidLine`), but for the precompiler's own
            // metacommands and empty lines, which the main pass reads as usual (`qb64pe.bas` 1833-1899).
            let precompiler = inactive[i] && crate::stmt::is_precompiler_line(&upper);
            if inactive[i] && !precompiler && !trimmed.is_empty() {
                plans.push(Plan::AsRead);
                continue;
            }
            // Whether `lineformat$` reads the line: not for an empty line or a metacommand the metacommand handler
            // takes.
            let skips_lineformat = trimmed.is_empty()
                || precompiler
                || (events[i].len() == 1
                    && matches!(events[i][0], Event::Stmt(s, _) if s.kind() == MetaStmt && self.meta(s).is_some()));
            let stale_line = stale && skips_lineformat && !as_read[i];
            if !skips_lineformat {
                stale = false;
            }
            // `'$FORMAT:OFF` and `'$FORMAT:ON` take effect as the line is read, before its statements.
            if let Some(c) = comment[i].filter(|_| !inactive[i]) {
                self.format_switch(c, &mut pass);
            }
            let start_level = pass.level;
            let mut lhs = start_level;
            let mut lay = Lay::new();
            let mut ok = true;
            if precompiler {
                let mut level = LineLevel::default();
                crate::stmt::meta_effect_of(&upper, &mut pass, &mut level);
                lhs += level.lhs;
                match self.meta_line(trimmed, &upper) {
                    Some(l) => {
                        lay.then(&l);
                    }
                    None => ok = false,
                }
            }
            for ev in events[i].iter().filter(|_| !inactive[i]) {
                match ev {
                    Event::Colon => {
                        if lay.is_empty() {
                            lay.text(b":");
                        } else {
                            lay.sp2().text(b":");
                        }
                    }
                    Event::Else { block } => {
                        if *block {
                            lhs -= 1;
                        }
                        lay.then(&Lay(self.kw("ELSE")));
                    }
                    Event::Stmt(s, block) => {
                        let mut level = LineLevel::default();
                        let got = self.statement(*s, *block, &mut pass, &mut level);
                        lhs += level.lhs;
                        match got {
                            Some(l) => {
                                lay.then(&l);
                            }
                            None => ok = false,
                        }
                    }
                }
            }
            if let Some(c) = comment[i].filter(|_| !inactive[i]) {
                lay.then(&self.comment(c));
            }
            let mut x = lhs.min(pass.level);
            if pass.defining_type == 2 {
                x += 1;
            }
            if pass.defining_type > 0 {
                pass.defining_type = 2;
            }
            if pass.declaring_library == 2 {
                x += 1;
            }
            if pass.declaring_library > 0 {
                pass.declaring_library = 2;
            }
            if as_read[i] {
                // The last line of a joined group leaves `linecontinuation` set.
                if !continued[i] {
                    stale = true;
                }
                plans.push(Plan::AsRead);
                continue;
            }
            if stale_line {
                plans.push(Plan::AsRead);
                continue;
            }
            let body = if ok { lay.0 } else { trimmed.to_vec() };
            let mut layout = vec![b' '; usize::try_from(x.max(0)).unwrap_or(0)];
            layout.extend_from_slice(&body);
            let apply = Apply {
                auto_indent: self.settings.auto_indent && !pass.format_off,
                indent_size: self.settings.indent_size,
                auto_layout: self.settings.auto_layout && !pass.format_off,
            };
            plans.push(Plan::Layout(layout, apply));
        }
        plans
    }

    /// The events of a container's children, by line; an inactive branch marks its lines.
    fn collect<'n>(
        &self,
        n: Node<'n>,
        line_of: &dyn Fn(u32) -> usize,
        events: &mut [Vec<Event<'n>>],
        inactive: &mut [bool],
    ) {
        let mut first = true;
        for c in n.children() {
            match c {
                Element::Token(t) => {
                    match t.kind {
                        Colon => events[line_of(t.span.start)].push(Event::Colon),
                        Ident if self.tok(t).eq_ignore_ascii_case(b"ELSE") => {
                            let block = n.kind() == ElseBranch && first;
                            events[line_of(t.span.start)].push(Event::Else { block });
                        }
                        _ => {}
                    }
                    if !t.kind.is_trivia() && t.kind != Newline {
                        first = false;
                    }
                }
                Element::Node(c) => {
                    first = false;
                    if c.kind() == InactiveCode {
                        // The branch's lines: from its first byte to its last that is not a blank or a line end (its
                        // span may take the indent of the metacommand line after it, which is not inactive).
                        let text = self.text(c.span());
                        let last = text.iter().rposition(|b| !b.is_ascii_whitespace()).unwrap_or(0);
                        let (a, b) = (
                            line_of(c.span().start),
                            line_of(c.span().start + qb64rust_base::to_u32(last)),
                        );
                        for l in inactive.iter_mut().take(b + 1).skip(a) {
                            *l = true;
                        }
                    } else if is_container(c.kind()) {
                        self.collect(c, line_of, events, inactive);
                    } else {
                        let start = c.first_token().map_or(c.span().start, |t| t.span.start);
                        events[line_of(start)].push(Event::Stmt(c, n.kind()));
                    }
                }
            }
        }
    }

    /// A comment after the statements (`lineformat$`, `qb64pe.bas` 25427-25500): `'` or `REM` and the rest of the
    /// line with its trailing blanks left out; in a comment that starts with `$`, the metacommands it names in the
    /// keyword style.
    fn comment(&self, t: Tok) -> Lay {
        let text = self.tok(t);
        let (lead, rest) = if text.first() == Some(&b'\'') {
            (b"'".to_vec(), &text[1..])
        } else {
            (self.kw("REM"), &text[3.min(text.len())..])
        };
        let mut c = lead;
        c.extend_from_slice(rest);
        while c.last().is_some_and(|&b| b == b' ') {
            c.pop();
        }
        if t.kind == MetaComment {
            self.metacomment_words(&mut c);
        }
        Lay(c)
    }

    /// `'$FORMAT:OFF` and `'$FORMAT:ON` in a comment (`qb64pe.bas` 25454-25459).
    fn format_switch(&self, t: Tok, pass: &mut Pass) {
        if t.kind != MetaComment {
            return;
        }
        let upper = self.tok(t).to_ascii_uppercase();
        let mut x = 0;
        while let Some(p) = upper[x..].iter().position(|&c| c == b'$') {
            let at = &upper[x + p..];
            if at.starts_with(b"$FORMAT:OFF") {
                pass.format_off = true;
            } else if at.starts_with(b"$FORMAT:ON") {
                pass.format_off = false;
            }
            x += p + 1;
        }
    }

    /// The metacommand words of a comment in the keyword style, at each `$` as `lineformat$` finds them.
    fn metacomment_words(&self, c: &mut [u8]) {
        let upper = c.to_ascii_uppercase();
        let mut x = 0;
        while let Some(p) = upper[x..].iter().position(|&b| b == b'$') {
            let at = x + p;
            let tail = &upper[at..];
            let word: Option<&str> = if tail.starts_with(b"$FORMAT:OFF") {
                Some("$FORMAT:OFF")
            } else if tail.starts_with(b"$FORMAT:ON") {
                Some("$FORMAT:ON")
            } else if tail.starts_with(b"$STATIC") {
                Some("$STATIC")
            } else if tail.starts_with(b"$DYNAMIC") {
                Some("$DYNAMIC")
            } else if tail.starts_with(b"$INCLUDE") && !tail[8..].starts_with(b"ONCE") {
                Some("$INCLUDE")
            } else {
                None
            };
            if let Some(w) = word {
                let spelled = self.kw(w);
                c[at..at + spelled.len()].copy_from_slice(&spelled);
            }
            x = at + 1;
        }
    }
}

/// A line without the spaces at its ends (`_TRIM$`).
fn trim_spaces(b: &[u8]) -> &[u8] {
    let start = b.iter().position(|&c| c != b' ').unwrap_or(b.len());
    let end = b.iter().rposition(|&c| c != b' ').map_or(start, |e| e + 1);
    &b[start..end]
}
