//! Typed accessors over the syntax tree (design D11 of `m2-procedures-and-errors`). One thin wrapper per node kind:
//! `cast` checks the kind, and every accessor returns an `Option` or an iterator, because a node from a statement
//! with a parse error may lack children. Callers never read children by position.

use crate::SyntaxKind::{self, *};
use crate::tree::{Element, Node, Tok};

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
    Literal,
    NameRef,
    CallExpr,
    ArgList,
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
    /// The assigned name (after an optional `LET`).
    pub fn target(self) -> Option<NameRef<'a>> {
        child(self.0, NameRef::cast)
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
    pub fn args(self) -> impl Iterator<Item = Expr<'a>> + 'a {
        self.0.child_nodes().filter_map(Expr::cast)
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
    use qb64rust_base::FileId;

    fn first_stmt<'a>(green: &'a crate::tree::GreenNode) -> Node<'a> {
        let root = SourceFile::cast(Node::root(green, FileId(0))).unwrap();
        root.statements().next().unwrap()
    }

    #[test]
    fn binary_operands_and_operator() {
        let p = crate::parse(FileId(0), b"x = a + -b\n");
        let assign = AssignStmt::cast(first_stmt(&p.green)).unwrap();
        assert_eq!(assign.target().and_then(|n| n.name()).map(|t| t.span.start), Some(0));
        let Some(Expr::Bin(bin)) = assign.value() else {
            panic!("not a BinExpr")
        };
        assert!(matches!(bin.lhs(), Some(Expr::NameRef(_))));
        assert_eq!(bin.op().map(|t| t.kind), Some(Plus));
        assert!(matches!(bin.rhs(), Some(Expr::Prefix(_))));
    }

    #[test]
    fn assignment_of_a_name() {
        let p = crate::parse(FileId(0), b"LET d# = s!\n");
        let assign = AssignStmt::cast(first_stmt(&p.green)).unwrap();
        assert_eq!(assign.target().and_then(|n| n.name()).map(|t| t.span.start), Some(4));
        let Some(Expr::NameRef(value)) = assign.value() else {
            panic!("not a NameRef")
        };
        assert_eq!(value.name().map(|t| t.span.start), Some(9));
    }

    #[test]
    fn missing_children_give_none() {
        // `x = ` has no value; `a +` has no right operand.
        let p = crate::parse(FileId(0), b"x =\n");
        let assign = AssignStmt::cast(first_stmt(&p.green)).unwrap();
        assert!(assign.target().is_some());
        assert!(assign.value().is_none());

        let p = crate::parse(FileId(0), b"x = a +\n");
        let assign = AssignStmt::cast(first_stmt(&p.green)).unwrap();
        let Some(Expr::Bin(bin)) = assign.value() else {
            panic!("not a BinExpr")
        };
        assert!(bin.lhs().is_some());
        assert!(bin.rhs().is_none());
    }

    #[test]
    fn dim_items_and_type_words() {
        let p = crate::parse(FileId(0), b"DIM a AS LONG, b%\n");
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
        let root = SourceFile::cast(Node::root(green, FileId(0))).unwrap();
        root.statements().filter_map(ProcDef::cast).collect()
    }

    #[test]
    fn procedure_parts() {
        let src = b"FUNCTION f& (a AS LONG, b$)\nf& = a\nEND FUNCTION\n";
        let p = crate::parse(FileId(0), src);
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
        let p = crate::parse(FileId(0), b"SUB a\nPRINT 1\n");
        let [a] = procs(&p.green)[..] else {
            panic!("one ProcDef")
        };
        assert!(a.end().is_none());
        assert_eq!(a.body().count(), 1);
    }

    #[test]
    fn nested_header_ends_the_outer_block() {
        let p = crate::parse(FileId(0), b"SUB a\nPRINT 1\nSUB b\nEND SUB\n");
        let [a, b] = procs(&p.green)[..] else {
            panic!("two ProcDefs")
        };
        assert!(a.end().is_none());
        assert!(b.end().is_some());
    }

    #[test]
    fn wrong_end_kind_still_closes() {
        let p = crate::parse(FileId(0), b"SUB a\nEND FUNCTION\n");
        let [a] = procs(&p.green)[..] else {
            panic!("one ProcDef")
        };
        assert_eq!(a.end().and_then(|e| e.keyword()).map(|t| t.span.start), Some(10));
    }

    #[test]
    fn header_without_a_name() {
        let p = crate::parse(FileId(0), b"SUB\nEND SUB\n");
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
        let p = crate::parse(FileId(0), b"CALL s(n)\n");
        let c = CallStmt::cast(first_stmt(&p.green)).unwrap();
        assert!(c.call_keyword().is_some());
        assert_eq!(c.name().map(|t| t.span.start), Some(5));
        assert!(matches!(c.arg_list().unwrap().args().next(), Some(Expr::NameRef(_))));
        assert!(c.unparsed_args().is_none());

        // Without CALL the parentheses make a ParenExpr argument (by value).
        let p = crate::parse(FileId(0), b"s (n)\n");
        let c = CallStmt::cast(first_stmt(&p.green)).unwrap();
        assert!(c.call_keyword().is_none());
        assert!(matches!(c.arg_list().unwrap().args().next(), Some(Expr::Paren(_))));

        let p = crate::parse(FileId(0), b"t\n");
        let c = CallStmt::cast(first_stmt(&p.green)).unwrap();
        assert!(c.arg_list().is_none());
        assert!(c.unparsed_args().is_none());

        let p = crate::parse(FileId(0), b"LOCATE , 5\n");
        assert!(p.diagnostics.list().is_empty());
        let c = CallStmt::cast(first_stmt(&p.green)).unwrap();
        assert!(c.unparsed_args().is_some());
    }

    #[test]
    fn declare_exit_shared() {
        let p = crate::parse(FileId(0), b"DECLARE SUB s (a)\nDIM SHARED g\nDIM h\nEXIT SUB\n");
        let root = SourceFile::cast(Node::root(&p.green, FileId(0))).unwrap();
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
        let p = crate::parse(FileId(0), src);
        let root = SourceFile::cast(Node::root(&p.green, FileId(0))).unwrap();
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
    fn cast_checks_the_kind() {
        let p = crate::parse(FileId(0), b"PRINT 1; 2\n");
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
