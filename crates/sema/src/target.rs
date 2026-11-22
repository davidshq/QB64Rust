//! What a build target does not have, rejected at compile time (spec `compiler/wasm-target`; design D5 of
//! `wasm-console`). A pass over the checked program, after `sema`: `sema` itself takes only a pointer's size from
//! the platform, and the rules of a target stand in one place, with one table that the language reference's
//! chapter on the target lists.
//!
//! For the wasm target it reports, each at its place, each use of a word of [`WASM_UNAVAILABLE`], but `DECLARE
//! LIBRARY` of a prebuilt library, which only the driver's lookup of the library's files can tell (it calls
//! [`unavailable`] for its message), and each built-in that needs a part of libqb of [`WASM_PARTS_NOT_YET`] (and
//! `ON STRIG`): "... yet". A window program is built for a browser page and a `$CONSOLE:ONLY` program for Node
//! (OpenSpec change `wasm-window`).
//!
//! `_OFFSET`, `_MEM` and a member array of `STRING` are built: a pointer's size is the platform's in every pass
//! ([`crate::Program::pointer_bytes`], OpenSpec change `wasm-pointer-width`).
//!
//! Nothing is reported in QB64pe's `afterlastline.bm`, which the program did not write: it is included when the
//! program names a network function, whose use is already reported.
//!
//! For the other platforms it reports nothing.

use crate::{
    Arg, ArrayRef, Event, Expr, ExprKind, Handler, InputSource, KeyForm, LibraryKind, MemValue, Place, PrintItem,
    PrintTo, Program, Reference, RunTarget, Stmt, StmtArg, StmtKind, UsingValue,
};
use qb64rust_base::{Diagnostics, FileId, Platform, SourceMap, Span};
use qb64rust_builtins::BuiltinId;

/// The words the wasm target does not have, each with when it is rejected (spec `compiler/wasm-target`, "What the
/// target does not have is rejected"). The language reference's chapter on the target lists this table.
pub const WASM_UNAVAILABLE: &[(&str, &str)] = &[
    (
        "SHELL",
        "as a statement and as a function, with or without `_HIDE` and `_DONTWAIT`",
    ),
    ("_SHELLHIDE", "the function"),
    (
        "RUN",
        "with a file name; `RUN` alone and `RUN` of a label or a line number are kept",
    ),
    ("CHAIN", "the statement"),
    ("_OPENHOST", "the function"),
    ("_OPENCLIENT", "the function"),
    ("_OPENCONNECTION", "the function"),
    ("DECLARE DYNAMIC LIBRARY", "the block"),
    (
        "DECLARE LIBRARY",
        "a block whose library is a prebuilt file (`.a`, `.o`, `.so`, `.lib`); a header alone is compiled with the program",
    ),
    (
        "$DEBUG",
        "the metacommand, whose code talks to the IDE through libqb's network part",
    ),
    ("_CLIPBOARD$", "the statement and the function: Node has no clipboard"),
    (
        "_WINDOWHANDLE",
        "in a window program, the function: a page has no window of the system (OpenSpec change `wasm-window`)",
    ),
    (
        "_SCREENCLICK",
        "in a window program, the statement: a page cannot click outside itself",
    ),
    (
        "_SCREENPRINT",
        "in a window program, the statement: a page cannot type into other programs",
    ),
    (
        "_MESSAGEBOX",
        "in a window program, as a statement and as a function: libqb's dialogs have no browser form yet",
    ),
    ("_INPUTBOX$", "in a window program, as `_MESSAGEBOX`"),
    ("_OPENFILEDIALOG$", "in a window program, as `_MESSAGEBOX`"),
    ("_SAVEFILEDIALOG$", "in a window program, as `_MESSAGEBOX`"),
    ("_SELECTFOLDERDIALOG$", "in a window program, as `_MESSAGEBOX`"),
    ("_COLORCHOOSERDIALOG", "in a window program, as `_MESSAGEBOX`"),
    ("_NOTIFYPOPUP", "in a window program, as `_MESSAGEBOX`"),
    (
        "SUB _GL",
        "in a window program, the procedure: a page draws through WebGL, which has no OpenGL 1 for a program",
    ),
    ("_GLRENDER", "in a window program, as `SUB _GL`"),
];

