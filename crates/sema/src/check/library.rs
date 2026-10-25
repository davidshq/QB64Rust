//! `DECLARE LIBRARY` blocks (design D1 of `m3-declare-library`; measured, `verification\v37_*`, `study\00` §5): pass
//! 1 enters each block as a [`Library`] and each declaration in it as a [`Proc`] with an [`External`] part, where
//! the block stands in the file: in the main module, a SUB, an included file. Calls of them go through the checks of
//! every call (`proc.rs`); what differs is decided here.

use super::decl::name_end;
use super::{Checker, Failed, R, Scope, Skips};
use crate::consteval::Value;
use crate::{External, Library, LibraryId, LibraryKind, LibraryName, Proc, ProcId, ProcKind, SymbolKind, Ty};
use qb64rust_base::{show_bytes, to_u32};
use qb64rust_builtins::{Kind, find_any};
use qb64rust_syntax::SyntaxKind;
use qb64rust_syntax::ast;
use qb64rust_syntax::tree::Node;
use std::collections::HashSet;

/// A library block met by pass 1, with the names of the user types defined above it in the file.
pub(super) struct Block<'a> {
    node: ast::DeclareLibraryBlock<'a>,
    types_before: HashSet<String>,
}

/// A declaration entered by pass 1 whose parameters are read after every procedure name is known.
pub(super) struct Declared<'a> {
    header: ast::ProcHeader<'a>,
    id: ProcId,
    block: Node<'a>,
    types_before: HashSet<String>,
}

impl Declared<'_> {
    pub(super) fn id(&self) -> ProcId {
        self.id
    }
}

