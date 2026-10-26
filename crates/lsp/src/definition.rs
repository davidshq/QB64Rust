//! `textDocument/definition` for procedures and labels (design D4), by name over the program's trees; and
//! `textDocument/references` from the semantic index (design D4 of `m5b-language-features`).

use crate::analysis::Analysis;
use crate::index::{EntryKind, Indexed};
use crate::walk::is_expr;
use lsp_types::Location;
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
        // A line that is a procedure's name and a colon may be a call of it (measured, `verification\v28_c_labels`;
        // `m3-get-put-bload`): whether it is depends on the jumps of the whole program, so the semantic index answers
        // (design D4 of `m5b-language-features`).
        SyntaxKind::LabelDef if is(LabelDef::cast(parent)?.name()) && names_a_proc(a, tok) => None,
        SyntaxKind::LabelDef if is(LabelDef::cast(parent)?.name()) => Some(tok.span),
        SyntaxKind::LineNumber if is(LineNumber::cast(parent)?.number()) => Some(tok.span),
        _ => None,
    }
}

/// `textDocument/references` (design D4 of `m5b-language-features`): every use of the symbol at `offset` of the
/// program's main file, with its definition when `with_def`, by file and offset. A FUNCTION and its result variable
/// are one name, as for a rename: each lists the other's uses too, and the result variable's definition is a use.
/// Empty on a name with no symbol.
pub(crate) fn references(ix: &Indexed, offset: u32, with_def: bool) -> Vec<Location> {
    let main = ix.analysis.parsed.main().file;
    let Some((id, _)) = ix.index.entry_near(main, offset) else {
        return Vec::new();
    };
    let e = &ix.index.entries[id];
    let (mut def, mut spans) = (e.def, e.refs.clone());
    if let Some(t) = e.twin.map(|t| &ix.index.entries[t]) {
        // The definition is the FUNCTION's header, whichever of the two the cursor is on.
        let (f, v) = if e.kind == EntryKind::Procedure { (e, t) } else { (t, e) };
        def = f.def;
        spans = f.refs.iter().copied().chain(v.uses()).collect();
    }
    if with_def {
        spans.push(def);
    }
    spans.sort_by_key(|s| (s.file, s.start));
    spans.dedup();
    spans
        .into_iter()
        .map(|s| Location::new(ix.analysis.info(s.file).uri.clone(), ix.analysis.range(s)))
        .collect()
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

/// Some procedure header of the program (a `SUB` or `FUNCTION`, one of a `DECLARE LIBRARY` block or a `DECLARE`) has
/// that name.
fn names_a_proc(a: &Analysis, t: Tok) -> bool {
    let key = proc_key(a, t);
    let mut found = false;
    for tree in &a.parsed.trees {
        visit(tree.root(), true, &mut |n| {
            found = found
                || ProcHeader::cast(n)
                    .and_then(|h| h.name())
                    .is_some_and(|name| proc_key(a, name) == key);
        });
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
