//! Metacommands and metacommand comments. Every `$…` line is one `MetaStmt`; the preprocessor's lines (`$IF`,
//! `$ELSEIF`, `$ELSE`, `$END IF`, `$LET`, `$ERROR`) are evaluated here, in file order, with the parser's
//! [`PpState`](crate::pp::PpState) (design D8). A branch that is not taken becomes one `InactiveCode` node.
//!
//! An open `$IF` is an entry on the block stack, as in the old compiler (`qb64pe.bas` 3430–3470: `$IF` pushes a
//! control level and `$END IF` pops the top one, measured M6): blocks and `$IF`s must nest properly. A block closer
//! that meets an open `$IF` (`blocks.rs`) and a `$ELSE`/`$ELSEIF`/`$END IF` that meets an open block are errors;
//! procedure headers and `EXIT` look past `$IF` entries.

use super::Parser;
use super::blocks::Block;
use crate::SyntaxKind::*;
use crate::meta::comment_directives;
use crate::pp::{Directive, directive, precompiler_flag};
use crate::program::LoadError;
use qb64rust_base::show_bytes;

/// A metacommand line as a `MetaStmt`. Returns true when it also took its line end (an inactive branch follows,
/// which it put into an `InactiveCode` node).
pub(crate) fn metacommand(p: &mut Parser) -> bool {
    let span = p.current_span();
    let text = p.nth_text(0);
    let d = directive(text);
    let preprocessor = !matches!(d, Directive::Other);
    if preprocessor && p.after_colon() && !matches!(d, Directive::Error(_)) {
        // Measured M6: "Unexpected character on line" after `:`.
        p.error("a preprocessor metacommand must stand at the start of a line");
        p.start_node(MetaStmt);
        p.bump();
        p.finish_node();
        return false;
    }
    p.start_node(MetaStmt);
    p.bump();
    p.finish_node();
    let result = match d {
        Directive::Other => {
            if trimmed_upper(text) == b"$INCLUDEONCE" {
                // Later inclusions of this file are empty (`qb64pe.bas` 3141–3165, measured M7).
                p.inc.once.insert(p.file);
            }
            Ok(())
        }
        Directive::Malformed(msg) if msg.contains("$ELSEIF") => Err(msg.to_string()),
        Directive::If(_) | Directive::Malformed(_) => {
            // The level is opened even when the line has an error (as in the old compiler, which counts it before
            // evaluating), so its `$END IF` gets no error of its own; its code is then compiled.
            let r = match d {
                // `sema` marks it "not supported yet" (`MetaStmt::precompiler_flag`); taken, like a broken `$IF`.
                Directive::If(cond) if precompiler_flag(&cond).is_some() => Err(String::new()),
                Directive::If(cond) => p.pp.open_if(&cond),
                Directive::Malformed(msg) => Err(msg.to_string()),
                _ => Ok(()),
            };
            if r.is_err() {
                p.pp.open_broken();
            }
            p.push_block(Block::PpIf, span);
            p.pp_ifs.push((span, false));
            r
        }
        Directive::ElseIf(cond) => {
            p.pp_branch(span);
            match precompiler_flag(&cond) {
                // Not taken; `sema` marks it "not supported yet", which rejects the program.
                Some(_) => p.pp.else_if(b"0"),
                None => p.pp.else_if(&cond),
            }
        }
        Directive::Else => {
            p.pp_branch(span);
            p.pp.else_()
        }
        Directive::EndIf => {
            let r = p.pp.end_if();
            if r.is_ok() {
                p.pp_end(span);
            }
            r
        }
        Directive::Let(text) => p.pp.let_(&text),
        Directive::Error(text) => Err(format!("`$ERROR`: {}", show_bytes(&text))),
    };
    match result {
        // A precompiler flag, marked by `sema`.
        Err(msg) if msg.is_empty() => {}
        Err(msg) => p.error_at(span, msg),
        Ok(()) => {}
    }
    if p.pp.active() {
        return false;
    }
    // The branch is not taken: its lines up to the next `$ELSEIF`, `$ELSE` or `$END IF` of this level.
    if p.at(Newline) {
        p.bump();
    }
    inactive_code(p);
    true
}

