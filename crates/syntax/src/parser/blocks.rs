//! Blocks (design D4): `IF` (multi-line and single-line), `FOR`, `DO`, `WHILE`, `SELECT CASE`, `TYPE`, `DEF FN`,
//! and the statement loop they share with procedures and the main module.
//!
//! Every block is a node holding a header node, the body statements and the closer. The parser keeps a stack of
//! the open blocks; at the start of each statement the loop ([`Parser::body`]) looks for a closer (`NEXT`,
//! `END IF`, ...; also `ELSE`, `ELSEIF` and `CASE`, which close a branch). Recovery, one error per statement:
//! - a closer of the innermost block closes it;
//! - a closer of an outer block is an error at the closer (the old compiler rejects every crossing, measured M4);
//!   the inner blocks end there without their closers, and the outer block takes the closer;
//! - a closer of no open block is an error at the closer (`NEXT` without `FOR`);
//! - a missing closer is an error at the block's header; a `SUB`/`FUNCTION` header and the end of the file end
//!   every open block;
//! - inside a single-line `IF` the line end (and `ELSE`) ends every block opened on the line: a block must close on
//!   the line it opens on (`IF c THEN FOR …: NEXT` is fine, measured M4), and a closer there cannot close a block
//!   outside the `IF`.
//!
//! `NEXT j, i` closes two `FOR` blocks: the `NextStmt` is the closer of the inner one, and the outer `ForBlock`
//! has none of its own (`ast::ForBlock::next` finds it).
//!
//! A block may open in one file and close in another, as the old compiler reads included text (design D8 of
//! `m4-statements-metacommands`; measured, `verification\v57_b*`). An included file's parser starts with the open
//! blocks of the including file on its stack, marked as opened outside it; one it leaves open at its end goes on in
//! the including file's stack the same way. A closer or branch of such a block is a `CrossCloser` statement in the
//! file where it stands, and the block node ends where its file's part of it ends (`ParsedProgram::cross_open`).

use super::expr::{arg_list, expr};
use super::{Parser, meta, proc};
use crate::SyntaxKind::*;
use crate::tree::TreeId;
use qb64rust_base::{Diagnostic, Span};

/// How deep blocks may nest (single-line `IF`s included).
pub(super) const MAX_DEPTH: usize = 200;

/// The kinds of block on the parser's stack.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Block {
    Proc,
    If,
    For,
    Do,
    While,
    Select,
    DefFn,
    /// A single-line `IF`: blocks opened inside it must close before the line ends.
    LineIf,
    /// An open `$IF` (`meta.rs`). It is not parsed as a block node (no recursion level), but blocks must nest
    /// with it; its entry sits above the entry of the block whose body holds it.
    PpIf,
}

impl Block {
    /// How the block's header and closer are written, for messages.
    fn names(self) -> (&'static str, &'static str) {
        match self {
            Block::PpIf => ("$IF", "$END IF"),
            Block::Proc => ("SUB", "END SUB"),
            Block::If | Block::LineIf => ("IF", "END IF"),
            Block::For => ("FOR", "NEXT"),
            Block::Do => ("DO", "LOOP"),
            Block::While => ("WHILE", "WEND"),
            Block::Select => ("SELECT CASE", "END SELECT"),
            Block::DefFn => ("DEF FN", "END DEF"),
        }
    }
}

/// An open block.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Open {
    kind: Block,
    /// The header's first words, where a missing closer is reported; its start is the block node's offset.
    header: Span,
    /// The header's line in its own file, for messages.
    line: usize,
    /// The tree of the block's node.
    tree: TreeId,
    /// The block's node is in another tree than the parser's (design D8 of `m4-statements-metacommands`): a file
    /// that includes this one, or one this file includes, opened it. Its closer here is a `CrossCloser`.
    outside: bool,
    /// A `SELECT CASE` before its first `CASE`.
    before_case: bool,
    /// An `IF` that had its `ELSE`.
    has_else: bool,
    /// A `DO` with its condition after `DO`.
    test_at_top: bool,
    /// A procedure that is a FUNCTION.
    pub(super) function: bool,
}

impl Open {
    /// The key of the block's node.
    pub(crate) fn key(&self) -> (TreeId, u32) {
        (self.tree, self.header.start)
    }

    /// The block as a file it is handed to sees it: opened outside that file.
    pub(crate) fn outside(self) -> Open {
        Open { outside: true, ..self }
    }

    pub(crate) fn is_pp_if(&self) -> bool {
        self.kind == Block::PpIf
    }

    /// A single-line `IF`.
    pub(crate) fn is_line_if(&self) -> bool {
        self.kind == Block::LineIf
    }

    /// How its header and closer are written, and the header's line, for messages.
    pub(crate) fn names(&self) -> (&'static str, &'static str, usize) {
        let (opener, closer) = match self.kind {
            Block::Proc if self.function => ("FUNCTION", "END FUNCTION"),
            kind => kind.names(),
        };
        (opener, closer, self.line)
    }

    pub(crate) fn header(&self) -> Span {
        self.header
    }

    /// What the file it was handed to learned of the block, which stays open (its first `CASE`, its `ELSE`).
    pub(crate) fn take_state(&mut self, theirs: &Open) {
        self.before_case = theirs.before_case;
        self.has_else = theirs.has_else;
    }
}

/// A statement that closes a block or one of its branches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Closer {
    EndIf,
    Else,
    ElseIf,
    Next,
    Loop,
    Wend,
    Case,
    EndSelect,
    EndProc,
    EndDef,
    EndType,
    EndDeclare,
}

