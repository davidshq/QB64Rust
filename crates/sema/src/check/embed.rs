//! `$EMBED:'file','handle'` and `_EMBEDDED$("handle")` (design D1 and D2 of `m3-network-embed`; `qb64pe.bas`
//! 3743–3818 and 20443–20480; measured, `verification\v44_a_embed`, `v44_x01`–`x39`).
//!
//! The old compiler registers a `$EMBED` as its main pass meets the line, and `_EMBEDDED$` looks only at what is
//! registered: a call accepts a handle only from a line above it in file order (`x14`, `x15`, `x34`). The checker
//! does not visit procedures in file order, so every compiled `$EMBED` line is collected first
//! ([`Checker::collect_embeds`]), each with its place in file order (an included file's lines take the place of its
//! `$INCLUDE`; the file included after the last line comes after everything).

use super::{Checker, R, trim_spaces};
use crate::{Expr, ExprKind, Ty};
use qb64rust_base::{Span, show_bytes};
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::ast;
use qb64rust_syntax::pp::meta_line;
use qb64rust_syntax::tree::{Node, Tok, TreeId};
use std::collections::HashMap;

/// One compiled `$EMBED` line whose file and handle are well formed.
pub(super) struct EmbedLine {
    /// Its place in file order ([`Checker::place`]).
    place: Vec<u32>,
    /// The metacommand's node (tree and offset), and its token.
    key: (TreeId, u32),
    span: Span,
    /// The handle and the file's name as written, without the blanks at their ends.
    handle: Vec<u8>,
    file: Vec<u8>,
    /// Pass 2 checked the line without an error: the file is the program's.
    registered: bool,
    /// A compiled `_EMBEDDED$` names its handle.
    used: bool,
}

/// The file and the handle of a `$EMBED` line (`raw`, the metacommand token's text): the text between its first two
/// `'` and between the next two, without the spaces at their ends; the rest of the line is dropped. `None` when a
/// quote is missing or either is empty: "Expected $EMBED:'filename','handle'".
fn parts(raw: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    let mut quoted = raw.split(|&b| b == b'\'').skip(1);
    let file = trim_spaces(quoted.next()?).to_vec();
    // The closing quote of the file and the opening one of the handle must both be there.
    let _between = quoted.next()?;
    let handle = quoted.next()?;
    quoted.next()?;
    let handle = trim_spaces(handle).to_vec();
    (!file.is_empty() && !handle.is_empty()).then_some((file, handle))
}

/// The error of a handle that is not a letter followed by letters and digits (ASCII; measured, `x05`, `x06`, `x29`,
/// `x25`), as the old compiler checks it.
fn handle_error(handle: &[u8]) -> Option<String> {
    let shown = show_bytes(handle);
    if !handle[0].is_ascii_alphabetic() {
        return Some(format!(
            "the first character of the embed handle `{shown}` must be a letter"
        ));
    }
    (!handle.iter().all(u8::is_ascii_alphanumeric))
        .then(|| format!("the embed handle `{shown}` may hold only letters A to Z and digits"))
}

/// Whether a `$EMBED` file name is absolute: a drive, a colon and a slash, or two slashes.
fn absolute(name: &[u8]) -> bool {
    matches!(name, [drive, b':', b'/' | b'\\', ..] if drive.is_ascii_alphabetic())
        || matches!(name, [b'/' | b'\\', b'/' | b'\\', ..])
}

/// Whether a `$EMBED` file name starts with `./` or `.\`: the main source file's folder.
fn beside_source(name: &[u8]) -> bool {
    name.starts_with(b"./") || name.starts_with(b".\\")
}

/// Whether two `$EMBED` file names name the same file as the old compiler compares them: it puts the folder in front
/// of the rest of a `./` or `.\` name, so the two prefixes are the same and the rest is compared as written
/// (measured, `verification\v44_x31`, `x69`, `x70`).
fn same_file(a: &[u8], b: &[u8]) -> bool {
    match (beside_source(a), beside_source(b)) {
        (true, true) => a[2..] == b[2..],
        (false, false) => a == b,
        _ => false,
    }
}

