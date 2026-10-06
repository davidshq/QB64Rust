//! Blocks: for now marked as a whole, with the statements inside checked one by one.

use super::{Checker, Skips, first_token_span};
use qb64rust_syntax::ast;
use qb64rust_syntax::tree::Node;

impl Checker<'_> {
    /// A block (design D10 of `m2-parser-breadth`): one "not supported yet" error at its first token, unless its
    /// header has a parse error; then the statements inside are checked one by one, as everywhere else.
    pub(super) fn block(&mut self, node: Node, block: BlockParts, skips: &Skips) {
        if block.header.is_none_or(|h| skips.usable(h)) && !skips.past_cap(node) {
            self.stmt_error = false;
            let span = first_token_span(node);
            let _ = match block.verdict {
                Verdict::Unsupported(what) => self.unsupported(span, what),
                Verdict::Error(msg) => self.error(span, msg),
            };
            self.flush_names();
        }
        for s in block.inner {
            if skips.past_cap(s) {
                break;
            }
            self.check_statement(s, skips);
        }
    }
}

/// What `sema` says about a block it does not compile.
pub(super) enum Verdict {
    /// Not supported yet; names the construct.
    Unsupported(&'static str),
    /// A real error: the old compiler rejects the construct too.
    Error(&'static str),
}

/// A block statement as `sema` sees it for now: its header (whose parse errors suppress the verdict), the
/// verdict, and the statements inside it, in order.
pub(super) struct BlockParts<'a> {
    header: Option<Node<'a>>,
    verdict: Verdict,
    pub(super) inner: Vec<Node<'a>>,
}

fn unsupported<'a>(header: Option<Node<'a>>, what: &'static str, inner: Vec<Node<'a>>) -> Option<BlockParts<'a>> {
    Some(BlockParts {
        header,
        verdict: Verdict::Unsupported(what),
        inner,
    })
}

/// The parts of a block statement; `None` for any other statement.
pub(super) fn block_parts(node: Node) -> Option<BlockParts> {
    const DEF_FN: &str = "`DEF FN` is not available in QB64; use a FUNCTION";
    if let Some(b) = ast::IfBlock::cast(node) {
        let first = b.if_branch();
        let mut inner: Vec<Node> = first.into_iter().flat_map(|f| f.body()).collect();
        for e in b.else_if_branches() {
            inner.extend(e.body());
        }
        inner.extend(b.else_branch().into_iter().flat_map(|e| e.body()));
        unsupported(first.and_then(|f| f.header()).map(|h| h.node()), "`IF` blocks", inner)
    } else if let Some(s) = ast::IfStmt::cast(node) {
        let mut inner: Vec<Node> = s.then_branch().into_iter().flat_map(|b| b.statements()).collect();
        inner.extend(s.else_branch().into_iter().flat_map(|b| b.statements()));
        unsupported(s.header().map(|h| h.node()), "single-line `IF`", inner)
    } else if let Some(b) = ast::ForBlock::cast(node) {
        unsupported(b.header().map(|h| h.node()), "`FOR` loops", b.body().collect())
    } else if let Some(b) = ast::DoBlock::cast(node) {
        unsupported(b.header().map(|h| h.node()), "`DO` loops", b.body().collect())
    } else if let Some(b) = ast::WhileBlock::cast(node) {
        unsupported(b.header().map(|h| h.node()), "`WHILE` loops", b.body().collect())
    } else if let Some(b) = ast::SelectBlock::cast(node) {
        let mut inner: Vec<Node> = b.before_cases().collect();
        for c in b.cases() {
            inner.extend(c.body());
        }
        unsupported(b.header().map(|h| h.node()), "`SELECT CASE`", inner)
    } else if let Some(b) = ast::TypeBlock::cast(node) {
        unsupported(b.header().map(|h| h.node()), "`TYPE` blocks", Vec::new())
    } else if let Some(b) = ast::DeclareLibraryBlock::cast(node) {
        unsupported(b.header().map(|h| h.node()), "`DECLARE LIBRARY`", Vec::new())
    } else if let Some(b) = ast::DefFnBlock::cast(node) {
        Some(BlockParts {
            header: b.header().map(|h| h.node()),
            verdict: Verdict::Error(DEF_FN),
            inner: b.body().collect(),
        })
    } else if ast::DefFnStmt::cast(node).is_some() {
        Some(BlockParts {
            header: Some(node),
            verdict: Verdict::Error(DEF_FN),
            inner: Vec::new(),
        })
    } else {
        None
    }
}