impl Closer {
    /// The block on the stack this closes; `None` for the closers of blocks that hold no statements (`TYPE`,
    /// `DECLARE LIBRARY`), which never meet the statement loop as closers.
    fn block(self) -> Option<Block> {
        match self {
            Closer::EndIf | Closer::Else | Closer::ElseIf => Some(Block::If),
            Closer::Next => Some(Block::For),
            Closer::Loop => Some(Block::Do),
            Closer::Wend => Some(Block::While),
            Closer::Case | Closer::EndSelect => Some(Block::Select),
            Closer::EndProc => Some(Block::Proc),
            Closer::EndDef => Some(Block::DefFn),
            Closer::EndType | Closer::EndDeclare => None,
        }
    }

    /// The header the closer needs, for messages.
    fn opener(self) -> &'static str {
        match self {
            Closer::EndIf | Closer::Else | Closer::ElseIf => "IF",
            Closer::Next => "FOR",
            Closer::Loop => "DO",
            Closer::Wend => "WHILE",
            Closer::Case | Closer::EndSelect => "SELECT CASE",
            Closer::EndProc => "SUB",
            Closer::EndDef => "DEF FN",
            Closer::EndType => "TYPE",
            Closer::EndDeclare => "DECLARE LIBRARY",
        }
    }
}

/// Why the statement loop stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Stop {
    /// The end of the file.
    Eof,
    /// At a `SUB` or `FUNCTION` header.
    ProcHeader,
    /// At a closer of the innermost block (not consumed).
    Closer(Closer),
    /// At a closer of an outer block; its error is reported and the statement carries it.
    Outer,
    /// At the line end or `ELSE` inside a single-line `IF`.
    LineEnd,
    /// The innermost block is a `FOR` that an inner `NEXT j, i` closed too.
    InnerNext,
    /// An included file closed the innermost block (design D8 of `m4-statements-metacommands`).
    Elsewhere,
}

/// Which open block a closer belongs to.
enum Target {
    /// The innermost one.
    Top,
    /// An outer one.
    Outer,
    /// None in reach: `past_line_if` when the block is open outside the single-line `IF` the closer is in.
    Stray { past_line_if: bool },
}

impl Parser<'_, '_> {
    pub(super) fn push_block(&mut self, kind: Block, header: Span) {
        let line = self.line_of(header);
        self.blocks.push(Open {
            kind,
            header,
            line,
            tree: self.tree,
            outside: false,
            before_case: kind == Block::Select,
            has_else: false,
            test_at_top: false,
            function: false,
        });
    }

    /// The innermost open block of this file's own, whose parse function is running: not a `$IF` and not one
    /// opened outside this file.
    fn own_block(&self) -> Option<usize> {
        self.blocks.iter().rposition(|o| o.kind != Block::PpIf && !o.outside)
    }

    /// The innermost open block is one opened outside this file.
    fn top_outside(&self) -> bool {
        self.blocks.last().is_some_and(|o| o.outside)
    }

    /// Notes something of the innermost open block, which its parse function reads (`has_else` of an `IF`,
    /// `test_at_top` of a `DO`, `function` of a procedure).
    pub(super) fn set_top(&mut self, set: impl FnOnce(&mut Open)) {
        if let Some(open) = self.blocks.last_mut() {
            set(open);
        }
    }

    /// A "not supported yet" mark at a statement other than the current one (a block's header).
    pub(super) fn block_unsupported(&mut self, span: Span, message: impl Into<String>) {
        if !self.quiet {
            self.diags.push(Diagnostic::unsupported(span, message));
        }
    }

    /// Ends this file's innermost own block: the block whose function calls this. A `$IF` opened in its body and
    /// still open stays on the stack (its error, if any, is reported elsewhere), and so do blocks opened outside
    /// this file. A block an included file leaves open at its end stays too, as one the including file may close
    /// (`keep_open`, set by [`Self::unclosed`]).
    pub(super) fn pop_block(&mut self) {
        let Some(i) = self.own_block() else {
            return;
        };
        if std::mem::take(&mut self.keep_open) {
            self.blocks[i].outside = true;
        } else {
            self.blocks.remove(i);
        }
    }

    pub(super) fn top_is_pp_if(&self) -> bool {
        self.top() == Some(Block::PpIf)
    }

    /// Removes the innermost `$IF` entry.
    pub(super) fn remove_last_pp_if(&mut self) {
        if let Some(i) = self.blocks.iter().rposition(|o| o.kind == Block::PpIf) {
            self.blocks.remove(i);
        }
    }

    /// The innermost open entry, for messages: how its header and closer are written, and its line.
    pub(super) fn innermost_block(&self) -> Option<(&'static str, &'static str, usize)> {
        Some(self.blocks.last()?.names())
    }

    pub(super) fn text_of(&self, span: Span) -> &[u8] {
        &self.bytes[span.start as usize..span.end as usize]
    }

    /// A block closer met `$IF` entries above its block (already reported): they leave the stack, and their
    /// `$END IF`s will need no error of their own.
    fn drop_crossed_pp_ifs(&mut self) {
        while self.top_is_pp_if() {
            self.blocks.pop();
            if let Some(entry) = self.pp_ifs.iter_mut().rev().find(|(_, crossed)| !crossed) {
                entry.1 = true;
            }
        }
    }

