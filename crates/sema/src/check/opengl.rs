//! The OpenGL names of a program with `SUB _GL` (design D3 of `m4-sub-gl`; measured, `verification\v59_*`).
//!
//! The old compiler registers the constants and routines it reads from `gl.h` after its pre-pass, when the program
//! has `SUB _GL` (`qb64pe.bas` 3331, `gl_include_content`). Here they come from the table
//! ([`crate::opengl`]) the first time a program names one: a routine becomes a procedure of a library of its own
//! kind ([`LibraryKind::OpenGl`]), each parameter `BYVAL` of the table's type, as the old compiler's `id.ccall`
//! registration has it; a constant becomes an `_INTEGER64` constant of the main module, as an auto-included one
//! does (`auto.rs`). A program that names none is checked as before. Without `SUB _GL` the names are what any
//! name with a leading `_` is (measured, `v59_b1`–`b4`: "Syntax error", "Invalid variable name").

use super::{Checker, R};
use crate::consteval::Value;
use crate::opengl::{gl_constant, gl_routine};
use crate::{Const, ConstId, External, Library, LibraryId, LibraryKind, ProcId, ProcKind, Storage, Ty};
use qb64rust_base::{show_bytes, to_u32};
use qb64rust_syntax::tree::Tok;

impl Checker<'_> {
    /// The program has `SUB _GL` (pass 1 found it).
    pub(super) fn has_gl(&self) -> bool {
        self.settings.runtime_parts.contains(&crate::RuntimePart::Gl)
    }

    /// Enters the OpenGL routine `name` (upper case, without suffix) as a procedure, named at `t` in an expression
    /// (`args` when arguments follow), if the program has `SUB _GL`, the name is a routine of the table and it is
    /// not entered yet. A routine written with a type suffix is no name of the table: "Invalid variable name"
    /// alone, "Invalid array name" with arguments (measured, `verification\v59_x25`–`x27`), worded apart for the
    /// note after the error.
    pub(super) fn gl_routine_named(&mut self, name: &str, suffix: Option<Ty>, args: bool, t: Tok) -> R<()> {
        if !self.has_gl() {
            return Ok(());
        }
        let Some(r) = gl_routine(name) else {
            return Ok(());
        };
        if suffix.is_some() {
            let shown = show_bytes(self.text(t.span));
            let how = if args { "called" } else { "named" };
            return Err(self.error(
                t.span,
                format!("`{shown}`: an OpenGL routine is {how} without a type suffix"),
            ));
        }
        if self.procs_by_name.contains_key(name) {
            return Ok(());
        }
        let library = match self.gl_library {
            Some(id) => id,
            None => {
                let id = LibraryId(to_u32(self.prog.libraries.len()));
                self.prog.libraries.push(Library {
                    kind: LibraryKind::OpenGl,
                    names: Vec::new(),
                    span: t.span,
                    at: crate::Track {
                        file: t.span.file,
                        line: self.line(t.span),
                        include_line: None,
                    },
                });
                self.gl_library = Some(id);
                id
            }
        };
        let id = self.new_proc(name.to_string(), r.ret.map_or(ProcKind::Sub, ProcKind::Function));
        let line = self.line(t.span);
        let proc = &mut self.prog.procs[id.0 as usize];
        proc.line = line;
        proc.end_line = line;
        proc.external = Some(External {
            library,
            c_name: r.c_name.to_string(),
            byval: vec![true; r.args.len()],
            at: crate::Track {
                file: t.span.file,
                line,
                include_line: None,
            },
        });
        // The parameters are named as the helper's are (`gl_helper_code.h`: `a`, `b`, ...); no routine has more
        // than 26 (the table's test).
        for (k, &ty) in r.args.iter().enumerate() {
            let letter = (b'A'..=b'Z').nth(k).expect("at most 26 arguments");
            let v = self.new_var(char::from(letter).to_string(), ty, Storage::Param(id));
            self.prog.procs[id.0 as usize].params.push(v);
        }
        Ok(())
    }

    /// Enters the OpenGL constant `name` (upper case, without suffix) as a constant of the main module, if the
    /// program has `SUB _GL`, the name is a constant of the table and it is not entered yet.
    pub(super) fn gl_constant_named(&mut self, name: &str) {
        if !self.has_gl() || self.consts_main.contains_key(name) {
            return;
        }
        let Some(c) = gl_constant(name) else {
            return;
        };
        let id = ConstId(to_u32(self.prog.consts.len()));
        self.prog.consts.push(Const {
            name: name.to_string(),
            ty: Ty::I64,
            value: Value::Int(c.value),
            proc: None,
        });
        self.consts_main.insert(name.to_string(), id);
        self.gl_consts.insert(id);
    }

    /// Whether procedure `p` is an OpenGL routine.
    pub(super) fn is_gl_proc(&self, p: ProcId) -> bool {
        self.prog
            .proc(p)
            .external
            .as_ref()
            .is_some_and(|e| self.prog.libraries[e.library.0 as usize].kind == LibraryKind::OpenGl)
    }

    /// Whether `name` (upper case, without suffix) starts as the OpenGL names do (`_GL…`) and is no built-in
    /// (`_GLRENDER`): the table holds every such name the old compiler knows, so any other is no name of QB64pe
    /// (measured, `verification\v59_b1`, `x33`, `x47`: "Syntax error"), with `SUB _GL` or without.
    pub(super) fn is_gl_like(&self, name: &str) -> bool {
        name.starts_with("_GL") && qb64rust_builtins::find_any(name.as_bytes()).next().is_none()
    }

    /// Whether `name` is an OpenGL constant of the program: the old compiler registers them after the pre-pass in
    /// which it reads every `CONST` line, so a `CONST` cannot name one (measured, `verification\v59_x43`, `x44`:
    /// "Unexpected element").
    pub(super) fn is_gl_constant(&self, name: &str) -> bool {
        self.has_gl() && gl_constant(name).is_some()
    }
}
