//! Every function of the `CONST` evaluator against the old compiler (task 3.4 of `m3-constant-evaluation`):
//! `verification\v45_b_funcs` holds each one as a `_FLOAT` constant and its recorded output prints each value's 19
//! digits; `v45_b_funcs_plain` some as plain constants. A constant compiled here prints the same text, or its line
//! has the warning of `DIVERGENCES.md` D-026 (the old value depends on the CPU and the system's library, and the
//! correctly rounded one was taken), or it is one of [`DECIDED`]. None may differ otherwise.

/// The constants of `v45_b_funcs` that differ from the old compiler's by a decided row of `DIVERGENCES.md`: the
/// recorded expression, the row, and the value here.
const DECIDED: [(&str, &str, &str); 2] = [
    ("_COTH(.2)", "D-031", "5.066489563439472654"),
    ("2 ^ -1", "D-028", "0.5"),
];

use qb64rust_driver::frontend;
use qb64rust_sema::consteval::Value;
use std::path::PathBuf;

fn verification() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../verification")
}

/// The constants of `name` (by their names, in source order) with their line, and the recorded output's lines.
struct Program {
    consts: Vec<(String, usize, String)>,
    warned: Vec<usize>,
    out: Vec<String>,
}

#[expect(
    clippy::disallowed_methods,
    reason = "the lines of an ASCII verification program and its recorded output, matched as text"
)]
fn compile(name: &str) -> Program {
    let dir = verification();
    let bytes = std::fs::read(dir.join(format!("{name}.bas"))).unwrap();
    let source = String::from_utf8_lossy(&bytes).into_owned();
    let fe = frontend(&format!("{name}.bas"), bytes);
    assert!(!fe.has_errors(), "{name}: {}", fe.render_diagnostics());
    let warned: Vec<usize> = fe
        .render_with_warnings()
        .lines()
        .filter(|l| l.contains(": warning: ") && l.contains("correctly rounded"))
        .map(|l| l.split(':').nth(1).unwrap().parse().unwrap())
        .collect();
    let mut consts = Vec::new();
    for c in &fe.program.consts {
        let line = source
            .lines()
            .position(|l| {
                l.to_ascii_uppercase().starts_with(&format!("CONST {} ", c.name))
                    || l.to_ascii_uppercase().starts_with(&format!("CONST {}## ", c.name))
            })
            .map(|i| i + 1);
        let Some(line) = line else { continue };
        let text = match &c.value {
            Value::Float(r, _) => r.one().unwrap().to_g19(),
            Value::Int(i) => i.to_string(),
            other => panic!("{name}: {} is {other:?}", c.name),
        };
        consts.push((c.name.clone(), line, text));
    }
    let out = std::fs::read_to_string(dir.join(format!("{name}.out.txt"))).unwrap();
    let out = out
        .lines()
        .filter(|l| !l.starts_with("exit="))
        .map(str::to_string)
        .collect();
    Program { consts, warned, out }
}

#[test]
fn every_function_has_the_old_value_or_its_warning() {
    let p = compile("v45_b_funcs");
    assert_eq!(p.consts.len(), p.out.len(), "one constant per line of output");
    let mut fell_back = Vec::new();
    let mut decided = Vec::new();
    for ((name, line, ours), recorded) in p.consts.iter().zip(&p.out) {
        let (what, theirs) = recorded.split_once(": ").unwrap();
        // `_TOSTR$` writes the exponent's `E` as `F`.
        let theirs = theirs.replace('F', "E");
        if let Some((_, row, want)) = DECIDED.iter().find(|(w, _, _)| *w == what) {
            assert_eq!(ours, want, "{name} ({what}) by {row}");
            decided.push(what);
            continue;
        }
        if *ours == theirs {
            continue;
        }
        assert!(
            p.warned.contains(line),
            "{name} ({what}) is {ours}, the old compiler's {theirs}, with no D-026 warning on line {line}"
        );
        fell_back.push(format!("{what}: {ours} (old {theirs})"));
    }
    // Each decided row matched a recorded line, so none is listed in vain.
    for (what, row, _) in DECIDED {
        assert!(decided.contains(&what), "{what} ({row}) is in no recorded line");
    }
    // The values D-026 took, for the record (`cargo test … -- --nocapture`).
    println!(
        "{} of {} differ, each with its warning: {fell_back:#?}",
        fell_back.len(),
        p.consts.len()
    );
}