    fn top(&self) -> Option<Block> {
        self.blocks.last().map(|o| o.kind)
    }

    /// A block of kind `want` is open in the current procedure (or main module); `EXIT FOR` and the like look
    /// through other blocks and single-line `IF`s.
    pub(super) fn inside(&self, want: Block) -> bool {
        for open in self.blocks.iter().rev() {
            if open.kind == want {
                return true;
            }
            if open.kind == Block::Proc {
                return false;
            }
        }
        false
    }

    /// Whether a `FOR`, `DO` or `WHILE` loop of the current procedure is open, as [`Self::inside`] looks.
    pub(super) fn inside_loop(&self) -> bool {
        [Block::For, Block::Do, Block::While]
            .into_iter()
            .any(|b| self.inside(b))
    }

    /// An error at a statement other than the current one (a block's header), so not limited by `stmt_error`.
    pub(super) fn block_error(&mut self, span: Span, message: impl Into<String>) {
        if !self.quiet {
            self.diags.push(Diagnostic::error(span, message));
        }
    }

    /// The 1-based line of `span` in this file, for messages.
    fn line_of(&self, span: Span) -> usize {
        let start = span.start as usize;
        let lf = self.bytes[..start].iter().filter(|&&b| b == b'\n').count();
        // A lone CR ends a line too; the byte after the last one may be the span's own first byte.
        let lone_cr = (0..start)
            .filter(|&i| self.bytes[i] == b'\r' && self.bytes.get(i + 1) != Some(&b'\n'))
            .count();
        lf + lone_cr + 1
    }

    // ---- the statement loop ----

    /// Parses statements until the end of the file, a procedure header, a closer of an open block, or (inside a
    /// single-line `IF`) the line end or `ELSE`. Stray closers are reported and skipped.
    pub(super) fn body(&mut self) -> Stop {
        loop {
            if !std::mem::take(&mut self.carry_error) {
                self.stmt_error = false;
            }
            // An included file closed blocks of this file (design D8 of `m4-statements-metacommands`): one opened
            // outside this file just leaves the stack; this file's own block ends here, after the include.
            while self.closed_elsewhere > 0 {
                self.closed_elsewhere -= 1;
                let Some(i) = self.blocks.iter().rposition(|o| o.kind != Block::PpIf) else {
                    continue;
                };
                if self.blocks[i].outside {
                    self.blocks.remove(i);
                    continue;
                }
                return Stop::Elsewhere;
            }
            // Then the blocks it left open are open here.
            let handed = std::mem::take(&mut self.left_open);
            self.blocks.extend(handed);
            if self.pending_next.is_some() && self.top() == Some(Block::For) {
                return Stop::InnerNext;
            }
            if self.line_if > 0 && (matches!(self.current(), None | Some(Newline)) || self.at_word("ELSE")) {
                return Stop::LineEnd;
            }
            if self.current().is_none() {
                return Stop::Eof;
            }
            self.line_number();
            if self.at_proc_start() {
                return Stop::ProcHeader;
            }
            self.line_prefix();
            if let Some(c) = self.closer_here() {
                match self.closer_target(c) {
                    Target::Top if self.top_outside() => {
                        self.cross_closer(c);
                        continue;
                    }
                    Target::Top => return Stop::Closer(c),
                    Target::Outer if self.top_is_pp_if() => {
                        // `$IF … NEXT … $END IF` with the `FOR` before the `$IF`: the closer still closes its block.
                        self.crossing_error();
                        self.drop_crossed_pp_ifs();
                        match self.closer_target(c) {
                            // The block was opened in another file (`verification\v57_b21`).
                            Target::Top if self.top_outside() => {
                                self.cross_closer(c);
                                continue;
                            }
                            Target::Top => return Stop::Closer(c),
                            Target::Outer => {
                                self.carry_error = true;
                                return Stop::Outer;
                            }
                            Target::Stray { .. } => {
                                self.rest_into_error_node();
                                self.separator();
                                continue;
                            }
                        }
                    }
                    Target::Outer => {
                        self.crossing_error();
                        self.carry_error = true;
                        return Stop::Outer;
                    }
                    Target::Stray { .. } => {
                        self.stray_closer(c);
                        self.separator();
                        continue;
                    }
                }
            }
            if let Some(top) = self.blocks.last()
                && top.before_case
                && !self.at_stmt_end()
            {
                // Measured M4: "Expected CASE expression".
                self.error("only comments may stand between `SELECT CASE` and the first `CASE`");
            }
            self.statement_and_separator();
        }
    }

