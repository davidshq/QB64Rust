//! Byte lexer, lossless syntax tree and parser (design D2–D4).

pub mod ast;
mod kind;
pub mod lexer;
mod parser;
pub mod tree;

pub use kind::SyntaxKind;
pub use parser::keywords::is_keyword;
pub use parser::{Parse, parse};

use qb64rust_base::show_bytes;
use std::fmt::Write as _;
use tree::{Element, Node};

/// `--dump tokens`: one token per line, `<kind> <start>..<end> <text>`.
pub fn dump_tokens(bytes: &[u8]) -> String {
    let mut out = String::new();
    let mut o = 0u32;
    for t in lexer::tokenize(bytes) {
        let text = &bytes[o as usize..(o + t.len) as usize];
        writeln!(out, "{:?} {}..{} {}", t.kind, o, o + t.len, show_bytes(text)).unwrap();
        o += t.len;
    }
    out
}

/// `--dump tree`: nodes indented by depth, tokens with their text.
pub fn dump_tree(root: Node, bytes: &[u8]) -> String {
    let mut out = String::new();
    dump_node(root, bytes, 0, &mut out);
    out
}

fn dump_node(node: Node, bytes: &[u8], depth: usize, out: &mut String) {
    let s = node.span();
    writeln!(
        out,
        "{:indent$}{:?} {}..{}",
        "",
        node.kind(),
        s.start,
        s.end,
        indent = depth * 2
    )
    .unwrap();
    for c in node.children() {
        match c {
            Element::Node(n) => dump_node(n, bytes, depth + 1, out),
            Element::Token(t) => {
                let text = &bytes[t.span.start as usize..t.span.end as usize];
                writeln!(
                    out,
                    "{:indent$}{:?} {}",
                    "",
                    t.kind,
                    show_bytes(text),
                    indent = (depth + 1) * 2
                )
                .unwrap();
            }
        }
    }
}