/// The parts of libqb the wasm target does not build yet, by the `dependency` of the built-in table, with what
/// they hold (spec `compiler/wasm-target`, "Parts of libqb the target does not build yet"). Each built-in that
/// needs one is "not available on the wasm target yet". The language reference's chapter on the target lists
/// this table. The network part's words are in [`WASM_UNAVAILABLE`].
pub const WASM_PARTS_NOT_YET: &[(&str, &str, &str)] = &[
    (
        "DEPENDENCY_MINIAUDIO",
        "audio",
        "`SOUND`, `PLAY`, `_WAVE`, `_MEMSOUND`, `_MIDISOUNDBANK` and the `_SND` words, in a `$CONSOLE:ONLY` program: \
         Node has no Web Audio (a window program plays them in its page)",
    ),
    (
        "DEPENDENCY_DEVICEINPUT",
        "device-input",
        "`STICK`, `STRIG`, `ON STRIG`, `_DEVICES` and the other words of game controllers and devices",
    ),
    (
        "DEPENDENCY_SCREENIMAGE",
        "screen-image",
        "`_SCREENIMAGE`, `_CLIPBOARDIMAGE`",
    ),
];

/// The error of a word whose part of libqb the wasm target does not build yet.
fn part_not_yet(word: &str, part: &str) -> String {
    format!("{}: libqb's {part} part is not built for it", not_yet(word))
}

/// The built-ins of [`WASM_UNAVAILABLE`], which are found by the built-in a statement or call resolved to. The
/// table names a built-in without its `$`.
const UNAVAILABLE_BUILTINS: [&str; 7] = [
    "SHELL",
    "_SHELLHIDE",
    "CHAIN",
    "_OPENHOST",
    "_OPENCLIENT",
    "_OPENCONNECTION",
    "_CLIPBOARD$",
];

/// The built-ins of [`WASM_UNAVAILABLE`] that a window program cannot have, since a page has no form of them
/// (OpenSpec change `wasm-window`). A `$CONSOLE:ONLY` program keeps them, as the console target always had: it
/// never shows a window, so `SUB _GL` never runs, and the dialogs fall back to the console.
const PAGE_UNAVAILABLE_BUILTINS: [&str; 11] = [
    "_WINDOWHANDLE",
    "_SCREENCLICK",
    "_SCREENPRINT",
    "_MESSAGEBOX",
    "_INPUTBOX$",
    "_OPENFILEDIALOG$",
    "_SAVEFILEDIALOG$",
    "_SELECTFOLDERDIALOG$",
    "_COLORCHOOSERDIALOG",
    "_NOTIFYPOPUP",
    "_GLRENDER",
];

/// The error for a word of [`WASM_UNAVAILABLE`].
pub fn unavailable(word: &str) -> String {
    format!("`{word}` is not available on the wasm target")
}

/// The start of the error for a word the wasm target does not build yet ([`part_not_yet`]).
pub fn not_yet(word: &str) -> String {
    format!("`{word}` is not available on the wasm target yet")
}

/// Reports what the program uses that `platform` does not have.
pub fn check_target(program: &Program, platform: Platform, map: &SourceMap, diagnostics: &mut Diagnostics) {
    match platform {
        Platform::Windows | Platform::Linux => {}
        Platform::Wasm => {
            let after = map
                .files()
                .find(|(_, f)| f.name == qb64rust_builtins::AFTER_LAST_LINE_NAME)
                .map(|(id, _)| id);
            // `SUB _GL` of a window program, at its header's line.
            let gl = program.procs.iter().filter(|p| p.name == "_GL" && p.external.is_none());
            for proc in gl.filter(|_| program.settings.window) {
                if let Some((start, end)) = map.file(proc.file).line_content(proc.line.saturating_sub(1)) {
                    diagnostics.error(Span::new(proc.file, start, end), unavailable("SUB _GL"));
                }
            }
            Wasm::new(program, diagnostics, after).program();
        }
    }
}

/// The walk of a program for the wasm target.
struct Wasm<'p, 'd> {
    program: &'p Program,
    diagnostics: &'d mut Diagnostics,
    /// QB64pe's `afterlastline.bm`, when the program includes it: nothing in it is reported.
    after: Option<FileId>,
}

impl<'p, 'd> Wasm<'p, 'd> {
    fn new(program: &'p Program, diagnostics: &'d mut Diagnostics, after: Option<FileId>) -> Self {
        Wasm {
            program,
            diagnostics,
            after,
        }
    }