    /// A closer or a branch of the innermost block, which was opened outside this file (design D8 of
    /// `m4-statements-metacommands`): a `CrossCloser` statement that holds it, as the block's own function would
    /// parse it. A closer ends the block, and the pair is noted in `cross_open`; the file that holds the block's
    /// node learns it from the blocks this file hands back. A branch's statements follow as statements.
    fn cross_closer(&mut self, c: Closer) {
        let open = *self.blocks.last().expect("the innermost block is open");
        self.start_node(CrossCloser);
        let at = (self.tree, self.current_span().start);
        let mut close: usize = 1;
        match c {
            Closer::Next => {
                let (count, span) = next_stmt(self);
                close = count as usize;
                // Every block it closes must be a `FOR` opened outside this file.
                let n = self.blocks.len();
                let fors = self.blocks[..n - 1]
                    .iter()
                    .rev()
                    .take_while(|o| o.kind == Block::For && o.outside)
                    .count();
                if close > fors + 1 {
                    close = fors + 1;
                    if self.blocks.len() > fors + 1 && self.blocks[n - 2 - fors].kind == Block::For {
                        self.block_unsupported(span, "`NEXT` that closes a `FOR` of this file and one of another file");
                    } else {
                        self.block_error(
                            span,
                            "`NEXT` names more variables than there are `FOR` blocks open around it",
                        );
                    }
                }
            }
            Closer::Loop => {
                self.start_node(LoopStmt);
                let span = self.current_span();
                self.bump(); // LOOP
                if loop_condition(self) && open.test_at_top {
                    self.error_at(span, "a `DO` with a condition cannot have one at `LOOP` too");
                }
                self.recover();
                self.finish_node();
                self.separator();
            }
            Closer::EndProc => {
                proc::proc_end(self, if open.function { "FUNCTION" } else { "SUB" });
                self.header_end();
            }
            Closer::EndIf | Closer::Wend | Closer::EndSelect | Closer::EndDef => self.block_end(),
            Closer::ElseIf => {
                close = 0;
                self.start_node(ElseIfBranch);
                if open.has_else {
                    self.error("`ELSEIF` cannot follow `ELSE`");
                }
                if_header(self);
                if self.stmt_error || self.at_stmt_end() {
                    self.header_end();
                }
                self.finish_node();
            }
            Closer::Else => {
                close = 0;
                self.start_node(ElseBranch);
                if open.has_else {
                    self.error("this `IF` already has an `ELSE`");
                }
                self.set_top(|o| o.has_else = true);
                self.bump(); // ELSE
                if self.at_stmt_end() {
                    self.header_end();
                }
                self.finish_node();
            }
            Closer::Case => {
                close = 0;
                self.start_node(CaseClause);
                self.set_top(|o| o.before_case = false);
                case_header(self);
                self.header_end();
                self.finish_node();
            }
            Closer::EndType | Closer::EndDeclare => unreachable!("no block on the stack closes with them"),
        }
        self.finish_node();
        for _ in 0..close {
            if let Some(o) = self.blocks.pop() {
                self.inc.cross_open.insert(o.key(), at);
            }
        }
    }

    /// The closer at the current token, if the statement is one.
    pub(super) fn closer_here(&self) -> Option<Closer> {
        if !self.at(Ident) {
            return None;
        }
        if self.at_word("END") {
            let second = [
                ("IF", Closer::EndIf),
                ("SELECT", Closer::EndSelect),
                ("SUB", Closer::EndProc),
                ("FUNCTION", Closer::EndProc),
                ("DEF", Closer::EndDef),
                ("TYPE", Closer::EndType),
                ("DECLARE", Closer::EndDeclare),
            ];
            return second.into_iter().find(|(w, _)| self.nth_is_word(1, w)).map(|(_, c)| c);
        }
        let words = [
            ("ENDIF", Closer::EndIf),
            ("ELSE", Closer::Else),
            ("ELSEIF", Closer::ElseIf),
            ("NEXT", Closer::Next),
            ("LOOP", Closer::Loop),
            ("WEND", Closer::Wend),
            ("CASE", Closer::Case),
        ];
        words.into_iter().find(|(w, _)| self.at_word(w)).map(|(_, c)| c)
    }

    fn closer_target(&self, c: Closer) -> Target {
        let Some(want) = c.block() else {
            return Target::Stray { past_line_if: false };
        };
        let last = self.blocks.len().wrapping_sub(1);
        let mut crossed = false;
        for (i, open) in self.blocks.iter().enumerate().rev() {
            if open.kind == want {
                return match (crossed, i == last) {
                    (true, _) => Target::Stray { past_line_if: true },
                    (false, true) => Target::Top,
                    (false, false) => Target::Outer,
                };
            }
            match open.kind {
                Block::LineIf => crossed = true,
                Block::Proc => break,
                Block::If | Block::For | Block::Do | Block::While | Block::Select | Block::DefFn | Block::PpIf => {}
            }
        }
        Target::Stray { past_line_if: false }
    }

    /// The closer's words, upper case (`END IF`, `NEXT`).
    fn closer_words(&self) -> String {
        let show = |n| qb64rust_base::show_bytes(self.nth_text(n)).to_ascii_uppercase();
        if self.at_word("END") {
            format!("END {}", show(1))
        } else {
            show(0)
        }
    }

    fn closer_span(&self) -> Span {
        if self.at_word("END") {
            self.current_span().cover(self.next_span(1))
        } else {
            self.current_span()
        }
    }

    /// A closer of an outer block while the innermost one is open (measured M4: rejected by the old compiler in
    /// every form tried).
    fn crossing_error(&mut self) {
        let Some(open) = self.blocks.last().copied() else {
            return;
        };
        let (opener, closer, line) = open.names();
        let words = self.closer_words();
        let span = self.closer_span();
        self.error_at(
            span,
            format!("`{words}` while the `{opener}` on line {line} is still open; expected `{closer}` first"),
        );
    }

    /// A closer of no block in reach: an error, and the statement goes into an `Error` node (its separator is left
    /// for the caller).
    pub(super) fn stray_closer(&mut self, c: Closer) {
        let words = self.closer_words();
        let target = self.closer_target(c);
        let msg = match (target, c) {
            (Target::Stray { past_line_if: true }, _) => {
                format!("`{words}` inside a single-line `IF` cannot close a block outside it")
            }
            (_, Closer::EndProc) => format!("`{words}` without a `{}`", &words[4..]),
            _ => format!("`{words}` without `{}`", c.opener()),
        };
        let span = self.closer_span();
        self.error_at(span, msg);
        self.rest_into_error_node();
    }

