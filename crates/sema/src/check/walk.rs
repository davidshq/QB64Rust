//! The walk of pass 2 and of the label pre-pass (design D8 of `m4-statements-metacommands`): the trees read as one
//! stream of events in text order, each include statement followed by its tree's events in place, as the old
//! compiler reads included text. A block's checker takes the events after its start up to its end, wherever they
//! stand.
//!
//! Each block start gets exactly one end: its closer, the inner `NEXT j, i` that closes it too, or, when its closer
//! is missing, the end of its node.

use super::Skips;
use qb64rust_syntax::ast;
use qb64rust_syntax::tree::Node;
use qb64rust_syntax::{ParsedProgram, SyntaxKind};

/// One step of the walk.
#[derive(Clone, Copy, Debug)]
pub(super) enum Event<'t> {
    /// A statement that is no block with a body; an include statement among them, whose tree's events follow.
    Stmt(Node<'t>),
    /// The start of a block: its node (`IfBlock`, `ForBlock`, `DoBlock`, `WhileBlock`, `SelectBlock`, `ProcDef`,
    /// `DefFnBlock`).
    Open(Node<'t>),
    /// A branch of the innermost open block: an `ElseIfBranch`, `ElseBranch` or `CaseClause` node, whose header is
    /// its first child.
    Branch(Node<'t>),
    /// The end of the innermost open block: its closer (`NextStmt`, `LoopStmt`, `BlockEnd`, `ProcEnd`) with the
    /// place of this block among the variables a `NEXT` names, or `None` when the block ends with its node.
    Close(Option<Node<'t>>, usize),
}

/// What a frame of the walk holds.
#[derive(Clone, Copy)]
enum Item<'t> {
    /// A statement node, a block among them.
    Node(Node<'t>),
    Branch(Node<'t>),
    Close(Node<'t>, usize),
    /// A block whose closer is missing ends here.
    End,
}

/// The walk over a list of statements and everything they hold or include.
pub(super) struct Cursor<'t, 's> {
    program: &'t ParsedProgram,
    skips: &'s Skips,
    frames: Vec<(Vec<Item<'t>>, usize)>,
}

impl<'t, 's> Cursor<'t, 's> {
    pub(super) fn new(
        program: &'t ParsedProgram,
        skips: &'s Skips,
        nodes: impl IntoIterator<Item = Node<'t>>,
    ) -> Cursor<'t, 's> {
        Cursor {
            program,
            skips,
            frames: vec![(nodes.into_iter().map(Item::Node).collect(), 0)],
        }
    }

    /// The next event; `None` at the end of the list the walk started with.
    pub(super) fn next(&mut self) -> Option<Event<'t>> {
        loop {
            let (items, i) = self.frames.last_mut()?;
            let Some(&item) = items.get(*i) else {
                self.frames.pop();
                continue;
            };
            *i += 1;
            let n = match item {
                Item::Node(n) => n,
                Item::Branch(n) => return Some(Event::Branch(n)),
                Item::Close(n, k) => return Some(Event::Close(Some(n), k)),
                Item::End => return Some(Event::Close(None, 0)),
            };
            if self.skips.past_cap(n) {
                // Past the error cap a statement list stops, as a `break` in it: the later statements of the frame
                // are not checked, its branches and end still come. A `CrossCloser` among them is skipped too, so a
                // block it would close stays open to the end of the walk; what that adds is never shown, since a
                // capped tree already holds the run's [`MAX_ERRORS`](qb64rust_base::MAX_ERRORS) errors, and
                // `--report`, which lifts the cap, caps no tree.
                while matches!(items.get(*i), Some(Item::Node(_))) {
                    *i += 1;
                }
                continue;
            }
            if let Some(c) = ast::CrossCloser::cast(n) {
                // A closer or branch of a block opened in another file: the end of each block it closes (a
                // `NEXT j, i` closes two), or the branch.
                let Some(inner) = c.inner() else {
                    continue;
                };
                let closes = self.program.cross_open.values().filter(|&&k| k == n.key()).count();
                let branch = matches!(
                    inner.kind(),
                    SyntaxKind::ElseIfBranch | SyntaxKind::ElseBranch | SyntaxKind::CaseClause
                );
                let items = if branch {
                    vec![Item::Branch(inner)]
                } else {
                    (0..closes).map(|k| Item::Close(inner, k)).collect()
                };
                self.frames.push((items, 0));
                continue;
            }
            if ast::MetaCommentStmt::cast(n).is_some()
                && let Some(tree) = self.program.included(n)
            {
                let inner = ast::SourceFile::cast(tree.root())
                    .into_iter()
                    .flat_map(|f| f.statements())
                    .map(Item::Node)
                    .collect();
                self.frames.push((inner, 0));
                return Some(Event::Stmt(n));
            }
            if let Some(items) = block_items(self.program, n) {
                self.frames.push((items, 0));
                return Some(Event::Open(n));
            }
            return Some(Event::Stmt(n));
        }
    }

    /// Takes the events of the block just opened up to its end, unchecked (a procedure where none may stand).
    pub(super) fn skip_block(&mut self) {
        let mut depth = 1;
        while let Some(e) = self.next() {
            match e {
                Event::Open(_) => depth += 1,
                Event::Close(..) => {
                    depth -= 1;
                    if depth == 0 {
                        return;
                    }
                }
                Event::Stmt(_) | Event::Branch(_) => {}
            }
        }
    }
}

/// The items of a block with a body, in the order its checker takes them; `None` for any other node. A block whose
/// closer stands in another tree gets no end of its own: the `CrossCloser` there ends it.
fn block_items<'t>(program: &ParsedProgram, node: Node<'t>) -> Option<Vec<Item<'t>>> {
    fn body<'t>(nodes: impl Iterator<Item = Node<'t>>, items: &mut Vec<Item<'t>>) {
        items.extend(nodes.map(Item::Node));
    }
    let across = program.cross_open.contains_key(&node.key());
    let end = |closer: Option<Node<'t>>, items: &mut Vec<Item<'t>>| match closer {
        Some(c) => items.push(Item::Close(c, 0)),
        None if !across => items.push(Item::End),
        None => {}
    };
    let mut items = Vec::new();
    if let Some(b) = ast::IfBlock::cast(node) {
        // The `ELSEIF` branches come before the `ELSE` branch, wherever they stand, and a second `ELSE` (a parse
        // error) is not checked.
        body(b.if_branch().into_iter().flat_map(|f| f.body()), &mut items);
        for e in b.else_if_branches() {
            items.push(Item::Branch(e.node()));
            body(e.body(), &mut items);
        }
        if let Some(e) = b.else_branch() {
            items.push(Item::Branch(e.node()));
            body(e.body(), &mut items);
        }
        end(b.end().map(|e| e.node()), &mut items);
    } else if let Some(b) = ast::ForBlock::cast(node) {
        body(b.body(), &mut items);
        match b.closer() {
            Some((n, k)) => items.push(Item::Close(n.node(), k)),
            None => end(None, &mut items),
        }
    } else if let Some(b) = ast::DoBlock::cast(node) {
        body(b.body(), &mut items);
        end(b.end().map(|e| e.node()), &mut items);
    } else if let Some(b) = ast::WhileBlock::cast(node) {
        body(b.body(), &mut items);
        end(b.end().map(|e| e.node()), &mut items);
    } else if let Some(b) = ast::SelectBlock::cast(node) {
        body(b.before_cases(), &mut items);
        for c in b.cases() {
            items.push(Item::Branch(c.node()));
            body(c.body(), &mut items);
        }
        end(b.end().map(|e| e.node()), &mut items);
    } else if let Some(d) = ast::ProcDef::cast(node) {
        body(d.body(), &mut items);
        end(d.end().map(|e| e.node()), &mut items);
    } else {
        let b = ast::DefFnBlock::cast(node)?;
        body(b.body(), &mut items);
        end(b.end().map(|e| e.node()), &mut items);
    }
    Some(items)
}