    fn program(&mut self) {
        let p = self.program;
        if let Some(span) = p.settings.debug {
            self.diagnostics.error(span, unavailable("$DEBUG"));
        }
        for lib in &p.libraries {
            if lib.kind == LibraryKind::Dynamic {
                self.diagnostics.error(lib.span, unavailable("DECLARE DYNAMIC LIBRARY"));
            }
        }
        self.stmts(&p.stmts);
        let after = self.after;
        for proc in p.procs.iter().filter(|proc| Some(proc.file) != after) {
            self.stmts(&proc.stmts);
        }
    }

    /// A built-in that a statement or a call resolved to, at `span`. Says whether an error was reported.
    fn builtin(&mut self, id: BuiltinId, span: Span) -> bool {
        let b = id.get();
        let name = b.name.to_ascii_uppercase();
        let page = if self.program.settings.window {
            &PAGE_UNAVAILABLE_BUILTINS[..]
        } else {
            &[]
        };
        if let Some(word) = UNAVAILABLE_BUILTINS
            .iter()
            .chain(page)
            .find(|w| w.trim_end_matches('$') == name)
        {
            self.diagnostics.error(span, unavailable(word));
            return true;
        }
        // A window program's page plays sound through Web Audio (OpenSpec change `wasm-window`).
        let in_page = |d: &str| d == "DEPENDENCY_MINIAUDIO" && self.program.settings.window;
        if let Some(dependency) = b.dependency
            && !in_page(dependency)
            && let Some((_, part, _)) = WASM_PARTS_NOT_YET.iter().find(|(d, ..)| *d == dependency)
        {
            self.diagnostics.error(span, part_not_yet(&name, part));
            return true;
        }
        false
    }

    fn stmts(&mut self, list: &[Stmt]) {
        for s in list {
            self.stmt(s);
        }
    }

