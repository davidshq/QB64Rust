//! The statements: what each does to the indent level, and its layout, one module per family (design D2). Each
//! module's comment names the handlers of `qb64pe.bas` it follows.
//!
//! A statement kind whose handler sets no layout is written as read: [`KEPT`].

mod call;
mod control;
mod decl;
mod io;
mod meta;

pub(crate) use meta::{is_precompiler_line, meta_effect_of};

use crate::fmt::Fmt;
use crate::lay::Lay;
use crate::walk::{LineLevel, Pass};
use qb64rust_syntax::SyntaxKind::{self, *};
use qb64rust_syntax::tree::{Element, Node, Tok};

/// The statement kinds whose QB64pe handler sets no layout (`layoutdone = 0`): the line they stand on is written as
/// its own text, trimmed and indented. Filled from the oracle: `DECLARE SUB|FUNCTION` ("skip DECLARE SUB/FUNCTION",
/// `qb64pe.bas` 4448: "no layout required").
pub(crate) const KEPT: &[SyntaxKind] = &[Error, DeclareStmt];

impl Fmt<'_> {
    /// One statement of a line, standing in a container of kind `block`: its effect on the level, then its layout
    /// (`None` for a kept one).
    pub(crate) fn statement(
        &self,
        n: Node<'_>,
        block: SyntaxKind,
        pass: &mut Pass,
        level: &mut LineLevel,
    ) -> Option<Lay> {
        self.effect(n, pass, level);
        self.block_effect(n, block, pass, level);
        if KEPT.contains(&n.kind()) {
            return None;
        }
        self.layout(n)
    }

    /// What a statement does to `controllevel`, `lhscontrollevel`, `definingtype` and `declaringlibrary`.
    fn effect(&self, n: Node<'_>, pass: &mut Pass, level: &mut LineLevel) {
        match n.kind() {
            ForHeader | DoHeader | WhileHeader => pass.level += 1,
            NextStmt => {
                let vars = n.child_nodes().count().max(1);
                pass.level -= i32::try_from(vars).unwrap_or(1);
            }
            LoopStmt => pass.level -= 1,
            SelectHeader => {
                pass.level += 1;
                pass.selects.push(false);
            }
            CaseHeader => {
                if pass.selects.last() == Some(&true) {
                    level.lhs -= 1;
                    pass.level -= 1;
                }
                pass.level += 1;
                if let Some(open) = pass.selects.last_mut() {
                    *open = true;
                }
            }
            TypeHeader => pass.defining_type = 1,
            DeclareLibraryHeader => pass.declaring_library = 1,
            MetaStmt => self.meta_effect(n, pass, level),
            _ => {}
        }
    }

    /// The effect of a statement that depends on the block it stands in: headers, branches and closers.
    pub(crate) fn block_effect(&self, n: Node<'_>, block: SyntaxKind, pass: &mut Pass, level: &mut LineLevel) {
        match (n.kind(), block) {
            (IfHeader, IfBranch) => pass.level += 1,
            (IfHeader, ElseIfBranch) => level.lhs -= 1,
            (BlockEnd, IfBlock | WhileBlock) => pass.level -= 1,
            (BlockEnd, SelectBlock) => {
                if pass.selects.pop() == Some(true) {
                    pass.level -= 1;
                }
                pass.level -= 1;
            }
            (BlockEnd, TypeBlock) => pass.defining_type = 0,
            (BlockEnd, DeclareLibraryBlock) => pass.declaring_library = 0,
            (ProcHeader, ProcDef) if self.settings.indent_subs => pass.level += 1,
            (ProcEnd, ProcDef) if self.settings.indent_subs => pass.level -= 1,
            _ => {}
        }
    }

    /// The layout of a statement, `None` where QB64pe sets none.
    fn layout(&self, n: Node<'_>) -> Option<Lay> {
        match n.kind() {
            AssignStmt => Some(self.assign(n)),
            PrintStmt | LprintStmt => Some(self.print(n)),
            BuiltinStmt => self.builtin_stmt(n),
            CallStmt => self.call_stmt(n),
            MetaStmt => self.meta(n),
            MetaCommentStmt => Some(Lay::new()),
            LabelDef => Some(self.label_def(n)),
            LineNumber => Some(Lay(self.trimmed(n))),
            _ => self.control(n).or_else(|| self.decl(n)).or_else(|| self.io(n)),
        }
    }

    /// `name:` of a label (`qb64pe.bas` 4040): the label as its symbol spells it, then the colon. A built-in statement
    /// read as a label by the parser and as the statement by `sema` (`CLS : PRINT`) is the statement and its colon.
    fn label_def(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        for t in n.child_tokens() {
            match t.kind {
                Colon => l.text(b":"),
                _ if self.label_symbol(t) => l.text(&self.label_ref(t)),
                _ => l.text(&self.name(t)),
            };
        }
        l
    }

    /// Whether a token names a label.
    pub(crate) fn label_symbol(&self, t: Tok) -> bool {
        self.program
            .symbols
            .at(t.span.file, t.span.start)
            .is_some_and(|id| matches!(self.program.symbols.get(id).kind, qb64rust_sema::SymbolKind::Label(_)))
    }

    /// The tokens of a node that are not trivia.
    pub(crate) fn toks(&self, n: Node<'_>) -> Vec<Tok> {
        n.child_tokens().collect()
    }

    /// The child elements of a node that are not trivia.
    pub(crate) fn elems<'n>(&self, n: Node<'n>) -> Vec<Element<'n>> {
        n.children()
            .filter(|e| !matches!(e, Element::Token(t) if t.kind.is_trivia() || t.kind == Newline))
            .collect()
    }

    /// Whether a token spells a word, in any case.
    pub(crate) fn is_word(&self, t: Tok, w: &str) -> bool {
        t.kind == Ident && self.tok(t).eq_ignore_ascii_case(w.as_bytes())
    }

    /// QB64pe's `CompactMemberRefLayout$` (`qb64pe.bas` 16362) over tokens, each written by `spell`: `, ` after a
    /// comma, nothing around `.` and `(`, nothing before `)`, `sp` between any other two elements, and a sign glued
    /// to what follows it after `(`, `,`, `TO` or at the start.
    pub(crate) fn compact(&self, toks: &[Tok], spell: &dyn Fn(Tok) -> Vec<u8>) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        let mut prev: Option<Vec<u8>> = None;
        let mut space = false;
        let rtrim = |out: &mut Vec<u8>| {
            while out.last() == Some(&b' ') {
                out.pop();
            }
        };
        for &t in toks.iter().filter(|t| !t.kind.is_trivia() && t.kind != Newline) {
            let tok = spell(t);
            match tok.as_slice() {
                b"," => {
                    rtrim(&mut out);
                    out.extend_from_slice(b", ");
                    space = false;
                }
                b"." | b"(" => {
                    rtrim(&mut out);
                    out.extend_from_slice(&tok);
                    space = false;
                }
                b")" => {
                    rtrim(&mut out);
                    out.push(b')');
                    space = true;
                }
                _ => {
                    if !out.is_empty() && space {
                        out.push(crate::apply::SP);
                    }
                    out.extend_from_slice(&tok);
                    space = true;
                    if (tok == b"-" || tok == b"+")
                        && prev
                            .as_deref()
                            .is_none_or(|p| p == b"(" || p == b"," || p.eq_ignore_ascii_case(b"TO"))
                    {
                        space = false;
                    }
                }
            }
            prev = Some(tok);
        }
        out
    }

    /// A token as `lineformat$` makes it an element and the layout keeps it as written: a number as `number`
    /// keeps it, anything else as it is.
    pub(crate) fn element(&self, t: Tok) -> Vec<u8> {
        match t.kind {
            Number | StringLit => self.literal(t),
            _ => self.tok(t).to_vec(),
        }
    }

    /// `#n` of a file number as the I/O handlers write it: `#` then the number with no space.
    pub(crate) fn file_number(&self, n: Node<'_>) -> Lay {
        let mut l = Lay::new();
        for e in self.elems(n) {
            match e {
                Element::Token(t) if t.kind == Hash => {
                    l.text(b"#").sp2();
                }
                Element::Token(t) if t.kind == Comma => {
                    l.sp2().text(b",");
                }
                Element::Token(_) => {}
                Element::Node(c) => {
                    l.append(&self.expr(c));
                }
            }
        }
        l
    }
}
