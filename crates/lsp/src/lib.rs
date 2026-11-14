//! The language server, `qb64rust lsp` (OpenSpec change `m2-language-server`, spec `editor/language-server`): syntax
//! errors as the user types, the outline, folding ranges and go to definition for procedures and labels, from the
//! parser alone; the request `qb64rust/check`, which runs the whole front end (`extension-diagnostics`); and
//! formatting of a document or a range as `qb64rust -y` lays it out (`m5b-formatter`); hover, references, rename,
//! completion and the definition of any name from the semantic index, built once per text (`m5b-language-features`).
//! The front end after the parse, the formatter and the index are given by the driver as [`Hooks`], so this crate
//! depends on `syntax`, `base` and the built-in table only and never on `sema` (design D1 of `m2-language-server`,
//! D2 of `extension-diagnostics`, D5 of `m5b-formatter`, D1 of `m5b-language-features`).

pub mod analysis;
mod completion;
pub mod definition;
pub mod encoding;
mod encoding_tables;
pub mod folding;
mod hover;
pub mod index;
pub mod owner;
pub mod rename;
mod server;
pub mod symbols;
pub mod uri;
mod walk;

pub use server::{CHECK, DEBOUNCE, DOCUMENT_ENCODING, capabilities, format_settings, serve};

use lsp_server::Connection;
use qb64rust_base::layout::{Rewritable, Settings};
use qb64rust_base::{Diagnostics, FileId, SourceMap};
use qb64rust_syntax::ParsedProgram;
use std::path::{Path, PathBuf};

/// The front end after the parse, for a check: every diagnostic the CLI reports for the program parsed into the
/// map, the parse's included (`qb64rust_driver::check_for_server`). The path is the main file's on disk, for the
/// checks that read the disk; none for a document that is not a file. The last is the compiler root, under which
/// those checks look for a relative `$EXEICON` or `$EMBED` name too.
pub type Check = fn(&SourceMap, &ParsedProgram, Option<&Path>, &Path) -> Diagnostics;

/// The formatter, for a formatting request (OpenSpec change `m5b-formatter`, design D5): the file named by the
/// [`FileId`] (the document's, which may be an included file of the program: design D4 of `include-file-program`)
/// of the program parsed into the map laid out as `qb64rust -y` lays it out with the settings, or the diagnostics
/// of the front end when the parser reports an error (`qb64rust_driver::format_for_server`). The paths as for
/// [`Check`]. The `bool` is true for a code action's edit: the two rewrite settings given then win over the settings
/// file's (design D8 of `format-statement-lines`).
pub type Format =
    fn(&SourceMap, &ParsedProgram, Option<&Path>, &Path, &Settings, bool, FileId) -> Result<Formatted, Diagnostics>;

/// What the formatter gives a formatting request.
#[derive(Clone, Debug, Default)]
pub struct Formatted {
    /// For each line of the file, its output lines without their line ends: one, unless a rewrite setting cuts it
    /// (OpenSpec change `format-statement-lines`, design D8).
    pub lines: Vec<Vec<Vec<u8>>>,
    /// The settings the file was laid out with: the client's, under the `[format]` table of the settings file.
    pub settings: Settings,
    /// The diagnostics of the check. With an error the program is incomplete and the rewrites do nothing (design D7).
    pub diagnostics: Diagnostics,
}

/// What the rewrites can do to each line of a file of the program parsed into the map, from the parse alone (OpenSpec
/// change `format-statement-lines`, design D8; `qb64rust_format::rewritable`): for the code actions.
pub type Rewritables = fn(&SourceMap, &ParsedProgram, FileId) -> Vec<Rewritable>;

/// The semantic index, for hover, references, rename, completion and the wider definition (OpenSpec change
/// `m5b-language-features`, design D1): every symbol the front end resolved in the program parsed into the map, with
/// its hover text and scope, and the first error (`qb64rust_driver::index_for_server`). The paths as for [`Check`].
pub type Index = fn(&SourceMap, &ParsedProgram, Option<&Path>, &Path) -> index::SymbolIndex;

/// The main program the settings file names for a document on disk (OpenSpec change `include-file-program`, design
/// D7): the `[project]` table's `main` of the nearest `qb64rust.toml`, when that file exists
/// (`qb64rust_driver::fixes::main_of`).
pub type MainOf = fn(&Path) -> Option<PathBuf>;

/// The front end the driver hands the server: the check, the formatter, the index, the settings file's main
/// program and what the rewrites can do, so that this crate needs no `sema`.
#[derive(Clone, Copy)]
pub struct Hooks {
    pub check: Check,
    pub format: Format,
    pub index: Index,
    pub main_of: MainOf,
    pub rewritable: Rewritables,
}

/// Runs the server over stdin and stdout until `exit`. Call it on a thread with [`qb64rust_base::STACK_SIZE`] of
/// stack: the walks over the trees recurse.
pub fn run_stdio(default_root: PathBuf, hooks: Hooks) -> Result<(), String> {
    let (conn, io) = Connection::stdio();
    serve(conn, default_root, hooks)?;
    io.join().map_err(|e| e.to_string())
}
