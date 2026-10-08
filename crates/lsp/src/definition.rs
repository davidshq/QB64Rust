//! `textDocument/definition` for procedures and labels (design D4), by name over the program's trees.

use crate::analysis::Analysis;
use crate::walk::is_expr;
use qb64rust_base::Span;
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::ast::{
    GosubStmt, GotoStmt, ImplicitGoto, LabelDef, LineNumber, OnErrorStmt, OnJumpStmt, ProcDef, ProcHeader, RestoreStmt,
    ResumeStmt, ReturnStmt,
};
use qb64rust_syntax::tree::{Element, Node, Tok};

/// The definition of the name at `offset` in the program's main file: the name token of a procedure's header or of
/// a label (or line number). `None` for anything else, and for a name with no definition.
pub fn definition(a: &Analysis, offset: u32) -> Option<Span> {
    let root = a.parsed.main().root();
    let is_name = |(_, t): &(Vec<Node>, Tok)| matches!(t.kind, SyntaxKind::Ident | SyntaxKind::Number);
    // The cursor in a name, or just after one.
    let (chain, tok) = token_at(root, offset)
        .filter(is_name)
        .or_else(|| token_at(root, offset.checked_sub(1)?).filter(is_name))?;
    let parent = *chain.last()?;
    let proc = chain.iter().rev().find(|n| n.kind() == SyntaxKind::ProcDef).copied();
    let is = |t: Option<Tok>| t == Some(tok);
    match parent.kind() {
        SyntaxKind::ProcHeader if is(ProcHeader::cast(parent)?.name()) => find_proc(a, tok),
        // The name of a call (with or without `CALL`), of a function call, or a name that may be a variable: only a
        // procedure of that name answers.
        SyntaxKind::NameRef => find_proc(a, tok),
        SyntaxKind::GotoStmt if is(GotoStmt::cast(parent)?.target()) => find_label(a, proc, tok),
        SyntaxKind::GosubStmt if is(GosubStmt::cast(parent)?.target()) => find_label(a, proc, tok),
        SyntaxKind::ReturnStmt if is(ReturnStmt::cast(parent)?.target()) => find_label(a, proc, tok),
        SyntaxKind::ResumeStmt if is(ResumeStmt::cast(parent)?.target()) => find_label(a, proc, tok),
        SyntaxKind::OnJumpStmt if OnJumpStmt::cast(parent)?.targets().contains(&Some(tok)) => find_label(a, proc, tok),
        SyntaxKind::ImplicitGoto if is(ImplicitGoto::cast(parent)?.number()) => find_label(a, proc, tok),
        // The handler is the program's: a label of the main module, also inside a procedure (measured,
        // `v14_on_error_sub_to_main`); `DATA` is the main module's too.
        SyntaxKind::OnErrorStmt if is(OnErrorStmt::cast(parent)?.target()) => find_label(a, None, tok),
        SyntaxKind::RestoreStmt if is(RestoreStmt::cast(parent)?.target()) => find_label(a, None, tok),
        SyntaxKind::LabelDef if is(LabelDef::cast(parent)?.name()) => Some(tok.span),
        SyntaxKind::LineNumber if is(LineNumber::cast(parent)?.number()) => Some(tok.span),
        _ => None,
    }
}

/// The token at `offset` and the nodes from the root down to it.
fn token_at(root: Node, offset: u32) -> Option<(Vec<Node>, Tok)> {
    let mut chain = vec![root];
    let mut node = root;
    loop {
        let hit = node.children().find(|c| {
            let s = c.span();
            s.start <= offset && offset < s.end
        })?;
        match hit {
            Element::Node(n) => {
                chain.push(n);
                node = n;
            }
            Element::Token(t) => return Some((chain, t)),
        }
    }
}

/// A procedure's name as the old compiler compares it: upper case, without its type suffix.
fn proc_key(a: &Analysis, t: Tok) -> Vec<u8> {
    let mut name = a.map.text(t.span).to_ascii_uppercase();
    if let Some(i) = name.iter().position(|&b| b == b'`') {
        name.truncate(i);
    }
    while name.last().is_some_and(|b| b"$%&!#~".contains(b)) {
        name.pop();
    }
    name
}

/// The name of the first `SUB`/`FUNCTION` of that name, in tree order.
fn find_proc(a: &Analysis, t: Tok) -> Option<Span> {
    let key = proc_key(a, t);
    let mut found = None;
    for tree in &a.parsed.trees {
        visit(tree.root(), true, &mut |n| {
            if found.is_none()
                && let Some(name) = ProcDef::cast(n).and_then(|p| p.header()).and_then(|h| h.name())
                && proc_key(a, name) == key
            {
                found = Some(name.span);
            }
        });
        if found.is_some() {
            break;
        }
    }
    found
}

/// The label (or line number) of that name in `proc`'s body, or in the main module's (in every tree, outside
/// procedures) when `proc` is `None`. Label names are compared in upper case with their suffix: the old compiler
/// keeps `a` and `a$` apart, and line numbers apart from names.
fn find_label(a: &Analysis, proc: Option<Node>, t: Tok) -> Option<Span> {
    let key = a.map.text(t.span).to_ascii_uppercase();
    let mut found = None;
    let mut check = |n: Node| {
        let def = LabelDef::cast(n)
            .and_then(|l| l.name())
            .or_else(|| LineNumber::cast(n).and_then(|l| l.number()));
        if found.is_none()
            && let Some(d) = def
            && d.kind == t.kind
            && a.map.text(d.span).to_ascii_uppercase() == key
        {
            found = Some(d.span);
        }
    };
    match proc {
        Some(p) => visit(p, true, &mut check),
        None => {
            for tree in &a.parsed.trees {
                visit(tree.root(), false, &mut check);
            }
        }
    }
    found
}

/// Every statement node below `node` (expressions are not entered); procedures only when `into_procs`.
fn visit<'a>(node: Node<'a>, into_procs: bool, f: &mut impl FnMut(Node<'a>)) {
    for n in node.child_nodes() {
        if is_expr(n) {
            continue;
        }
        f(n);
        if into_procs || n.kind() != SyntaxKind::ProcDef {
            visit(n, into_procs, f);
        }
    }
}
