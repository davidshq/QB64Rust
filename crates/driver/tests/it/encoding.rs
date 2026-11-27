//! UTF-8 source files (OpenSpec change `utf8-source`; spec `compiler/source-encoding`): which files a settings
//! file's `encoding` key reaches, its bad value, the byte order mark, and the twin test of task 2.6, which compares
//! the C++ of one program saved as code page 437 and as UTF-8. Each run stops at a dump or an error, so none builds.

use crate::support::{self, COMPILER_LIMIT, run};
use std::path::Path;
use std::process::{Command, Output};

/// The box-drawing character `─` in UTF-8, and its code page 437 byte as the IR dump writes it.
const LINE_UTF8: &str = "\u{2500}";

fn qb64rust(dir: &Path, args: &[&str]) -> Output {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_qb64rust"));
    cmd.env(support::OBJECTS_LOCK_ENV, support::objects_lock());
    run(cmd.current_dir(dir).args(args), None, COMPILER_LIMIT, "compiler")
}

#[expect(clippy::disallowed_methods, reason = "the compiler's own messages")]
fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).to_string()
}

fn write(path: &Path, bytes: &[u8]) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, bytes).unwrap();
}

/// The IR dump of `main` run in `dir`, which must succeed.
fn ir(dir: &Path, args: &[&str]) -> String {
    let mut all = vec!["--dump", "ir"];
    all.extend_from_slice(args);
    let o = qb64rust(dir, &all);
    let out = stdout(&o);
    assert_eq!(o.status.code(), Some(0), "{out}");
    out
}

