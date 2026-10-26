//! The language server, `qb64rust lsp` (OpenSpec change `m2-language-server`, spec `editor/language-server`): syntax
//! errors as the user types, the outline, folding ranges and go to definition for procedures and labels, from the
//! parser alone; the request `qb64rust/check`, which runs the whole front end (`extension-diagnostics`); and
//! formatting of a document or a range as `qb64rust -y` lays it out (`m5b-formatter`). The front end after the parse
//! and the formatter are given by the driver as [`Hooks`], so this crate depends on `syntax` and `base` only and
//! never on `sema` (design D1 of `m2-language-server`, D2 of `extension-diagnostics`, D5 of `m5b-formatter`).

pub mod analysis;
pub mod definition;
pub mod encoding;
mod encoding_tables;
pub mod folding;
mod server;
pub mod symbols;
pub mod uri;
mod walk;

pub use server::{CHECK, DEBOUNCE, DOCUMENT_ENCODING, capabilities, format_settings, serve};

use lsp_server::Connection;
use qb64rust_base::layout::Settings;
use qb64rust_base::{Diagnostics, SourceMap};
use qb64rust_syntax::ParsedProgram;
use std::path::{Path, PathBuf};

/// The front end after the parse, for a check: every diagnostic the CLI reports for the program parsed into the
/// map, the parse's included (`qb64rust_driver::check_for_server`). The path is the main file's on disk, for the
/// checks that read the disk; none for a document that is not a file. The last is the compiler root, under which
/// those checks look for a relative `$EXEICON` or `$EMBED` name too.
pub type Check = fn(&SourceMap, &ParsedProgram, Option<&Path>, &Path) -> Diagnostics;

/// The formatter, for a formatting request (OpenSpec change `m5b-formatter`, design D5): the main file of the program
/// parsed into the map laid out as `qb64rust -y` lays it out with the settings, every line followed by CR LF, or the
/// diagnostics of the front end when it reports an error (`qb64rust_driver::format_for_server`). The paths as for
/// [`Check`].
pub type Format = fn(&SourceMap, &ParsedProgram, Option<&Path>, &Path, &Settings) -> Result<Vec<u8>, Diagnostics>;

/// The front end the driver hands the server: the check and the formatter, so that this crate needs no `sema`.
#[derive(Clone, Copy)]
pub struct Hooks {
    pub check: Check,
    pub format: Format,
}

/// Runs the server over stdin and stdout until `exit`. Call it on a thread with [`qb64rust_base::STACK_SIZE`] of
/// stack: the walks over the trees recurse.
pub fn run_stdio(default_root: PathBuf, hooks: Hooks) -> Result<(), String> {
    let (conn, io) = Connection::stdio();
    serve(conn, default_root, hooks)?;
    io.join().map_err(|e| e.to_string())
}