    /// Reports the innermost block as not closed, when the loop stopped for a reason other than a closer.
    pub(super) fn unclosed(&mut self, stop: Stop) {
        let Some(open) = self.own_block().map(|i| self.blocks[i]) else {
            return;
        };
        let (opener, closer) = open.kind.names();
        match stop {
            // The including file may close it (design D8 of `m4-statements-metacommands`); if it does not, its
            // error is reported at the end of the main file.
            Stop::Eof if self.included && self.line_if == 0 => self.keep_open = true,
            Stop::Eof | Stop::ProcHeader => self.block_error(open.header, format!("`{opener}` without `{closer}`")),
            Stop::LineEnd => self.block_error(
                open.header,
                format!("`{opener}` without `{closer}` before the end of the single-line `IF`"),
            ),
            Stop::Closer(_) | Stop::Outer | Stop::InnerNext | Stop::Elsewhere => {}
        }
    }

    /// A word closer (`END IF`, `ENDIF`, `WEND`, `END SELECT`...) as a `BlockEnd`, and the end of its statement.
    pub(super) fn block_end(&mut self) {
        self.start_node(BlockEnd);
        let two_words = self.at_word("END");
        self.bump();
        if two_words {
            self.bump();
        }
        self.finish_node();
        self.header_end();
    }

    /// Inside a block that holds no statements (`TYPE`, `DECLARE LIBRARY`): takes an empty line, a separator, a
    /// metacommand or a metacommand comment. Returns false when something else is next.
    pub(super) fn skip_blank_or_meta(&mut self) -> bool {
        match self.current() {
            Some(Newline | Colon) => self.bump(),
            Some(Metacommand) => {
                meta::metacommand(self);
            }
            Some(MetaComment) => meta::meta_comment(self),
            _ => return false,
        }
        true
    }

    /// `NEXT j, i` closed the innermost `FOR` and must close `left` more: the next one out must be a `FOR` too.
    fn chain_next(&mut self, left: u32, span: Span) {
        if left == 0 {
            return;
        }
        let n = self.blocks.len();
        if n >= 2 && self.blocks[n - 2].kind == Block::For && self.blocks[n - 2].outside {
            self.block_unsupported(span, "`NEXT` that closes a `FOR` of this file and one of another file");
        } else if n >= 2 && self.blocks[n - 2].kind == Block::For {
            self.pending_next = Some((left, span));
        } else {
            self.block_error(
                span,
                "`NEXT` names more variables than there are `FOR` blocks open around it",
            );
        }
    }

    /// After `IF cond THEN` comes the line end, a `'` comment or a metacommand comment: a multi-line `IF`. `THEN`
    /// followed by `REM` is a single-line `IF` with an empty branch (the old compiler's `lineformat`, measured).
    fn block_if_ahead(&self) -> bool {
        let mut n = 1;
        loop {
            match self.nth(n) {
                None | Some(Newline | Colon | MetaComment) => return false,
                Some(Ident) if self.nth_is_word(n, "THEN") => break,
                Some(_) => n += 1,
            }
        }
        if !matches!(self.nth(n + 1), None | Some(Newline | MetaComment)) {
            return false;
        }
        let Some(then) = self.nth_index(n) else {
            return false;
        };
        let is_rem = |i: usize| {
            let text = &self.bytes[self.offsets[i] as usize..self.offsets[i + 1] as usize];
            text.len() >= 3 && text[..3].eq_ignore_ascii_case(b"REM")
        };
        !(then + 1..self.tokens.len())
            .take_while(|&i| self.tokens[i].kind.is_trivia() || self.tokens[i].kind == MetaComment)
            .any(|i| matches!(self.tokens[i].kind, Comment | MetaComment) && is_rem(i))
    }
}

// ---- IF ----

/// `IF` at the start of a statement: a multi-line `IfBlock` or a single-line `IfStmt`. Returns true for the block.
pub(crate) fn if_stmt(p: &mut Parser) -> bool {
    if p.block_if_ahead() {
        if_block(p);
        true
    } else {
        line_if(p);
        false
    }
}

/// `IF cond THEN` or `ELSEIF cond THEN` as an `IfHeader`. In a single-line `IF`, `IF cond GOTO` leaves the `GOTO`
/// for the branch.
fn if_header(p: &mut Parser) {
    p.start_node(IfHeader);
    let is_if = p.at_word("IF");
    p.bump(); // IF or ELSEIF
    if p.at_word("THEN") {
        // No condition: "Syntax error" after `IF`, "Expected ... THEN" after `ELSEIF` (design D2 of
        // `m4-statements-metacommands`; measured, `verification\v57_e10`, `e11`).
        let msg = if is_if {
            "`IF` needs a condition before `THEN`"
        } else {
            "`ELSEIF` needs a condition before `THEN`"
        };
        p.error(msg);
        p.bump(); // THEN, so that the header holds its error
    } else if expr(p) {
        if p.at_word("THEN") {
            p.bump();
        } else if !is_if {
            p.syntax_error("expected `THEN`");
        } else if !p.at_word("GOTO") {
            p.syntax_error("expected `THEN` or `GOTO`");
        }
    }
    p.finish_node();
}

