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
use qb64rust_base::SourceMap;
use qb64rust_base::layout::Rewritable;
use qb64rust_syntax::SyntaxKind::{self, *};
use qb64rust_syntax::tree::{Element, Node, Tok};

/// What a line becomes.
pub(crate) enum Plan {
    /// Written as read.
    AsRead,
    /// The output lines of the line: QB64pe's `layout$` for each, its indent level as leading spaces, with the
    /// settings at the line. One for a line the rewrites leave alone (OpenSpec change `format-statement-lines`,
    /// design D2).
    Layout(Vec<(Vec<u8>, Apply)>),
}

/// One thing a line holds, in source order.
#[derive(Clone, Copy)]
enum Event<'a> {
    /// A statement and the kind of the container it stands in.
    Stmt(Node<'a>, SyntaxKind),
    /// A `:` between statements.
    Colon,
    /// `ELSE` of a block (`block`), or of a single-line `IF`.
    Else { block: bool },
    /// The end of a single-line `IF`, after the events of its branches.
    LineIfEnd,
}

/// One part of an output line of a rewritten line.
#[derive(Clone, Copy)]
enum Item {
    /// The event of that index on the line.
    Ev(usize),
    /// The `END IF` of a single-line `IF` written as a block.
    EndIf,
}

/// What the scan of a file's tokens and tree gives for each line, before the main pass.
struct Scan<'a> {
    /// The first comment on the line.
    comment: Vec<Option<Tok>>,
    /// The line ends in `_`.
    continued: Vec<bool>,
    /// Joined to the next line, or joined to by the one before.
    as_read: Vec<bool>,
    /// A line of a `$IF` branch that is not taken.
    inactive: Vec<bool>,
    events: Vec<Vec<Event<'a>>>,
}

/// The scan of the file whose tree is `root`.
fn scan<'a>(map: &SourceMap, root: Node<'a>, lines: &[(u32, u32)]) -> Scan<'a> {
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
    collect(map, root, &line_of, &mut events, &mut inactive);
    Scan {
        comment,
        continued,
        as_read,
        inactive,
        events,
    }
}

/// Whether the rewrites may touch line `i` (design D6), as far as the tree tells: not a line joined by `_`, a line
/// of a branch that is not taken, a line between `'$FORMAT:OFF` and `'$FORMAT:ON` (`format_off`), or a line with a
/// metacommand as a statement or in its comment.
fn may_rewrite(s: &Scan<'_>, i: usize, format_off: bool) -> bool {
    !s.as_read[i]
        && !s.inactive[i]
        && !format_off
        && s.comment[i].is_none_or(|c| c.kind != MetaComment)
        && !s.events[i]
            .iter()
            .any(|e| matches!(e, Event::Stmt(n, _) if matches!(n.kind(), MetaStmt | MetaCommentStmt)))
}

/// What the rewrites can do to each line of the file whose tree is `root` (design D8): from the tree alone, with
/// no check, as the language server lists its code actions. The settings are not read: a line that `AutoIndent` or
/// `AutoLayout` off, or an incomplete program, would leave alone is still named.
pub(crate) fn rewritable(map: &SourceMap, root: Node<'_>, lines: &[(u32, u32)]) -> Vec<Rewritable> {
    let s = scan(map, root, lines);
    let mut format_off = false;
    (0..lines.len())
        .map(|i| {
            if let Some(c) = s.comment[i].filter(|_| !s.inactive[i]) {
                format_switch(map, c, &mut format_off);
            }
            if !may_rewrite(&s, i, format_off) {
                return Rewritable::default();
            }
            let events = &s.events[i];
            Rewritable {
                split: segments(map, events, true, false).is_some(),
                block_if: events
                    .iter()
                    .any(|e| matches!(e, Event::Stmt(n, IfStmt) if n.kind() == IfHeader)),
            }
        })
        .collect()
}

