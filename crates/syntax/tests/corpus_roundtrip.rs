//! Tier 1 (`study\19`): every corpus program parses without a panic and its tree prints back to the exact bytes.

use qb64rust_base::SourceMap;
use qb64rust_syntax::tree::print;
use qb64rust_syntax::{NoLoader, parse};
use std::path::{Path, PathBuf};

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
fn corpus_round_trip() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests");
    let mut files = Vec::new();
    bas_files(&root, &mut files);
    assert!(files.len() >= 270, "found only {} .bas files under tests/", files.len());
    for f in &files {
        let bytes = std::fs::read(f).unwrap();
        let mut map = SourceMap::new();
        let file = map.add(f.display().to_string(), bytes.clone());
        let p = parse(&mut map, file, &mut NoLoader);
        assert!(
            print(&p.main().green, &bytes) == bytes,
            "round trip failed: {}",
            f.display()
        );
    }
}
