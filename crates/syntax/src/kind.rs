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
    /// `.` of member access (`a(1).b`): after `)` or a member name, followed (blanks allowed) by a name. Elsewhere
    /// a dot is part of a name (`a.b`) or a number.
    Dot,
    /// The items of a `DATA` statement, raw: everything after `DATA` and its blanks up to the line end or a `:`
    /// outside quotes (measured M2: `'` and `REM` are data there). The parser splits it into items.
    DataText,
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
    /// `AS <type words>`; in a `TypeField` also `* <size>` (`AS STRING * 8`).
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
    /// `EXIT SUB|FUNCTION|FOR|DO|WHILE|SELECT|CASE|DEF`
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
    /// A `Number` at the start of a line: a line number (the statement after it, if any, is a sibling node).
    LineNumber,
    /// `GOTO label|number`
    GotoStmt,
    /// `GOSUB label|number`
    GosubStmt,
    /// `RETURN [label|number]`
    ReturnStmt,
    /// `DATA` and a `DataText` token (none for `DATA` alone).
    DataStmt,
    /// `READ target, ...`
    ReadStmt,
    /// `RESTORE`, `RESTORE label`, `RESTORE 100`
    RestoreStmt,
    /// `CONST item, ...`
    ConstStmt,
    /// `name[suffix] = expr` of a `CONST`.
    ConstItem,
    /// `OPTION BASE n`, `OPTION _EXPLICIT`, `OPTION _EXPLICITARRAY` (and the spellings without `_`).
    OptionStmt,

    // ---- blocks (design D4): a header node, the body statements and, unless missing, the closer ----
    /// `END IF`/`ENDIF`, `END SELECT`, `WEND`, `END TYPE`, `END DECLARE` or `END DEF` closing a block.
    BlockEnd,
    /// Multi-line `IF`: an `IfBranch`, any `ElseIfBranch`es, an optional `ElseBranch`, and a `BlockEnd`.
    IfBlock,
    /// `IF cond THEN` or `ELSEIF cond THEN` (in a single-line `IF` also `IF cond` before `GOTO`).
    IfHeader,
    /// An `IfHeader` (`IF`) and the statements up to the next branch or the closer.
    IfBranch,
    /// An `IfHeader` (`ELSEIF`) and its statements; a statement may follow `THEN` on the same line.
    ElseIfBranch,
    /// `ELSE` and its statements; a statement may follow `ELSE` on the same line.
    ElseBranch,
    /// Single-line `IF`: an `IfHeader`, a `LineBranch`, and optionally `ELSE` and a second `LineBranch`. Each
    /// `ELSE` belongs to the innermost `IF` (measured M4).
    IfStmt,
    /// The statements of one branch of a single-line `IF`, separated by `:`; for `IF c GOTO x` the `GotoStmt`.
    LineBranch,
    /// A line number alone after `THEN` or `ELSE` (`IF c THEN 10 ELSE 20`): a jump.
    ImplicitGoto,
    /// `FOR` loop: a `ForHeader`, the body, and a `NextStmt` (none when an inner `NEXT j, i` closed it too).
    ForBlock,
    /// `FOR var = start TO end [STEP step]`
    ForHeader,
    /// `NEXT [var, ...]`; each variable closes one `FOR` block.
    NextStmt,
    /// `DO` loop: a `DoHeader`, the body and a `LoopStmt`.
    DoBlock,
    /// `DO [WHILE|UNTIL cond]`
    DoHeader,
    /// `LOOP [WHILE|UNTIL cond]`
    LoopStmt,
    /// `WHILE` loop: a `WhileHeader`, the body and a `BlockEnd` (`WEND`).
    WhileBlock,
    /// `WHILE cond`
    WhileHeader,
    /// `SELECT CASE`: a `SelectHeader`, statements before the first `CASE` (an error, kept for recovery),
    /// `CaseClause`s and a `BlockEnd`.
    SelectBlock,
    /// `SELECT CASE expr` or `SELECT EVERYCASE expr`
    SelectHeader,
    /// A `CaseHeader` and the statements up to the next `CASE` or `END SELECT`.
    CaseClause,
    /// `CASE ELSE` or `CASE item, ...`
    CaseHeader,
    /// `IS <op> expr`, `expr TO expr` or `expr`
    CaseItem,
    /// `TYPE` block: a `TypeHeader`, `TypeField`s and a `BlockEnd`.
    TypeBlock,
    /// `TYPE name`
    TypeHeader,
    /// `name [bounds] AS type` (one `FieldName`, then the `AsClause`) or `AS type name, ...` (the `AsClause`,
    /// then `FieldName`s).
    TypeField,
    /// A field's name, its element bounds (`ArrayBounds`) and `_DYNAMIC`/`_STATIC`, if any.
    FieldName,
    /// `(range, ...)` of an array declaration; a range is `expr [TO expr]`.
    ArrayBounds,
    /// `DECLARE [CUSTOMTYPE|DYNAMIC|STATIC] LIBRARY` block: a `DeclareLibraryHeader`, `ProcHeader`s (with `ALIAS`
    /// and `BYVAL`) and a `BlockEnd`.
    DeclareLibraryBlock,
    /// `DECLARE [CUSTOMTYPE|DYNAMIC|STATIC] LIBRARY ["name", ...]`
    DeclareLibraryHeader,
    /// Multi-line `DEF FNname[(params)]` … `END DEF`: a `DefFnHeader`, the body and a `BlockEnd`.
    DefFnBlock,
    /// `DEF FNname [(params)]`
    DefFnHeader,
    /// Single-line `DEF FNname[(params)] = expr`: a `DefFnHeader`, `=` and the expression.
    DefFnStmt,
    Literal,
    NameRef,
    CallExpr,
    /// `(` arguments `)`; an argument may be left out (`f(a, , b)`).
    ArgList,
    /// Member access after an index, call or other member: `<expr> . name [(args)]` (`a(1).b`, `a(1).b(2).c`).
    FieldExpr,
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
