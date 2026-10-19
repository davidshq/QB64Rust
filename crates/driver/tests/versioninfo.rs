//! `$VERSIONINFO` against the old compiler's records (design D5 of `m3-versioninfo`; spec `compiler/pipeline`, "The
//! files of every measured program"). For each `verification\v40_*.bas`: a program the old compiler builds and runs
//! gives the resource files `qb64pe -z` wrote for it (`verification\v40_rc\`, recorded by `v40_rc.py`), byte for
//! byte, and the warnings of `-w` it printed; a program whose old build fails or whose executable does not start
//! (design D3) is a compile error here; a program the old compiler rejects is rejected. Nothing is built.

use qb64rust_base::Severity;
use qb64rust_driver::{FileLoader, build, frontend_with};
use std::path::{Path, PathBuf};

fn verification() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../verification")
}

/// The programs the old compiler accepts but whose build fails (`h*`) or whose executable Windows does not start
/// (`g*`), measured.
const REFUSED: [&str; 9] = [
    "v40_g2_lt_in_name",
    "v40_g3_amp_in_company",
    "v40_g4_lt_in_description",
    "v40_g6_byte_e9_in_name",
    "v40_h1_empty_part",
    "v40_h2_large_parts",
    "v40_h4_leading_comma",
    "v40_h5_octal_looking",
    "v40_h7_one_over",
];

/// The warnings of the old compiler's `-z -w` output: each `<file>:<line>: warning: <message>` line with the
/// indented line after it, as one text.
fn old_warnings(z: &str) -> Vec<String> {
    let lines: Vec<&str> = z.lines().collect();
    let mut out = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        if l.contains(": warning: ") {
            let mut w = l.to_string();
            if let Some(next) = lines.get(i + 1).filter(|n| n.starts_with("    ")) {
                w.push('\n');
                w.push_str(next);
            }
            out.push(w);
        }
    }
    out
}

#[test]
#[expect(
    clippy::disallowed_methods,
    reason = "the old compiler's console output and resource files, shown"
)]
fn resource_files_equal_the_old_compilers() {
    let dir = verification();
    let rc = dir.join("v40_rc");
    let mut programs: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            let n = p.file_name().unwrap().to_string_lossy();
            n.starts_with("v40_") && n.ends_with(".bas")
        })
        .collect();
    programs.sort();
    assert_eq!(programs.len(), 47, "the v40 family");
    let mut failures = Vec::new();
    let (mut compared, mut refused, mut rejected) = (0, 0, 0);
    for path in &programs {
        let name = path.file_stem().unwrap().to_string_lossy().into_owned();
        let file = format!("{name}.bas");
        let bytes = std::fs::read(path).unwrap();
        let mut fe = frontend_with(&file, bytes, |f| Box::new(FileLoader::new(&dir, f, path)));
        qb64rust_driver::check_exe_icon(&fe.program, &mut fe.diagnostics, path);
        let errors = fe.render_diagnostics();
        let recorded_rc = rc.join(format!("{name}.icon.rc"));
        if REFUSED.contains(&name.as_str()) {
            refused += 1;
            let says_why = errors.contains("resource compiler") || errors.contains("manifest cannot hold");
            if fe.diagnostics.error_count() != 1 || !says_why {
                failures.push(format!("{name}: expected one error of design D3, got:\n{errors}"));
            }
            continue;
        }
        if !recorded_rc.is_file() {
            rejected += 1;
            if !fe.diagnostics.has_real_errors() {
                failures.push(format!("{name}: rejected by the old compiler, accepted here"));
            }
            continue;
        }
        if fe.has_errors() {
            failures.push(format!("{name}: errors:\n{errors}"));
            continue;
        }
        compared += 1;
        let ir = qb64rust_driver::lower(&fe);
        let icon = ir.settings.exe_icon;
        let files = build::version_files(icon, ir.settings.version_info.as_ref(), &format!("{name}.exe"));
        let recorded = [
            (format!("{name}.exe.manifest"), rc.join(format!("{name}.manifest"))),
            ("manifest.h".to_string(), rc.join(format!("{name}.manifest.h"))),
            ("icon.rc".to_string(), recorded_rc),
        ];
        let written: Vec<&String> = files.iter().map(|(n, _)| n).collect();
        let expected: Vec<&String> = recorded.iter().map(|(n, _)| n).collect();
        if written != expected {
            failures.push(format!("{name}: files {written:?}, expected {expected:?}"));
            continue;
        }
        for ((n, bytes), (_, want)) in files.iter().zip(&recorded) {
            if *bytes != std::fs::read(want).unwrap() {
                failures.push(format!(
                    "{name}: {n} differs:\n--- written\n{}\n--- recorded\n{}",
                    String::from_utf8_lossy(bytes),
                    String::from_utf8_lossy(&std::fs::read(want).unwrap())
                ));
            }
        }
        let z = std::fs::read_to_string(rc.join(format!("{name}.z.txt"))).unwrap();
        let ours: Vec<String> = fe
            .diagnostics
            .list()
            .iter()
            .filter(|d| d.severity == Severity::Warning)
            .map(|d| {
                let line = fe.map.file(d.span.file).line_col(d.span.start).0;
                format!("{file}:{line}: warning: {}", d.message)
            })
            .collect();
        if ours != old_warnings(&z) {
            failures.push(format!(
                "{name}: warnings {ours:?}, the old compiler's {:?}",
                old_warnings(&z)
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n\n"));
    // Seventeen built (`x20` among them), nine refused, twenty-one rejected.
    assert_eq!((compared, refused, rejected), (17, 9, 21));
}
