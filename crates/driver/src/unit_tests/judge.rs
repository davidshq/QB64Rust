//! How a test is judged (spec `compiler/unit-tests`, "How a test is judged"; design D4 of `basic-unit-tests`), from
//! the result file, the program's stdout and stderr, and its exit code. Printed output never decides a pass.

/// A test's result.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Pass,
    Fail,
    Error,
    Timeout,
    /// The test file did not build.
    BuildError,
}

impl Outcome {
    /// The word of the JSON lines.
    pub fn json(self) -> &'static str {
        match self {
            Outcome::Pass => "pass",
            Outcome::Fail => "fail",
            Outcome::Error => "error",
            Outcome::Timeout => "timeout",
            Outcome::BuildError => "build-error",
        }
    }
}

/// The verdict on one run: its outcome, a message (empty for a pass), and where the failure is when libqb named
/// it: the file as libqb names it (`main module` is the entry point) and the line.
#[derive(Debug, PartialEq, Eq)]
pub struct Verdict {
    pub outcome: Outcome,
    pub message: String,
    pub location: Option<(String, u32)>,
}

impl Verdict {
    fn new(outcome: Outcome, message: impl Into<String>) -> Verdict {
        Verdict {
            outcome,
            message: message.into(),
            location: None,
        }
    }
}

/// What a run of a test left: the result file's text, stdout, stderr, the exit code (`None`: killed by a signal),
/// and whether it was ended at the timeout.
pub struct Run<'a> {
    pub result: &'a str,
    pub stdout: &'a str,
    pub stderr: &'a str,
    pub exit: Option<i32>,
    pub timed_out: bool,
}

/// libqb's line before an error's message: `Runtime error: Line: <n> (in <file>)`; the line and the file.
fn error_location(line: &str) -> Option<(String, u32)> {
    let rest = line.trim().strip_prefix("Runtime error: Line: ")?;
    let (n, file) = rest.split_once(" (in ")?;
    Some((file.strip_suffix(')')?.to_string(), n.trim().parse().ok()?))
}

