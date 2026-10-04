//! Token and node kinds of the lossless tree.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SyntaxKind {
    // ---- tokens: trivia ----
    /// Spaces and tabs.
    Whitespace,
    /// `_` at the end of a line (optionally followed by spaces) and the line end after it.
    LineContinuation,
    /// `'` or `REM` and the rest of the line, without the line end.
    Comment,

    // ---- tokens ----
    /// CR LF, LF or a lone CR.
    Newline,
    /// `$` at the start of a statement and the rest of the line (`$CONSOLE:ONLY`).
    Metacommand,
    /// A name with its type suffix, if any. Keywords and word operators (`PRINT`, `MOD`) are identifiers too; the
    /// parser recognises them by text.
    Ident,
    /// A numeric literal in any form (`12`, `1.5E-3`, `5&&`, `&HFF`); typed later by `sema`.
    Number,
    /// `"…"`, closed by the next `"` or, unterminated, by the end of the line.
    StringLit,
    Plus,
    Minus,
    Star,
    Slash,
    Backslash,
    Caret,
    Eq,
    Lt,
    Gt,
    /// `<=` or `=<`
    Le,
    /// `>=` or `=>`
    Ge,
    /// `<>` or `><`
    Ne,
    LParen,
    RParen,
    Comma,
    Semicolon,
    Colon,
    /// `?`, short for `PRINT`.
    Question,
    Hash,
    /// A byte that starts no token (e.g. `@`, or 0x80–0xFF outside strings and comments).
    Unknown,

    // ---- nodes ----
    SourceFile,
    /// Tokens of a statement the parser could not handle, kept for the round trip.
    Error,
    MetaStmt,
    PrintStmt,
    DimStmt,
    DimItem,
    /// `AS <type words>`
    AsClause,
    AssignStmt,
    EndStmt,
    Literal,
    NameRef,
    CallExpr,
    ArgList,
    ParenExpr,
    PrefixExpr,
    BinExpr,
}

impl SyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(
            self,
            SyntaxKind::Whitespace | SyntaxKind::LineContinuation | SyntaxKind::Comment
        )
    }

    pub fn is_token(self) -> bool {
        (self as u16) < (SyntaxKind::SourceFile as u16)
    }
}
