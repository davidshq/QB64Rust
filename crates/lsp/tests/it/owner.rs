//! The lookup of a document's program on scratch folders (OpenSpec change `include-file-program`, design D7): the
//! disk search's order and its limits. The scenarios through the server are in `server.rs`.

use qb64rust_lsp::analysis::OpenFile;
use qb64rust_lsp::encoding::Encoding;
use qb64rust_lsp::owner::{Lookup, MAX_FILES_READ, Owner, Source, find_owner};
use qb64rust_lsp::uri::{from_path, path_key};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn write(dir: &Path, rel: &str, text: &str) -> PathBuf {
    let path = dir.join(rel);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, text).unwrap();
    path
}

/// The program of the file `doc` on disk, with no document open but `open` and no settings file's `main`
/// unless given.
fn owner_of(doc: &Path, open: &[&Path], settings_main: Option<PathBuf>) -> Option<Owner> {
    let open: HashMap<String, OpenFile> = open
        .iter()
        .map(|p| {
            let file = OpenFile {
                uri: from_path(p),
                bytes: std::fs::read(p).unwrap().into(),
                encoding: Encoding::DEFAULT,
            };
            (path_key(p), file)
        })
        .collect();
    let programs: Vec<String> = open.keys().filter(|k| k.ends_with(".bas")).cloned().collect();
    let (key, uri) = (path_key(doc), from_path(doc));
    find_owner(&Lookup {
        doc: &key,
        uri: &uri,
        open: &open,
        programs: &programs,
        settings_main,
        folders: &[],
        root: Path::new("no-root"),
        default: Encoding::DEFAULT,
    })
}

fn on_disk(p: &Path) -> Option<Owner> {
    Some(Owner {
        main: path_key(p),
        uri: from_path(p),
        source: Source::Disk,
    })
}

/// "A test file beside the program": a name that does not end in `_test.bas` first, whatever the order of names.
#[test]
fn program_before_its_test() {
    let dir = tempfile::tempdir().unwrap();
    let doc = write(dir.path(), "lib.bm", "SUB s\nEND SUB\n");
    write(dir.path(), "a_test.bas", "'$INCLUDE: 'lib.bm'\n");
    let game = write(dir.path(), "game.bas", "'$INCLUDE: 'lib.bm'\n");
    assert_eq!(owner_of(&doc, &[], None), on_disk(&game));
}

/// "The top of a chain on disk": a program another one names comes after it, and the include of an include is
/// followed.
#[test]
fn top_of_a_chain() {
    let dir = tempfile::tempdir().unwrap();
    let doc = write(dir.path(), "lib.bm", "SUB s\nEND SUB\n");
    write(dir.path(), "a_mid.bas", "'$INCLUDE: 'lib.bm'\n");
    let top = write(dir.path(), "z_top.bas", "'$INCLUDE: 'a_mid.bas'\n");
    assert_eq!(owner_of(&doc, &[], None), on_disk(&top));
}

/// A file that only names the document in a comment is read, but its program does not include it.
#[test]
fn a_name_in_a_comment_is_not_an_include() {
    let dir = tempfile::tempdir().unwrap();
    let doc = write(dir.path(), "lib.bm", "SUB s\nEND SUB\n");
    write(dir.path(), "notes.bas", "' see lib.bm\nPRINT 1\n");
    assert_eq!(owner_of(&doc, &[], None), None);
}

/// The settings file's `main` that names the document: it is its own program, even when an open program
/// includes it.
#[test]
fn settings_main_names_the_document() {
    let dir = tempfile::tempdir().unwrap();
    let game = write(dir.path(), "game.bas", "PRINT 1\n");
    let test = write(dir.path(), "game_test.bas", "'$INCLUDE: 'game.bas'\n");
    assert_eq!(owner_of(&game, &[&test], None).map(|o| o.source), Some(Source::Open));
    assert_eq!(owner_of(&game, &[&test], Some(game.clone())), None);
}

/// The search stops after [`MAX_FILES_READ`] files: a program after them by name is not found.
#[test]
fn search_stops_at_the_file_limit() {
    let dir = tempfile::tempdir().unwrap();
    let doc = write(dir.path(), "lib.bm", "SUB s\nEND SUB\n");
    let main = write(dir.path(), "main.bas", "'$INCLUDE: 'lib.bm'\n");
    for i in 0..MAX_FILES_READ - 1 {
        write(dir.path(), &format!("a{i:04}.bi"), "' filler\n");
    }
    assert_eq!(owner_of(&doc, &[], None), on_disk(&main), "the last file read");
    write(dir.path(), "a9999.bi", "' one more\n");
    assert_eq!(owner_of(&doc, &[], None), None, "past the limit");
}
