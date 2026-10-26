//! The reader of the debug symbol file against the emitter's own output: every `debug_info_*` snapshot of the
//! driver's front-end tests (the file's text, `crates\driver\tests\snapshots\`), so that a change of the file's form
//! fails here in the same run (design D3 of `m5-dap-proxy`).

use qb64rust_dap::symbols::{Kind, Reach, Shape, Storage, Symbols};
use serde_json::{Value, json};
use std::path::Path;

/// The JSON of each snapshot: what follows insta's header.
fn snapshots() -> Vec<(String, Value)> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../driver/tests/snapshots");
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if !(name.starts_with("debug_info_") && name.ends_with(".debug-info.snap")) {
            continue;
        }
        let bytes = std::fs::read(&path).unwrap();
        let body = bytes
            .windows(5)
            .enumerate()
            .filter(|(_, w)| *w == b"---\n{" || *w == b"--\r\n{")
            .map(|(i, _)| i + 4)
            .next_back()
            .unwrap_or_else(|| panic!("{name}: no JSON after the header"));
        let v: Value = serde_json::from_slice(&bytes[body..]).unwrap_or_else(|e| panic!("{name}: {e}"));
        out.push((name, v));
    }
    assert!(out.len() >= 5, "the five snapshots of m5-debug-build-symbols");
    out
}

#[test]
fn every_snapshot_parses() {
    for (name, v) in snapshots() {
        let s = Symbols::parse(&v).unwrap_or_else(|e| panic!("{name}: {e}"));
        assert_eq!(s.main_c_name, "QBMAIN", "{name}");
        assert_eq!(s.layouts.pointer_size, 8, "{name}");
    }
}

#[test]
fn the_storages_and_their_reaches() {
    let (_, v) = snapshots().into_iter().find(|(n, _)| n.contains("storages")).unwrap();
    let s = Symbols::parse(&v).unwrap();
    let show = s.procedure_of_function("SUB_SHOW(int*)").unwrap();
    assert_eq!(s.procedures[show].kind, Kind::Sub);
    assert!(s.is_main("QBMAIN(void*)"));
    assert_eq!(s.procedure_of_function("QBMAIN(void*)"), None);
    let names: Vec<_> = s.variables_of(Some(show)).map(|(_, v)| v.name.as_str()).collect();
    assert_eq!(names, ["N", "CALLS", "X"]);
    let main: Vec<_> = s.variables_of(None).map(|(_, v)| (v.name.as_str(), v.reach)).collect();
    assert!(main.contains(&("A", Reach::Descriptor)));
    assert!(main.contains(&("S", Reach::String)));
    let a = s.variables.iter().find(|v| v.name == "A").unwrap();
    assert_eq!((a.shape, a.bounds.clone()), (Shape::Static, vec![(1, 3), (0, 1)]));
    let n = &s.variables[s.procedures[show].params[0]];
    assert_eq!((n.storage, n.reach), (Storage::Param, Reach::ParamPointer));
    let l = &s.layouts.array;
    assert_eq!((l.dimension_slot(2, 0), l.dimension_slot(2, 1)), (8, 4));
}

#[test]
fn version_2_is_refused() {
    let (_, mut v) = snapshots().remove(0);
    v["version"] = json!(2);
    let e = Symbols::parse(&v).unwrap_err();
    assert!(e.contains("version 2"), "{e}");
}

#[test]
fn a_missing_field_is_named() {
    let (_, mut v) = snapshots().into_iter().find(|(n, _)| n.contains("storages")).unwrap();
    let mut no_layouts = v.clone();
    v["variables"][0].as_object_mut().unwrap().remove("reach");
    let e = Symbols::parse(&v).unwrap_err();
    assert!(e.contains("no `reach` in a variable"), "{e}");
    no_layouts.as_object_mut().unwrap().remove("layouts");
    assert!(Symbols::parse(&no_layouts).unwrap_err().contains("`layouts`"));
}
