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
    /// A comment whose text after `'` or `REM` starts with `$` (`'$INCLUDE:'x.bi'`), without the line end. Not
    /// trivia: it ends the statement before it and is a statement of its own (`MetaCommentStmt`; `crate::meta`).
    MetaComment,
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
    /// A `MetaComment` token as a statement.
    MetaCommentStmt,
    PrintStmt,
    DimStmt,
    DimItem,
    /// `AS <type words>`
    AsClause,
    AssignStmt,
    EndStmt,
    /// `SYSTEM` (without an exit code).
    SystemStmt,
    /// `SUB`/`FUNCTION` block: a `ProcHeader`, the body statements and, unless missing, a `ProcEnd`.
    ProcDef,
    /// `SUB name [(params)]` or `FUNCTION name[suffix] [(params)]`; also inside a `DeclareStmt`.
    ProcHeader,
    /// `(param, ...)` of a procedure header.
    ParamList,
    /// `name[suffix] [AS type]`
    Param,
    /// `END SUB` / `END FUNCTION` closing a `ProcDef`.
    ProcEnd,
    /// `CALL name[(args)]` or `name [args]`. Without `CALL`, arguments the parser cannot read are kept in an
    /// `Error` node without a diagnostic; `sema` reports them, because only it knows built-in statement names.
    CallStmt,
    /// `EXIT SUB` / `EXIT FUNCTION`
    ExitStmt,
    /// `DECLARE SUB|FUNCTION <header>`
    DeclareStmt,
    /// `SHARED name [AS type], ...` inside a procedure; items are `DimItem`s.
    SharedStmt,
    /// `STATIC name [AS type], ...` inside a procedure; items are `DimItem`s.
    StaticStmt,
    /// `name:` at the start of a statement: a label (the statement after it, if any, is a sibling node).
    LabelDef,
    /// `ON ERROR GOTO label` / `ON ERROR GOTO 0`
    OnErrorStmt,
    /// `RESUME`, `RESUME 0`, `RESUME NEXT`, `RESUME label`
    ResumeStmt,
    /// `ERROR n`
    ErrorStmt,
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
