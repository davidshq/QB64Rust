# Tasks

Standing rules: `CLAUDE.md`. Different here: the fuzz runs go in WSL on the host with `--jobs 2`, or in the
`Fuzz` workflow. They are not runs of `check`.

## 1. Measure first

- [x] 1.1 In WSL, install a nightly Rust and `cargo-fuzz`, make `fuzz\` with `cargo fuzz init`, and build one
      empty target that calls `qb64rust_driver::frontend`. Verify: `cargo fuzz build` succeeds. Record the
      nightly's date and the build time in `design.md` under D1. If it does not build, stop and put D1's
      alternatives to the user.
- [x] 1.2 Run that target for 2 minutes with `ulimit -s` set to `STACK_SIZE` in KB. Verify: the run ends by
      itself, and an input of 5,000 nested parentheses given by hand either passes or overflows the same way
      `qb64rust -z` does on it. Record inputs per second and the outcome in `design.md` under D4.

## 2. The shared properties and the regression test

- [x] 2.1 Add `Probe`, `Reached` and `probe` to `crates\driver\src\lib.rs` (design D3). Change `check` in
      `crates\driver\tests\it\mutate.rs` to call `probe(Probe::Compile, …)`. Verify: `cargo test -p
      qb64rust-driver --test it mutate::` passes and reports the same share of mutants reaching C++
      generation as before (about 43%).
- [x] 2.2 Add `tests\fuzz\regressions\` with a `README.md` (what a file is, how it is named, how to add one)
      and the `-text` line in `.gitattributes`. Add `crates\driver\tests\it\fuzz_regressions.rs` (design D6).
      Verify: the test passes on the empty folder, and fails naming the file when a file is added for which a
      temporary `panic!` in `probe` fires; remove the temporary panic.
- [x] 2.3 Write `fuzz\fuzz_targets\parse.rs`, `compile.rs` and `server.rs`, `fuzz\Cargo.toml` with its own
      `[workspace]`, and `fuzz\rust-toolchain.toml`. Add `fuzz\target`, `fuzz\corpus` and `fuzz\artifacts` to
      `.gitignore`. Verify: `cargo fuzz build` builds all three in WSL, and `cargo build` in the workspace on
      Windows is unchanged.
- [x] 2.4 Run `cargo deny check` on `fuzz\Cargo.lock` in the existing deny job of
      `.github\workflows\rust.yml`. Verify: `cargo deny --manifest-path fuzz\Cargo.toml check` passes
      locally; if a license of `libfuzzer-sys`'s dependencies is not on the allow list, stop and ask.

## 3. The run command

- [x] 3.1 Write `tools\fuzz\fuzz.py` (design D5): the WSL hand-over, the seeds, the flags, `tmin` on a
      failure, the summary line, `--clean`. Count the corpus programs of at most 16 KB and record the number
      in `design.md` under D5. Verify: its unit tests in `tools\fuzz\test_fuzz.py` cover the argument list
      built for each target and the summary read from sample libFuzzer output; add them to the tool tests
      that `check` runs.
- [x] 3.2 Generate the keyword dictionary from the compiler's tables (design D5, step 1). Verify: the file
      holds `PRINT`, `_RGB32` and `$INCLUDE` in libFuzzer's dictionary format, and a run accepts it without
      a warning.
- [x] 3.3 Add the `fuzz` recipe to `justfile` and `tools\fuzz\README.md` (how to install the nightly and
      `cargo-fuzz` in WSL, how to run, where findings go). Add rows for `fuzz\`, `tools\fuzz\` and
      `tests\fuzz\` to `docs\LAYOUT.md`, and the command to the test commands of `crates\README.md`. Verify:
      `just fuzz parse 1` run from Windows ends after a minute with the summary line and exit 0, or with a
      finding.
- [x] 3.4 Add `.github\workflows\fuzz.yml` (design D7). Verify: `actionlint` or the repository's workflow
      check passes; the first real run is the user's to start, and its result is noted in task 4.3.
      Done (2026-11-12): there is no workflow check in the repository, and `actionlint` was not run (not
      installed; fetching it was refused). The file parses as YAML with its three inputs and nine steps; the
      user took that as enough, with the first real run as the test.

## 4. The first campaign

- [x] 4.1 Run each target for 30 minutes (`--jobs 2`, one after another, in the background). Verify: three
      summary lines. Record in `design.md`: inputs tried, findings, and libFuzzer's coverage of the `sema`
      and `codegen-cpp` crates for `compile`.
- [x] 4.2 Handle each finding by design D8: fix it in its crate with its file in `tests\fuzz\regressions\`,
      or write the `SOMEDAY.md` row, or stop and ask (a nesting limit; more than about 20 findings). Verify:
      `cargo test -p` of each touched crate and the regression test pass, and each fixed input run alone
      through its fuzz target no longer fails.
- [ ] 4.3 Run each target again for 10 minutes on the fixed tree. Verify: no finding, or the new ones are
      handled as in 4.2. Note the result of the user's first `Fuzz` workflow run here when there is one.

## 5. Records and closing

- [ ] 5.1 Add the `DECISIONS.md` row (fuzzing is by `cargo-fuzz` in WSL and a hand-started workflow, findings
      are files under `tests\fuzz\regressions\`). Remove the `SOMEDAY.md` row "Fuzz the lexer and parser" and
      add the row "formatting twice gives the same text, as a fuzz property". Update `STATUS.md`'s "Next".
      Verify: `python tools\repo_check\check_repo.py` passes.
- [ ] 5.2 Run `just quick` in the background and read its log. Verify: it passes, and the report says what
      ran, what stood and why.

## Workflow follow-up

- Sync the delta spec into `openspec\specs\testing\fuzzing\` and archive the change after the user's review.