fn if_block(p: &mut Parser) {
    p.start_node(IfBlock);
    let span = p.current_span();
    p.start_node(IfBranch);
    if_header(p);
    p.header_end();
    p.push_block(Block::If, span);
    let mut has_else = false;
    loop {
        match p.body() {
            Stop::Closer(Closer::ElseIf) => {
                p.finish_node();
                p.start_node(ElseIfBranch);
                if has_else {
                    p.error("`ELSEIF` cannot follow `ELSE`");
                }
                if_header(p);
                // A statement may follow on the same line (measured M4).
                if p.stmt_error || p.at_stmt_end() {
                    p.header_end();
                }
            }
            Stop::Closer(Closer::Else) => {
                p.finish_node();
                p.start_node(ElseBranch);
                if has_else {
                    p.error("this `IF` already has an `ELSE`");
                }
                has_else = true;
                p.set_top(|o| o.has_else = true);
                p.bump(); // ELSE
                if p.at_stmt_end() {
                    p.header_end();
                }
            }
            Stop::Closer(Closer::EndIf) => {
                p.finish_node();
                p.block_end();
                break;
            }
            stop => {
                p.finish_node();
                p.unclosed(stop);
                break;
            }
        }
    }
    p.pop_block();
    p.finish_node();
}

/// `IF cond THEN … [ELSE …]` on one line, `THEN 10`, `IF cond GOTO label`.
fn line_if(p: &mut Parser) {
    p.start_node(IfStmt);
    let span = p.current_span();
    if_header(p);
    if p.stmt_error {
        p.recover();
        p.finish_node();
        return;
    }
    p.line_if += 1;
    p.push_block(Block::LineIf, span);
    // `IF c GOTO 20` needs no special case: the branch starts with the `GOTO` statement.
    p.start_node(LineBranch);
    line_branch(p);
    p.finish_node();
    if p.at_word("ELSE") {
        p.bump();
        p.start_node(LineBranch);
        line_branch(p);
        p.finish_node();
    }
    p.pop_block();
    p.line_if -= 1;
    // A further `ELSE` belongs to an enclosing single-line `IF`; with none, this `IF` has two.
    if p.line_if == 0 && p.at_word("ELSE") {
        p.stmt_error = false;
        p.error("this `IF` already has an `ELSE`");
        p.rest_into_error_node();
    }
    p.finish_node();
}

/// The statements of one branch of a single-line `IF`, up to the line end or `ELSE`. A line number first is a
/// jump (`THEN 10`, `ELSE 20`); statements after it and a `:` still belong to the branch, as after `GOTO 20:`
/// (measured, `verification\v16_m4_line_if_jump_colon`).
fn line_branch(p: &mut Parser) {
    let mut first = true;
    loop {
        if !std::mem::take(&mut p.carry_error) {
            p.stmt_error = false;
        }
        if matches!(p.current(), None | Some(Newline)) || p.at_word("ELSE") {
            break;
        }
        if first && p.at(Number) {
            p.start_node(ImplicitGoto);
            p.bump();
            p.finish_node();
            p.recover();
        } else if !p.statement() && !p.at_stmt_end() {
            p.recover();
        }
        first = false;
        if p.at(Colon) {
            p.bump();
        }
    }
}

// ---- loops ----

pub(crate) fn for_block(p: &mut Parser) {
    p.start_node(ForBlock);
    let span = p.current_span();
    p.start_node(ForHeader);
    p.bump(); // FOR
    for_header_rest(p);
    p.finish_node();
    p.header_end();
    p.push_block(Block::For, span);
    match p.body() {
        Stop::Closer(Closer::Next) => {
            let (count, next) = next_stmt(p);
            p.chain_next(count - 1, next);
        }
        Stop::InnerNext => {
            if let Some((left, next)) = p.pending_next.take() {
                p.chain_next(left - 1, next);
            }
        }
        stop => p.unclosed(stop),
    }
    p.pop_block();
    p.finish_node();
}

/// `var = start TO end [STEP step]`, after `FOR`. The variable may have indexes (`FOR r.m(i) = …`), which `sema`
/// reads: the old compiler takes an element of a member array there (design D8 of `m4-memory-records`).
fn for_header_rest(p: &mut Parser) {
    if !p.at(Ident) {
        p.syntax_error("expected a variable after `FOR`");
        return;
    }
    name_ref(p);
    if p.at(LParen) && !arg_list(p) {
        return;
    }
    if !p.expect(Eq, "`=`") || !expr(p) {
        return;
    }
    if !p.at_word("TO") {
        p.syntax_error("expected `TO`");
        return;
    }
    p.bump();
    if expr(p) && p.at_word("STEP") {
        p.bump();
        expr(p);
    }
}

fn name_ref(p: &mut Parser) {
    p.start_node(NameRef);
    p.bump();
    p.finish_node();
}

/// `NEXT [var, ...]`, at `NEXT`. Returns how many `FOR` blocks it closes, and its span.
fn next_stmt(p: &mut Parser) -> (u32, Span) {
    p.start_node(NextStmt);
    let span = p.current_span();
    p.bump(); // NEXT
    let mut count = 1;
    if p.at(Ident) {
        name_ref(p);
        while p.at(Comma) {
            p.bump();
            if !p.at(Ident) {
                p.syntax_error("expected a variable after `,`");
                break;
            }
            name_ref(p);
            count += 1;
        }
    }
    p.recover();
    p.finish_node();
    p.separator();
    (count, span)
}