impl Checker<'_> {
    /// Collects every `$EMBED` line of compiled code whose file and handle are well formed, in file order.
    pub(super) fn collect_embeds(&mut self) {
        let program = self.program;
        let mut parents: HashMap<TreeId, (TreeId, u32)> = HashMap::new();
        for (&at, &tree) in &program.includes {
            parents.insert(tree, at);
        }
        self.embed_parents = parents;
        let mut found = Vec::new();
        let mut roots = vec![program.main().root()];
        roots.extend(program.after_last_line.map(|t| program.tree(t).root()));
        for root in roots {
            self.find_embeds(root, &mut found);
        }
        let mut lines: Vec<EmbedLine> = found
            .into_iter()
            .filter_map(|(key, tok): ((TreeId, u32), Tok)| {
                let (file, handle) = parts(self.text(tok.span))?;
                handle_error(&handle).is_none().then(|| EmbedLine {
                    place: self.place(key),
                    key,
                    span: tok.span,
                    handle,
                    file,
                    registered: false,
                    used: false,
                })
            })
            .collect();
        lines.sort_by(|a, b| a.place.cmp(&b.place));
        self.embeds = lines;
    }

    /// The `$EMBED` metacommands under `node`, through blocks, procedure bodies and included files; not in a `$IF`
    /// branch that is not taken (its lines are no statements).
    fn find_embeds(&self, node: Node, found: &mut Vec<((TreeId, u32), Tok)>) {
        for child in node.child_nodes() {
            if child.kind() == SyntaxKind::InactiveCode {
                continue;
            }
            if let Some(m) = ast::MetaStmt::cast(child) {
                if let Some(tok) = m.token()
                    && meta_line(self.text(tok.span)).starts_with(b"$EMBED")
                {
                    found.push((child.key(), tok));
                }
            } else if ast::MetaCommentStmt::cast(child).is_some() {
                if let Some(tree) = self.program.included(child) {
                    self.find_embeds(tree.root(), found);
                }
            } else {
                self.find_embeds(child, found);
            }
        }
    }

    /// The place of a node in file order: the offsets of the `$INCLUDE`s that lead to its tree, then its own; the
    /// file included after the last line after everything.
    fn place(&self, (mut tree, offset): (TreeId, u32)) -> Vec<u32> {
        let mut place = vec![offset];
        loop {
            if self.program.after_last_line == Some(tree) {
                place.insert(0, u32::MAX);
                break;
            }
            match self.embed_parents.get(&tree) {
                Some(&(parent, at)) => {
                    place.insert(0, at);
                    tree = parent;
                }
                None => break,
            }
        }
        place
    }

    /// A `$EMBED` line as pass 2 meets it: its form, its handle, the class of its file's name, and no earlier line
    /// with the same file or handle (measured, `x01`–`x09`, `x26`, `x29`, `x31`). A second line names the first
    /// one's line, a line of the main file (for one in an included file, its `$INCLUDE`'s line).
    pub(super) fn embed_meta(&mut self, node: Node, tok: Tok) -> R<()> {
        let Some((file, handle)) = parts(self.text(tok.span)) else {
            return Err(self.error(tok.span, "expected `$EMBED:'filename','handle'`"));
        };
        if !beside_source(&file) && !absolute(&file) {
            // The old compiler looks for such a name in its own folder (`x26`), which has no counterpart here (as
            // for `$EXEICON`, design D4 of `m3-sound`).
            let msg = "a `$EMBED` file name that neither starts with `./` nor is absolute";
            return Err(self.unsupported(tok.span, msg));
        }
        if let Some(msg) = handle_error(&handle) {
            return Err(self.error(tok.span, msg));
        }
        let at = self.place(node.key());
        let earlier = self.embeds.iter().take_while(|e| e.place < at).find_map(|e| {
            if same_file(&e.file, &file) {
                Some((e.key, format!("the file `{}` was already embedded", show_bytes(&file))))
            } else if e.handle == handle {
                Some((
                    e.key,
                    format!("the embed handle `{}` is already used", show_bytes(&handle)),
                ))
            } else {
                None
            }
        });
        if let Some((first, what)) = earlier {
            let line = self.main_line(first);
            return Err(self.error(tok.span, format!("{what} in line {line}")));
        }
        if let Some(e) = self.embeds.iter_mut().find(|e| e.key == node.key()) {
            e.registered = true;
        }
        Ok(())
    }

    /// `_EMBEDDED$("handle")` (design D2): one string literal by itself, not empty, a valid handle named by a
    /// `$EMBED` line above the call in file order (measured, `x10`–`x15`, `x18`, `x20`, `x21`, `x25`, `x33`, `x34`,
    /// `x36`). The call marks the file used; it is the plain call of its row.
    pub(super) fn embedded(
        &mut self,
        s: crate::builtins::Supported,
        list: Option<ast::ArgList>,
        span: Span,
    ) -> R<Expr> {
        let arity = || format!("`{}` takes 1 argument", s.name);
        let Some(list) = list else {
            return Err(self.error(span, arity()));
        };
        let nodes = self.present_args(Some(list))?;
        let [arg] = nodes[..] else {
            return Err(self.error(span, arity()));
        };
        let literal = match arg {
            ast::Expr::Literal(lit) => lit.token().filter(|t| t.kind == SyntaxKind::StringLit),
            ast::Expr::Paren(_) => {
                let msg = format!("the argument of `{}` in parentheses", s.name);
                return Err(self.unsupported(arg.node().span(), msg));
            }
            ast::Expr::NameRef(_)
            | ast::Expr::Call(_)
            | ast::Expr::Field(_)
            | ast::Expr::Prefix(_)
            | ast::Expr::Bin(_) => None,
        };
        let Some(t) = literal else {
            let msg = format!(
                "the embed handle of `{}` must be one string literal, not an expression",
                s.name
            );
            return Err(self.error(arg.node().span(), msg));
        };
        let e = self.expr(arg)?;
        let ExprKind::Str(handle) = &e.kind else {
            unreachable!("a string literal is a string constant")
        };
        if handle.is_empty() {
            return Err(self.error(t.span, "an embed handle cannot be an empty string"));
        }
        if let Some(msg) = handle_error(handle) {
            return Err(self.error(t.span, msg));
        }
        let at = self.place(list.node().key());
        let Some(i) = self.embeds.iter().position(|l| l.place < at && l.handle == *handle) else {
            let msg = format!(
                "the embed handle `{}` is named by no `$EMBED` line above",
                show_bytes(handle)
            );
            return Err(self.error(t.span, msg));
        };
        self.embeds[i].used = true;
        self.runtime_parts.insert(crate::RuntimePart::Embed);
        Ok(Expr {
            span,
            ty: Ty::Str,
            qb: Ty::Str,
            kind: ExprKind::Call {
                builtin: s.id,
                args: vec![Some(e)],
            },
        })
    }

    /// The files the program's `$EMBED` lines name, for its settings.
    pub(super) fn embedded_files(&self) -> Vec<crate::EmbeddedFile> {
        self.embeds
            .iter()
            .filter(|e| e.registered)
            .map(|e| {
                // A file name, taken as UTF-8 as an included file's is.
                #[expect(clippy::disallowed_methods, reason = "a file name, not BASIC source")]
                let name = String::from_utf8_lossy(&e.file).into_owned();
                crate::EmbeddedFile {
                    handle: show_bytes(&e.handle),
                    name,
                    shown: show_bytes(&e.file),
                    span: e.span,
                    used: e.used,
                }
            })
            .collect()
    }
}
