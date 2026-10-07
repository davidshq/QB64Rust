//! The lexer: bytes in, tokens (kind and byte length) out. Every byte of the input belongs to exactly one token,
//! so the tokens concatenated give the input back.

use crate::SyntaxKind::{self, *};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: SyntaxKind,
    pub len: u32,
}

pub fn tokenize(bytes: &[u8]) -> Vec<Token> {
    let mut lexer = Lexer {
        bytes,
        pos: 0,
        line_start: true,
        prev: None,
        member: false,
        data: false,
    };
    let mut tokens = Vec::new();
    while lexer.pos < bytes.len() {
        let start = lexer.pos;
        let kind = lexer.next_kind();
        debug_assert!(lexer.pos > start);
        tokens.push(Token {
            kind,
            len: qb64rust_base::to_u32(lexer.pos - start),
        });
        if kind == Newline || kind == LineContinuation || kind == Colon {
            lexer.line_start = true;
        } else if !kind.is_trivia() {
            lexer.line_start = false;
        }
        if !kind.is_trivia() {
            lexer.member = kind == Ident && lexer.prev == Some(Dot);
            lexer.prev = Some(kind);
        }
    }
    tokens
}

pub fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

pub fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// Type suffixes, longest first (`study\02` §1.2).
const SUFFIXES: &[&[u8]] = &[
    b"~%%", b"~&&", b"~%&", b"~`", b"%%", b"%&", b"~%", b"~&", b"&&", b"##", b"%", b"&", b"!", b"#", b"$", b"`",
];

struct Lexer<'a> {
    bytes: &'a [u8],
    pos: usize,
    /// At the start of a statement (only trivia since the last line end or `:`); `$` starts a metacommand there.
    line_start: bool,
    /// Kind of the last non-trivia token.
    prev: Option<SyntaxKind>,
    /// The last non-trivia token is a member name (an `Ident` after a `Dot`).
    member: bool,
    /// After the word `DATA`: the next token that is not a blank is `DataText` (unless the statement ends there).
    data: bool,
}

