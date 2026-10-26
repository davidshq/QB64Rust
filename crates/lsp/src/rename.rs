//! `textDocument/prepareRename` and `textDocument/rename` (design D5 of `m5b-language-features`): every use of a
//! symbol renamed, each keeping the suffix written there, or the rename refused with the reason; and the check of
//! the renamed program's index before the edit is given.

use crate::hover::name_end;
use crate::index::{EntryKind, Indexed, SymbolIndex};
use crate::uri::{path_key, to_path};
use qb64rust_base::{FileId, Span};
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::tree::Node;
use std::path::Path;

/// A rename that passed the checks of the index: the symbol, and each use's name without its suffix with the new
/// name.
pub struct Plan {
    pub target: usize,
    pub new_name: String,
    pub edits: Vec<Span>,
    /// The program has `$IF` branches that are not compiled, which the rename does not reach.
    pub inactive: bool,
}

/// The symbol at `offset` of the main file that can be renamed, and the span of its name there without its suffix.
pub fn prepare(ix: &Indexed, offset: u32) -> Result<(usize, Span), String> {
    let main = ix.analysis.parsed.main().file;
    let Some((id, span)) = ix.index.entry_near(main, offset) else {
        return Err("there is no name to rename here".into());
    };
    let e = &ix.index.entries[id];
    if e.kind == EntryKind::Label && e.name.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!("`{}` is a line number, which cannot be renamed", e.name));
    }
    Ok((id, bare(ix, span, &e.name)))
}

/// The span of a use's name without its type suffix, and without the members after it when the token is a dotted
/// name: `p.x`, whose `x` is a use of a member, or `m.OFFSET` of a `_MEM`, whose members are no symbols (cut where
/// the name `written` at the definition ends).
fn bare(ix: &Indexed, span: Span, written: &str) -> Span {
    let text = ix.analysis.map.text(span);
    let own = name_end(written.as_bytes());
    // A dotted name written with blanks around its dots is one token (`p . r` of the array `p.r`): the name is as
    // much of it as spells the written name, blanks aside.
    if text.contains(&b' ') {
        let (mut seen, mut i) = (0, 0);
        while i < text.len() && seen < own {
            if text[i] != b' ' {
                if !text[i].eq_ignore_ascii_case(&written.as_bytes()[seen]) {
                    break;
                }
                seen += 1;
            }
            i += 1;
        }
        let name_char = |b: u8| b.is_ascii_alphanumeric() || b == b'_' || b == b'.';
        if seen == own && own > 0 && text.get(i).is_none_or(|&b| !name_char(b)) {
            return Span::new(span.file, span.start, span.start + qb64rust_base::to_u32(i));
        }
    }
    let mut end = name_end(text);
    if let Some(next) = ix.index.next_start(span.file, span.start).filter(|&n| n < span.end)
        && let Some(dot) = text[..(next - span.start) as usize].iter().rposition(|&b| b == b'.')
    {
        end = end.min(dot);
    }
    if own < end
        && text[..own].eq_ignore_ascii_case(&written.as_bytes()[..own])
        && text[own..end].iter().find(|&&b| b != b' ') == Some(&b'.')
    {
        end = own;
    }
    // Blanks before the dot of a name written `p . x`.
    while end > 0 && text[end - 1] == b' ' {
        end -= 1;
    }
    Span::new(span.file, span.start, span.start + qb64rust_base::to_u32(end))
}

/// The checks of the index (design D5, cases 1 to 5) for renaming the symbol at `offset` to `new_name`; `root` is
/// the compiler root.
pub fn plan(ix: &Indexed, offset: u32, new_name: &str, root: &Path) -> Result<Plan, String> {
    let (target, _) = prepare(ix, offset)?;
    plan_entry(ix, target, new_name, root)
}