/// Whether a `LabelDef` is a built-in statement's name and a colon (`CLS:`), which `sema` reads as the statement
/// whatever the jumps (spec `language/control-flow`): the name is a SUB of the built-ins table.
fn builtin_label(map: &SourceMap, n: Node<'_>) -> bool {
    n.kind() == LabelDef
        && n.child_tokens().find(|t| t.kind == Ident).is_some_and(|t| {
            let text = map.text(t.span);
            qb64rust_builtins::find_any(&text[..crate::fmt::name_len(text)])
                .any(|b| b.kind == qb64rust_builtins::Kind::Sub)
        })
}

/// The output lines of a rewritten line (design D3 and D4): the line's events cut where the settings say, each
/// output line as its items. `None` when no cut is made, so that the line is written as without the settings.
fn segments(map: &SourceMap, events: &[Event<'_>], split: bool, block_if: bool) -> Option<Vec<Vec<Item>>> {
    let mut segs: Vec<Vec<Item>> = vec![Vec::new()];
    let cut = |segs: &mut Vec<Vec<Item>>| {
        if segs.last().is_some_and(|s| !s.is_empty()) {
            segs.push(Vec::new());
        }
    };
    // How many single-line `IF`s the event is inside. Without `BlockIf` nothing inside one is cut: the `IF` is one
    // statement, whatever its branches hold (a colon in a `FOR` of a branch too).
    let mut depth = 0;
    // A statement stands before on the line: a line number or a label does not count, so that it stays with the
    // first statement.
    let mut seen = false;
    for (k, ev) in events.iter().enumerate() {
        let free = depth == 0 || block_if;
        match *ev {
            Event::Colon => {
                segs.last_mut().expect("one segment").push(Item::Ev(k));
                if split && free && seen {
                    cut(&mut segs);
                }
            }
            Event::Stmt(n, container) => {
                segs.last_mut().expect("one segment").push(Item::Ev(k));
                let builtin = builtin_label(map, n);
                if !matches!(n.kind(), LineNumber | LabelDef) || builtin {
                    seen = true;
                }
                match (n.kind(), container) {
                    (IfHeader, IfStmt) => {
                        depth += 1;
                        if block_if {
                            cut(&mut segs);
                        }
                    }
                    (CaseHeader, _) | (IfHeader, ElseIfBranch) if split && free => cut(&mut segs),
                    (LabelDef, _) if builtin && split && free => cut(&mut segs),
                    _ => {}
                }
            }
            Event::Else { block: true } => {
                segs.last_mut().expect("one segment").push(Item::Ev(k));
                if split && free {
                    cut(&mut segs);
                }
            }
            Event::Else { block: false } => {
                if block_if {
                    cut(&mut segs);
                }
                segs.last_mut().expect("one segment").push(Item::Ev(k));
                if block_if {
                    cut(&mut segs);
                }
            }
            Event::LineIfEnd => {
                depth -= 1;
                if block_if {
                    cut(&mut segs);
                    segs.last_mut().expect("one segment").push(Item::EndIf);
                    cut(&mut segs);
                }
            }
        }
    }
    // A colon at the start or the end of an output line is dropped.
    let colon = |i: &Item| matches!(i, Item::Ev(k) if matches!(events[*k], Event::Colon));
    for s in &mut segs {
        while s.last().is_some_and(colon) {
            s.pop();
        }
        let lead = s.iter().take_while(|i| colon(i)).count();
        s.drain(..lead);
    }
    segs.retain(|s| !s.is_empty());
    (segs.len() > 1).then_some(segs)
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
#[derive(Clone)]
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

/// What the rewrites do to one output line's events: none on a line they leave alone.
#[derive(Clone, Copy, Default)]
struct Rewrite {
    /// The line is cut: a built-in statement's name at the end of an output line loses its colon.
    cut: bool,
    /// Single-line `IF`s are written as blocks.
    block_if: bool,
}

impl Fmt<'_> {
    /// The plan of every line of the file, from its tree (tree 0 of the program).
    pub(crate) fn plan(&self, root: Node<'_>, lines: &[(u32, u32)]) -> Vec<Plan> {
        let n = lines.len();
        let s = scan(self.map, root, lines);
        let (comment, continued, as_read, inactive, events) =
            (&s.comment, &s.continued, &s.as_read, &s.inactive, &s.events);
        // The rewrites (OpenSpec change `format-statement-lines`): only in a complete program (design D7), and not
        // while the source line's indent or spacing is kept, since a new line has no source line (design D6).
        let settings = self.settings;
        let rewrites = (settings.one_statement_per_line || settings.block_if)
            && self.complete
            && settings.auto_indent
            && settings.auto_layout;

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
                format_switch(self.map, c, &mut pass.format_off);
            }
            let apply = Apply {
                auto_indent: settings.auto_indent && !pass.format_off,
                indent_size: settings.indent_size,
                auto_layout: settings.auto_layout && !pass.format_off,
            };
            // The output lines of a rewritten line, each laid out as a line of its own; `None` when the line is not
            // rewritten, or a statement on it has no layout (then the pass is taken back and the line laid out as
            // without the settings).
            let rewritten = (rewrites && !precompiler && !stale_line && may_rewrite(&s, i, pass.format_off))
                .then(|| segments(self.map, &events[i], settings.one_statement_per_line, settings.block_if))
                .flatten()
                .and_then(|segs| {
                    let before = pass.clone();
                    let mode = Rewrite {
                        cut: true,
                        block_if: settings.block_if,
                    };
                    let mut out = Vec::with_capacity(segs.len() + 1);
                    for seg in &segs {
                        let mut lhs = pass.level;
                        let mut lay = Lay::new();
                        if !self.segment(&events[i], seg, mode, &mut pass, &mut lay, &mut lhs) {
                            pass = before;
                            return None;
                        }
                        out.push((self.indent(lhs, &mut pass), lay));
                    }
                    // The comment goes on a line of its own above, at the indent of the first (design D5).
                    if let Some(c) = comment[i] {
                        out.insert(0, (out[0].0, self.comment(c)));
                    }
                    Some(out)
                });
            if let Some(out) = rewritten {
                let lines = out.into_iter().map(|(x, lay)| (indented(x, &lay.0), apply)).collect();
                plans.push(Plan::Layout(lines));
                continue;
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
            if !inactive[i] {
                let all: Vec<Item> = (0..events[i].len()).map(Item::Ev).collect();
                ok &= self.segment(&events[i], &all, Rewrite::default(), &mut pass, &mut lay, &mut lhs);
            }
            if let Some(c) = comment[i].filter(|_| !inactive[i]) {
                lay.then(&self.comment(c));
            }
            let x = self.indent(lhs, &mut pass);
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
            plans.push(Plan::Layout(vec![(indented(x, &body), apply)]));
        }
        plans
    }

    /// The layout of one output line (`Fmt::segment` of design D2): the items of `events` it holds, appended to
    /// `lay`, with their effect on the pass and on `lhs`. False when a statement has no layout.
    fn segment(
        &self,
        events: &[Event<'_>],
        items: &[Item],
        mode: Rewrite,
        pass: &mut Pass,
        lay: &mut Lay,
        lhs: &mut i32,
    ) -> bool {
        let mut ok = true;
        for (at, item) in items.iter().enumerate() {
            let k = match *item {
                Item::Ev(k) => k,
                Item::EndIf => {
                    // As `END IF` of a block (`block_effect`).
                    pass.level -= 1;
                    lay.then(&Lay(self.kw("END IF")));
                    continue;
                }
            };
            match events[k] {
                Event::Colon => {
                    if lay.is_empty() {
                        lay.text(b":");
                    } else {
                        lay.sp2().text(b":");
                    }
                }
                Event::Else { block } => {
                    // A single-line `IF`'s `ELSE` written as a block's.
                    if block || mode.block_if {
                        *lhs -= 1;
                    }
                    lay.then(&Lay(self.kw("ELSE")));
                }
                Event::LineIfEnd => {}
                Event::Stmt(s, block) => {
                    if mode.block_if && s.kind() == ImplicitGoto {
                        // `THEN 100` in a block: `GoTo 100` (design D4).
                        let mut l = Lay(self.kw("GOTO"));
                        l.then(&self.control(s).unwrap_or_default());
                        lay.then(&l);
                        continue;
                    }
                    let mut level = LineLevel::default();
                    let got = self.statement(s, block, pass, &mut level);
                    *lhs += level.lhs;
                    match got {
                        Some(mut l) => {
                            if mode.block_if && s.kind() == IfHeader && block == IfStmt {
                                // As the header of a block (`block_effect`); `IF c GOTO x` gets its `THEN`.
                                pass.level += 1;
                                if !s.child_tokens().any(|t| self.is_word(t, "THEN")) {
                                    l.then(&Lay(self.kw("THEN")));
                                }
                            }
                            if mode.cut && at + 1 == items.len() && builtin_label(self.map, s) && l.last() == Some(b':')
                            {
                                // `CLS:` at the end of an output line is the statement alone.
                                l.pop();
                            }
                            lay.then(&l);
                        }
                        None => ok = false,
                    }
                }
            }
        }
        ok
    }

    /// The indent level of a line from its `lhs` and the pass after it, one more inside `TYPE` and `DECLARE
    /// LIBRARY` but for their first and last lines.
    fn indent(&self, lhs: i32, pass: &mut Pass) -> i32 {
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
        x
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

/// The events of a container's children, by line; an inactive branch marks its lines.
fn collect<'n>(
    map: &SourceMap,
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
                    Ident if map.text(t.span).eq_ignore_ascii_case(b"ELSE") => {
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
                    let text = map.text(c.span());
                    let last = text.iter().rposition(|b| !b.is_ascii_whitespace()).unwrap_or(0);
                    let (a, b) = (
                        line_of(c.span().start),
                        line_of(c.span().start + qb64rust_base::to_u32(last)),
                    );
                    for l in inactive.iter_mut().take(b + 1).skip(a) {
                        *l = true;
                    }
                } else if is_container(c.kind()) {
                    collect(map, c, line_of, events, inactive);
                    if c.kind() == IfStmt {
                        let last = c.span().end.saturating_sub(1).max(c.span().start);
                        events[line_of(last)].push(Event::LineIfEnd);
                    }
                } else {
                    let start = c.first_token().map_or(c.span().start, |t| t.span.start);
                    events[line_of(start)].push(Event::Stmt(c, n.kind()));
                }
            }
        }
    }
}

/// `'$FORMAT:OFF` and `'$FORMAT:ON` in a comment (`qb64pe.bas` 25454-25459).
fn format_switch(map: &SourceMap, t: Tok, format_off: &mut bool) {
    if t.kind != MetaComment {
        return;
    }
    let upper = map.text(t.span).to_ascii_uppercase();
    let mut x = 0;
    while let Some(p) = upper[x..].iter().position(|&c| c == b'$') {
        let at = &upper[x + p..];
        if at.starts_with(b"$FORMAT:OFF") {
            *format_off = true;
        } else if at.starts_with(b"$FORMAT:ON") {
            *format_off = false;
        }
        x += p + 1;
    }
}

/// A layout with its indent level as leading spaces.
fn indented(x: i32, body: &[u8]) -> Vec<u8> {
    let mut layout = vec![b' '; usize::try_from(x.max(0)).unwrap_or(0)];
    layout.extend_from_slice(body);
    layout
}

/// A line without the spaces at its ends (`_TRIM$`).
fn trim_spaces(b: &[u8]) -> &[u8] {
    let start = b.iter().position(|&c| c != b' ').unwrap_or(b.len());
    let end = b.iter().rposition(|&c| c != b' ').map_or(start, |e| e + 1);
    &b[start..end]
}