fn trimmed_upper(text: &[u8]) -> Vec<u8> {
    text.trim_ascii().to_ascii_uppercase()
}

/// A metacommand comment (`'$INCLUDE:'x.bi'`) as a statement of its own; `sema` reads its other directives. An
/// `$INCLUDE` in it includes the file right after it (a comment ends its line, so that is after the line, as in
/// the old compiler).
pub(crate) fn meta_comment(p: &mut Parser) {
    let span = p.current_span();
    let include = comment_directives(p.nth_text(0)).ok().and_then(|d| d.include);
    p.start_node(MetaCommentStmt);
    p.bump();
    p.finish_node();
    if let Some(name) = include {
        p.include(&name, span);
    }
}

/// How deep files may include each other (the old compiler's limit, "Too many indwelling INCLUDE files", M7).
const MAX_INCLUDE_DEPTH: u32 = 100;

/// The tokens of an inactive branch, up to (not including) the `$ELSEIF`, `$ELSE` or `$END IF` that ends it, or
/// the end of the file. Other lines are not looked at, but the old compiler's prepass checks the preprocessor lines
/// of nested `$IF`s there too (`qb64pe.bas` 1836–1892, before its skip test; measured: `$IF` without `THEN`, a
/// duplicate operator and a second `$ELSE` are errors in an inactive branch): their conditions are evaluated for
/// errors only, and every `$IF` line opens a level, also one without `THEN`. `$LET` and `$ERROR` there are skipped,
/// as in the old compiler.
fn inactive_code(p: &mut Parser) {
    p.start_node(InactiveCode);
    // Per nested `$IF`: whether it had its `$ELSE`.
    let mut nested: Vec<bool> = Vec::new();
    while let Some(k) = p.current() {
        if k == Metacommand {
            let span = p.current_span();
            let d = directive(p.nth_text(0));
            let error = match (&d, nested.last_mut()) {
                (Directive::EndIf | Directive::Else | Directive::ElseIf(_), None) => break,
                // Malformed `$ELSEIF` of this level: `metacommand` reports it.
                (Directive::Malformed(msg), None) if msg.contains("$ELSEIF") => break,
                (Directive::EndIf, Some(_)) => {
                    nested.pop();
                    None
                }
                (Directive::Else, Some(has_else)) => {
                    let again = std::mem::replace(has_else, true);
                    again.then(|| "this `$IF` already has an `$ELSE`".to_string())
                }
                (Directive::ElseIf(cond), Some(has_else)) => {
                    if *has_else {
                        Some("`$ELSEIF` cannot follow `$ELSE`".to_string())
                    } else {
                        nested_condition_error(p, cond)
                    }
                }
                (Directive::Malformed(msg), Some(_)) if msg.contains("$ELSEIF") => Some(msg.to_string()),
                (Directive::If(cond), _) => {
                    nested.push(false);
                    nested_condition_error(p, cond)
                }
                (Directive::Malformed(msg), _) => {
                    nested.push(false);
                    Some(msg.to_string())
                }
                (Directive::Let(_) | Directive::Error(_) | Directive::Other, _) => None,
            };
            if let Some(msg) = error {
                // Each preprocessor line is a statement of its own: one error per line, not per branch.
                p.stmt_error = false;
                p.error_at(span, msg);
            }
        }
        p.bump();
    }
    p.eat_trivia();
    p.finish_node();
}

/// The error of a nested `$IF`/`$ELSEIF` condition in an inactive branch, if any (its value does not matter; a
/// precompiler flag is not evaluated).
fn nested_condition_error(p: &Parser, cond: &[u8]) -> Option<String> {
    if precompiler_flag(cond).is_some() {
        return None;
    }
    p.pp.eval(cond).err()
}