/// The spec's scenario "A value that cannot be certified" (`language/constants`): `20 ROOT 3` (`powl`, 10 ulp) does
/// not certify, so it compiles with the correctly rounded value and the warning.
#[test]
fn an_uncertified_constant_warns() {
    let fe = frontend("root.bas", b"CONST r = 20 ROOT 3\r\nPRINT r\r\n".to_vec());
    assert!(!fe.has_errors(), "{}", fe.render_diagnostics());
    let all = fe.render_with_warnings();
    assert!(
        all.contains("root.bas:1:11: warning: this `CONST` is computed with correctly rounded math functions"),
        "{all}"
    );
    let Value::Float(r, _) = &fe.program.consts[0].value else {
        panic!("a float")
    };
    // The cube root of 20, 2.714417616594906572…, through `_TOSTR$`'s 19 digits.
    assert_eq!(r.one().unwrap().to_g19(), "2.714417616594906572");
}

/// The lines (1-based) of `source` that have the warning of D-026.
fn warned_lines(source: &[u8]) -> Vec<usize> {
    let fe = frontend("e.bas", source.to_vec());
    assert!(!fe.has_errors(), "{}", fe.render_diagnostics());
    fe.render_with_warnings()
        .lines()
        .filter(|l| l.contains(": warning: ") && l.contains("correctly rounded"))
        .map(|l| l.split(':').nth(1).unwrap().parse().unwrap())
        .collect()
}

/// `_E` (`expl`, 1.5 ulp) does not certify: every use warns, wherever it is first named, and so does a `CONST`
/// computed from a constant that took the correctly rounded value.
#[test]
fn each_use_of_an_uncertified_constant_warns() {
    assert_eq!(warned_lines(b"PRINT _E\r\n"), [1]);
    assert_eq!(
        warned_lines(b"CONST x = _E * 2\r\nCONST y = _E * 3\r\nPRINT x, y\r\n"),
        [1, 2]
    );
    assert_eq!(
        warned_lines(b"PRINT _E\r\nCONST x = _E * 2\r\nCONST y = _E * 3\r\nPRINT x, y, _E\r\n"),
        [1, 2, 3, 4]
    );
    assert_eq!(
        warned_lines(b"CONST r = 20 ROOT 3\r\nCONST s = r * 2\r\nCONST t = 5\r\nPRINT s, t\r\n"),
        [1, 2]
    );
}

#[test]
fn plain_constants_print_as_the_old_ones() {
    let p = compile("v45_b_funcs_plain");
    assert_eq!(p.consts.len(), p.out.len(), "one constant per line of output");
    // Each output line is `<expression>: <_TOSTR$ of the DOUBLE> <PRINT of it>`.
    for ((name, line, ours), recorded) in p.consts.iter().zip(&p.out) {
        let theirs = recorded.split_once(": ").unwrap().1.split(' ').next().unwrap();
        // C++ reads the constant's 19 digits as a `double`; `_TOSTR$` of a DOUBLE prints 16 significant digits.
        let ours = format!("{:.15e}", ours.parse::<f64>().unwrap()).parse::<f64>().unwrap();
        let theirs = theirs.parse::<f64>().unwrap();
        assert!(
            ours == theirs || p.warned.contains(line),
            "{name} is {ours}, the old compiler's {theirs}, with no D-026 warning on line {line}"
        );
    }
}
