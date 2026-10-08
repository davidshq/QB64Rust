//! Recursive-descent parser over the token stream (design D4). Statement families have their own modules,
//! dispatched by the first token (FreeBASIC lesson L1).
//!
//! Recovery: at most one error per statement. After an error the rest of the statement (up to a line end, or a
//! `:` outside parentheses) goes into an `Error` node and parsing continues with the next statement. Procedures
//! and the other blocks share one statement loop and one stack of open blocks (`blocks.rs`); their recovery is
//! described there.

mod assign;
mod blocks;
mod call;
mod data;
mod decl;
mod errors;
mod expr;
mod flow;
mod io;
pub(crate) mod keywords;
mod meta;
mod print;
mod proc;
mod template;

use crate::SyntaxKind::{self, *};
use crate::lexer::{Token, tokenize};
use crate::pp::PpState;
use crate::program::{Includer, Tree};
use crate::tree::{TreeBuilder, TreeId};
use qb64rust_base::{Diagnostic, Diagnostics, FileId, Span};

/// Parses one file on its own into tree `id`, with no included files (unit tests).
#[cfg(test)]
pub(crate) fn parse_tree(id: TreeId, file: FileId, bytes: &[u8]) -> Tree {
    let mut map = qb64rust_base::SourceMap::new();
    let mut loader = crate::NoLoader;
    let mut inc = Includer::new(&mut map, &mut loader);
    parse_tree_with(id, file, bytes, PpState::default(), &mut inc, false).0
}

/// Parses one file into tree `id`, starting from preprocessor state `pp`; gives back the state the file leaves
/// (a `$LET` in an included file reaches the including one). Files it includes are parsed through `inc` as they
/// come (`meta.rs`); `included` says that this file is itself an included one.
pub(crate) fn parse_tree_with(
    id: TreeId,
    file: FileId,
    bytes: &[u8],
    pp: PpState,
    inc: &mut Includer<'_>,
    included: bool,
) -> (Tree, PpState) {
    let tokens = tokenize(bytes);
    let mut offsets = Vec::with_capacity(tokens.len() + 1);
    let mut o = 0u32;
    for t in &tokens {
        offsets.push(o);
        o += t.len;
    }
    offsets.push(o);
    let significant = (0..tokens.len()).filter(|&i| !tokens[i].kind.is_trivia()).collect();
    let mut p = Parser {
        file,
        bytes,
        tokens,
        offsets,
        significant,
        next_significant: 0,
        pos: 0,
        builder: TreeBuilder::default(),
        diags: Diagnostics::new(),
        stmt_error: false,
        carry_error: false,
        quiet: false,
        quiet_failed: false,
        last_kind: None,
        blocks: Vec::new(),
        line_if: 0,
        pending_next: None,
        after_line_number: usize::MAX,
        in_const: false,
        expr_nesting: 0,
        pp,
        pp_ifs: Vec::new(),
        redim_items: false,
        inc,
        tree: id,
        included,
    };
    p.source_file();
    let tree = Tree {
        id,
        file,
        green: p.builder.finish(),
        diagnostics: p.diags,
    };
    (tree, p.pp)
}

pub(crate) struct Parser<'a, 'h> {
    file: FileId,
    bytes: &'a [u8],
    tokens: Vec<Token>,
    offsets: Vec<u32>,
    /// Indices of the tokens that are not trivia, in order.
    significant: Vec<usize>,
    /// Index into `significant` of the first such token at or after `pos`.
    next_significant: usize,
    pos: usize,
    builder: TreeBuilder,
    diags: Diagnostics,
    /// An error was already reported for the current statement.
    stmt_error: bool,
    /// The next statement the loop looks at already has its error (a closer reported by an inner block, a nested
    /// `SUB` header), so `stmt_error` is not reset for it.
    carry_error: bool,
    /// Errors are not reported but only noted in `quiet_failed` (arguments of a call without `CALL`).
    quiet: bool,
    quiet_failed: bool,
    /// Kind of the last non-trivia token added to the tree.
    last_kind: Option<SyntaxKind>,
    /// The open blocks, innermost last (`blocks.rs`).
    blocks: Vec<blocks::Open>,
    /// How many single-line `IF`s are open: inside one, the line end and `ELSE` end every statement.
    line_if: u32,
    /// `NEXT j, i` closed an inner `FOR` and still has this many `FOR` blocks to close (and its span).
    pending_next: Option<(u32, Span)>,
    /// Token index just after the last line number: a label may stand there (`10 lab: PRINT`).
    after_line_number: usize,
    /// Inside a `CONST` value, where the old compiler's evaluator also knows the operator `ROOT` (`study\02` §7).
    in_const: bool,
    /// Expression nodes open around the one being parsed (`expr.rs`, [`Self::within_expr_depth`]). While an
    /// expression is built, its own place counts too: a node built there at height `h` reaches depth
    /// `expr_nesting + h - 1`. The statement is not counted, so a top-level expression's root has depth 1.
    expr_nesting: u32,
    /// The preprocessor's names and open `$IF` levels (`meta.rs`).
    pp: PpState,
    /// Per `$IF` opened in this file and still open: its span, and whether a block closer already crossed it
    /// (its entry then left the block stack, and its `$END IF` needs no error of its own).
    pp_ifs: Vec<(Span, bool)>,
    /// The items being parsed are a `REDIM`'s: a member array may stand there (`decl.rs`).
    redim_items: bool,
    /// Loads and parses included files (`meta.rs`).
    inc: &'a mut Includer<'h>,
    /// This file's tree.
    tree: TreeId,
    /// This file is an included one: a block closer without its block, and a block left open at the end, may
    /// belong to the including file (design D4).
    included: bool,
}

