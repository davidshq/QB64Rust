//! Recursive-descent parser over the token stream (design D4). Statement families have their own modules,
//! dispatched by the first token (FreeBASIC lesson L1).
//!
//! Recovery: at most one error per statement. After an error the rest of the statement (up to a line end, or a
//! `:` outside parentheses) goes into an `Error` node and parsing continues with the next statement.

mod assign;
mod decl;
mod expr;
mod meta;
mod print;

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
        if !self.stmt_error {
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
        while self.current().is_some() {
            self.stmt_error = false;
            self.statement();
            if !self.at_stmt_end() {
                self.error("expected the end of the statement");
                self.recover();
            }
            if matches!(self.current(), Some(Newline | Colon)) {
                self.bump();
            }
        }
        self.eat_trivia();
        self.finish_node();
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
        } else if self.at_word("LET") {
            assign::assign_stmt(self)
        } else if self.at_word("END") {
            self.end_stmt()
        } else if self.nth(1) == Some(Eq) {
            assign::assign_stmt(self)
        } else {
            self.not_supported_statement()
        }
    }

    fn end_stmt(&mut self) {
        if self.nth(1).is_none_or(|k| matches!(k, Newline | Colon)) {
            self.start_node(EndStmt);
            self.bump();
            self.finish_node();
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

    /// A statement word the slice does not handle (`FOR`, `CLS`, a SUB call, an array element assignment...).
    fn not_supported_statement(&mut self) {
        let word = qb64rust_base::show_bytes(self.nth_text(0));
        self.error(format!("`{word}` is not supported yet"));
        self.recover();
    }
}
