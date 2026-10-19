//! The language server, `qb64rust lsp` (OpenSpec change `m2-language-server`, spec `editor/language-server`): syntax
//! errors as the user types, the outline, folding ranges and go to definition for procedures and labels, from the
//! parser alone; and the request `qb64rust/check`, which runs the whole front end (`extension-diagnostics`). The
//! front end after the parse is given by the driver as a [`Check`], so this crate depends on `syntax` and `base`
//! only and never on `sema` (design D1 of `m2-language-server`, D2 of `extension-diagnostics`).

pub mod analysis;
pub mod definition;
pub mod encoding;
mod encoding_tables;
pub mod folding;
mod server;
pub mod symbols;
pub mod uri;
mod walk;

pub use server::{CHECK, DEBOUNCE, DOCUMENT_ENCODING, capabilities, serve};

use lsp_server::Connection;
use qb64rust_base::{Diagnostics, SourceMap};
use qb64rust_syntax::ParsedProgram;
use std::path::{Path, PathBuf};

/// The front end after the parse, for a check: every diagnostic the CLI reports for the program parsed into the
/// map, the parse's included (`qb64rust_driver::check_for_server`). The path is the main file's on disk, for the
/// checks that read the disk; none for a document that is not a file.
pub type Check = fn(&SourceMap, &ParsedProgram, Option<&Path>) -> Diagnostics;

/// Runs the server over stdin and stdout until `exit`. Call it on a thread with [`qb64rust_base::STACK_SIZE`] of
/// stack: the walks over the trees recurse.
pub fn run_stdio(default_root: PathBuf, check: Check) -> Result<(), String> {
    let (conn, io) = Connection::stdio();
    serve(conn, default_root, check)?;
    io.join().map_err(|e| e.to_string())
}