impl Parser<'_, '_> {
    /// Includes the file `name` (as written) at the include statement at `span` (design D9): loaded through the
    /// loader (which resolves the path), parsed at once as a tree of its own with the preprocessor state as it is
    /// here, which it then hands back. A file that held `$INCLUDEONCE` is not included again. Blocks open here
    /// are marked as crossing a file boundary (`blocks.rs`).
    fn include(&mut self, name: &[u8], span: qb64rust_base::Span) {
        let shown = show_bytes(name);
        if self.inc.depth >= MAX_INCLUDE_DEPTH {
            let here = self.inc.map.file(self.file).name.clone();
            let msg = format!("`$INCLUDE` nested more than {MAX_INCLUDE_DEPTH} files deep (in `{here}`)");
            self.error_at(span, msg);
            return;
        }
        let file = match self.inc.loader.load(self.inc.map, self.file, name) {
            Ok(f) => f,
            Err(LoadError::NotFound) => {
                self.error_at(span, format!("included file `{shown}` not found"));
                return;
            }
            Err(LoadError::Unreadable(e)) => {
                self.error_at(span, format!("cannot read the included file `{shown}`: {e}"));
                return;
            }
        };
        if self.inc.once.contains(&file) {
            return;
        }
        let id = self.inc.next_tree();
        self.inc.includes.insert((self.tree, span.start), id);
        let bytes = self.inc.map.file(file).bytes.clone();
        let depth = self.pp.depth();
        let saved = self.pp.start_tracking();
        let pp = std::mem::take(&mut self.pp);
        self.inc.depth += 1;
        let (tree, pp) = super::parse_tree_with(id, file, &bytes, pp, self.inc, true);
        self.inc.depth -= 1;
        self.pp = pp;
        let low = self.pp.stop_tracking(saved);
        self.inc.trees[id.0 as usize] = Some(tree);
        self.mark_blocks_crossed();
        self.pp_ifs_closed_elsewhere(depth.saturating_sub(low));
    }

    /// The included file closed `closed` open `$IF`s (its `$END IF` found none of them in its own file): the
    /// innermost ones of this file, as far as this file has them (the rest belong to a file including this one,
    /// which sees the same when this file is done). Marked like a `$IF` an included file leaves open (design D4):
    /// a `$IF` and its `$END IF` in two files are not supported yet.
    fn pp_ifs_closed_elsewhere(&mut self, closed: usize) {
        for _ in 0..closed {
            let Some((span, crossed)) = self.pp_ifs.pop() else {
                return;
            };
            self.block_unsupported(span, "a `$IF` closed in another file");
            if !crossed {
                self.remove_last_pp_if();
            }
        }
    }

    /// The current token follows a `:` on its line (the lexer starts a metacommand there too).
    fn after_colon(&self) -> bool {
        self.last_kind == Some(Colon)
    }

    /// `$ELSE` or `$ELSEIF`: the `$IF` must be the innermost open entry.
    fn pp_branch(&mut self, span: qb64rust_base::Span) {
        if self.pp_ifs.last().is_some_and(|&(_, crossed)| !crossed) && !self.top_is_pp_if() {
            self.pp_crossing(span);
        }
    }

    /// `$END IF` (its `$IF` exists): pops the `$IF` entry, which must be the innermost one.
    fn pp_end(&mut self, span: qb64rust_base::Span) {
        let Some((_, crossed)) = self.pp_ifs.pop() else {
            // The `$IF` is in another file (an include): nothing of it is on this file's stack.
            return;
        };
        if crossed {
            return;
        }
        if !self.top_is_pp_if() {
            self.pp_crossing(span);
        }
        self.remove_last_pp_if();
    }

    /// A `$ELSE`/`$ELSEIF`/`$END IF` while a block opened after the `$IF` is still open.
    fn pp_crossing(&mut self, span: qb64rust_base::Span) {
        let words = match directive(self.text_of(span)) {
            Directive::EndIf => "$END IF",
            Directive::Else => "$ELSE",
            _ => "$ELSEIF",
        };
        let msg = match self.innermost_block() {
            Some((opener, closer, line)) => {
                format!("`{words}` while the `{opener}` on line {line} is still open; expected `{closer}` first")
            }
            None => format!("`{words}` does not match the open blocks"),
        };
        self.error_at(span, msg);
    }
}