pub(crate) fn do_block(p: &mut Parser) {
    p.start_node(DoBlock);
    let span = p.current_span();
    p.start_node(DoHeader);
    p.bump(); // DO
    let at_top = loop_condition(p);
    p.finish_node();
    p.header_end();
    p.push_block(Block::Do, span);
    p.set_top(|o| o.test_at_top = at_top);
    match p.body() {
        Stop::Closer(Closer::Loop) => {
            p.start_node(LoopStmt);
            let span = p.current_span();
            p.bump(); // LOOP
            if loop_condition(p) && at_top {
                // Measured M4: "PROGRAM FLOW ERROR!".
                p.error_at(span, "a `DO` with a condition cannot have one at `LOOP` too");
            }
            p.recover();
            p.finish_node();
            p.separator();
        }
        stop => p.unclosed(stop),
    }
    p.pop_block();
    p.finish_node();
}

/// `WHILE cond` or `UNTIL cond` after `DO` or `LOOP`, if there. Returns whether there was one.
fn loop_condition(p: &mut Parser) -> bool {
    if p.at_word("WHILE") || p.at_word("UNTIL") {
        p.bump();
        expr(p);
        true
    } else {
        false
    }
}

pub(crate) fn while_block(p: &mut Parser) {
    p.start_node(WhileBlock);
    let span = p.current_span();
    p.start_node(WhileHeader);
    p.bump(); // WHILE
    expr(p);
    p.finish_node();
    p.header_end();
    p.push_block(Block::While, span);
    match p.body() {
        Stop::Closer(Closer::Wend) => p.block_end(),
        stop => p.unclosed(stop),
    }
    p.pop_block();
    p.finish_node();
}

// ---- SELECT CASE ----

pub(crate) fn select_block(p: &mut Parser) {
    p.start_node(SelectBlock);
    let span = p.current_span();
    p.start_node(SelectHeader);
    p.bump(); // SELECT
    if p.at_word("CASE") || p.at_word("EVERYCASE") {
        p.bump();
        expr(p);
    } else {
        p.syntax_error("expected `CASE` or `EVERYCASE` after `SELECT`");
    }
    p.finish_node();
    p.header_end();
    p.push_block(Block::Select, span);
    let mut in_case = false;
    loop {
        match p.body() {
            Stop::Closer(Closer::Case) => {
                if in_case {
                    p.finish_node();
                }
                p.start_node(CaseClause);
                in_case = true;
                if let Some(top) = p.blocks.last_mut() {
                    top.before_case = false;
                }
                case_header(p);
                p.header_end();
            }
            Stop::Closer(Closer::EndSelect) => {
                if in_case {
                    p.finish_node();
                }
                p.block_end();
                break;
            }
            stop => {
                if in_case {
                    p.finish_node();
                }
                p.unclosed(stop);
                break;
            }
        }
    }
    p.pop_block();
    p.finish_node();
}

/// `CASE ELSE` or `CASE item, ...`, at `CASE`.
fn case_header(p: &mut Parser) {
    p.start_node(CaseHeader);
    p.bump(); // CASE
    if p.at_word("ELSE") {
        p.bump();
    } else {
        while case_item(p) && p.at(Comma) {
            p.bump();
        }
    }
    p.finish_node();
}

/// `IS <op> expr`, `expr TO expr` or `expr`. Returns false after an error.
fn case_item(p: &mut Parser) -> bool {
    p.start_node(CaseItem);
    let ok = if p.at_word("IS") {
        p.bump();
        if matches!(p.current(), Some(Eq | Ne | Lt | Gt | Le | Ge)) {
            p.bump();
            expr(p)
        } else {
            p.syntax_error("expected a comparison operator after `IS`");
            false
        }
    } else {
        expr(p)
            && (!p.at_word("TO") || {
                p.bump();
                expr(p)
            })
    };
    p.finish_node();
    ok
}

// ---- TYPE ----

/// `TYPE name`, fields, `END TYPE`. Only fields, comments and metacommands may stand inside (measured M4: a
/// statement there is an error). After such a statement, or at the end of the file, the block ends with an error
/// and the rest is parsed as usual.
pub(crate) fn type_block(p: &mut Parser) {
    p.start_node(TypeBlock);
    let span = p.current_span();
    p.start_node(TypeHeader);
    p.bump(); // TYPE
    if p.at(Ident) {
        p.bump();
    } else {
        p.syntax_error("expected a type name after `TYPE`");
    }
    p.finish_node();
    p.header_end();
    loop {
        p.stmt_error = false;
        if p.current().is_none() {
            p.block_error(span, "`TYPE` without `END TYPE`");
            break;
        }
        if p.skip_blank_or_meta() {
            continue;
        }
        if p.closer_here() == Some(Closer::EndType) {
            p.block_end();
            break;
        }
        let field = p.at_word("AS")
            || ((p.at_word("_DYNAMIC") || p.at_word("_STATIC")) && p.nth(1) == Some(Ident))
            || (p.at(Ident)
                && (p.nth_is_word(1, "AS")
                    || p.nth(1) == Some(LParen)
                    || p.nth_is_word(1, "_DYNAMIC")
                    || p.nth_is_word(1, "_STATIC")));
        if !field {
            p.error("expected a field (`name AS type` or `AS type name, ...`) or `END TYPE`");
            p.carry_error = true;
            break;
        }
        type_field(p);
        p.header_end();
    }
    p.finish_node();
}

