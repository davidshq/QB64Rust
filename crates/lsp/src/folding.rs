//! `textDocument/foldingRange` (design D4): every multi-line block from its header's line to the line before its
//! closer, each `CASE`, every `$IF` region, and every run of three or more comment lines.

use crate::analysis::Analysis;
use crate::walk::{first_token, is_expr, last_token, line};
use lsp_types::{FoldingRange, FoldingRangeKind};
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::ast::{
    DeclareLibraryBlock, DefFnBlock, DoBlock, ForBlock, IfBlock, MetaStmt, ProcDef, SelectBlock, TypeBlock, WhileBlock,
};
use qb64rust_syntax::pp::{Directive, directive};
use qb64rust_syntax::tree::Node;

/// The folding ranges of the program's main file, in no particular order.
pub fn folding_ranges(a: &Analysis) -> Vec<FoldingRange> {
    let root = a.parsed.main().root();
    let mut out = Vec::new();
    let mut ifs = Vec::new();
    blocks(a, root, &mut out, &mut ifs);
    comments(a, root, &mut out);
    out
}

/// Where the block `node` ends: the closer's node, `Some(None)` for a block that has no closer of its own (a
/// `CASE`, or a `FOR` closed by an inner `NEXT j, i`), `None` for a node that is not a block.
fn closer(node: Node) -> Option<Option<Node>> {
    Some(if let Some(b) = ProcDef::cast(node) {
        b.end().map(|e| e.node())
    } else if let Some(b) = TypeBlock::cast(node) {
        b.end().map(|e| e.node())
    } else if let Some(b) = IfBlock::cast(node) {
        b.end().map(|e| e.node())
    } else if let Some(b) = ForBlock::cast(node) {
        b.next().map(|e| e.node())
    } else if let Some(b) = DoBlock::cast(node) {
        b.end().map(|e| e.node())
    } else if let Some(b) = WhileBlock::cast(node) {
        b.end().map(|e| e.node())
    } else if let Some(b) = SelectBlock::cast(node) {
        b.end().map(|e| e.node())
    } else if let Some(b) = DeclareLibraryBlock::cast(node) {
        b.end().map(|e| e.node())
    } else if let Some(b) = DefFnBlock::cast(node) {
        b.end().map(|e| e.node())
    } else if node.kind() == SyntaxKind::CaseClause {
        None
    } else {
        return None;
    })
}

/// The block ranges, and the `$IF` regions (`ifs` holds the lines of the open `$IF`/`$ELSEIF`/`$ELSE`s).
fn blocks(a: &Analysis, node: Node, out: &mut Vec<FoldingRange>, ifs: &mut Vec<u32>) {
    for n in node.child_nodes() {
        if let Some(m) = MetaStmt::cast(n) {
            if let Some(t) = m.token() {
                region(a.map.text(t.span), line(&a.map, t.span), out, ifs);
            }
            continue;
        }
        if let Some(end) = closer(n)
            && let Some(first) = first_token(n)
        {
            let start = line(&a.map, first.span);
            let end_line = match end {
                Some(e) => first_token(e).map(|t| line(&a.map, t.span).saturating_sub(1)),
                // A `FOR` closed by an inner `NEXT j, i` ends before that line; a `CASE` at its last line.
                None if n.kind() == SyntaxKind::ForBlock => {
                    last_token(n).map(|t| line(&a.map, t.span).saturating_sub(1))
                }
                None => last_token(n).map(|t| line(&a.map, t.span)),
            };
            if let Some(end_line) = end_line.filter(|&e| e > start) {
                out.push(range(start, end_line, None));
            }
        }
        if !is_expr(n) {
            blocks(a, n, out, ifs);
        }
    }
}

fn region(text: &[u8], line: u32, out: &mut Vec<FoldingRange>, ifs: &mut Vec<u32>) {
    let close = |out: &mut Vec<FoldingRange>, ifs: &mut Vec<u32>| {
        if let Some(start) = ifs.pop()
            && line > start + 1
        {
            out.push(range(start, line - 1, Some(FoldingRangeKind::Region)));
        }
    };
    match directive(text) {
        Directive::If(_) => ifs.push(line),
        Directive::ElseIf(_) | Directive::Else => {
            close(out, ifs);
            ifs.push(line);
        }
        Directive::EndIf => close(out, ifs),
        Directive::Let(_) | Directive::Error(_) | Directive::Malformed(_) | Directive::Other => {}
    }
}

/// Runs of three or more lines that hold only a comment.
fn comments(a: &Analysis, root: Node, out: &mut Vec<FoldingRange>) {
    // For each line: Some(true) only a comment so far, Some(false) something else.
    let mut lines: Vec<Option<bool>> = vec![None; a.map.file(root.file).line_count() as usize];
    for t in root.tokens() {
        match t.kind {
            SyntaxKind::Whitespace | SyntaxKind::Newline | SyntaxKind::LineContinuation => {}
            SyntaxKind::Comment => {
                let l = &mut lines[line(&a.map, t.span) as usize];
                *l = Some(l.unwrap_or(true));
            }
            _ => lines[line(&a.map, t.span) as usize] = Some(false),
        }
    }
    let mut run_start = None;
    for (i, l) in lines.iter().chain([&None]).enumerate() {
        let i = qb64rust_base::to_u32(i);
        match (l, run_start) {
            (Some(true), None) => run_start = Some(i),
            (Some(true), Some(_)) => {}
            (_, Some(s)) => {
                if i - s >= 3 {
                    out.push(range(s, i - 1, Some(FoldingRangeKind::Comment)));
                }
                run_start = None;
            }
            (_, None) => {}
        }
    }
}

fn range(start: u32, end: u32, kind: Option<FoldingRangeKind>) -> FoldingRange {
    FoldingRange {
        start_line: start,
        end_line: end,
        kind,
        ..FoldingRange::default()
    }
}
