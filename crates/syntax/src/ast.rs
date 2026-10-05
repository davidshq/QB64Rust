//! Typed accessors over the syntax tree (design D11 of `m2-procedures-and-errors`). One thin wrapper per node kind:
//! `cast` checks the kind, and every accessor returns an `Option` or an iterator, because a node from a statement
//! with a parse error may lack children. Callers never read children by position.

use crate::SyntaxKind::{self, *};
use crate::tree::{Element, Node, Tok};
use qb64rust_base::Span;

macro_rules! node_wrapper {
    ($($(#[$doc:meta])* $name:ident),* $(,)?) => {$(
        $(#[$doc])*
        #[derive(Clone, Copy, Debug)]
        pub struct $name<'a>(Node<'a>);

        impl<'a> $name<'a> {
            pub fn cast(node: Node<'a>) -> Option<$name<'a>> {
                (node.kind() == SyntaxKind::$name).then_some($name(node))
            }

            pub fn node(self) -> Node<'a> {
                self.0
            }
        }
    )*};
}

node_wrapper!(
    /// The whole file: a sequence of statements.
    SourceFile,
    /// `$CONSOLE:ONLY` and other metacommands.
    MetaStmt,
    /// A metacommand comment (`'$INCLUDE:'x.bi'`, `REM $DYNAMIC`).
    MetaCommentStmt,
    PrintStmt,
    DimStmt,
    DimItem,
    AsClause,
    AssignStmt,
    EndStmt,
    SystemStmt,
    ProcDef,
    ProcHeader,
    ParamList,
    Param,
    ProcEnd,
    CallStmt,
    ExitStmt,
    DeclareStmt,
    SharedStmt,
    StaticStmt,
    LabelDef,
    OnErrorStmt,
    ResumeStmt,
    ErrorStmt,
    LineNumber,
    GotoStmt,
    GosubStmt,
    ReturnStmt,
    DataStmt,
    ReadStmt,
    RestoreStmt,
    Literal,
    NameRef,
    CallExpr,
    ArgList,
    FieldExpr,
    ParenExpr,
    PrefixExpr,
    BinExpr,
);

/// The first child node that casts to `T`.
fn child<'a, T>(node: Node<'a>, cast: fn(Node<'a>) -> Option<T>) -> Option<T> {
    node.child_nodes().find_map(cast)
}

impl<'a> SourceFile<'a> {
    /// The statement nodes, `Error` nodes included (a statement the parser could not handle).
    pub fn statements(self) -> impl Iterator<Item = Node<'a>> + 'a {
        self.0.child_nodes()
    }
}

impl MetaStmt<'_> {
    /// The `Metacommand` token: `$` and the rest of the line.
    pub fn token(self) -> Option<Tok> {
        self.0.child_tokens().find(|t| t.kind == Metacommand)
    }
}

impl MetaCommentStmt<'_> {
    /// The `MetaComment` token: `'` or `REM` and the rest of the line.
    pub fn token(self) -> Option<Tok> {
        self.0.child_tokens().find(|t| t.kind == MetaComment)
    }
}

/// One part of a `PRINT` list, in source order.
#[derive(Clone, Copy, Debug)]
pub enum PrintPart<'a> {
    Expr(Expr<'a>),
    Semicolon(Tok),
    Comma(Tok),
}

impl<'a> PrintStmt<'a> {
    /// The items and separators after the `PRINT` or `?` keyword.
    pub fn parts(self) -> impl Iterator<Item = PrintPart<'a>> + 'a {
        self.0.children().filter_map(|e| match e {
            Element::Token(t) if t.kind == Semicolon => Some(PrintPart::Semicolon(t)),
            Element::Token(t) if t.kind == Comma => Some(PrintPart::Comma(t)),
            Element::Node(n) => Expr::cast(n).map(PrintPart::Expr),
            Element::Token(_) => None,
        })
    }
}

impl<'a> DimStmt<'a> {
    /// The `SHARED` word of `DIM SHARED`.
    pub fn shared(self) -> Option<Tok> {
        self.0.child_tokens().nth(1).filter(|t| t.kind == Ident)
    }

    pub fn items(self) -> impl Iterator<Item = DimItem<'a>> + 'a {
        self.0.child_nodes().filter_map(DimItem::cast)
    }
}

impl<'a> SharedStmt<'a> {
    pub fn items(self) -> impl Iterator<Item = DimItem<'a>> + 'a {
        self.0.child_nodes().filter_map(DimItem::cast)
    }
}

