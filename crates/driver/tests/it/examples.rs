//! The example programs the editor ships (OpenSpec change `editor-side-panel`, spec `editor/examples`, "Every
//! shipped example builds"): each program of `examples\` and each example of the language reference
//! (`tests\reference\examples\`) is checked with `qb64rust -z` and gives no diagnostic. Each runs in a scratch
//! folder of its own, with the contents of its `.files` folder copied in, so that nothing is written into the
//! repository.

use crate::support::{self, COMPILER_LIMIT, repo, run};
use std::path::{Path, PathBuf};
use std::process::Command;
use toml::de::DeTable;

/// The programs of `examples\`, as `<group>/<name>`.
fn programs() -> Vec<String> {
    let root = repo().join("examples");
    let mut out = Vec::new();
    for group in std::fs::read_dir(&root).unwrap() {
        let group = group.unwrap().path();
        if !group.is_dir() {
            continue;
        }
        for f in std::fs::read_dir(&group).unwrap() {
            let f = f.unwrap().path();
            if f.extension().is_some_and(|e| e == "bas") {
                let g = group.file_name().unwrap().to_string_lossy();
                out.push(format!("{g}/{}", f.file_stem().unwrap().to_string_lossy()));
            }
        }
    }
    out.sort();
    out
}

/// The examples `examples\index.toml` lists, as `<group>/<name>`, in its order.
fn listed() -> Vec<String> {
    #[expect(
        clippy::disallowed_methods,
        reason = "index.toml is TOML, which is UTF-8 by its specification; not BASIC source"
    )]
    let text = std::fs::read_to_string(repo().join("examples/index.toml")).unwrap();
    let doc = DeTable::parse(&text).unwrap();
    let mut out = Vec::new();
    for g in list(doc.get_ref(), "group") {
        let id = field(g, "id");
        assert!(!field(g, "title").is_empty(), "group {id} has a title");
        for e in list(g, "example") {
            assert!(!field(e, "title").is_empty(), "{id}: an example has a title");
            out.push(format!("{id}/{}", field(e, "name")));
        }
    }
    out
}

/// The string `name` of a table of `index.toml`.
fn field(t: &DeTable<'_>, name: &str) -> String {
    let v = t.get(name).unwrap_or_else(|| panic!("no {name}"));
    v.get_ref().as_str().unwrap().to_string()
}

/// The array of tables `name` of a table of `index.toml`.
fn list<'t>(t: &'t DeTable<'_>, name: &str) -> Vec<&'t DeTable<'t>> {
    let v = t.get(name).unwrap_or_else(|| panic!("no {name}"));
    let items = v.get_ref().as_array().unwrap();
    items.iter().map(|i| i.get_ref().as_table().unwrap()).collect()
}

/// The diagnostics of `qb64rust -q -z` on a copy of `bas` (and of its `.files` folder) in a scratch folder: empty
/// when it compiles with none.
fn check(bas: &Path) -> String {
    let dir = tempfile::Builder::new().prefix("qb64rust-example-").tempdir().unwrap();
    let name = bas.file_name().unwrap();
    std::fs::copy(bas, dir.path().join(name)).unwrap();
    let files = bas.with_extension("files");
    if files.is_dir() {
        copy_dir(&files, dir.path());
    }
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_qb64rust"));
    cmd.env(support::OBJECTS_LOCK_ENV, support::objects_lock());
    cmd.current_dir(dir.path()).args(["-q", "-z"]).arg(name);
    let o = run(&mut cmd, None, COMPILER_LIMIT, "compiler");
    #[expect(clippy::disallowed_methods, reason = "the compiler's own messages")]
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
    // A diagnostic reads `<file>:<line>:<column>: error: …` or `…: warning: …`.
    let found: Vec<&str> = text
        .lines()
        .filter(|l| l.contains(": error") || l.contains(": warning"))
        .collect();
    if o.status.code() == Some(0) && found.is_empty() {
        String::new()
    } else {
        format!("{}: exit {:?}\n{text}", bas.display(), o.status.code())
    }
}

fn copy_dir(from: &Path, to: &Path) {
    for e in std::fs::read_dir(from).unwrap() {
        let p = e.unwrap().path();
        let target = to.join(p.file_name().unwrap());
        if p.is_dir() {
            std::fs::create_dir_all(&target).unwrap();
            copy_dir(&p, &target);
        } else {
            std::fs::copy(&p, &target).unwrap();
        }
    }
}

/// Checks `files` on a few threads and gives the failures.
fn check_all(files: Vec<PathBuf>) -> Vec<String> {
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get().min(8));
    let chunk = files.len().div_ceil(threads).max(1);
    std::thread::scope(|s| {
        let handles: Vec<_> = files
            .chunks(chunk)
            .map(|part| {
                s.spawn(move || {
                    part.iter()
                        .map(|f| check(f))
                        .filter(|t| !t.is_empty())
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles.into_iter().flat_map(|h| h.join().unwrap()).collect()
    })
}

/// Every program of `examples\` is listed in `examples\index.toml` once, and each one listed is there.
#[test]
fn examples_are_listed() {
    let mut listed = listed();
    let count = listed.len();
    listed.sort();
    listed.dedup();
    assert_eq!(listed.len(), count, "an example is listed twice");
    assert_eq!(listed, programs());
    assert!(listed.contains(&"first-steps/hello".to_string()));
}

/// "Every shipped example builds": the programs of `examples\` and the reference's examples give no diagnostic.
#[test]
fn examples_check_with_no_diagnostic() {
    let root = repo();
    let mut files: Vec<PathBuf> = programs()
        .iter()
        .map(|p| root.join(format!("examples/{p}.bas")))
        .collect();
    let reference = root.join("tests/reference/examples");
    let mut theirs: Vec<PathBuf> = std::fs::read_dir(&reference)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "bas"))
        .collect();
    theirs.sort();
    assert!(theirs.len() > 300, "the reference's examples");
    files.extend(theirs);
    let failures = check_all(files);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
