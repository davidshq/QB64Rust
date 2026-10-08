//! The language server, `qb64rust lsp` (OpenSpec change `m2-language-server`, spec `editor/language-server`): syntax
//! errors as the user types, the outline, folding ranges and go to definition for procedures and labels, from the
//! parser alone (no `sema`: this crate depends on `syntax` and `base` only, design D1).

pub mod analysis;
pub mod definition;
pub mod encoding;
mod encoding_tables;
pub mod folding;
mod server;
pub mod symbols;
pub mod uri;
mod walk;

pub use server::{DEBOUNCE, DOCUMENT_ENCODING, capabilities, serve};

use lsp_server::Connection;
use std::path::PathBuf;

/// Runs the server over stdin and stdout until `exit`. Call it on a thread with [`qb64rust_base::STACK_SIZE`] of
/// stack: the walks over the trees recurse.
pub fn run_stdio(default_root: PathBuf) -> Result<(), String> {
    let (conn, io) = Connection::stdio();
    serve(conn, default_root)?;
    io.join().map_err(|e| e.to_string())
}