/// "The project's key": the main file and a file it includes from a folder below are both UTF-8. "A file's line
/// wins": a file of the project with `'$ENCODING:CP437` keeps its byte. "Nothing declared": without the key the
/// bytes stay as written.
#[test]
fn the_projects_key_and_a_files_line() {
    let d = tempfile::tempdir().unwrap();
    let main = format!("$CONSOLE:ONLY\n'$INCLUDE:'ui/menu.bm'\n'$INCLUDE:'old.bi'\nPRINT \"{LINE_UTF8}\"\n");
    write(&d.path().join("main.bas"), main.as_bytes());
    write(
        &d.path().join("ui/menu.bm"),
        format!("PRINT \"{LINE_UTF8}{LINE_UTF8}\"\n").as_bytes(),
    );
    write(&d.path().join("old.bi"), b"'$ENCODING:CP437\nPRINT \"\xC4\xC4\xC4\"\n");
    let nothing = ir(d.path(), &["main.bas"]);
    assert!(nothing.contains(r#"Str Const "\xE2\x94\x80""#), "{nothing}");
    assert!(nothing.contains(r#"Str Const "\xE2\x94\x80\xE2\x94\x80""#), "{nothing}");
    write(&d.path().join("qb64rust.toml"), b"[project]\nencoding = \"utf-8\"\n");
    let declared = ir(d.path(), &["main.bas"]);
    for text in [
        r#"Str Const "\xC4""#,
        r#"Str Const "\xC4\xC4""#,
        r#"Str Const "\xC4\xC4\xC4""#,
    ] {
        assert!(declared.contains(text), "{text}: {declared}");
    }
    assert!(!declared.contains(r"\xE2"), "{declared}");
}

/// "A file outside the project's folder": a library file found under the compiler root is code page 437 unless it
/// says otherwise, though the project's key is `"utf-8"`.
#[test]
fn a_file_outside_the_projects_folder() {
    let d = tempfile::tempdir().unwrap();
    let project = d.path().join("game");
    write(&project.join("qb64rust.toml"), b"[project]\nencoding = \"utf-8\"\n");
    write(&project.join("main.bas"), b"$CONSOLE:ONLY\n'$INCLUDE:'lib.bm'\n");
    write(
        &d.path().join("root/lib.bm"),
        format!("PRINT \"{LINE_UTF8}\"\n").as_bytes(),
    );
    let root = d.path().join("root");
    let out = ir(&project, &["--include-root", root.to_str().unwrap(), "main.bas"]);
    assert!(out.contains(r#"Str Const "\xE2\x94\x80""#), "{out}");
}

/// "A code page 437 file in a UTF-8 project": an error at the line of the byte that is not UTF-8, which names the
/// byte and `qb64rust.toml`.
#[test]
fn a_code_page_437_file_in_a_utf8_project() {
    let d = tempfile::tempdir().unwrap();
    write(&d.path().join("qb64rust.toml"), b"[project]\nencoding = \"utf-8\"\n");
    write(&d.path().join("main.bas"), b"$CONSOLE:ONLY\n'$INCLUDE:'map.bm'\n");
    let mut map = b"' map\n".repeat(11);
    map.extend_from_slice(b"PRINT \"\xC4\"\n");
    write(&d.path().join("map.bm"), &map);
    let o = qb64rust(d.path(), &["--dump", "ir", "main.bas"]);
    let out = stdout(&o);
    assert_eq!(o.status.code(), Some(1), "{out}");
    assert!(
        out.contains("map.bm:12:8: error: the byte 0xC4 is not UTF-8") && out.contains("`qb64rust.toml`"),
        "{out}"
    );
}

/// "A bad value": the compile fails at the key's line, naming both values.
#[test]
fn a_bad_encoding_value() {
    let d = tempfile::tempdir().unwrap();
    write(&d.path().join("qb64rust.toml"), b"[project]\nencoding = \"latin1\"\n");
    write(&d.path().join("p.bas"), b"PRINT 1\n");
    let o = qb64rust(d.path(), &["--dump", "ir", "p.bas"]);
    let out = stdout(&o);
    assert_eq!(o.status.code(), Some(1), "{out}");
    assert!(
        out.contains("qb64rust.toml:2: the value of `encoding` must be \"utf-8\" or \"cp437\""),
        "{out}"
    );
}

/// "A file with the mark": a UTF-8 file that starts with 0xEF 0xBB 0xBF compiles as the file without them, by
/// its line or by the project's key. In a file that is not UTF-8 the mark is an error at line 1, as in QB64pe
/// (`verification\v74_d1_bom`).
#[test]
fn a_byte_order_mark() {
    let d = tempfile::tempdir().unwrap();
    let body = format!("'$ENCODING:UTF8\n$CONSOLE:ONLY\nPRINT \"{LINE_UTF8}\"\n");
    write(&d.path().join("plain/p.bas"), body.as_bytes());
    write(
        &d.path().join("mark/p.bas"),
        &[b"\xEF\xBB\xBF", body.as_bytes()].concat(),
    );
    let cpp = |dir: &str| {
        let o = qb64rust(&d.path().join(dir), &["--dump", "cpp", "p.bas"]);
        assert_eq!(o.status.code(), Some(0), "{dir}: {}", stdout(&o));
        o.stdout
    };
    assert_eq!(cpp("mark"), cpp("plain"));
    write(
        &d.path().join("key/qb64rust.toml"),
        b"[project]\nencoding = \"utf-8\"\n",
    );
    write(&d.path().join("key/p.bas"), b"\xEF\xBB\xBF$CONSOLE:ONLY\nPRINT 1\n");
    ir(&d.path().join("key"), &["p.bas"]);
    write(&d.path().join("none/p.bas"), b"\xEF\xBB\xBF$CONSOLE:ONLY\nPRINT 1\n");
    let o = qb64rust(&d.path().join("none"), &["--dump", "ir", "p.bas"]);
    let out = stdout(&o);
    assert_eq!(o.status.code(), Some(1), "{out}");
    assert!(
        out.starts_with("p.bas:1:1: error: the file starts with a byte order mark"),
        "{out}"
    );
}

/// "An accent in the product name" ("Text that stays as written"): in a UTF-8 file the text of `$VERSIONINFO`
/// reaches the manifest as its UTF-8 bytes, while the file's string holds the code page 437 byte of `é`.
#[test]
fn versioninfo_stays_utf8() {
    let src = "'$ENCODING:UTF8\n$VERSIONINFO:ProductName='caf\u{e9}'\nPRINT \"\u{e9}\"\n";
    let fe = qb64rust_driver::frontend("p.bas", src.as_bytes().to_vec());
    assert!(!fe.has_errors(), "{}", fe.render_diagnostics());
    let ir = qb64rust_driver::lower(&fe);
    let files = qb64rust_driver::build::version_files(false, ir.settings.version_info.as_ref(), "p.exe");
    let (_, manifest) = files.iter().find(|(n, _)| n == "p.exe.manifest").expect("a manifest");
    let utf8 = "caf\u{e9}".as_bytes();
    assert!(manifest.windows(utf8.len()).any(|w| w == utf8), "{manifest:?}");
    assert!(qb64rust_driver::dump_ir(&fe).contains(r#"Str Const "\x82""#));
}

/// `qb64rust convert` (task 4.2; "The convert command"): "A game's file", "There and back", a file of ASCII only
/// left as it is, "Already UTF-8", and `--to cp437` on a file with `日` in a comment. When one file cannot be
/// converted, none is changed.
#[test]
fn the_convert_command() {
    let d = tempfile::tempdir().unwrap();
    let game = d.path().join("battle.bm");
    let original = b"' map\r\nPRINT \"\xC4\xCD\x03\"\r\n".to_vec();
    write(&game, &original);
    write(&d.path().join("plain.bas"), b"PRINT 1\n");
    let o = qb64rust(d.path(), &["convert", "--to", "utf8", "battle.bm", "plain.bas"]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    assert!(stdout(&o).contains("left plain.bas as it is"), "{}", stdout(&o));
    assert_eq!(
        std::fs::read(&game).unwrap(),
        b"' map\r\nPRINT \"\xE2\x94\x80\xE2\x95\x90\x03\"\r\n"
    );
    assert_eq!(std::fs::read(d.path().join("plain.bas")).unwrap(), b"PRINT 1\n");
    let o = qb64rust(d.path(), &["convert", "--to", "cp437", "battle.bm"]);
    assert_eq!(o.status.code(), Some(0), "{}", stdout(&o));
    assert_eq!(std::fs::read(&game).unwrap(), original, "there and back");

    // Already UTF-8: neither file changes.
    write(&d.path().join("a.bas"), b"PRINT \"\xC4\"\n");
    write(&d.path().join("b.bas"), "PRINT 1\nPRINT \"\u{e9}\"\n".as_bytes());
    let o = qb64rust(d.path(), &["convert", "--to", "utf8", "a.bas", "b.bas"]);
    let out = stdout(&o);
    assert_eq!(o.status.code(), Some(1), "{out}");
    assert!(
        out.contains("b.bas:2: the file is UTF-8 already") && out.contains("no file was changed"),
        "{out}"
    );
    assert_eq!(std::fs::read(d.path().join("a.bas")).unwrap(), b"PRINT \"\xC4\"\n");

    // A comment with a character code page 437 does not hold.
    write(&d.path().join("j.bas"), "PRINT 1\n' \u{65e5}\n".as_bytes());
    let o = qb64rust(d.path(), &["convert", "--to", "cp437", "j.bas"]);
    let out = stdout(&o);
    assert_eq!(o.status.code(), Some(1), "{out}");
    assert!(
        out.contains("j.bas:2: `\u{65e5}` (U+65E5) has no code page 437 byte"),
        "{out}"
    );

    for args in [
        &["convert", "j.bas"][..],
        &["convert", "--to", "utf8", "--help", "j.bas"],
    ] {
        let o = qb64rust(d.path(), args);
        assert_eq!(o.status.code(), Some(1), "{args:?}");
        assert!(
            stdout(&o).contains("usage: qb64rust convert --to utf8|cp437"),
            "{}",
            stdout(&o)
        );
    }
}

/// The twin test (task 2.6; "The same C++ as the code page 437 file"): the program of
/// `verification\v74_f1_twin_cp437` and the same text saved as UTF-8 (`v74_f2_twin_utf8`), each with its
/// `'$ENCODING` line, write the same C++ but for the comment that names how each was saved.
#[test]
fn the_twins_write_the_same_cpp() {
    let verification = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../verification");
    let d = tempfile::tempdir().unwrap();
    let cpp = |name: &str| {
        let dir = d.path().join(name);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::copy(verification.join(format!("{name}.bas")), dir.join("p.bas")).unwrap();
        let o = qb64rust(&dir, &["--dump", "cpp", "p.bas"]);
        assert_eq!(o.status.code(), Some(0), "{name}: {}", stdout(&o));
        o.stdout
    };
    let cp437 = cpp("v74_f1_twin_cp437");
    let utf8 = cpp("v74_f2_twin_utf8");
    assert!(!cp437.is_empty());
    assert_eq!(cp437, utf8);
}
