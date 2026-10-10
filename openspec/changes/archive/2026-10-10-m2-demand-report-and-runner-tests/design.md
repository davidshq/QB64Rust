# Design

## D1. The report counts messages as they are

A row of the report is one "not supported yet" message, byte for byte. Most messages name a construct (`` `COLOR` ``,
`` `REDIM` ``, "arrays as `TYPE` members"); a few carry a name from the program (`` `hit(...)` (an array used
without `DIM`, or no FUNCTION of this name) ``) and so make one row per name. They are left as they are: the rows
that matter are at the top, and a rule that folds names would have to know every message.

Three numbers per row, over the set:

- **programs**: how many programs have the message;
- **alone**: in how many of them it is the only message and there is no other error, so bringing it in makes them
  compile;
- **uses**: how often it occurs.

Rows are ordered by programs, then alone, then uses, then message. A program with an `.err` file beside it is left
out and counted in a line of its own: the old compiler rejects it, so what it uses is no demand. A program with an
error that is not marked is counted in the summary and named after the table, since the usual cause is an included
file the given compiler root does not hold (upstream's `qb64pe\` folder includes the old compiler's sources: give
the clone's `tests\compile_tests` and no `--include-root` is needed for them, or expect them listed).

The exit status is 0 whatever the programs hold: the report is no verdict, and no test compares its numbers (they
change with every feature).

## D2. Lifting the cap: one switch for the process

A compile run stops at 100 errors (`base::MAX_ERRORS`), which is what a person wants and what cut the earlier hand
counts short (Civil War Strategy has 1,740 marked errors in 38 messages; a compile run shows the first 100). The parser, `sema` and the merge of their lists each make
a `Diagnostics`; passing a limit through `parse` and `check_with` would change those signatures for one tool.
Instead `base::lift_error_cap()` sets a process-wide atomic that `Diagnostics::push` reads. Only `qb64rust --report`
calls it. It is never called in a test process: other tests of that process rely on the cap. The CLI test runs the
binary, so it is its own process.

With the cap lifted, `sema`'s "past the parser's cap" rule (`Skips::cap_start`) never applies, which is right: the
parser recorded every error.

## D3. The runner's tests: a stand-in for `run_proc`

Every process the runner starts goes through `run_proc`. `corpus_test` is tested with `run_proc` replaced by a
stand-in that plays the compiler (a command with `-x`: writes the compile output, creates the executable or not,
returns an exit code) and the program (writes the run's output, returns an exit code or a timeout). That reaches
every outcome of one corpus program (equal and different output, each kind of `.err`, `.norun`, `.stdin`,
`.normalize`, a known failure, `--record` with equal runs, differing runs, a rejection, and the new compiler's
rule for what may be recorded) in milliseconds and without a compiler, a clone or a C++ build, so it runs in CI's
`test` job.

A real fake compiler (an executable that writes an executable) was considered: on Windows the "program" would have
to be a real PE file and pass through `press_any_key.py`'s console, which tests the helper more than the rules.

`unittest`, not `pytest`: nothing to install, locally or in CI.

Checked once by hand that the tests bite: 15 seeded mistakes in the runner (a bare CR ignored, a marked-only
failure counted as a rejection, differing runs recorded, the output never compared, and so on), each caught.