    fn stmt(&mut self, s: &Stmt) {
        match &s.kind {
            StmtKind::Builtin { id, args } => {
                let _ = self.builtin(*id, s.span);
                for a in args {
                    self.stmt_arg(a);
                }
            }
            StmtKind::MemGet { block, offset, target } => {
                self.place(block);
                self.expr(offset);
                self.reference(target);
            }
            StmtKind::MemPut { block, offset, value } => {
                self.place(block);
                self.expr(offset);
                self.mem_value(value);
            }
            StmtKind::MemFill {
                block,
                offset,
                bytes,
                value,
            } => {
                self.place(block);
                self.expr(offset);
                self.expr(bytes);
                self.mem_value(value);
            }
            StmtKind::Run(target) => match target {
                RunTarget::File(e) => {
                    self.diagnostics.error(s.span, unavailable("RUN"));
                    self.expr(e);
                }
                RunTarget::Restart | RunTarget::Label(_) => {}
            },
            StmtKind::Assign { place, value } => {
                self.place(place);
                self.expr(value);
            }
            StmtKind::CopyWhole { from, to } => {
                self.place(from);
                self.place(to);
            }
            StmtKind::Allocate { array, bounds, .. } => {
                self.array(array);
                for (lower, upper) in bounds {
                    self.expr(lower);
                    self.expr(upper);
                }
            }
            StmtKind::Clear { array, .. } | StmtKind::Release { array, .. } => self.array(array),
            StmtKind::CopyArray { from, to } => {
                self.array(from);
                self.array(to);
            }
            StmtKind::ArrayCopy {
                from,
                to,
                ranges,
                starts,
            } => {
                self.array(from);
                self.array(to);
                for (a, b) in ranges {
                    self.expr(a);
                    self.expr(b);
                }
                self.exprs(starts);
            }
            StmtKind::Print { to, items, .. } => {
                self.print_to(to);
                self.print_items(items);
            }
            StmtKind::PrintUsing {
                to,
                before,
                format,
                values,
                ..
            } => {
                self.print_to(to);
                self.print_items(before);
                self.expr(format);
                for v in values {
                    match v {
                        UsingValue::Str(e) | UsingValue::Num { value: e, .. } | UsingValue::Blanks(e) => self.expr(e),
                    }
                }
            }
            StmtKind::Field { file, parts } => {
                self.expr(file);
                for (place, width) in parts {
                    self.place(place);
                    self.expr(width);
                }
            }
            StmtKind::Lset { target, value, .. } => {
                self.place(target);
                self.expr(value);
            }
            StmtKind::Write { to, items, .. } => {
                if let Some(e) = to {
                    self.expr(e);
                }
                self.print_items(items);
            }
            StmtKind::Input { from, targets, .. } => {
                match from {
                    InputSource::File(e) => self.expr(e),
                    InputSource::Console { .. } => {}
                }
                for t in targets {
                    self.place(t);
                }
            }
            StmtKind::Read(places) => {
                for p in places {
                    self.place(p);
                }
            }
            StmtKind::End(e) | StmtKind::System(e) => {
                if let Some(e) = e {
                    self.expr(e);
                }
            }
            StmtKind::Call { args, .. } => self.args(args),
            StmtKind::CallAbsolute { args, offset } => {
                for a in args {
                    match a {
                        crate::AbsArg::Address(e) | crate::AbsArg::Value(e, _) => self.expr(e),
                    }
                }
                self.expr(offset);
            }
            StmtKind::Log { msg, .. } => self.expr(msg),
            StmtKind::Error(e) => self.expr(e),
            StmtKind::PaletteUsing { index, .. } => self.exprs(index),
            StmtKind::If { branches, else_, .. } => {
                for b in branches {
                    self.stmts(&b.allocs);
                    self.expr(&b.cond);
                    self.stmts(&b.body);
                }
                if let Some(e) = else_ {
                    self.stmts(e);
                }
            }
            StmtKind::For {
                var,
                start,
                end,
                step,
                body,
                ..
            } => {
                self.place(var);
                self.expr(start);
                self.expr(end);
                if let Some(e) = step {
                    self.expr(e);
                }
                self.stmts(body);
            }
            StmtKind::Do { test, body, .. } => {
                if let Some(t) = test {
                    self.stmts(&t.allocs);
                    self.expr(&t.cond);
                }
                self.stmts(body);
            }
            StmtKind::While { cond, body, .. } => {
                self.expr(cond);
                self.stmts(body);
            }
            StmtKind::OnJump { value, .. } => self.expr(value),
            StmtKind::OnEvent { event, handler } => {
                match event {
                    Event::Timer { number, seconds } => {
                        if let Some(n) = number {
                            self.expr(n);
                        }
                        self.expr(seconds);
                    }
                    Event::Key { key } => self.expr(key),
                    Event::Strig { button, controller } => {
                        self.diagnostics.error(s.span, part_not_yet("ON STRIG", "device-input"));
                        self.expr(button);
                        if let Some(c) = controller {
                            self.expr(c);
                        }
                    }
                }
                match handler {
                    Handler::Sub { arg: Some(e), .. } => self.expr(e),
                    Handler::Sub { arg: None, .. } | Handler::Label(_) => {}
                }
            }
            StmtKind::Key(form) => match form {
                KeyForm::Assign { number, text } => {
                    self.expr(number);
                    self.expr(text);
                }
                KeyForm::On | KeyForm::Off | KeyForm::List => {}
            },
            StmtKind::Select {
                selector, cases, else_, ..
            } => {
                self.expr(selector);
                for c in cases {
                    self.stmts(&c.allocs);
                    for item in &c.items {
                        match item {
                            crate::CaseItem::Is(_, e) => self.expr(e),
                            crate::CaseItem::Range(a, b) => {
                                self.expr(a);
                                self.expr(b);
                            }
                        }
                    }
                    self.stmts(&c.body);
                }
                if let Some(e) = else_ {
                    self.stmts(e);
                }
            }
            // No expression inside.
            StmtKind::ConsoleOnly
            | StmtKind::EraseParam { .. }
            | StmtKind::Restore(_)
            | StmtKind::Stop
            | StmtKind::Exit
            | StmtKind::OnError(_)
            | StmtKind::Resume(_)
            | StmtKind::Label(_)
            | StmtKind::Goto(_)
            | StmtKind::Gosub(_)
            | StmtKind::Return(_)
            | StmtKind::ExitLoop(_)
            | StmtKind::Continue
            | StmtKind::ExitSelect { .. } => {}
        }
    }

    fn print_to(&mut self, to: &PrintTo) {
        match to {
            PrintTo::File(e) => self.expr(e),
            PrintTo::Screen | PrintTo::Printer => {}
        }
    }

