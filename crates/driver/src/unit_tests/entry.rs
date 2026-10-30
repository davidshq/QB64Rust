//! The entry point of a test file (design D4 of `basic-unit-tests`): a console program with `$ASSERTS` that includes
//! the library and the test file, then runs the test SUB that `QB64RUST_TEST_NAME` names.

use super::discover::TestSub;
use std::path::{Path, PathBuf};

/// The two files of the library, served from memory under these names (`loader.rs`).
pub const LIBRARY_BI: &str = "qb64rust-test/qbtest.bi";
pub const LIBRARY_BM: &str = "qb64rust-test/qbtest.bm";
pub const LIBRARY_BI_TEXT: &str = include_str!("qbtest.bi");
pub const LIBRARY_BM_TEXT: &str = include_str!("qbtest.bm");

/// The variables the program reads (design D4).
pub const NAME_ENV: &str = "QB64RUST_TEST_NAME";
pub const RESULT_ENV: &str = "QB64RUST_TEST_RESULT";
pub const TEMP_ENV: &str = "QB64RUST_TEST_TEMP";

/// The path of the virtual main file of `test_file`: `<its folder>/<its name>.test-entry.bas`, never written, so
/// that everything the compiler finds from the main file's folder is found as for a build of the test file.
pub fn entry_path(test_file: &Path) -> PathBuf {
    let mut name = test_file.file_name().unwrap_or_default().to_os_string();
    name.push(".test-entry.bas");
    test_file.with_file_name(name)
}

/// The text of the entry point of the test file named `file_name` (its bare name, included from its own folder)
/// with the tests `tests`.
pub fn entry_point(file_name: &str, tests: &[TestSub]) -> String {
    let mut out = format!("$CONSOLE:ONLY\n$ASSERTS\n'$INCLUDE:'{LIBRARY_BI}'\n'$INCLUDE:'{file_name}'\nqbtest_begin\n");
    out.push_str(&format!("SELECT CASE UCASE$(ENVIRON$(\"{NAME_ENV}\"))\n"));
    for t in tests {
        out.push_str(&format!("CASE \"{}\": {}\n", t.name.to_ascii_uppercase(), t.name));
    }
    out.push_str(&format!(
        "CASE ELSE: qbtest_unknown\nEND SELECT\nqbtest_done\nSYSTEM\n'$INCLUDE:'{LIBRARY_BM}'\n"
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_entry_point_of_a_fixture() {
        let tests = [
            TestSub {
                name: "TestAdd".into(),
                line: 3,
            },
            TestSub {
                name: "test_sub".into(),
                line: 7,
            },
        ];
        assert_eq!(
            entry_point("math_test.bas", &tests),
            "$CONSOLE:ONLY\n$ASSERTS\n'$INCLUDE:'qb64rust-test/qbtest.bi'\n'$INCLUDE:'math_test.bas'\nqbtest_begin\n\
             SELECT CASE UCASE$(ENVIRON$(\"QB64RUST_TEST_NAME\"))\nCASE \"TESTADD\": TestAdd\nCASE \"TEST_SUB\": test_sub\n\
             CASE ELSE: qbtest_unknown\nEND SELECT\nqbtest_done\nSYSTEM\n'$INCLUDE:'qb64rust-test/qbtest.bm'\n"
        );
    }

    #[test]
    fn the_entry_path_is_beside_the_test_file() {
        let p = entry_path(Path::new("lib/math_test.bas"));
        assert_eq!(p, Path::new("lib/math_test.bas.test-entry.bas"));
    }
}
