//! Recursive-descent parser over the token stream (design D4). Statement families have their own modules,
//! dispatched by the first token (FreeBASIC lesson L1).
//!
//! Recovery: at most one error per statement. After an error the rest of the statement (up to a line end, or a
//! `:` outside parentheses) goes into an `Error` node and parsing continues with the next statement. Procedures
//! are blocks (`proc.rs`); their recovery is described there.

mod assign;
mod call;
mod decl;
mod expr;
pub(crate) mod keywords;
mod meta;
mod print;
mod proc;

use crate::SyntaxKind::{self, *};
use crate::lexer::{Token, tokenize};
use crate::tree::{GreenNode, TreeBuilder};
use qb64rust_base::{Diagnostics, FileId, Span};

pub struct Parse {
    pub green: GreenNode,
    pub diagnostics: Diagnostics,
}

pub fn parse(file: FileId, bytes: &[u8]) -> Parse {
    let tokens = tokenize(bytes);
    let mut offsets = Vec::with_capacity(tokens.len() + 1);
    let mut o = 0u32;
    for t in &tokens {
        offsets.push(o);
        o += t.len;
    }
    offsets.push(o);
    let mut p = Parser {
        file,
        bytes,
        tokens,
        offsets,
        pos: 0,
        builder: TreeBuilder::default(),
        diags: Diagnostics::new(),
        stmt_error: false,
        quiet: false,
        quiet_failed: false,
        last_kind: None,
    };
    p.source_file();
    Parse {
        green: p.builder.finish(),
        diagnostics: p.diags,
    }
}

pub(crate) struct Parser<'a> {
    file: FileId,
    bytes: &'a [u8],
    tokens: Vec<Token>,
    offsets: Vec<u32>,
    pos: usize,
    builder: TreeBuilder,
    diags: Diagnostics,
    /// An error was already reported for the current statement.
    stmt_error: bool,
    /// Errors are not reported but only noted in `quiet_failed` (arguments of a call without `CALL`).
    quiet: bool,
    quiet_failed: bool,
    /// Kind of the last non-trivia token added to the tree.
    last_kind: Option<SyntaxKind>,
}

impl<'a> Parser<'a> {
    // ---- token access (trivia skipped) ----