/// Judges a run, in this order: a timeout; a `FAIL` line of the library; libqb's `_ASSERT failed`; another
/// runtime error; `UNKNOWN` (no such test); `DONE` and exit code 0 (a pass); anything else is an error.
pub fn judge(run: &Run) -> Verdict {
    if run.timed_out {
        return Verdict::new(Outcome::Timeout, "timed out");
    }
    let results: Vec<&str> = run.result.lines().map(str::trim_end).collect();
    if let Some(msg) = results.iter().find_map(|l| l.strip_prefix("FAIL\t")) {
        return Verdict::new(Outcome::Fail, msg);
    }
    let stderr: Vec<&str> = run.stderr.lines().map(str::trim_end).collect();
    // The location is the `Runtime error: Line:` line before the message, when libqb wrote one.
    let located = |at: usize| stderr[..at].iter().rev().find_map(|l| error_location(l));
    if let Some(at) = stderr.iter().position(|l| l.starts_with("_ASSERT failed")) {
        // With a message (error 315), libqb writes it to stdout, as its last line; without one (error 314), stdout
        // holds only what the test printed.
        let message = if stderr[at].contains("check console") {
            run.stdout
                .lines()
                .map(str::trim_end)
                .rfind(|l| !l.is_empty())
                .unwrap_or("_ASSERT failed")
        } else {
            "_ASSERT failed"
        };
        return Verdict {
            location: located(at),
            ..Verdict::new(Outcome::Fail, message)
        };
    }
    if let Some(at) = stderr.iter().position(|l| l.starts_with("Runtime error:")) {
        let (message, location) = match error_location(stderr[at]) {
            Some(location) => (
                stderr[at + 1..]
                    .iter()
                    .find(|l| !l.is_empty())
                    .copied()
                    .unwrap_or("runtime error"),
                Some(location),
            ),
            None => (stderr[at].trim_start_matches("Runtime error:").trim(), located(at)),
        };
        return Verdict {
            location,
            ..Verdict::new(Outcome::Error, message)
        };
    }
    // The entry point goes on to `DONE` after `UNKNOWN`.
    if results.contains(&"UNKNOWN") {
        return Verdict::new(Outcome::Error, "unknown test: the program has no test of this name");
    }
    let done = results.contains(&"DONE");
    if done && run.exit == Some(0) {
        return Verdict::new(Outcome::Pass, "");
    }
    let code = match run.exit {
        Some(0) => String::new(),
        Some(c) => format!(" (exit code {c})"),
        None => " (killed by a signal)".to_string(),
    };
    if !results.contains(&"BEGIN") {
        return Verdict::new(
            Outcome::Error,
            format!("the test file's module-level code ended the program before the test ran{code}"),
        );
    }
    if !done {
        return Verdict::new(Outcome::Error, format!("the test ended before its SUB returned{code}"));
    }
    Verdict::new(
        Outcome::Error,
        format!("the program exited after its SUB returned{code}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run<'a>(result: &'a str, stdout: &'a str, stderr: &'a str, exit: Option<i32>) -> Run<'a> {
        Run {
            result,
            stdout,
            stderr,
            exit,
            timed_out: false,
        }
    }

    #[test]
    fn a_timeout_comes_first() {
        let r = Run {
            timed_out: true,
            ..run("BEGIN\nDONE\n", "", "", None)
        };
        assert_eq!(judge(&r).outcome, Outcome::Timeout);
    }

    #[test]
    fn a_fail_line() {
        let v = judge(&run("BEGIN\r\nFAIL\tsum: expected 4, got 5\r\n", "", "", Some(2)));
        assert_eq!(v, Verdict::new(Outcome::Fail, "sum: expected 4, got 5"));
    }

    #[test]
    fn a_failed_assert() {
        // Measured: the message on stdout, the location and `_ASSERT failed` on stderr.
        let v = judge(&run(
            "BEGIN\n",
            "some output\nmath\n",
            "\nRuntime error: Line: 9 (in a_test.bas)\n_ASSERT failed (check console for description)\n",
            Some(1),
        ));
        assert_eq!(
            v,
            Verdict {
                outcome: Outcome::Fail,
                message: "math".into(),
                location: Some(("a_test.bas".into(), 9)),
            }
        );
        // `_ASSERT` with no message: what the test printed is not taken for one.
        let v = judge(&run(
            "BEGIN\n",
            "before\n",
            "\nRuntime error: Line: 9 (in a_test.bas)\n_ASSERT failed\n",
            Some(1),
        ));
        assert_eq!(
            v,
            Verdict {
                outcome: Outcome::Fail,
                message: "_ASSERT failed".into(),
                location: Some(("a_test.bas".into(), 9)),
            }
        );
    }

    #[test]
    fn a_runtime_error() {
        let v = judge(&run(
            "BEGIN\n",
            "PASS\nok\n",
            "\nRuntime error: Line: 4 (in main module)\nSubscript out of range\n",
            Some(1),
        ));
        assert_eq!(
            v,
            Verdict {
                outcome: Outcome::Error,
                message: "Subscript out of range".into(),
                location: Some(("main module".into(), 4)),
            }
        );
        // A critical error without a line.
        let v = judge(&run("BEGIN\n", "", "\nRuntime error: Out of memory\n", Some(1)));
        assert_eq!(v, Verdict::new(Outcome::Error, "Out of memory"));
    }

    #[test]
    fn done_and_exit_code_0_pass() {
        assert_eq!(judge(&run("BEGIN\nDONE\n", "PASS", "", Some(0))).outcome, Outcome::Pass);
    }

    #[test]
    fn anything_else_is_an_error() {
        // Output that looks like a pass decides nothing.
        let v = judge(&run("BEGIN\n", "PASS\nok\n", "", Some(0)));
        assert_eq!(
            v,
            Verdict::new(Outcome::Error, "the test ended before its SUB returned")
        );
        let v = judge(&run("BEGIN\n", "", "", Some(-1073741819)));
        assert_eq!(
            v.message,
            "the test ended before its SUB returned (exit code -1073741819)"
        );
        let v = judge(&run("BEGIN\nUNKNOWN\nDONE\n", "", "", Some(0)));
        assert_eq!(v.message, "unknown test: the program has no test of this name");
        let v = judge(&run("", "", "", Some(0)));
        assert!(
            v.message.starts_with("the test file's module-level code ended"),
            "{}",
            v.message
        );
        let v = judge(&run("BEGIN\nDONE\n", "", "", Some(3)));
        assert_eq!(
            v,
            Verdict::new(
                Outcome::Error,
                "the program exited after its SUB returned (exit code 3)"
            )
        );
        assert_eq!(judge(&run("BEGIN\nDONE\n", "", "", None)).outcome, Outcome::Error);
    }
}