    fn print_items(&mut self, items: &[PrintItem]) {
        for item in items {
            match item {
                PrintItem::Str(e) | PrintItem::Num(e) => self.expr(e),
                PrintItem::Zone => {}
            }
        }
    }

    fn mem_value(&mut self, v: &MemValue) {
        match v {
            MemValue::Typed { value, .. } | MemValue::Str(value) => self.expr(value),
            MemValue::Bytes(r) => self.reference(r),
        }
    }

    fn stmt_arg(&mut self, a: &StmtArg) {
        match a {
            StmtArg::Value(e) => self.expr(e),
            StmtArg::Place(p) | StmtArg::Rest(p) | StmtArg::Bytes(p) => self.place(p),
            StmtArg::Block { index, .. } => self.exprs(index),
            StmtArg::Word(_) | StmtArg::Absent => {}
        }
    }

    fn args(&mut self, args: &[Arg]) {
        for a in args {
            match a {
                Arg::Ref(p) => self.place(p),
                Arg::Temp(e, _) | Arg::Value(e) => self.expr(e),
                Arg::Array(a) => self.array(a),
                Arg::Null => {}
            }
        }
    }

    fn exprs(&mut self, list: &[Expr]) {
        for e in list {
            self.expr(e);
        }
    }

    fn reference(&mut self, r: &Reference) {
        match r {
            Reference::Place(p) => self.place(p),
            Reference::Array(a) => self.array(a),
        }
    }

    fn array(&mut self, a: &ArrayRef) {
        match a {
            ArrayRef::Var(_) => {}
            ArrayRef::Member { base, .. } => self.place(base),
        }
    }

    fn place(&mut self, p: &Place) {
        for i in p.indexes() {
            self.expr(i);
        }
    }

    fn expr(&mut self, e: &Expr) {
        match &e.kind {
            ExprKind::Int(_) | ExprKind::Float(_) | ExprKind::Str(_) | ExprKind::ArrayBytes(_) => {}
            ExprKind::Load(place) => self.place(place),
            ExprKind::Bound { array, dim, .. } => {
                self.array(array);
                if let Some(d) = dim {
                    self.expr(d);
                }
            }
            ExprKind::Address { of, .. } => self.place(of),
            ExprKind::Block { index, .. } => self.exprs(index),
            ExprKind::Convert { from, .. } | ExprKind::Unary { operand: from, .. } => self.expr(from),
            ExprKind::Binary { lhs, rhs, .. } | ExprKind::Concat(lhs, rhs) | ExprKind::StrCompare { lhs, rhs, .. } => {
                self.expr(lhs);
                self.expr(rhs);
            }
            ExprKind::Choice { condition, then, else_ } => {
                self.expr(condition);
                self.expr(then);
                self.expr(else_);
            }
            ExprKind::Call { builtin, args } => {
                let _ = self.builtin(*builtin, e.span);
                for a in args.iter().flatten() {
                    self.expr(a);
                }
            }
            ExprKind::CallProc { args, .. } => self.args(args),
            ExprKind::MemOf { of, .. } | ExprKind::AddressOf(of) => self.reference(of),
            ExprKind::MemAt { offset, size } => {
                self.expr(offset);
                self.expr(size);
            }
            ExprKind::MemRead { block, offset, .. } => {
                self.place(block);
                self.expr(offset);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_table_holds_the_builtins_found_by_name() {
        for name in UNAVAILABLE_BUILTINS.iter().chain(&PAGE_UNAVAILABLE_BUILTINS) {
            assert!(WASM_UNAVAILABLE.iter().any(|(w, _)| w == name), "{name}");
            assert!(
                qb64rust_builtins::BUILTINS
                    .iter()
                    .any(|b| b.name.eq_ignore_ascii_case(name.trim_end_matches('$'))),
                "{name} is a built-in"
            );
        }
    }

    #[test]
    fn each_part_is_named_by_builtins() {
        for (dependency, ..) in WASM_PARTS_NOT_YET {
            assert!(
                qb64rust_builtins::BUILTINS
                    .iter()
                    .any(|b| b.dependency == Some(*dependency)),
                "{dependency}"
            );
        }
    }

    #[test]
    fn messages() {
        assert_eq!(unavailable("SHELL"), "`SHELL` is not available on the wasm target");
        assert_eq!(
            part_not_yet("SOUND", "audio"),
            "`SOUND` is not available on the wasm target yet: libqb's audio part is not built for it"
        );
    }
}