/// `[marker] name [markers] [bounds] [markers] AS type [markers]` or `AS type [markers] name [markers] [bounds]
/// [markers], ...`, where a marker is `_DYNAMIC` or `_STATIC` (`qb64pe.bas` 2213–2290, 2356–2440: the old compiler
/// reads one before a single name and any number in the other places; `sema` counts them). The markers before a
/// single name and after the type are `FieldMarker`s of the `TypeField`, the others of their `FieldName`.
fn type_field(p: &mut Parser) {
    p.start_node(TypeField);
    if at_field_marker(p) && p.nth(1) == Some(Ident) {
        field_marker(p);
    }
    if p.at_word("AS") {
        type_as_clause(p, true);
        while field_name(p, true) && p.at(Comma) {
            p.bump();
        }
    } else if field_name(p, false) {
        if p.at_word("AS") {
            type_as_clause(p, false);
            while at_field_marker(p) {
                field_marker(p);
            }
        } else {
            p.syntax_error("expected `AS`");
        }
    }
    p.finish_node();
}

fn at_field_marker(p: &Parser) -> bool {
    p.at_word("_DYNAMIC") || p.at_word("_STATIC")
}

/// One `_DYNAMIC` or `_STATIC`, at it.
fn field_marker(p: &mut Parser) {
    p.start_node(FieldMarker);
    p.bump();
    p.finish_node();
}

/// A field's name with its bounds and markers; in an `AS type` list (`in_list`) also markers before the name.
/// Returns false after an error.
fn field_name(p: &mut Parser, in_list: bool) -> bool {
    if !p.at(Ident) {
        p.syntax_error("expected a field name");
        return false;
    }
    p.start_node(FieldName);
    while in_list && at_field_marker(p) {
        field_marker(p);
    }
    if !p.at(Ident) {
        p.syntax_error("expected a field name");
        p.finish_node();
        return false;
    }
    p.bump();
    while at_field_marker(p) {
        field_marker(p);
    }
    let ok = !p.at(LParen) || array_bounds(p);
    while ok && at_field_marker(p) {
        field_marker(p);
    }
    p.finish_node();
    ok
}

/// `AS type [* size]` in a field. Before the names (`leading`), the type is one word, or two after `_UNSIGNED`;
/// after a name it is every word up to the end or to a marker. A marker right after `AS` is an error, as in the
/// old compiler (measured, `verification\v35_x05`).
fn type_as_clause(p: &mut Parser, leading: bool) {
    p.start_node(AsClause);
    p.bump(); // AS
    if at_field_marker(p) {
        p.error("a member's `_DYNAMIC` or `_STATIC` cannot stand between `AS` and its type");
    } else if !p.at(Ident) {
        p.syntax_error("expected a type name after `AS`");
    } else if leading {
        if p.at_word("_UNSIGNED") && p.nth(1) == Some(Ident) {
            p.bump();
        }
        p.bump();
    } else {
        while p.at(Ident) && !at_field_marker(p) {
            p.bump();
        }
    }
    if p.at(Star) {
        p.bump();
        if p.at(Number) || p.at(Ident) {
            p.bump();
        } else {
            p.syntax_error("expected a size after `*`");
        }
    }
    p.finish_node();
}

/// `(range, ...)` with `range` = `expr [TO expr]`, at `(`; `()` is allowed. Returns false after an error.
pub(super) fn array_bounds(p: &mut Parser) -> bool {
    p.start_node(ArrayBounds);
    p.bump(); // (
    let mut ok = true;
    if !p.at(RParen) {
        loop {
            if !expr(p)
                || (p.at_word("TO") && {
                    p.bump();
                    !expr(p)
                })
            {
                ok = false;
                break;
            }
            if p.at(Comma) {
                p.bump();
            } else {
                break;
            }
        }
    }
    let ok = ok && p.expect(RParen, "`,` or `)`");
    p.finish_node();
    ok
}

// ---- DEF FN ----

/// `DEF FNname[(params)] = expr` (a `DefFnStmt`) or `DEF FNname[(params)]` … `END DEF` (a `DefFnBlock`). The old
/// compiler rejects both ("Command not implemented", measured M4); `sema` reports them. Other `DEF` statements
/// (`DEF SEG`) are not supported yet. Returns true for the block.
pub(crate) fn def_stmt(p: &mut Parser) -> bool {
    let name = p.nth_text(1);
    let is_fn = p.nth(1) == Some(Ident) && name.len() > 2 && name[..2].eq_ignore_ascii_case(b"FN");
    if !is_fn {
        // `DEF SEG [= address]`, by its template.
        if !p.template_statement() {
            p.not_supported_statement();
        }
        return false;
    }
    p.eat_trivia();
    let cp = p.builder.checkpoint();
    let span = p.current_span();
    p.start_node(DefFnHeader);
    p.bump(); // DEF
    p.bump(); // FNname
    if p.at(LParen) {
        proc::param_list(p, false);
    }
    p.finish_node();
    if p.at(Eq) {
        p.builder.start_node_at(cp, DefFnStmt);
        p.bump(); // =
        expr(p);
        p.recover();
        p.finish_node();
        return false;
    }
    p.builder.start_node_at(cp, DefFnBlock);
    p.header_end();
    p.push_block(Block::DefFn, span);
    match p.body() {
        Stop::Closer(Closer::EndDef) => p.block_end(),
        stop => p.unclosed(stop),
    }
    p.pop_block();
    p.finish_node();
    true
}