impl<'a> Checker<'a> {
    /// The library blocks of the program in file order: of the main module, of procedure bodies and blocks, of
    /// included files at their include; not those in a `$IF` branch that is not compiled.
    pub(super) fn library_blocks(&self, top: &[Node<'a>]) -> Vec<Block<'a>> {
        let mut out = Vec::new();
        let mut types = HashSet::new();
        for &n in top {
            self.walk_libraries(n, &mut types, &mut out);
        }
        out
    }

    fn walk_libraries(&self, node: Node<'a>, types: &mut HashSet<String>, out: &mut Vec<Block<'a>>) {
        if ast::InactiveCode::cast(node).is_some() {
            return;
        }
        if let Some(b) = ast::TypeBlock::cast(node) {
            if let Some(t) = b.header().and_then(|h| h.name()) {
                types.insert(show_bytes(
                    &self.text(t.span)[..name_end(self.text(t.span))].to_ascii_uppercase(),
                ));
            }
            return;
        }
        if let Some(b) = ast::DeclareLibraryBlock::cast(node) {
            out.push(Block {
                node: b,
                types_before: types.clone(),
            });
            return;
        }
        if ast::MetaCommentStmt::cast(node).is_some() {
            let program = self.program;
            if let Some(tree) = program.included(node) {
                self.walk_libraries(tree.root(), types, out);
            }
            return;
        }
        for n in node.child_nodes() {
            self.walk_libraries(n, types, out);
        }
    }

    /// Enters a block and its declarations (pass 1, after the program's own procedures). A block whose header has
    /// a parse error, and a declaration with one, enter their names as broken procedures, so their uses get no
    /// second error.
    pub(super) fn declare_library(&mut self, block: Block<'a>, skips: &Skips, declared: &mut Vec<Declared<'a>>) {
        let b = block.node;
        self.at.defaults = self.defaults_at_node(b.node());
        let usable = b.header().is_some_and(|h| skips.usable(h.node())) && !skips.past_cap(b.node());
        let lib = if usable {
            self.stmt.stmt_error = false;
            let lib = self.library(b);
            self.flush_names();
            lib.ok()
        } else {
            None
        };
        for header in b.procs() {
            if skips.past_cap(header.node()) {
                break;
            }
            let id = match lib {
                Some(lib) if skips.usable(header.node()) => {
                    self.stmt.stmt_error = false;
                    let id = self.declare_external(header, lib);
                    self.flush_names();
                    id
                }
                _ => Err(Failed),
            };
            match id {
                Ok(id) => declared.push(Declared {
                    header,
                    id,
                    block: b.node(),
                    types_before: block.types_before.clone(),
                }),
                Err(Failed) => self.declare_broken(header),
            }
        }
    }

    /// The block's form and names (measured, `verification\v37_x03`, `x06`, `x27`).
    fn library(&mut self, b: ast::DeclareLibraryBlock) -> R<LibraryId> {
        let header = self.need(b.header(), b.node().span())?;
        let kind = match header.kind_word().map(|t| self.word(t)).as_deref() {
            None => LibraryKind::Plain,
            Some("STATIC") => LibraryKind::Static,
            Some("DYNAMIC") => LibraryKind::Dynamic,
            Some(_) => LibraryKind::Customtype,
        };
        let mut names = Vec::new();
        for t in header.names() {
            let raw = self.text(t.span);
            let inner = raw.strip_prefix(b"\"").unwrap_or(raw);
            let inner = inner.strip_suffix(b"\"").unwrap_or(inner);
            // An empty name is no name: nothing is linked or included (measured, `x27`).
            if !inner.is_empty() {
                names.push(LibraryName {
                    text: show_bytes(inner),
                    span: t.span,
                });
            }
        }
        let span = header.node().span();
        if kind == LibraryKind::Dynamic {
            if names.is_empty() {
                // Measured: "Expected DECLARE DYNAMIC LIBRARY "..."" (`x03`).
                return Err(self.error(span, "`DECLARE DYNAMIC LIBRARY` needs the name of a DLL in quotes"));
            }
            if names.len() > 1 {
                // Measured: "Cannot specify multiple DYNAMIC LIBRARY names in a single DECLARE statement" (`x06`).
                return Err(self.error(span, "`DECLARE DYNAMIC LIBRARY` takes one name"));
            }
        }
        let id = LibraryId(to_u32(self.prog.libraries.len()));
        let at = self.track(header.node());
        self.prog.libraries.push(Library { kind, names, span, at });
        Ok(id)
    }

    /// Enters one declaration as a procedure with its C name (its parameters come later, [`Self::external_params`]).
    fn declare_external(&mut self, header: ast::ProcHeader, library: LibraryId) -> R<ProcId> {
        let span = header.node().span();
        let keyword = self.need(header.keyword(), span)?;
        let name_tok = self.need(header.name(), span)?;
        let shown = show_bytes(self.text(name_tok.span));
        let function = self.word(keyword) == "FUNCTION";
        if super::proc::fixed_name(self.text(name_tok.span)).is_some() {
            // Not measured.
            return Err(self.unsupported(name_tok.span, format!("a library procedure named with `$n`: `{shown}`")));
        }
        let (name, suffix) = self.split_name(name_tok)?;
        let kind = if !function {
            if suffix.is_some() {
                // Measured: "Type symbols after a SUB name are invalid" (`verification\v37_x44`).
                return Err(self.error(
                    name_tok.span,
                    format!("a SUB name cannot have a type suffix: `{shown}`"),
                ));
            }
            ProcKind::Sub
        } else {
            let ty = suffix.unwrap_or_else(|| self.default_ty(&name));
            if let Ty::FixedStr(_) = ty {
                // A default type `STRING * n`: not measured.
                let msg = format!("a library FUNCTION whose default type is `STRING * n`: `{shown}`");
                return Err(self.unsupported(name_tok.span, msg));
            }
            ProcKind::Function(ty)
        };
        // Measured: a built-in's name, with its suffix too, is "Name already in use" (`sqr#`, `x32`).
        let builtin_kind = if function { Kind::Function } else { Kind::Sub };
        if suffix.is_some() && find_any(name.as_bytes()).any(|b| b.kind == builtin_kind) {
            return Err(self.in_use(name_tok));
        }
        self.reserved_proc(name_tok, &name, suffix, function)?;
        let mut twin = false;
        if let Some(&other) = self.procs_by_name.get(&name) {
            let other = self.prog.proc(other);
            if other.external.is_none() && matches!(other.kind, ProcKind::Sub) != matches!(kind, ProcKind::Sub) {
                // A library FUNCTION beside a program SUB of the name, or a library SUB beside a program FUNCTION:
                // the name as a statement is the SUB, as a value the FUNCTION (design D11 of
                // `m4-statements-metacommands`; measured, `x22`, `v57_l01`–`l04`).
                twin = true;
            } else {
                // Measured: a declaration in a second block (`x40`).
                return Err(self.in_use(name_tok));
            }
        }
        let c_name = self.c_name(header, name_tok)?;
        let id = ProcId(to_u32(self.prog.procs.len()));
        self.prog.procs.push(Proc {
            name: name.clone(),
            kind,
            params: Vec::new(),
            result: None,
            stmts: Vec::new(),
            line: self.line(name_tok.span),
            end_line: self.line(name_tok.span),
            external: Some(External {
                library,
                c_name,
                byval: Vec::new(),
                at: self.track(header.node()),
            }),
        });
        self.param_scopes.push(Scope::default());
        if twin {
            self.proc_twins.insert(name, id);
        } else {
            self.procs_by_name.insert(name, id);
        }
        self.stmt.names.push((SymbolKind::Proc(id), name_tok.span));
        Ok(id)
    }

    /// The C name: the `ALIAS` text, else the name as written without its suffix, in its own letter case (measured,
    /// `verification\v37_x31`). A string `ALIAS` holds letters, digits and ` !%&()*+,-./:;<=>?[]^_{|}~`, so C++ names
    /// (`"h_max<int32_t>"`, `"hns::twice"`, `v37_e`); an empty one or another character is "Expected ALIAS
    /// name-in-library" (`x48`, `x29`).
    fn c_name(&mut self, header: ast::ProcHeader, name_tok: qb64rust_syntax::tree::Tok) -> R<String> {
        let Some(alias) = header.alias() else {
            let raw = self.text(name_tok.span);
            return Ok(show_bytes(&raw[..name_end(raw)]));
        };
        let raw = self.text(alias.span);
        if alias.kind != SyntaxKind::StringLit {
            return Ok(show_bytes(raw));
        }
        let inner = raw.strip_prefix(b"\"").unwrap_or(raw);
        let inner = inner.strip_suffix(b"\"").unwrap_or(inner);
        let allowed = |b: &u8| b.is_ascii_alphanumeric() || b" !%&()*+,-./:;<=>?[]^_{|}~".contains(b);
        if inner.is_empty() || !inner.iter().all(allowed) {
            let msg = format!("`ALIAS {}` is no name a library can hold", show_bytes(raw));
            return Err(self.error(alias.span, msg));
        }
        Ok(show_bytes(inner))
    }

    /// The parameters of a declaration, read as a program procedure's are ([`Self::declare_params`]), after the
    /// forms a library rejects (measured, `verification\v37_x10`, `x11`, `x15`, `x26`): an array, `STRING * n`, a
    /// `STRING` with `BYVAL`, a user type defined below the block. A `_BIT` parameter is accepted here; its use is
    /// the error (`x25`).
    pub(super) fn external_params(&mut self, d: Declared<'a>) -> R<()> {
        self.at.defaults = self.defaults_at_node(d.block);
        let mut byval = Vec::new();
        for param in d.header.params() {
            let span = param.node().span();
            let name_tok = self.need(param.name(), span)?;
            let shown = show_bytes(self.text(name_tok.span));
            if param.array_parens().is_some() {
                // Measured: "Arrays cannot be passed to a library" (`x11`).
                return Err(self.error(span, format!("an array cannot be passed to a library: `{shown}()`")));
            }
            let fixed = param.as_clause().is_some_and(|a| self.fixed_string_size(a));
            if fixed || super::proc::fixed_name(self.text(name_tok.span)).is_some() {
                // Measured: "Cannot convert type ($4) to symbol" (`x26`).
                let msg = format!("a library parameter cannot be a `STRING * n`: `{shown}`");
                return Err(self.error(span, msg));
            }
            if param.as_clause().is_none() && self.split_name(name_tok)?.1.is_none() {
                let (name, _) = self.split_name(name_tok)?;
                if let Ty::FixedStr(_) = self.default_ty(&name) {
                    // A default type `STRING * n`: not measured.
                    let msg = format!("a library parameter whose default type is `STRING * n`: `{shown}`");
                    return Err(self.unsupported(span, msg));
                }
            }
            byval.push(param.byval().is_some());
        }
        if let Some(e) = self.prog.procs[d.id.0 as usize].external.as_mut() {
            e.byval.clone_from(&byval);
        }
        self.declare_params(d.header, d.id)?;
        let params = self.prog.proc(d.id).params.clone();
        for ((param, v), byval) in d.header.params().zip(params).zip(byval) {
            let ty = self.prog.var(v).ty;
            let span = param.node().span();
            if byval && ty == Ty::Str {
                // Measured: "STRINGs cannot be passed using BYVAL" (`x10`).
                return Err(self.error(span, "a `STRING` cannot be passed `BYVAL`"));
            }
            if let Ty::User(_) = ty
                && !d.types_before.contains(&self.prog.type_name(ty).to_ascii_uppercase())
            {
                // Measured: "Illegal SUB/FUNCTION parameter" (`x15`).
                let msg = format!(
                    "the `TYPE {}` of a library parameter must be defined above the block",
                    self.prog.type_name(ty)
                );
                return Err(self.error(span, msg));
            }
        }
        Ok(())
    }

    /// Whether a call of library procedure `p` is "Name already in use" for a `_BIT` parameter (measured,
    /// `verification\v37_x25`; a `_BIT` result is the same error, as for any FUNCTION).
    pub(super) fn bit_param(&self, p: ProcId) -> bool {
        let proc = self.prog.proc(p);
        proc.external.is_some()
            && proc
                .params
                .iter()
                .any(|&v| matches!(self.prog.var(v).ty, Ty::Bit { .. }))
    }

    /// Whether an argument to a `BYVAL` parameter is one the old compiler writes C++ `NULL`: its C++ text is `0`
    /// (`qb64pe.bas` 12307, 21954). Measured: `0`, `(0)`, `0&` and a `CONST` whose value is 0 are (`verification\v37_c`,
    /// `x56`, `x58`, `x59`, `x61`), `1 - 1` is not (`x57`). `0%` is taken with `0&` (not measured); a float `0.0`,
    /// `0#` and the constants of the auto-included file (`_FALSE`) are not taken (not measured).
    pub(super) fn null_arg(&self, node: ast::Expr) -> bool {
        match node {
            ast::Expr::Paren(p) => p.inner().is_some_and(|inner| self.null_arg(inner)),
            ast::Expr::Literal(l) => l
                .token()
                .is_some_and(|t| matches!(self.text(t.span), b"0" | b"0%" | b"0&")),
            ast::Expr::NameRef(n) => n.name().is_some_and(|t| {
                let raw = self.text(t.span);
                // A suffix names a variable, not the constant (`z&` beside `CONST z`).
                name_end(raw) == raw.len()
                    && self
                        .visible_const(&show_bytes(&raw.to_ascii_uppercase()))
                        // An OpenGL constant is written as an `_INTEGER64` literal, `0ll` (`_GL_FALSE`).
                        .is_some_and(|c| {
                            !self.gl_consts.contains(&c) && self.prog.consts[c.0 as usize].value == Value::Int(0)
                        })
            }),
            ast::Expr::Call(_) | ast::Expr::Field(_) | ast::Expr::Prefix(_) | ast::Expr::Bin(_) => false,
        }
    }
}
