//! `textDocument/documentLink`: each `$INCLUDE` of a file opens the file it includes, from the parser alone. The
//! link covers the file name between the quotes; its target is the file the parse loaded for that line. An
//! `$INCLUDE` the parse did not load (the file is missing, or the line stands in a `$IF` branch that is off) has no
//! link.

use crate::analysis::Analysis;
use crate::walk::is_expr;
use lsp_types::DocumentLink;
use qb64rust_base::{FileId, Span};
use qb64rust_syntax::ast::MetaCommentStmt;
use qb64rust_syntax::meta::comment_directives;
use qb64rust_syntax::tree::Node;

/// The links of the program's file `file` (an included file's at its first inclusion), in source order.
pub fn document_links_in(a: &Analysis, file: FileId) -> Vec<DocumentLink> {
    let mut out = Vec::new();
    if let Some(tree) = a.tree(file) {
        collect(a, tree.root(), &mut out);
    }
    out
}

fn collect(a: &Analysis, node: Node, out: &mut Vec<DocumentLink>) {
    for n in node.child_nodes() {
        if let Some(m) = MetaCommentStmt::cast(n) {
            if let Some(link) = link(a, m) {
                out.push(link);
            }
        } else if !is_expr(n) {
            collect(a, n, out);
        }
    }
}

/// The link of one metacommand comment, when the parse loaded a file for it.
fn link(a: &Analysis, m: MetaCommentStmt) -> Option<DocumentLink> {
    let included = a.parsed.included(m.node())?;
    let token = m.token()?;
    let text = a.map.text(token.span);
    let name = comment_directives(text).ok()?.include?;
    // The last `$INCLUDE` of a comment is the one that counts (`meta.rs`), so the name is found from the end.
    let quoted = [&b"'"[..], &name, b"'"].concat();
    let at = text.windows(quoted.len()).rposition(|w| w == quoted.as_slice())?;
    let start = token.span.start + qb64rust_base::to_u32(at) + 1;
    let span = Span::new(token.span.file, start, start + qb64rust_base::to_u32(name.len()));
    Some(DocumentLink {
        range: a.range(span),
        target: Some(a.info(included.file).uri.clone()),
        tooltip: Some("Open the included file".to_string()),
        data: None,
    })
}
