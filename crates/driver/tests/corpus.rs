//! Tier 1 (`study\19`, spec `testing/compiler-tests`): the front end over the golden corpus, without C++.
//! Every corpus program must go through lexer, parser and `sema` without a panic and round-trip its tree; the
//! programs named in `tests/corpus/slice.list` must give no diagnostics.

use qb64rust_driver::frontend;
use qb64rust_syntax::tree::print;
use std::path::{Path, PathBuf};

fn corpus_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/corpus")
}

fn bas_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(dir).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            bas_files(&p, out);
        } else if p.extension().is_some_and(|x| x.eq_ignore_ascii_case("bas")) {
            out.push(p);
        }
    }
}

#[test]
fn every_corpus_program_goes_through_the_front_end() {
    let mut files = Vec::new();
    bas_files(&corpus_root(), &mut files);
    assert!(files.len() >= 270, "found only {} corpus programs", files.len());
    for f in &files {
        let bytes = std::fs::read(f).unwrap();
        let name = f.file_name().unwrap().to_string_lossy().to_string();
        let fe = frontend(&name, bytes.clone());
        assert!(
            print(&fe.parse.green, &bytes) == bytes,
            "round trip failed: {}",
            f.display()
        );
    }
}

#[test]
fn slice_list_programs_have_no_diagnostics() {
    let root = corpus_root();
    let list = std::fs::read_to_string(root.join("slice.list")).unwrap();
    let mut n = 0;
    for line in list.lines() {
        let line = line.split('#').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }
        let path = root.join(format!("{line}.bas"));
        let fe = frontend(&format!("{line}.bas"), std::fs::read(&path).unwrap());
        assert!(fe.diagnostics.list().is_empty(), "{line}:\n{}", fe.render_diagnostics());
        n += 1;
    }
    assert_eq!(n, 14);
}