    fn nth_index(&self, n: usize) -> Option<usize> {
        let mut seen = 0;
        let mut i = self.pos;
        while i < self.tokens.len() {
            if !self.tokens[i].kind.is_trivia() {
                if seen == n {
                    return Some(i);
                }
                seen += 1;
            }
            i += 1;
        }
        None
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

    /// At a line end, a `:` or the end of the file.
    fn at_stmt_end(&self) -> bool {
        matches!(self.current(), None | Some(Newline) | Some(Colon))
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

    /// Reports an error at `span` unless the statement already has one.
    fn error_at(&mut self, span: Span, message: impl Into<String>) {
        if self.quiet {
            self.quiet_failed = true;
        } else if !self.stmt_error {
            self.stmt_error = true;
            self.diags.error(span, message);
        }
    }

    fn error(&mut self, message: impl Into<String>) {
        let span = self.current_span();
        self.error_at(span, message);
    }

    /// Puts the rest of the statement into an `Error` node. Tokens left over without an error reported yet (e.g.
    /// `x = 5 6`) are an error themselves.
    fn recover(&mut self) {
        if self.at_stmt_end() {
            return;
        }
        self.error("expected the end of the statement");
        self.rest_into_error_node();
    }

    /// Puts the rest of the statement into an `Error` node, without a diagnostic.
    fn rest_into_error_node(&mut self) {
        self.start_node(Error);
        let mut depth = 0i32;
        while let Some(k) = self.current() {
            match k {
                Newline => break,
                Colon if depth <= 0 => break,
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
            self.error(format!("expected {what}"));
            false
        }
    }

    // ---- file and statements ----

    fn source_file(&mut self) {
        self.builder.start_node(SourceFile);
        // A procedure ended by a nested header leaves that header's error in place, so the header reports no
        // second one.
        let mut keep_error = false;
        while self.current().is_some() {
            if !keep_error {
                self.stmt_error = false;
            }
            keep_error = false;
            if self.at_proc_start() {
                keep_error = proc::proc_def(self);
            } else {
                self.statement_and_separator();
            }
        }
        self.eat_trivia();
        self.finish_node();
    }

    /// At `SUB` or `FUNCTION` (the start of a procedure; only called at the start of a statement).
    fn at_proc_start(&self) -> bool {
        self.at_word("SUB") || self.at_word("FUNCTION")
    }

    /// One statement and the line end or `:` after it.
    fn statement_and_separator(&mut self) {
        self.statement();
        if !self.at_stmt_end() {
            self.error("expected the end of the statement");
            self.recover();
        }
        if matches!(self.current(), Some(Newline | Colon)) {
            self.bump();
        }
    }

    fn statement(&mut self) {
        match self.current() {
            None | Some(Newline) | Some(Colon) => {}
            Some(Metacommand) => meta::metacommand(self),
            Some(Question) => print::print_stmt(self),
            Some(Ident) => self.word_statement(),
            Some(Number) => {
                self.error("line numbers and labels are not supported yet");
                self.recover();
            }
            Some(_) => {
                self.error("expected a statement");
                self.recover();
            }
        }
    }

    fn word_statement(&mut self) {
        if self.at_word("PRINT") {
            print::print_stmt(self)
        } else if self.at_word("DIM") {
            decl::dim_stmt(self)
        } else if self.at_word("SHARED") {
            decl::list_stmt(self, SharedStmt)
        } else if self.at_word("STATIC") {
            decl::list_stmt(self, StaticStmt)
        } else if self.at_word("LET") {
            assign::assign_stmt(self)
        } else if self.at_word("END") {
            self.end_stmt()
        } else if self.at_word("CALL") {
            call::call_stmt(self)
        } else if self.at_word("EXIT") {
            proc::exit_stmt(self)
        } else if self.at_word("DECLARE") {
            proc::declare_stmt(self)
        } else if self.nth(1) == Some(Eq) || (self.nth(1) == Some(LParen) && self.parens_then_eq()) {
            assign::assign_stmt(self)
        } else if keywords::is_keyword(name_part(self.nth_text(0))) {
            self.not_supported_statement()
        } else {
            // A SUB call without `CALL`, or a built-in statement (`CLS`); `sema` tells them apart.
            call::call_stmt(self)
        }
    }

    /// The token after the name is `(`, and the token after its matching `)` is `=` (`a(1) = 2`).
    fn parens_then_eq(&self) -> bool {
        let mut depth = 0i32;
        let mut n = 1;
        loop {
            match self.nth(n) {
                Some(LParen) => depth += 1,
                Some(RParen) => {
                    depth -= 1;
                    if depth == 0 {
                        return self.nth(n + 1) == Some(Eq);
                    }
                }
                None | Some(Newline) => return false,
                _ => {}
            }
            n += 1;
        }
    }

    /// The n-th non-trivia token ahead is an identifier spelled `word` (ASCII case ignored).
    fn nth_is_word(&self, n: usize, word: &str) -> bool {
        self.nth(n) == Some(Ident) && self.nth_text(n).eq_ignore_ascii_case(word.as_bytes())
    }

    fn end_stmt(&mut self) {
        if self.nth(1).is_none_or(|k| matches!(k, Newline | Colon)) {
            self.start_node(EndStmt);
            self.bump();
            self.finish_node();
        } else if self.nth_is_word(1, "SUB") || self.nth_is_word(1, "FUNCTION") {
            // Inside a procedure the block loop takes `END SUB` before it gets here.
            let span = self.current_span().cover(self.next_span(1));
            let word = qb64rust_base::show_bytes(self.nth_text(1)).to_ascii_uppercase();
            self.error_at(span, format!("`END {word}` without a `{word}`"));
            self.recover();
        } else {
            let span = self.current_span().cover(self.next_span(1));
            let text = format!("END {}", qb64rust_base::show_bytes(self.nth_text(1)));
            self.error_at(span, format!("`{text}` is not supported yet"));
            self.recover();
        }
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
        self.error(format!("`{word}` is not supported yet"));
        self.recover();
    }
}

/// A name without its type suffix (`a$` -> `a`).
pub(crate) fn name_part(text: &[u8]) -> &[u8] {
    let end = text
        .iter()
        .position(|&b| !(b.is_ascii_alphanumeric() || b == b'_' || b == b'.'))
        .unwrap_or(text.len());
    &text[..end]
}