impl<'a> Parser<'a, '_> {
    // ---- token access (trivia skipped) ----

    /// Token index of the n-th non-trivia token ahead. Constant time, so that the look-ahead scans over a whole
    /// statement (`parens_then_eq`, `block_if_ahead`) stay linear in its length.
    fn nth_index(&self, n: usize) -> Option<usize> {
        self.significant.get(self.next_significant + n).copied()
    }

    /// Kind of the n-th non-trivia token ahead; `None` at the end of the file.
    fn nth(&self, n: usize) -> Option<SyntaxKind> {
        self.nth_index(n).map(|i| self.tokens[i].kind)
    }

    fn current(&self) -> Option<SyntaxKind> {
        self.nth(0)
    }

    fn at(&self, kind: SyntaxKind) -> bool {
        self.current() == Some(kind)
    }

    fn nth_text(&self, n: usize) -> &'a [u8] {
        match self.nth_index(n) {
            Some(i) => &self.bytes[self.offsets[i] as usize..self.offsets[i + 1] as usize],
            None => b"",
        }
    }

    /// The current token is an identifier spelled `word` (ASCII case ignored).
    fn at_word(&self, word: &str) -> bool {
        self.at(Ident) && self.nth_text(0).eq_ignore_ascii_case(word.as_bytes())
    }

    fn current_span(&self) -> Span {
        match self.nth_index(0) {
            Some(i) => Span::new(self.file, self.offsets[i], self.offsets[i + 1]),
            None => {
                let end = *self.offsets.last().unwrap();
                Span::new(self.file, end, end)
            }
        }
    }

    /// At a line end, a `:`, a metacommand comment or the end of the file; inside a single-line `IF` also at `ELSE`.
    fn at_stmt_end(&self) -> bool {
        self.ends_at(0)
    }

    /// The n-th non-trivia token ahead ends a statement (see [`Self::at_stmt_end`]).
    fn ends_at(&self, n: usize) -> bool {
        ends_stmt(self.nth(n)) || (self.line_if > 0 && self.nth_is_word(n, "ELSE"))
    }

    // ---- tree building ----

    fn eat_trivia(&mut self) {
        while self.pos < self.tokens.len() && self.tokens[self.pos].kind.is_trivia() {
            let t = self.tokens[self.pos];
            self.builder.token(t.kind, t.len);
            self.pos += 1;
        }
    }

    /// Adds pending trivia and the current token to the open node.
    fn bump(&mut self) {
        self.eat_trivia();
        if self.pos < self.tokens.len() {
            let t = self.tokens[self.pos];
            self.builder.token(t.kind, t.len);
            self.last_kind = Some(t.kind);
            self.pos += 1;
            self.next_significant += 1;
        }
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.eat_trivia();
        self.builder.start_node(kind);
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    // ---- errors ----

    /// Reports a diagnostic unless the statement already has one.
    fn report(&mut self, d: Diagnostic) {
        if self.quiet {
            self.quiet_failed = true;
        } else if !self.stmt_error {
            self.stmt_error = true;
            self.diags.push(d);
        }
    }

    /// Reports an error at `span` unless the statement already has one.
    fn error_at(&mut self, span: Span, message: impl Into<String>) {
        self.report(Diagnostic::error(span, message));
    }

    fn error(&mut self, message: impl Into<String>) {
        let span = self.current_span();
        self.error_at(span, message);
    }

    /// A generic syntax error at the current token ("expected ..."). When that token is a BASIC word or operator
    /// the parser does not handle there yet, the error is marked "not supported yet" and names the token (design
    /// D3 of `m2-upstream-tests`; a heuristic, its misses show up in `tests\known_false_errors.list`). Specific
    /// errors the old compiler also reports use [`Self::error`].
    fn syntax_error(&mut self, message: impl Into<String>) {
        let span = self.current_span();
        let message = message.into();
        match self.unhandled_token() {
            Some(what) => self.unsupported_at(span, format!("{what} ({message})")),
            None => self.error_at(span, message),
        }
    }

    /// `operator `MOD``, `word `TO``, `` `#` ``... when the current token is a known BASIC word or operator.
    fn unhandled_token(&self) -> Option<String> {
        let op = |s: &str| Some(format!("operator `{s}`"));
        match self.current()? {
            Backslash => op("\\"),
            Caret => op("^"),
            Eq => op("="),
            Ne => op("<>"),
            Lt => op("<"),
            Gt => op(">"),
            Le => op("<="),
            Ge => op(">="),
            Hash => Some("`#`".to_string()),
            Semicolon => Some("`;`".to_string()),
            Ident => {
                let text = self.nth_text(0);
                let word = qb64rust_base::show_bytes(text).to_ascii_uppercase();
                if ["MOD", "AND", "OR", "NOT", "XOR", "EQV", "IMP"].contains(&word.as_str())
                    || (self.in_const && word == "ROOT")
                {
                    op(&word)
                } else if word == "ELSE" {
                    // Outside a single-line `IF`, an `ELSE` within a statement is an error for the old compiler
                    // too ("Invalid Syntax for ELSE", `qb64pe.bas` 6594-6624).
                    None
                } else if keywords::is_keyword(text) {
                    Some(format!("word `{word}`"))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Reports a construct the parser does not handle yet, marked "not supported yet"; `message` names it.
    fn unsupported_at(&mut self, span: Span, message: impl Into<String>) {
        self.report(Diagnostic::unsupported(span, message));
    }

    fn unsupported(&mut self, message: impl Into<String>) {
        let span = self.current_span();
        self.unsupported_at(span, message);
    }

    /// Whether an expression node `extra` levels below the open expression nodes stays within
    /// [`expr::MAX_EXPR_DEPTH`]; if not, reports it, marked "not supported yet". The report is made even while
    /// errors are quiet (arguments of a call without `CALL`): reading the statement another way would not help.
    fn within_expr_depth(&mut self, extra: u32) -> bool {
        if self.expr_nesting + extra <= expr::MAX_EXPR_DEPTH {
            return true;
        }
        let quiet = std::mem::replace(&mut self.quiet, false);
        self.unsupported(format!(
            "an expression nested more than {} levels deep",
            expr::MAX_EXPR_DEPTH
        ));
        self.quiet = quiet;
        self.quiet_failed |= quiet;
        false
    }

    /// Puts the rest of the statement into an `Error` node. Tokens left over without an error reported yet (e.g.
    /// `x = 5 6`) are an error themselves.
    fn recover(&mut self) {
        if self.at_stmt_end() {
            return;
        }
        self.syntax_error("expected the end of the statement");
        self.rest_into_error_node();
    }

    /// Puts the rest of the statement into an `Error` node, without a diagnostic.
    fn rest_into_error_node(&mut self) {
        self.start_node(Error);
        let mut depth = 0i32;
        while let Some(k) = self.current() {
            match k {
                Newline | MetaComment => break,
                Colon if depth <= 0 => break,
                Ident if depth <= 0 && self.line_if > 0 && self.at_word("ELSE") => break,
                LParen => depth += 1,
                RParen => depth -= 1,
                _ => {}
            }
            self.bump();
        }
        self.finish_node();
    }

    fn expect(&mut self, kind: SyntaxKind, what: &str) -> bool {
        if self.at(kind) {
            self.bump();
            true
        } else {
            self.syntax_error(format!("expected {what}"));
            false
        }
    }

    // ---- file and statements ----

    fn source_file(&mut self) {
        self.builder.start_node(SourceFile);
        loop {
            match self.body() {
                // A procedure ended by a nested header leaves that header's error in place, so the header reports
                // no second one.
                blocks::Stop::ProcHeader => self.carry_error = proc::proc_def(self),
                blocks::Stop::Eof => break,
                // Cannot happen: with no block open, every closer is a stray one (reported by `body`). If it did,
                // take a token so that the loop always ends.
                blocks::Stop::Closer(_) | blocks::Stop::Outer | blocks::Stop::LineEnd | blocks::Stop::InnerNext => {
                    self.bump()
                }
            }
        }
        // `$IF`s of this file still open: the old compiler's "$IF without $END IF". In an included file the
        // `$END IF` may follow in the including one: marked, and the level stays open for it.
        for (span, _) in std::mem::take(&mut self.pp_ifs) {
            if self.included {
                self.block_unsupported(span, "a `$IF` closed in another file");
            } else {
                self.block_error(span, "`$IF` without `$END IF`");
                let _ = self.pp.end_if();
            }
        }
        self.eat_trivia();
        self.finish_node();
    }

    /// At `SUB` or `FUNCTION` (the start of a procedure; only called at the start of a statement).
    fn at_proc_start(&self) -> bool {
        self.at_word("SUB") || self.at_word("FUNCTION")
    }

    /// One statement and the line end or `:` after it. A multi-line block takes care of its own end.
    fn statement_and_separator(&mut self) {
        self.line_prefix();
        if self.statement() {
            return;
        }
        if !self.at_stmt_end() {
            self.syntax_error("expected the end of the statement");
            self.recover();
        }
        self.separator();
    }

    /// The `:` or line end after a statement. Inside a single-line `IF` the line end is left for the statement
    /// that holds the `IF`.
    fn separator(&mut self) {
        if self.at(Colon) || (self.at(Newline) && self.line_if == 0) {
            self.bump();
        }
    }

    /// The end of a block header or closer: anything left over is an error; then the separator.
    fn header_end(&mut self) {
        self.recover();
        self.separator();
    }

    /// One statement. Returns true for a multi-line block, which has consumed its separator already.
    fn statement(&mut self) -> bool {
        match self.current() {
            None | Some(Newline) | Some(Colon) => {}
            Some(Metacommand) => return meta::metacommand(self),
            Some(MetaComment) => meta::meta_comment(self),
            Some(Question) => print::print_stmt(self),
            Some(Ident) => return self.word_statement(),
            Some(Number) => {
                // A line number at the start of a line is taken by `line_prefix`; anywhere else (after a label or
                // a `:`) the old compiler rejects it too (measured M3).
                self.error("a line number must stand first on its line");
                self.recover();
            }
            Some(_) => {
                self.syntax_error("expected a statement");
                self.recover();
            }
        }
        false
    }

    /// A statement that starts with a word. Returns true for a multi-line block (see [`Self::statement`]).
    fn word_statement(&mut self) -> bool {
        if let Some(c) = self.closer_here() {
            // Only reached where the statement loop does not look for closers (a single-line `IF`).
            self.stray_closer(c);
            return false;
        }
        let opens_block = ["IF", "FOR", "DO", "WHILE", "SELECT", "TYPE", "DEF", "DECLARE"]
            .iter()
            .any(|w| self.at_word(w));
        if opens_block && self.blocks.len() >= blocks::MAX_DEPTH {
            // Each open block is a few frames of recursion; this keeps a pathological file from overflowing the
            // stack.
            self.unsupported(format!("blocks nested more than {} deep", blocks::MAX_DEPTH));
            self.recover();
            return false;
        }
        if self.at_word("IF") {
            return blocks::if_stmt(self);
        } else if self.at_word("FOR") {
            blocks::for_block(self);
            return true;
        } else if self.at_word("DO") {
            blocks::do_block(self);
            return true;
        } else if self.at_word("WHILE") {
            blocks::while_block(self);
            return true;
        } else if self.at_word("SELECT") {
            blocks::select_block(self);
            return true;
        } else if self.at_word("TYPE") {
            blocks::type_block(self);
            return true;
        } else if self.at_word("DEF") {
            return blocks::def_stmt(self);
        } else if self.at_word("DECLARE") {
            return proc::declare_stmt(self);
        }
        self.simple_word_statement();
        false
    }

    fn simple_word_statement(&mut self) {
        if self.nth(1) == Some(Eq)
            && !keywords::is_keyword(name_part(self.nth_text(0)))
            && !self.at_word("END")
            && !self.at_word("SYSTEM")
            && !self.assignment_template()
        {
            // A word that is not reserved and `=`: an assignment, also when the word names a statement this
            // function dispatches on (`close = 3` inside `FUNCTION close`, measured `v19_proc_names`). `END` and
            // `SYSTEM` stay statements there ("Expected variable/value before '='", measured likewise).
            assign::assign_stmt(self)
        } else if self.at_word("PRINT") {
            print::print_stmt(self)
        } else if self.at_word("LPRINT") {
            print::lprint_stmt(self)
        } else if self.at_word("WRITE") {
            io::write_stmt(self)
        } else if self.at_word("INPUT") {
            io::input_stmt(self)
        } else if self.at_word("LINE") && self.nth_is_word(1, "INPUT") {
            io::line_input_stmt(self)
        } else if self.at_word("CLOSE") {
            io::close_stmt(self)
        } else if self.at_word("FIELD") {
            io::field_stmt(self)
        } else if self.at_word("LSET") || self.at_word("RSET") {
            io::set_stmt(self)
        } else if self.at_word("SWAP") {
            io::swap_stmt(self)
        } else if self.at_word("_MEMPUT") || self.at_word("_MEMFILL") {
            io::mem_stmt(self)
        } else if self.at_word("_ARRAYCOPY") {
            io::array_copy_stmt(self)
        } else if self.at_word("DIM") {
            decl::dim_stmt(self)
        } else if self.at_word("REDIM") {
            decl::redim_stmt(self)
        } else if self.at_word("COMMON") {
            decl::common_stmt(self)
        } else if self.at_word("ERASE") {
            decl::erase_stmt(self)
        } else if decl::DEF_TYPE_WORDS.iter().any(|w| self.at_word(w))
            || ((self.at_word("_DEFINE") || self.at_word("DEFINE")) && self.nth(1) == Some(Ident))
        {
            decl::def_type_stmt(self)
        } else if self.at_word("CONST") {
            decl::const_stmt(self)
        } else if self.at_word("OPTION") && self.nth(1) == Some(Ident) {
            // `OPTION` is not a reserved word; followed by anything but a word it is a name.
            decl::option_stmt(self)
        } else if self.at_word("SHARED") {
            decl::list_stmt(self, SharedStmt)
        } else if self.at_word("STATIC") {
            decl::list_stmt(self, StaticStmt)
        } else if self.at_word("LET") {
            assign::assign_stmt(self)
        } else if self.at_word("END") {
            self.end_stmt(EndStmt)
        } else if self.at_word("SYSTEM") {
            self.end_stmt(SystemStmt)
        } else if self.at_word("STOP") {
            self.word_and_operand(StopStmt)
        } else if self.at_word("RUN") {
            self.word_and_operand(RunStmt)
        } else if errors::EVENT_WORDS.iter().any(|w| self.at_word(w)) && errors::event_switch(self) {
            // `TIMER ON`, `KEY(1) OFF` (other statements starting with these words fall through).
        } else if self.at_word("CALL") {
            call::call_stmt(self)
        } else if self.at_word("EXIT") {
            proc::exit_stmt(self)
        } else if self.at_word("ON") {
            errors::on_stmt(self)
        } else if self.at_word("RESUME") {
            errors::resume_stmt(self)
        } else if self.at_word("ERROR") {
            errors::error_stmt(self)
        } else if self.at_word("GOTO") {
            flow::jump_stmt(self, GotoStmt)
        } else if self.at_word("GOSUB") {
            flow::jump_stmt(self, GosubStmt)
        } else if self.at_word("RETURN") {
            flow::jump_stmt(self, ReturnStmt)
        } else if self.at_word("DATA") {
            data::data_stmt(self)
        } else if self.at_word("READ") {
            data::read_stmt(self)
        } else if self.at_word("RESTORE") {
            data::restore_stmt(self)
        } else if self.template_statement() {
            // A built-in statement read by its template (`template.rs`).
        } else if self.nth(1) == Some(Eq)
            || (matches!(self.nth(1), Some(LParen | Dot)) && self.parens_then_eq() && !self.comma_outside_parens())
        {
            assign::assign_stmt(self)
        } else if keywords::is_keyword(name_part(self.nth_text(0))) {
            self.not_supported_statement()
        } else {
            // A SUB call without `CALL`, or a built-in statement (`CLS`); `sema` tells them apart.
            call::call_stmt(self)
        }
    }

    /// The statement's word names a built-in statement written as an assignment (`TIME$ = t$`, `_CLIPBOARD$ =
    /// s$`: templates starting with `=`).
    fn assignment_template(&self) -> bool {
        qb64rust_builtins::statement_templates(self.nth_text(0))
            .iter()
            .any(|(_, items)| matches!(items.first(), Some(qb64rust_builtins::template::Item::Punct(b'='))))
    }

    /// The token after the name is `(`, and after its matching `)` and any member accesses (`.b`, `.b(2)`) comes
    /// `=` (`a(1) = 2`, `a(1).b = 2`).
    fn parens_then_eq(&self) -> bool {
        let mut n = 1;
        loop {
            if self.nth(n) == Some(LParen) {
                match self.skip_parens(n) {
                    Some(after) => n = after,
                    None => return false,
                }
            }
            match self.nth(n) {
                Some(Eq) => return true,
                Some(Dot) if self.nth(n + 1) == Some(Ident) => n += 2,
                _ => return false,
            }
        }
    }

    /// The statement has a `,` outside parentheses. Then `s (5 / 2) = 2, 0` is a SUB call with a comparison as
    /// its first argument, not an assignment, which has no such comma. The scan stops at `ELSE` inside a
    /// single-line `IF` (`IF c THEN a(1) = 2 ELSE s 1, 2`).
    fn comma_outside_parens(&self) -> bool {
        let mut depth = 0i32;
        let mut n = 0;
        while !self.ends_at(n) {
            match self.nth(n) {
                Some(LParen) => depth += 1,
                Some(RParen) => depth -= 1,
                Some(Comma) if depth == 0 => return true,
                _ => {}
            }
            n += 1;
        }
        false
    }

    /// At the `(` at `n`: the index just after its matching `)`, or `None` if the statement ends first.
    fn skip_parens(&self, mut n: usize) -> Option<usize> {
        let mut depth = 0i32;
        loop {
            match self.nth(n) {
                Some(LParen) => depth += 1,
                Some(RParen) => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(n + 1);
                    }
                }
                None | Some(Newline) | Some(MetaComment) => return None,
                _ => {}
            }
            n += 1;
        }
    }

    /// The n-th non-trivia token ahead is an identifier spelled `word` (ASCII case ignored).
    fn nth_is_word(&self, n: usize, word: &str) -> bool {
        self.nth(n) == Some(Ident) && self.nth_text(n).eq_ignore_ascii_case(word.as_bytes())
    }

    /// `END` or `END code` (block closers such as `END IF` are taken by the statement loop), and `SYSTEM` or `SYSTEM
    /// code` (`kind` is `EndStmt` or `SystemStmt`): the exit code is an expression.
    fn end_stmt(&mut self, kind: SyntaxKind) {
        self.start_node(kind);
        self.bump(); // END or SYSTEM
        if !self.at_stmt_end() {
            expr::expr(self);
        }
        self.recover();
        self.finish_node();
    }

    /// `STOP`, or `RUN [line number|label|file name]`: a statement word with at most one operand (`kind` is
    /// `StopStmt` or `RunStmt`).
    fn word_and_operand(&mut self, kind: SyntaxKind) {
        self.start_node(kind);
        self.bump();
        if !self.at_stmt_end() {
            if self.at(Ident) && self.ends_at(1) && kind == RunStmt {
                // `RUN label`.
                self.bump();
            } else {
                expr::expr(self);
            }
        }
        self.recover();
        self.finish_node();
    }

    fn next_span(&self, n: usize) -> Span {
        match self.nth_index(n) {
            Some(i) => Span::new(self.file, self.offsets[i], self.offsets[i + 1]),
            None => self.current_span(),
        }
    }

    /// A language word the parser does not handle yet (`FOR`, `IF`, `GOTO`...).
    fn not_supported_statement(&mut self) {
        let word = qb64rust_base::show_bytes(self.nth_text(0));
        self.unsupported(format!("statement `{word}`"));
        self.recover();
    }
}

/// Whether a token of kind `k` (`None`: the end of the file) ends a statement.
pub(crate) fn ends_stmt(k: Option<SyntaxKind>) -> bool {
    matches!(k, None | Some(Newline | Colon | MetaComment))
}

/// A name without its type suffix (`a$` -> `a`).
pub(crate) fn name_part(text: &[u8]) -> &[u8] {
    let end = text
        .iter()
        .position(|&b| !(b.is_ascii_alphanumeric() || b == b'_' || b == b'.'))
        .unwrap_or(text.len());
    &text[..end]
}