impl<'a> StaticStmt<'a> {
    pub fn items(self) -> impl Iterator<Item = DimItem<'a>> + 'a {
        self.0.child_nodes().filter_map(DimItem::cast)
    }
}

impl<'a> ProcDef<'a> {
    pub fn header(self) -> Option<ProcHeader<'a>> {
        child(self.0, ProcHeader::cast)
    }

    /// The body's statement nodes, `Error` nodes included; not the header or the `END SUB`.
    pub fn body(self) -> impl Iterator<Item = Node<'a>> + 'a {
        self.0
            .child_nodes()
            .filter(|n| !matches!(n.kind(), SyntaxKind::ProcHeader | SyntaxKind::ProcEnd))
    }

    /// The closing `END SUB`/`END FUNCTION`; `None` when it is missing.
    pub fn end(self) -> Option<ProcEnd<'a>> {
        child(self.0, ProcEnd::cast)
    }
}

impl<'a> ProcHeader<'a> {
    /// The `SUB` or `FUNCTION` word.
    pub fn keyword(self) -> Option<Tok> {
        self.0.child_tokens().next()
    }

    /// The procedure's name, with its suffix.
    pub fn name(self) -> Option<Tok> {
        self.0.child_tokens().nth(1).filter(|t| t.kind == Ident)
    }

    pub fn param_list(self) -> Option<ParamList<'a>> {
        child(self.0, ParamList::cast)
    }

    /// The parameters; none without a parameter list.
    pub fn params(self) -> impl Iterator<Item = Param<'a>> + 'a {
        self.param_list().into_iter().flat_map(|l| l.params())
    }
}

impl<'a> ParamList<'a> {
    pub fn params(self) -> impl Iterator<Item = Param<'a>> + 'a {
        self.0.child_nodes().filter_map(Param::cast)
    }
}

impl<'a> Param<'a> {
    /// The parameter's name, with its suffix.
    pub fn name(self) -> Option<Tok> {
        self.0.child_tokens().find(|t| t.kind == Ident)
    }

    pub fn as_clause(self) -> Option<AsClause<'a>> {
        child(self.0, AsClause::cast)
    }
}

impl ProcEnd<'_> {
    /// The `SUB` or `FUNCTION` word after `END`.
    pub fn keyword(self) -> Option<Tok> {
        self.0.child_tokens().nth(1)
    }
}

impl<'a> CallStmt<'a> {
    /// The `CALL` word, when written.
    pub fn call_keyword(self) -> Option<Tok> {
        self.0
            .children()
            .take_while(|e| e.as_node().is_none())
            .filter_map(|e| e.as_token())
            .find(|t| t.kind == Ident)
    }

    /// The called name, with its suffix.
    pub fn name(self) -> Option<Tok> {
        child(self.0, NameRef::cast).and_then(|n| n.name())
    }

    /// The arguments, with or without parentheses; `None` when there are none.
    pub fn arg_list(self) -> Option<ArgList<'a>> {
        child(self.0, ArgList::cast)
    }

    /// Arguments the parser could not read as expressions (only without `CALL`; no diagnostic was reported).
    pub fn unparsed_args(self) -> Option<Node<'a>> {
        self.arg_list()?
            .node()
            .child_nodes()
            .find(|n| n.kind() == SyntaxKind::Error)
    }
}

impl ExitStmt<'_> {
    /// The `SUB` or `FUNCTION` word after `EXIT`.
    pub fn keyword(self) -> Option<Tok> {
        self.0.child_tokens().nth(1)
    }
}

impl<'a> DeclareStmt<'a> {
    pub fn header(self) -> Option<ProcHeader<'a>> {
        child(self.0, ProcHeader::cast)
    }
}

impl LabelDef<'_> {
    /// The label's name (without the `:`).
    pub fn name(self) -> Option<Tok> {
        self.0.child_tokens().find(|t| t.kind == Ident)
    }
}

impl OnErrorStmt<'_> {
    /// The label after `GOTO`, or the `Number` token (`0`).
    pub fn target(self) -> Option<Tok> {
        self.0
            .child_tokens()
            .filter(|t| matches!(t.kind, Ident | Number))
            .nth(3)
    }
}

