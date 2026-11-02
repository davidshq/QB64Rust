//! The output of `qb64rust test` (spec `compiler/unit-tests`, "Results and exit status"; design D6 of
//! `basic-unit-tests`): text, or JSON lines for tools.

use super::discover::TestSub;
use super::judge::Outcome;
use super::run::TestResult;
use qb64rust_base::json_string;

/// The counts of a run.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Counts {
    pub passed: usize,
    pub failed: usize,
    pub errors: usize,
    pub timeouts: usize,
}

impl Counts {
    pub fn add(&mut self, outcome: Outcome) {
        match outcome {
            Outcome::Pass => self.passed += 1,
            Outcome::Fail => self.failed += 1,
            Outcome::Error | Outcome::BuildError => self.errors += 1,
            Outcome::Timeout => self.timeouts += 1,
        }
    }

    pub fn all_passed(&self) -> bool {
        self.failed + self.errors + self.timeouts == 0
    }
}

/// The word that starts a result's line of text.
fn word(outcome: Outcome) -> &'static str {
    match outcome {
        Outcome::Pass => "ok",
        Outcome::Fail => "FAIL",
        Outcome::Error | Outcome::BuildError => "ERROR",
        Outcome::Timeout => "TIMEOUT",
    }
}

/// A test found, for `--list`: `<file>:<line>: <SUB>`.
pub fn test_text(file: &str, t: &TestSub) -> String {
    format!("{file}:{}: {}", t.line, t.name)
}

/// One line per result: its word, file, SUB and time.
pub fn result_text(r: &TestResult) -> String {
    format!(
        "{:<7} {}:{} ({}.{:02} s)",
        word(r.outcome),
        r.file,
        r.name,
        r.ms / 1000,
        r.ms % 1000 / 10
    )
}

/// What follows the result lines for a test that did not pass: its message and where, its output, and its kept
/// folder.
pub fn details_text(r: &TestResult) -> String {
    let mut out = format!("--- {} {}:{}\n", word(r.outcome), r.file, r.name);
    if r.outcome == Outcome::BuildError {
        out.push_str("the test file did not build:\n");
    }
    for line in r.message.lines() {
        out.push_str(line);
        out.push('\n');
    }
    match &r.location {
        Some((file, line)) => out.push_str(&format!("  at {file}:{line}\n")),
        None if r.outcome != Outcome::BuildError => out.push_str(&format!("  in the SUB at {}:{}\n", r.file, r.line)),
        None => {}
    }
    if !r.output.trim().is_empty() {
        out.push_str("output:\n");
        for line in r.output.trim_end().lines() {
            out.push_str("  ");
            out.push_str(line.trim_end_matches('\r'));
            out.push('\n');
        }
    }
    if let Some(dir) = &r.kept {
        out.push_str(&format!("its folder is kept: {}\n", dir.display()));
    }
    out
}

/// The last line of text.
pub fn summary_text(c: &Counts) -> String {
    format!(
        "{} passed, {} failed, {} errors, {} timed out",
        c.passed, c.failed, c.errors, c.timeouts
    )
}

/// A test found, as a JSON line.
pub fn test_json(file: &str, t: &TestSub) -> String {
    format!(
        "{{\"event\":\"test\",\"file\":{},\"name\":{},\"line\":{}}}",
        json_string(file),
        json_string(&t.name),
        t.line
    )
}

/// A result, as a JSON line.
pub fn result_json(r: &TestResult) -> String {
    let location = match &r.location {
        Some((file, line)) => format!("{{\"file\":{},\"line\":{line}}}", json_string(file)),
        None => "null".to_string(),
    };
    format!(
        "{{\"event\":\"result\",\"file\":{},\"name\":{},\"line\":{},\"outcome\":\"{}\",\"message\":{},\
         \"location\":{location},\"output\":{},\"ms\":{}}}",
        json_string(&r.file),
        json_string(&r.name),
        r.line,
        r.outcome.json(),
        json_string(&r.message),
        json_string(&r.output),
        r.ms
    )
}

/// The counts, as the last JSON line.
pub fn summary_json(c: &Counts) -> String {
    format!(
        "{{\"event\":\"summary\",\"passed\":{},\"failed\":{},\"errors\":{},\"timeouts\":{}}}",
        c.passed, c.failed, c.errors, c.timeouts
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result(outcome: Outcome) -> TestResult {
        TestResult {
            file: "lib\\math_test.bas".into(),
            name: "TestAdd".into(),
            line: 5,
            outcome,
            message: "sum: expected 4, got 5".into(),
            location: None,
            output: "hello\r\n".into(),
            ms: 1234,
            kept: None,
        }
    }

    #[test]
    fn text_lines() {
        assert_eq!(
            result_text(&result(Outcome::Fail)),
            "FAIL    lib\\math_test.bas:TestAdd (1.23 s)"
        );
        assert_eq!(
            result_text(&result(Outcome::Pass)),
            "ok      lib\\math_test.bas:TestAdd (1.23 s)"
        );
        assert_eq!(
            details_text(&result(Outcome::Fail)),
            "--- FAIL lib\\math_test.bas:TestAdd\nsum: expected 4, got 5\n  in the SUB at lib\\math_test.bas:5\n\
             output:\n  hello\n"
        );
        let c = Counts {
            passed: 2,
            failed: 1,
            ..Counts::default()
        };
        assert_eq!(summary_text(&c), "2 passed, 1 failed, 0 errors, 0 timed out");
    }

    #[test]
    fn json_lines() {
        let t = TestSub {
            name: "TestAdd".into(),
            line: 5,
        };
        assert_eq!(
            test_json("lib\\math_test.bas", &t),
            "{\"event\":\"test\",\"file\":\"lib\\\\math_test.bas\",\"name\":\"TestAdd\",\"line\":5}"
        );
        let mut r = result(Outcome::Fail);
        r.location = Some(("lib\\math_test.bas".into(), 9));
        assert_eq!(
            result_json(&r),
            "{\"event\":\"result\",\"file\":\"lib\\\\math_test.bas\",\"name\":\"TestAdd\",\"line\":5,\
             \"outcome\":\"fail\",\"message\":\"sum: expected 4, got 5\",\"location\":{\"file\":\
             \"lib\\\\math_test.bas\",\"line\":9},\"output\":\"hello\\u000d\\u000a\",\"ms\":1234}"
        );
        assert_eq!(
            summary_json(&Counts::default()),
            "{\"event\":\"summary\",\"passed\":0,\"failed\":0,\"errors\":0,\"timeouts\":0}"
        );
    }
}
