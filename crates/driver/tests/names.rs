//! Reserved names (design D3 of `m2-procedures-and-errors`) against the measurement in
//! `verification/v15_builtin_names.txt`: every keyword and built-in name, bare, with `&` and (for a built-in that
//! must be written with `$`) with `$`, assigned and printed as a variable with the old compiler.
//! - `variable`: the old compiler made it a variable, so the new one must accept the program;
//! - `rejected`: the old compiler rejected the program, so the new one must report an error;
//! - `statement`: the old compiler compiled a statement of that name (`DATE$ = "a"`), not a variable; either
//!   verdict is fine here (the statements are "not supported yet" or, for `REM`, a comment).

use qb64rust_driver::frontend;
use std::path::Path;

#[test]
fn reserved_names_match_the_old_compiler() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../verification/v15_builtin_names.txt");
    #[expect(clippy::disallowed_methods, reason = "a recorded list of names, not BASIC source")]
    let text = std::fs::read_to_string(&path).unwrap();
    let mut checked = 0;
    let mut wrong = Vec::new();
    for line in text.lines() {
        let mut parts = line.splitn(3, ' ');
        let (form, class) = (parts.next().unwrap(), parts.next().unwrap());
        let value = if form.ends_with('$') { "\"a\"" } else { "5" };
        let src = format!("$CONSOLE:ONLY\n{form} = {value}\nPRINT {form}\n");
        let fe = frontend("n.bas", src.into_bytes());
        match (class, fe.has_errors()) {
            ("variable", true) => wrong.push(format!("{form}: a variable for the old compiler, rejected here")),
            ("rejected", false) => wrong.push(format!("{form}: rejected by the old compiler, accepted here")),
            ("variable" | "rejected" | "statement", _) => {}
            _ => panic!("unknown class in {line:?}"),
        }
        checked += 1;
    }
    assert!(checked > 1000, "only {checked} lines in {}", path.display());
    assert!(wrong.is_empty(), "{} mismatches:\n{}", wrong.len(), wrong.join("\n"));
}

/// Procedure names against `verification/v19_proc_names.txt`: every keyword and built-in name as a SUB (called
/// bare and with `CALL`) and as a FUNCTION (bare and with `&`), each program as the measurement wrote it.
/// - `proc`: the old compiler called the procedure, so the new one must accept the program;
/// - `rejected`: the old compiler rejected it, so the new one must report an error;
/// - `builtin`: it compiled but did not build (`CALL peek`, `FUNCTION poke`: the generated C++ clashes with the
///   built-in's); either verdict is fine.
#[test]
fn procedure_names_match_the_old_compiler() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../verification/v19_proc_names.txt");
    #[expect(clippy::disallowed_methods, reason = "a recorded list of names, not BASIC source")]
    let text = std::fs::read_to_string(&path).unwrap();
    let mut checked = 0;
    let mut wrong = Vec::new();
    for line in text.lines() {
        let mut parts = line.splitn(4, ' ');
        let (name, form, class) = (parts.next().unwrap(), parts.next().unwrap(), parts.next().unwrap());
        let src = match form {
            "sub" => format!("$CONSOLE:ONLY\n{name}\nSYSTEM\nSUB {name}\nPRINT \"s\"\nEND SUB\n"),
            "call" => format!("$CONSOLE:ONLY\nCALL {name}\nSYSTEM\nSUB {name}\nPRINT \"s\"\nEND SUB\n"),
            "func" | "func&" => {
                let n = if form == "func&" {
                    format!("{name}&")
                } else {
                    name.to_string()
                };
                format!("$CONSOLE:ONLY\nPRINT {n}\nSYSTEM\nFUNCTION {n}\n{n} = 5\nEND FUNCTION\n")
            }
            _ => panic!("unknown form in {line:?}"),
        };
        let fe = frontend("p.bas", src.into_bytes());
        match (class, fe.has_errors()) {
            ("proc", true) => wrong.push(format!(
                "{name} {form}: a procedure for the old compiler, rejected here:\n{}",
                fe.render_diagnostics()
            )),
            ("rejected", false) => wrong.push(format!("{name} {form}: rejected by the old compiler, accepted here")),
            ("proc" | "rejected" | "builtin", _) => {}
            _ => panic!("unknown class in {line:?}"),
        }
        checked += 1;
    }
    assert!(checked > 1900, "only {checked} lines in {}", path.display());
    assert!(wrong.is_empty(), "{} mismatches:\n{}", wrong.len(), wrong.join("\n"));
}