impl ResumeStmt<'_> {
    /// The word or number after `RESUME` (`NEXT`, `0`, a label); `None` for a bare `RESUME`.
    pub fn target(self) -> Option<Tok> {
        self.0
            .child_tokens()
            .filter(|t| matches!(t.kind, Ident | Number))
            .nth(1)
    }
}

impl<'a> ErrorStmt<'a> {
    /// The error number's expression.
    pub fn value(self) -> Option<Expr<'a>> {
        child(self.0, Expr::cast)
    }
}

impl LineNumber<'_> {
    /// The `Number` token.
    pub fn number(self) -> Option<Tok> {
        self.0.child_tokens().find(|t| t.kind == Number)
    }
}

/// The label or line number after the statement's word; `None` when missing (or for a bare `RETURN`).
fn jump_target(node: Node<'_>) -> Option<Tok> {
    node.child_tokens().filter(|t| matches!(t.kind, Ident | Number)).nth(1)
}

impl GotoStmt<'_> {
    pub fn target(self) -> Option<Tok> {
        jump_target(self.0)
    }
}

impl GosubStmt<'_> {
    pub fn target(self) -> Option<Tok> {
        jump_target(self.0)
    }
}

impl ReturnStmt<'_> {
    pub fn target(self) -> Option<Tok> {
        jump_target(self.0)
    }
}

/// One item of a `DATA` statement: its text without the blanks around it (`span`), quotes included when
/// `quoted`. An empty item (`DATA a,,b`, a trailing `,`) has an empty span.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataItem {
    pub span: Span,
    pub quoted: bool,
}

impl DataStmt<'_> {
    /// The `DataText` token; `None` for `DATA` with nothing after it.
    pub fn text(self) -> Option<Tok> {
        self.0.child_tokens().find(|t| t.kind == DataText)
    }

    /// The items, split from `bytes` (the file's bytes) by the old compiler's rule (`crate::data`); `n` commas
    /// give `n + 1` items (measured M2). None without a `DataText` token; whether the old compiler gives bare
    /// `DATA` one empty item is not measured (matters once `READ` is implemented).
    pub fn items(self, bytes: &[u8]) -> Vec<DataItem> {
        let Some(t) = self.text() else {
            return Vec::new();
        };
        let span = |s: usize, e: usize| Span::new(t.span.file, qb64rust_base::to_u32(s), qb64rust_base::to_u32(e));
        crate::data::scan(bytes, t.span.start as usize)
            .items
            .into_iter()
            .map(|it| DataItem {
                span: span(it.start, it.end),
                quoted: it.quoted,
            })
            .collect()
    }
}

impl<'a> ReadStmt<'a> {
    /// The targets, in order (parsed as expressions; `sema` checks that each is a variable).
    pub fn targets(self) -> impl Iterator<Item = Expr<'a>> + 'a {
        self.0.child_nodes().filter_map(Expr::cast)
    }
}

impl RestoreStmt<'_> {
    /// The label or line number after `RESTORE`; `None` for a bare `RESTORE`.
    pub fn target(self) -> Option<Tok> {
        jump_target(self.0)
    }
}

impl<'a> DimItem<'a> {
    /// The declared name, with its suffix.
    pub fn name(self) -> Option<Tok> {
        self.0.child_tokens().find(|t| t.kind == Ident)
    }

    pub fn as_clause(self) -> Option<AsClause<'a>> {
        child(self.0, AsClause::cast)
    }
}

impl<'a> AsClause<'a> {
    /// The type words after `AS` (`LONG`, or `_UNSIGNED` `LONG`).
    pub fn type_words(self) -> impl Iterator<Item = Tok> + 'a {
        self.0.child_tokens().filter(|t| t.kind == Ident).skip(1)
    }
}

impl<'a> AssignStmt<'a> {
    /// What is assigned (after an optional `LET`): a `NameRef`, or a `CallExpr` or `FieldExpr` for an index or
    /// member access.
    pub fn target(self) -> Option<Expr<'a>> {
        let eq_start = self
            .0
            .child_tokens()
            .find(|t| t.kind == Eq)
            .map_or(u32::MAX, |t| t.span.start);
        self.0
            .child_nodes()
            .filter(|n| n.offset < eq_start)
            .find_map(Expr::cast)
    }

    /// The expression after `=`.
    pub fn value(self) -> Option<Expr<'a>> {
        let eq_end = self.0.child_tokens().find(|t| t.kind == Eq)?.span.end;
        expr_after(self.0, eq_end)
    }
}

