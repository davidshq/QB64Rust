# Spec Delta

## MODIFIED Requirements

### Requirement: The list of built-ins no test runs
`tests\never_run.list` SHALL hold each built-in that no test runs, one per line as `<name>\t<reason>[ <note>]`,
with a reason from a fixed set: the desktop, the clipboard or a console the program shares; a printer; the
machine's keyboard state or bell; a person must answer; another compiler; run only in the sandbox family
(`sandbox`). `#` SHALL start a comment line. A built-in listed as `sandbox` is held by the sandbox family
(spec `testing/desktop-sandbox`) and is, for the suites run on the host, a built-in no test runs.

#### Scenario: A reason outside the set
- **WHEN** a line of the list gives a reason that is not one of the set
- **THEN** the check fails, naming the line of the list

#### Scenario: A built-in run in the sandbox
- **WHEN** `_SCREENMOVE` is listed as `sandbox` and no program of the corpus, upstream or verification suites runs it
- **THEN** `builtin_runs.py --check` on those suites' counts passes for `_SCREENMOVE`