/// [`plan`] for the symbol `ix.index.entries[target]`.
pub fn plan_entry(ix: &Indexed, target: usize, new_name: &str, root: &Path) -> Result<Plan, String> {
    let e = &ix.index.entries[target];
    if e.kind == EntryKind::Label && e.name.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!("`{}` is a line number, which cannot be renamed", e.name));
    }
    if let Some(e) = &ix.index.error {
        let file = ix.analysis.map.file(e.span.file);
        let (line, _) = file.line_col(e.span.start);
        let name = file_name(ix, e.span.file);
        return Err(format!(
            "the program has an error ({name}, line {line}: {}); fix it before renaming, as the uses found may be incomplete",
            e.message
        ));
    }
    let entry = &ix.index.entries[target];
    let text = entry.name.as_bytes();
    if entry.kind == EntryKind::Constant && qb64rust_builtins::auto_constant(&text[..name_end(text)]).is_some() {
        return Err(format!("`{}` is one of QB64pe's own constants", entry.name));
    }
    if entry.library {
        return Err(format!(
            "`{}` is a procedure of a `DECLARE LIBRARY` block, whose name may be its C function's",
            entry.name
        ));
    }
    // Compared by their keys (`uri::path_key`), as the server compares documents: letter case aside on Windows,
    // where the root's setting and a file's URI may spell a folder differently.
    let main_dir =
        to_path(&ix.analysis.info(ix.analysis.parsed.main().file).uri).and_then(|p| p.parent().map(path_key));
    let (root_key, internal) = (path_key(root), path_key(&root.join("internal")));
    let under = |key: &str, dir: &str| {
        key.strip_prefix(dir.trim_end_matches('/'))
            .is_some_and(|rest| rest.starts_with('/'))
    };
    // QB64pe's own files: under the root's `internal` folder, or elsewhere under the root but not in the main file's
    // folder (a program kept in the QB64pe folder is the user's, with the files beside it).
    let own = |key: &str| {
        under(key, &internal) || (under(key, &root_key) && !main_dir.as_deref().is_some_and(|d| under(key, d)))
    };
    for use_ in entry.uses() {
        if own(&ix.analysis.info(use_.file).key) {
            let name = file_name(ix, use_.file);
            return Err(format!(
                "`{}` is used in {name}, a file of the compiler's own",
                entry.name
            ));
        }
    }
    check_name(new_name)?;
    if let Some(other) = clash(&ix.index, target, new_name) {
        return Err(format!(
            "`{new_name}` is already the name of `{}`, which a use of `{}` can see",
            ix.index.entries[other].name, entry.name
        ));
    }
    // A FUNCTION and its result variable are one name.
    let twin = entry.twin.map(|t| &ix.index.entries[t]);
    let mut edits: Vec<Span> = entry.uses().map(|s| bare(ix, s, &entry.name)).collect();
    if let Some(t) = twin {
        edits.extend(t.uses().map(|s| bare(ix, s, &t.name)));
    }
    edits.sort_by_key(|s| (s.file, s.start));
    edits.dedup();
    let inactive = ix.analysis.parsed.trees.iter().any(|t| has_inactive(t.root()));
    Ok(Plan {
        target,
        new_name: new_name.to_string(),
        edits,
        inactive,
    })
}

/// The new name is a name of the program's own: letters, digits and `_`, a letter first or after leading `_`s; no
/// suffix, no `.` (a dotted name is another symbol in QB64), not a reserved word, not a built-in's or QB64pe
/// constant's name. (A name QB64 refuses otherwise is caught by the check of the edit.)
fn check_name(new_name: &str) -> Result<(), String> {
    let b = new_name.as_bytes();
    let first = b.iter().find(|&&c| c != b'_');
    if !first.is_some_and(u8::is_ascii_alphabetic) || !b.iter().all(|c| c.is_ascii_alphanumeric() || *c == b'_') {
        let why = if b.iter().any(|c| b"$%&!#~`".contains(c)) {
            "give it without a type suffix: each use keeps its own"
        } else {
            "a name is letters, digits and `_`, with a letter first"
        };
        return Err(format!("`{new_name}` is not a name ({why})"));
    }
    if b[0] == b'_' && b.get(1) != Some(&b'_') {
        return Err(format!("`{new_name}` starts with one `_`, as QB64's own names do"));
    }
    if qb64rust_syntax::is_keyword(b) {
        return Err(format!("`{new_name}` is a reserved word"));
    }
    if qb64rust_builtins::find_any(b).next().is_some() {
        return Err(format!("`{new_name}` is the name of a built-in"));
    }
    if qb64rust_builtins::auto_constant(b).is_some() {
        return Err(format!("`{new_name}` is the name of one of QB64pe's constants"));
    }
    Ok(())
}

