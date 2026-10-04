//! The lossless syntax tree (design D3): immutable green nodes that know only their kind, byte length and
//! children, and a cursor layer that adds absolute offsets. Token text is a slice of the file's bytes, so the tree
//! never holds text of its own and printing it gives back the input byte for byte.

use crate::SyntaxKind;
use qb64rust_base::{FileId, Span};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GreenNode {
    pub kind: SyntaxKind,
    pub len: u32,
    pub children: Vec<GreenElement>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GreenElement {
    Node(GreenNode),
    Token { kind: SyntaxKind, len: u32 },
}

impl GreenElement {
    pub fn len(&self) -> u32 {
        match self {
            GreenElement::Node(n) => n.len,
            GreenElement::Token { len, .. } => *len,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn kind(&self) -> SyntaxKind {
        match self {
            GreenElement::Node(n) => n.kind,
            GreenElement::Token { kind, .. } => *kind,
        }
    }
}

/// Builds a green tree from start/token/finish events. A checkpoint lets the parser wrap already-built children
/// into a new node (needed for binary expressions, whose left operand is parsed first).
#[derive(Default)]
pub struct TreeBuilder {
    stack: Vec<(SyntaxKind, Vec<GreenElement>)>,
}

#[derive(Clone, Copy)]
pub struct Checkpoint(usize);

impl TreeBuilder {
    pub fn start_node(&mut self, kind: SyntaxKind) {
        self.stack.push((kind, Vec::new()));
    }

    pub fn token(&mut self, kind: SyntaxKind, len: u32) {
        self.stack
            .last_mut()
            .expect("token outside a node")
            .1
            .push(GreenElement::Token { kind, len });
    }

    pub fn finish_node(&mut self) {
        let (kind, children) = self.stack.pop().expect("unbalanced finish_node");
        let node = GreenNode {
            kind,
            len: children.iter().map(GreenElement::len).sum(),
            children,
        };
        match self.stack.last_mut() {
            Some(parent) => parent.1.push(GreenElement::Node(node)),
            None => self
                .stack
                .push((SyntaxKind::SourceFile, vec![GreenElement::Node(node)])),
        }
    }

    pub fn checkpoint(&self) -> Checkpoint {
        Checkpoint(self.stack.last().map_or(0, |n| n.1.len()))
    }

    /// Starts a node that takes over the children added to the current node since the checkpoint.
    pub fn start_node_at(&mut self, cp: Checkpoint, kind: SyntaxKind) {
        let parent = self.stack.last_mut().expect("start_node_at outside a node");
        let moved = parent.1.split_off(cp.0);
        self.stack.push((kind, moved));
    }

    /// The finished root. Exactly one root node must have been finished.
    pub fn finish(mut self) -> GreenNode {
        assert_eq!(self.stack.len(), 1, "unfinished nodes");
        let (_, mut children) = self.stack.pop().unwrap();
        assert_eq!(children.len(), 1);
        match children.pop().unwrap() {
            GreenElement::Node(n) => n,
            GreenElement::Token { .. } => unreachable!(),
        }
    }
}

/// A node with its absolute position in a file.
#[derive(Clone, Copy, Debug)]
pub struct Node<'a> {
    pub green: &'a GreenNode,
    pub file: FileId,
    pub offset: u32,
}

/// A token with its absolute position in a file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tok {
    pub kind: SyntaxKind,
    pub span: Span,
}

#[derive(Clone, Copy, Debug)]
pub enum Element<'a> {
    Node(Node<'a>),
    Token(Tok),
}

impl<'a> Element<'a> {
    pub fn kind(&self) -> SyntaxKind {
        match self {
            Element::Node(n) => n.kind(),
            Element::Token(t) => t.kind,
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Element::Node(n) => n.span(),
            Element::Token(t) => t.span,
        }
    }

    pub fn as_node(&self) -> Option<Node<'a>> {
        match self {
            Element::Node(n) => Some(*n),
            Element::Token(_) => None,
        }
    }

    pub fn as_token(&self) -> Option<Tok> {
        match self {
            Element::Token(t) => Some(*t),
            Element::Node(_) => None,
        }
    }
}

impl<'a> Node<'a> {
    pub fn root(green: &'a GreenNode, file: FileId) -> Node<'a> {
        Node { green, file, offset: 0 }
    }

    pub fn kind(&self) -> SyntaxKind {
        self.green.kind
    }

    pub fn span(&self) -> Span {
        Span::new(self.file, self.offset, self.offset + self.green.len)
    }

    pub fn children(&self) -> impl Iterator<Item = Element<'a>> + use<'a> {
        let file = self.file;
        let mut offset = self.offset;
        let green: &'a GreenNode = self.green;
        green.children.iter().map(move |c| {
            let start = offset;
            offset += c.len();
            match c {
                GreenElement::Node(n) => Element::Node(Node {
                    green: n,
                    file,
                    offset: start,
                }),
                GreenElement::Token { kind, len } => Element::Token(Tok {
                    kind: *kind,
                    span: Span::new(file, start, start + len),
                }),
            }
        })
    }

    pub fn child_nodes(&self) -> impl Iterator<Item = Node<'a>> + use<'a> {
        self.children().filter_map(|e| e.as_node())
    }

    /// Child tokens that are not trivia.
    pub fn child_tokens(&self) -> impl Iterator<Item = Tok> + use<'a> {
        self.children()
            .filter_map(|e| e.as_token())
            .filter(|t| !t.kind.is_trivia())
    }

    /// All tokens below this node, in order, trivia included.
    pub fn tokens(&self) -> Vec<Tok> {
        let mut out = Vec::new();
        self.collect_tokens(&mut out);
        out
    }

    fn collect_tokens(&self, out: &mut Vec<Tok>) {
        for c in self.children() {
            match c {
                Element::Node(n) => n.collect_tokens(out),
                Element::Token(t) => out.push(t),
            }
        }
    }

    /// The first non-trivia token below this node.
    pub fn first_token(&self) -> Option<Tok> {
        self.tokens().into_iter().find(|t| !t.kind.is_trivia())
    }
}

/// Prints the tree back to bytes: the concatenation of its tokens' bytes in `source`.
pub fn print(green: &GreenNode, source: &[u8]) -> Vec<u8> {
    let root = Node::root(green, FileId(0));
    let mut out = Vec::with_capacity(source.len());
    for t in root.tokens() {
        out.extend_from_slice(&source[t.span.start as usize..t.span.end as usize]);
    }
    out
}