impl Lexer<'_> {
    fn peek(&self, ahead: usize) -> Option<u8> {
        self.bytes.get(self.pos + ahead).copied()
    }

    fn at(&self, s: &[u8]) -> bool {
        self.bytes[self.pos..].len() >= s.len() && self.bytes[self.pos..self.pos + s.len()].eq_ignore_ascii_case(s)
    }

    fn skip_to_line_end(&mut self) {
        while let Some(b) = self.peek(0) {
            if b == b'\r' || b == b'\n' {
                break;
            }
            self.pos += 1;
        }
    }

    fn eat_newline(&mut self) -> bool {
        match self.peek(0) {
            Some(b'\r') => {
                self.pos += if self.peek(1) == Some(b'\n') { 2 } else { 1 };
                true
            }
            Some(b'\n') => {
                self.pos += 1;
                true
            }
            _ => false,
        }
    }

    fn next_kind(&mut self) -> SyntaxKind {
        let b = self.bytes[self.pos];
        if self.data && !matches!(b, b' ' | b'\t') {
            self.data = false;
            if !matches!(b, b'\r' | b'\n' | b':') {
                return self.data_text();
            }
        }
        match b {
            b' ' | b'\t' => {
                while matches!(self.peek(0), Some(b' ' | b'\t')) {
                    self.pos += 1;
                }
                Whitespace
            }
            // A DOS end-of-file byte that ends a line is dropped by the old compiler's line reader (`qb64pe.bas`
            // 28060, `lineinput3`); elsewhere it stays an unexpected byte.
            0x1A if matches!(self.bytes.get(self.pos + 1), None | Some(b'\r' | b'\n')) => {
                self.pos += 1;
                Whitespace
            }
            b'\r' | b'\n' => {
                self.eat_newline();
                Newline
            }
            b'\'' => self.comment(self.pos),
            b'"' => {
                self.pos += 1;
                while let Some(c) = self.peek(0) {
                    if c == b'\r' || c == b'\n' {
                        break;
                    }
                    self.pos += 1;
                    if c == b'"' {
                        break;
                    }
                }
                StringLit
            }
            b'$' if self.line_start => {
                self.skip_to_line_end();
                Metacommand
            }
            b'_' if self.is_continuation() => {
                self.pos += 1;
                while matches!(self.peek(0), Some(b' ' | b'\t')) {
                    self.pos += 1;
                }
                self.eat_newline();
                LineContinuation
            }
            b'0'..=b'9' => self.number(),
            b'.' if self.peek(1).is_some_and(|c| c.is_ascii_digit()) => self.number(),
            b'.' if self.is_member_dot() => {
                self.pos += 1;
                Dot
            }
            b'&' if matches!(self.peek(1), Some(b'H' | b'h' | b'O' | b'o' | b'B' | b'b')) => self.radix_number(),
            _ if is_ident_start(b) => self.ident(),
            _ => self.punct(),
        }
    }

    /// `_` followed only by spaces or tabs up to the line end (or the end of the file).
    fn is_continuation(&self) -> bool {
        let mut i = self.pos + 1;
        while let Some(&c) = self.bytes.get(i) {
            match c {
                b' ' | b'\t' => i += 1,
                b'\r' | b'\n' => return true,
                _ => return false,
            }
        }
        true
    }

    /// A `.` at the current position is member access (design D3 of `m2-parser-breadth`, measured M8): it follows
    /// `)` or a member name, and a name follows it, with or without blanks on either side (`a(2) .b`, `a(2). b`).
    fn is_member_dot(&self) -> bool {
        if !(self.prev == Some(RParen) || self.member) {
            return false;
        }
        let mut i = self.pos + 1;
        while matches!(self.bytes.get(i), Some(b' ' | b'\t')) {
            i += 1;
        }
        self.bytes.get(i).is_some_and(|&c| is_ident_start(c))
    }

    fn ident(&mut self) -> SyntaxKind {
        let start = self.pos;
        // A member name ends at the next dot, which is member access again (`a(1).b.c`).
        let member = self.prev == Some(Dot);
        loop {
            while self.peek(0).is_some_and(is_ident_char) {
                self.pos += 1;
            }
            // Dots join name parts (`a.b`); a dot must be followed by a name character.
            if !member && self.peek(0) == Some(b'.') && self.peek(1).is_some_and(is_ident_char) {
                self.pos += 1;
                continue;
            }
            break;
        }
        // `REM` starts a comment that runs to the end of the line.
        if !member && self.bytes[start..self.pos].eq_ignore_ascii_case(b"REM") {
            return self.comment(start);
        }
        // `DATA` is reserved and never part of an expression, so it starts data mode wherever it stands (also
        // after `THEN`/`ELSE` of a single-line `IF`).
        if !member && self.bytes[start..self.pos].eq_ignore_ascii_case(b"DATA") && !self.has_suffix() {
            self.data = true;
            return Ident;
        }
        self.suffix();
        Ident
    }

    /// The rest of a comment that started at `start`: `Comment`, or `MetaComment` when it is a metacommand comment.
    fn comment(&mut self, start: usize) -> SyntaxKind {
        self.skip_to_line_end();
        if crate::meta::is_meta_comment(&self.bytes[start..self.pos]) {
            MetaComment
        } else {
            Comment
        }
    }

    /// A type suffix starts at the current position.
    fn has_suffix(&self) -> bool {
        SUFFIXES.iter().any(|s| self.at(s))
    }

    /// The items of a `DATA` statement up to the line end or a `:` outside quotes (`crate::data`).
    fn data_text(&mut self) -> SyntaxKind {
        self.pos = crate::data::scan(self.bytes, self.pos).end;
        DataText
    }

    fn suffix(&mut self) {
        for s in SUFFIXES {
            if self.at(s) {
                self.pos += s.len();
                if s.ends_with(b"`") {
                    while self.peek(0).is_some_and(|c| c.is_ascii_digit()) {
                        self.pos += 1;
                    }
                }
                return;
            }
        }
    }

    /// A decimal literal, scanned like `lineformat$`: digits, one `.`, one exponent letter `E`/`D`/`F` with an
    /// optional sign and digits; then a type suffix (integer suffixes only when there is no `.` or exponent).
    fn number(&mut self) -> SyntaxKind {
        let mut float = false;
        let mut exponent = false;
        while self.peek(0).is_some_and(|c| c.is_ascii_digit()) {
            self.pos += 1;
        }
        if self.peek(0) == Some(b'.') {
            float = true;
            self.pos += 1;
            while self.peek(0).is_some_and(|c| c.is_ascii_digit()) {
                self.pos += 1;
            }
        }
        if matches!(self.peek(0), Some(b'E' | b'e' | b'D' | b'd' | b'F' | b'f')) && !self.exponent_is_name() {
            float = true;
            exponent = true;
            self.pos += 1;
            if matches!(self.peek(0), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            while self.peek(0).is_some_and(|c| c.is_ascii_digit()) {
                self.pos += 1;
            }
        }
        if exponent {
            return Number; // no suffix is valid after an exponent letter
        }
        if self.at(b"##") {
            self.pos += 2;
        } else if matches!(self.peek(0), Some(b'!' | b'#')) {
            self.pos += 1;
        } else if !float {
            self.suffix();
        }
        Number
    }

    /// Whether an `E`/`D`/`F` after digits begins a name instead (`1 ELSE`-style text never occurs, but `5DIM`
    /// would not either; `lineformat$` takes the letter as an exponent unless another letter follows it).
    fn exponent_is_name(&self) -> bool {
        self.peek(1).is_some_and(|c| c.is_ascii_alphabetic() || c == b'_')
    }

    fn radix_number(&mut self) -> SyntaxKind {
        let radix = self.bytes[self.pos + 1].to_ascii_uppercase();
        self.pos += 2;
        let ok = |c: u8| match radix {
            b'H' => c.is_ascii_hexdigit(),
            b'O' => (b'0'..=b'7').contains(&c),
            _ => c == b'0' || c == b'1',
        };
        while self.peek(0).is_some_and(ok) {
            self.pos += 1;
        }
        self.suffix();
        Number
    }

    fn punct(&mut self) -> SyntaxKind {
        let b = self.bytes[self.pos];
        let next = self.peek(1);
        let (kind, len) = match (b, next) {
            (b'<', Some(b'=')) | (b'=', Some(b'<')) => (Le, 2),
            (b'>', Some(b'=')) | (b'=', Some(b'>')) => (Ge, 2),
            (b'<', Some(b'>')) | (b'>', Some(b'<')) => (Ne, 2),
            (b'+', _) => (Plus, 1),
            (b'-', _) => (Minus, 1),
            (b'*', _) => (Star, 1),
            (b'/', _) => (Slash, 1),
            (b'\\', _) => (Backslash, 1),
            (b'^', _) => (Caret, 1),
            (b'=', _) => (Eq, 1),
            (b'<', _) => (Lt, 1),
            (b'>', _) => (Gt, 1),
            (b'(', _) => (LParen, 1),
            (b')', _) => (RParen, 1),
            (b',', _) => (Comma, 1),
            (b';', _) => (Semicolon, 1),
            (b':', _) => (Colon, 1),
            (b'?', _) => (Question, 1),
            (b'#', _) => (Hash, 1),
            _ => (Unknown, 1),
        };
        self.pos += len;
        kind
    }
}