/// An expression node of any kind.
#[derive(Clone, Copy, Debug)]
pub enum Expr<'a> {
    Literal(Literal<'a>),
    NameRef(NameRef<'a>),
    Call(CallExpr<'a>),
    Field(FieldExpr<'a>),
    Paren(ParenExpr<'a>),
    Prefix(PrefixExpr<'a>),
    Bin(BinExpr<'a>),
}

impl<'a> Expr<'a> {
    pub fn cast(node: Node<'a>) -> Option<Expr<'a>> {
        Some(match node.kind() {
            SyntaxKind::Literal => Expr::Literal(Literal(node)),
            SyntaxKind::NameRef => Expr::NameRef(NameRef(node)),
            SyntaxKind::CallExpr => Expr::Call(CallExpr(node)),
            SyntaxKind::FieldExpr => Expr::Field(FieldExpr(node)),
            SyntaxKind::ParenExpr => Expr::Paren(ParenExpr(node)),
            SyntaxKind::PrefixExpr => Expr::Prefix(PrefixExpr(node)),
            SyntaxKind::BinExpr => Expr::Bin(BinExpr(node)),
            _ => return None,
        })
    }

    pub fn node(self) -> Node<'a> {
        match self {
            Expr::Literal(x) => x.0,
            Expr::NameRef(x) => x.0,
            Expr::Call(x) => x.0,
            Expr::Field(x) => x.0,
            Expr::Paren(x) => x.0,
            Expr::Prefix(x) => x.0,
            Expr::Bin(x) => x.0,
        }
    }
}

impl Literal<'_> {
    /// The `Number` or `StringLit` token.
    pub fn token(self) -> Option<Tok> {
        self.0
            .child_tokens()
            .find(|t| matches!(t.kind, Number | SyntaxKind::StringLit))
    }
}

impl NameRef<'_> {
    /// The name, with its suffix.
    pub fn name(self) -> Option<Tok> {
        self.0.child_tokens().find(|t| t.kind == Ident)
    }
}

impl<'a> CallExpr<'a> {
    /// The called name, with its suffix.
    pub fn name(self) -> Option<Tok> {
        child(self.0, NameRef::cast).and_then(|n| n.name())
    }

    pub fn arg_list(self) -> Option<ArgList<'a>> {
        child(self.0, ArgList::cast)
    }
}

impl<'a> ArgList<'a> {
    /// One entry per argument position, `None` where the argument is left out (`f(a, , b)` gives three, the
    /// second `None`); `()` gives none.
    pub fn args(self) -> Vec<Option<Expr<'a>>> {
        let mut out = Vec::new();
        let mut cur = None;
        let mut any = false;
        for e in self.0.children() {
            match e {
                Element::Node(n) => {
                    if let Some(x) = Expr::cast(n) {
                        cur = Some(x);
                        any = true;
                    }
                }
                Element::Token(t) if t.kind == Comma => {
                    out.push(cur.take());
                    any = true;
                }
                Element::Token(_) => {}
            }
        }
        if any {
            out.push(cur);
        }
        out
    }
}

impl<'a> FieldExpr<'a> {
    /// The expression whose member is taken (`a(1)` in `a(1).b`).
    pub fn base(self) -> Option<Expr<'a>> {
        child(self.0, Expr::cast)
    }

    /// The member name, with its suffix.
    pub fn member(self) -> Option<Tok> {
        self.0.child_tokens().find(|t| t.kind == Ident)
    }

    /// The index of an array member (`(2)` in `a(1).b(2)`).
    pub fn arg_list(self) -> Option<ArgList<'a>> {
        child(self.0, ArgList::cast)
    }
}

impl<'a> ParenExpr<'a> {
    pub fn inner(self) -> Option<Expr<'a>> {
        child(self.0, Expr::cast)
    }
}

impl<'a> PrefixExpr<'a> {
    /// The operator token (`-`, `NOT`, `_NEGATE`).
    pub fn op(self) -> Option<Tok> {
        self.0.child_tokens().next()
    }

    pub fn operand(self) -> Option<Expr<'a>> {
        child(self.0, Expr::cast)
    }
}

impl<'a> BinExpr<'a> {
    pub fn lhs(self) -> Option<Expr<'a>> {
        child(self.0, Expr::cast)
    }

    /// The operator token, between the operands.
    pub fn op(self) -> Option<Tok> {
        self.0.child_tokens().next()
    }

    pub fn rhs(self) -> Option<Expr<'a>> {
        // The right operand is the expression that starts after the operator.
        let op_end = self.op()?.span.end;
        expr_after(self.0, op_end)
    }
}