/// An entry the renamed symbol would clash with (design D5, case 3): of the new name (in any letter case) and seen
/// where the symbol is used, or seeing the symbol where it is used itself, its definition written with the same
/// suffix or either without one (a plain name may be either). A procedure or user type clashes with any entry of the name; a label
/// only with a label of the same body or a procedure. Cautious: some renames the compiler would take are refused.
fn clash(ix: &SymbolIndex, target: usize, new_name: &str) -> Option<usize> {
    let t = &ix.entries[target];
    let upper = new_name.to_ascii_uppercase();
    // The suffix its definition is written with.
    let suffix = |e: &crate::index::Entry| -> Vec<u8> {
        let text = e.name.as_bytes();
        text[name_end(text)..].to_vec()
    };
    let bodies_of = |e: &crate::index::Entry| -> Vec<Option<usize>> {
        let mut v: Vec<Option<usize>> = e.uses().map(|s| ix.body_at(s.file, s.start)).collect();
        v.sort_unstable();
        v.dedup();
        v
    };
    let wide = |k: EntryKind| matches!(k, EntryKind::Procedure | EntryKind::Type);
    (0..ix.entries.len()).find(|&i| {
        let o = &ix.entries[i];
        let text = o.name.as_bytes();
        if i == target || t.twin == Some(i) || text[..name_end(text)].to_ascii_uppercase() != upper.as_bytes() {
            return false;
        }
        if wide(t.kind) || wide(o.kind) {
            return true;
        }
        match (t.kind, o.kind) {
            (EntryKind::Label, EntryKind::Label) => return t.scope == o.scope,
            (EntryKind::Label, _) | (_, EntryKind::Label) => return false,
            (EntryKind::Member, EntryKind::Member) => return t.user_type == o.user_type,
            (EntryKind::Member, _) | (_, EntryKind::Member) => return false,
            _ => {}
        }
        let (ts, os) = (suffix(t), suffix(o));
        (ts.is_empty() || os.is_empty() || ts == os)
            && (bodies_of(t).into_iter().any(|b| ix.visible_in(i, b))
                || bodies_of(o).into_iter().any(|b| ix.visible_in(target, b)))
    })
}

/// A tree holds a `$IF` branch that is not compiled.
fn has_inactive(node: Node) -> bool {
    node.child_nodes()
        .any(|n| n.kind() == SyntaxKind::InactiveCode || has_inactive(n))
}

/// A file's name for a message: its file name, else its name in the map.
fn file_name(ix: &Indexed, file: FileId) -> String {
    to_path(&ix.analysis.info(file).uri)
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
        .unwrap_or_else(|| ix.analysis.map.file(file).name.clone())
}

/// The bytes of `file` with each of the plan's edits in it applied.
pub fn renamed_bytes(ix: &Indexed, plan: &Plan, file: FileId) -> Vec<u8> {
    let bytes = &ix.analysis.map.file(file).bytes;
    let mut edits: Vec<Span> = plan.edits.iter().copied().filter(|s| s.file == file).collect();
    edits.sort_by_key(|s| s.start);
    let mut out = Vec::with_capacity(bytes.len());
    let mut at = 0;
    for s in edits {
        out.extend_from_slice(&bytes[at..s.start as usize]);
        out.extend_from_slice(plan.new_name.as_bytes());
        at = s.end as usize;
    }
    out.extend_from_slice(&bytes[at..]);
    out
}

/// Where `offset` of `file` is after the plan's edits.
fn moved(plan: &Plan, file: FileId, offset: u32) -> u32 {
    let new = qb64rust_base::to_u32(plan.new_name.len());
    plan.edits
        .iter()
        .filter(|s| s.file == file && s.end <= offset)
        .fold(offset, |o, s| o + new - (s.end - s.start))
}

/// The check of the edit (design D5): the renamed program's index has no error, the same symbols with as many uses
/// each, and the renamed symbol now has the new name.
pub fn verify(old: &SymbolIndex, plan: &Plan, new: &SymbolIndex) -> Result<(), String> {
    let changed = "the rename would change the program";
    if let Some(e) = &new.error {
        return Err(format!("{changed} (it would have an error: {})", e.message));
    }
    if old.entries.len() != new.entries.len() {
        return Err(format!(
            "{changed} ({} names before, {} after)",
            old.entries.len(),
            new.entries.len()
        ));
    }
    for (i, e) in old.entries.iter().enumerate() {
        let at = moved(plan, e.def.file, e.def.start);
        let Some((j, _)) = new.entry_at(e.def.file, at) else {
            return Err(format!("{changed} (`{}` would no longer be a name)", e.name));
        };
        let n = &new.entries[j];
        if n.kind != e.kind || n.refs.len() != e.refs.len() || n.def.start != at {
            return Err(format!("{changed} (`{}` would mean something else)", e.name));
        }
        if n.ty != e.ty {
            return Err(format!(
                "{changed} (`{}` would be {} where it is {}; a `DEFtype` may type the new name's first letter)",
                e.name, n.ty, e.ty
            ));
        }
        let text = n.name.as_bytes();
        let renamed = i == plan.target || old.entries[plan.target].twin == Some(i);
        if renamed && !text[..name_end(text)].eq_ignore_ascii_case(plan.new_name.as_bytes()) {
            return Err(format!("{changed} (`{}` was not renamed)", e.name));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert!(check_name("total").is_ok());
        assert!(check_name("step_up2").is_ok());
        assert!(check_name("print").unwrap_err().contains("reserved"));
        assert!(check_name("LEFT").unwrap_err().contains("built-in"));
        assert!(check_name("x$").unwrap_err().contains("suffix"));
        assert!(check_name("a.b").unwrap_err().contains("not a name"));
        assert!(check_name("2x").unwrap_err().contains("not a name"));
        assert!(check_name("").is_err());
    }
}