/// The first child expression that starts at or after `offset`.
fn expr_after(node: Node<'_>, offset: u32) -> Option<Expr<'_>> {
    node.child_nodes().filter(|n| n.offset >= offset).find_map(Expr::cast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree::TreeId;
    use qb64rust_base::FileId;

    fn target_name(assign: AssignStmt) -> Option<Tok> {
        match assign.target() {
            Some(Expr::NameRef(n)) => n.name(),
            _ => None,
        }
    }

    fn first_stmt<'a>(green: &'a crate::tree::GreenNode) -> Node<'a> {
        let root = SourceFile::cast(Node::root(green, TreeId(0), FileId(0))).unwrap();
        root.statements().next().unwrap()
    }

    #[test]
    fn binary_operands_and_operator() {
        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"x = a + -b\n");
        let assign = AssignStmt::cast(first_stmt(&p.green)).unwrap();
        assert_eq!(target_name(assign).map(|t| t.span.start), Some(0));
        let Some(Expr::Bin(bin)) = assign.value() else {
            panic!("not a BinExpr")
        };
        assert!(matches!(bin.lhs(), Some(Expr::NameRef(_))));
        assert_eq!(bin.op().map(|t| t.kind), Some(Plus));
        assert!(matches!(bin.rhs(), Some(Expr::Prefix(_))));
    }

    #[test]
    fn assignment_of_a_name() {
        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"LET d# = s!\n");
        let assign = AssignStmt::cast(first_stmt(&p.green)).unwrap();
        assert_eq!(target_name(assign).map(|t| t.span.start), Some(4));
        let Some(Expr::NameRef(value)) = assign.value() else {
            panic!("not a NameRef")
        };
        assert_eq!(value.name().map(|t| t.span.start), Some(9));
    }

    #[test]
    fn missing_children_give_none() {
        // `x = ` has no value; `a +` has no right operand.
        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"x =\n");
        let assign = AssignStmt::cast(first_stmt(&p.green)).unwrap();
        assert!(assign.target().is_some());
        assert!(assign.value().is_none());

        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"x = a +\n");
        let assign = AssignStmt::cast(first_stmt(&p.green)).unwrap();
        let Some(Expr::Bin(bin)) = assign.value() else {
            panic!("not a BinExpr")
        };
        assert!(bin.lhs().is_some());
        assert!(bin.rhs().is_none());
    }

    #[test]
    fn dim_items_and_type_words() {
        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"DIM a AS LONG, b%\n");
        let dim = DimStmt::cast(first_stmt(&p.green)).unwrap();
        let items: Vec<_> = dim.items().collect();
        assert_eq!(items.len(), 2);
        let words: Vec<u32> = items[0]
            .as_clause()
            .unwrap()
            .type_words()
            .map(|t| t.span.start)
            .collect();
        assert_eq!(words, vec![9]);
        assert!(items[1].as_clause().is_none());
        assert_eq!(items[1].name().map(|t| t.span.start), Some(15));
    }

    fn procs(green: &crate::tree::GreenNode) -> Vec<ProcDef<'_>> {
        let root = SourceFile::cast(Node::root(green, TreeId(0), FileId(0))).unwrap();
        root.statements().filter_map(ProcDef::cast).collect()
    }

    #[test]
    fn procedure_parts() {
        let src = b"FUNCTION f& (a AS LONG, b$)\nf& = a\nEND FUNCTION\n";
        let p = crate::parser::parse_tree(TreeId(0), FileId(0), src);
        let [f] = procs(&p.green)[..] else {
            panic!("one ProcDef")
        };
        let h = f.header().unwrap();
        assert_eq!(h.keyword().map(|t| t.span.start), Some(0));
        assert_eq!(h.name().map(|t| (t.span.start, t.span.end)), Some((9, 11)));
        let params: Vec<_> = h.params().collect();
        assert_eq!(params.len(), 2);
        assert!(params[0].as_clause().is_some());
        assert_eq!(params[1].name().map(|t| t.span.start), Some(24));
        assert!(params[1].as_clause().is_none());
        assert_eq!(f.body().count(), 1);
        assert!(AssignStmt::cast(f.body().next().unwrap()).is_some());
        assert_eq!(f.end().and_then(|e| e.keyword()).map(|t| t.span.start), Some(39));
    }

    #[test]
    fn missing_end_gives_none() {
        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"SUB a\nPRINT 1\n");
        let [a] = procs(&p.green)[..] else {
            panic!("one ProcDef")
        };
        assert!(a.end().is_none());
        assert_eq!(a.body().count(), 1);
    }

    #[test]
    fn nested_header_ends_the_outer_block() {
        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"SUB a\nPRINT 1\nSUB b\nEND SUB\n");
        let [a, b] = procs(&p.green)[..] else {
            panic!("two ProcDefs")
        };
        assert!(a.end().is_none());
        assert!(b.end().is_some());
    }

    #[test]
    fn wrong_end_kind_still_closes() {
        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"SUB a\nEND FUNCTION\n");
        let [a] = procs(&p.green)[..] else {
            panic!("one ProcDef")
        };
        assert_eq!(a.end().and_then(|e| e.keyword()).map(|t| t.span.start), Some(10));
    }

    #[test]
    fn header_without_a_name() {
        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"SUB\nEND SUB\n");
        let [a] = procs(&p.green)[..] else {
            panic!("one ProcDef")
        };
        let h = a.header().unwrap();
        assert!(h.keyword().is_some());
        assert!(h.name().is_none());
        assert_eq!(h.params().count(), 0);
    }

    #[test]
    fn call_statement_parts() {
        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"CALL s(n)\n");
        let c = CallStmt::cast(first_stmt(&p.green)).unwrap();
        assert!(c.call_keyword().is_some());
        assert_eq!(c.name().map(|t| t.span.start), Some(5));
        assert!(matches!(
            c.arg_list().unwrap().args().first(),
            Some(Some(Expr::NameRef(_)))
        ));
        assert!(c.unparsed_args().is_none());

        // Without CALL the parentheses make a ParenExpr argument (by value).
        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"s (n)\n");
        let c = CallStmt::cast(first_stmt(&p.green)).unwrap();
        assert!(c.call_keyword().is_none());
        assert!(matches!(
            c.arg_list().unwrap().args().first(),
            Some(Some(Expr::Paren(_)))
        ));

        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"t\n");
        let c = CallStmt::cast(first_stmt(&p.green)).unwrap();
        assert!(c.arg_list().is_none());
        assert!(c.unparsed_args().is_none());

        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"LOCATE , 5\n");
        assert!(p.diagnostics.list().is_empty());
        let c = CallStmt::cast(first_stmt(&p.green)).unwrap();
        assert!(c.unparsed_args().is_some());
    }

    #[test]
    fn declare_exit_shared() {
        let p = crate::parser::parse_tree(
            TreeId(0),
            FileId(0),
            b"DECLARE SUB s (a)\nDIM SHARED g\nDIM h\nEXIT SUB\n",
        );
        let root = SourceFile::cast(Node::root(&p.green, TreeId(0), FileId(0))).unwrap();
        let s: Vec<_> = root.statements().collect();
        let d = DeclareStmt::cast(s[0]).unwrap();
        assert_eq!(d.header().unwrap().params().count(), 1);
        assert!(DimStmt::cast(s[1]).unwrap().shared().is_some());
        assert!(DimStmt::cast(s[2]).unwrap().shared().is_none());
        // `EXIT` starts at 37, `SUB` at 42.
        assert_eq!(ExitStmt::cast(s[3]).unwrap().keyword().map(|t| t.span.start), Some(42));
    }

    #[test]
    fn labels_and_error_statements() {
        let src = b"h: ON ERROR GOTO h\nON ERROR GOTO 0\nRESUME\nRESUME NEXT\nERROR 5\nON ERROR GOTO\n";
        let p = crate::parser::parse_tree(TreeId(0), FileId(0), src);
        let root = SourceFile::cast(Node::root(&p.green, TreeId(0), FileId(0))).unwrap();
        let s: Vec<_> = root.statements().collect();
        assert_eq!(LabelDef::cast(s[0]).unwrap().name().map(|t| t.span), Some(t(0, 1)));
        let on = OnErrorStmt::cast(s[1]).unwrap();
        assert_eq!(on.target().map(|t| (t.kind, t.span)), Some((Ident, t(17, 18))));
        let on0 = OnErrorStmt::cast(s[2]).unwrap();
        assert_eq!(on0.target().map(|t| t.kind), Some(Number));
        assert!(ResumeStmt::cast(s[3]).unwrap().target().is_none());
        assert_eq!(
            ResumeStmt::cast(s[4]).unwrap().target().map(|t| t.span),
            Some(t(49, 53))
        );
        assert!(matches!(ErrorStmt::cast(s[5]).unwrap().value(), Some(Expr::Literal(_))));
        // `ON ERROR GOTO` without a target: the node is there, the target is not.
        assert!(OnErrorStmt::cast(s[6]).unwrap().target().is_none());
        assert_eq!(p.diagnostics.list().len(), 1);
    }

    fn t(start: u32, end: u32) -> qb64rust_base::Span {
        qb64rust_base::Span::new(FileId(0), start, end)
    }

    #[test]
    fn member_access_and_omitted_arguments() {
        // `a(1).b.c(2)`: FieldExpr(FieldExpr(CallExpr a(1), b), c, (2)).
        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"PRINT a(1).b.c(2); x(2) .y; z(3). w\n");
        assert!(p.diagnostics.list().is_empty());
        let print = PrintStmt::cast(first_stmt(&p.green)).unwrap();
        let exprs: Vec<_> = print
            .parts()
            .filter_map(|part| match part {
                PrintPart::Expr(e) => Some(e),
                PrintPart::Semicolon(_) | PrintPart::Comma(_) => None,
            })
            .collect();
        assert_eq!(exprs.len(), 3);
        let Expr::Field(outer) = exprs[0] else {
            panic!("not a FieldExpr")
        };
        assert_eq!(outer.member().map(|t| t.span), Some(t(13, 14)));
        assert_eq!(outer.arg_list().unwrap().args().len(), 1);
        let Some(Expr::Field(inner)) = outer.base() else {
            panic!("not a FieldExpr")
        };
        assert_eq!(inner.member().map(|t| t.span), Some(t(11, 12)));
        assert!(inner.arg_list().is_none());
        assert!(matches!(inner.base(), Some(Expr::Call(_))));
        assert!(matches!(exprs[1], Expr::Field(_)));
        assert!(matches!(exprs[2], Expr::Field(_)));

        // A dotted name without an index stays one name.
        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"PRINT a.b\n");
        let print = PrintStmt::cast(first_stmt(&p.green)).unwrap();
        assert!(matches!(print.parts().next(), Some(PrintPart::Expr(Expr::NameRef(_)))));

        // Assignment to a member.
        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"a(1).b(2).c = 5\n");
        assert!(p.diagnostics.list().is_empty());
        let assign = AssignStmt::cast(first_stmt(&p.green)).unwrap();
        assert!(matches!(assign.target(), Some(Expr::Field(_))));
        assert!(matches!(assign.value(), Some(Expr::Literal(_))));

        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"x = f(a, , b) + g(, ) + h()\n");
        assert!(p.diagnostics.list().is_empty());
        let assign = AssignStmt::cast(first_stmt(&p.green)).unwrap();
        let mut calls = Vec::new();
        let mut e = assign.value();
        while let Some(Expr::Bin(b)) = e {
            let Some(Expr::Call(c)) = b.rhs() else {
                panic!("not a call")
            };
            calls.push(c);
            e = b.lhs();
        }
        let Some(Expr::Call(f)) = e else { panic!("not a call") };
        calls.push(f);
        let shape = |c: CallExpr| -> Vec<bool> { c.arg_list().unwrap().args().iter().map(Option::is_some).collect() };
        assert_eq!(shape(calls[2]), vec![true, false, true]);
        assert_eq!(shape(calls[1]), vec![false, false]);
        assert_eq!(shape(calls[0]), Vec::<bool>::new());
    }

    #[test]
    fn cast_checks_the_kind() {
        let p = crate::parser::parse_tree(TreeId(0), FileId(0), b"PRINT 1; 2\n");
        let stmt = first_stmt(&p.green);
        assert!(DimStmt::cast(stmt).is_none());
        let print = PrintStmt::cast(stmt).unwrap();
        let kinds: Vec<&str> = print
            .parts()
            .map(|part| match part {
                PrintPart::Expr(_) => "expr",
                PrintPart::Semicolon(_) => ";",
                PrintPart::Comma(_) => ",",
            })
            .collect();
        assert_eq!(kinds, vec!["expr", ";", "expr"]);
    }
}
