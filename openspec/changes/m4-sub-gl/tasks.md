# Tasks

Standing rules: CLAUDE.md. What this change does differently: the recordings with `qb64pe.exe` (the family `v59`)
and the reading of the old compiler's C++ (`-z`) are batched in group 1, before the code; they may change the delta
specs and the design before anything is built. A decision the user takes on the way goes into `DECISIONS.md` in the
task where it is taken.

## 1. Measurements and records

- [x] 1.1 The family `verification\v59_*` of design D1, recorded with `qb64pe.exe` (the verification suite), with
  one line in `verification\README.md`. First of all, `v59_g`: whether OpenGL gets a context on the hidden
  desktop, and which (`_glGetString(_GL_RENDERER)` printed once, then masked or dropped); if it gets none, the
  fallback of the design's first risk is applied and the design says so. Where a recording differs from a delta
  spec or the design, the spec or design is changed here. Verify: every program has its recorded output or error,
  two recordings equal; `tools\repo_check\check_repo.py --untracked` finds nothing in them;
  `openspec validate m4-sub-gl --strict` passes after any edit.
- [x] 1.2 The old compiler's C++ (`qb64pe.exe -z`) of `opengl/test`, of a `SUB _GL` with a body, and of calls of a
  routine with a helper, one without (`_glFlush`), `_glGetString`, a FUNCTION's result in an expression,
  `_gluPerspective` and a constant: the SUB's name and place in `regsf.txt` and the main file, the casts, the
  string wrap. Design D4 names the lines it rests on. Verify: D4 names them.

## 2. The OpenGL table

- [x] 2.1 `tools\opengl\gl_table.py` (design D2, D5): writes `crates\sema\src\gl_table.rs` and the call-site
  programs `tests\callsite\gl_*.bas` from `libqb\`'s `gl.h`, with `--check`; its tests
  `tools\opengl\test_gl_table.py` (the old compiler's rules for a `#define` naming an earlier one, a pointer, a
  pointer to a pointer, a `const` return, `glGetString`); a `README.md`. `CLAUDE.md`'s layout table gains its row,
  and `.github\workflows\rust.yml` runs its tests beside the other tools' tests. Verify: its tests pass; `--check`
  passes on the written files.
- [x] 2.2 The tier 1 test of the table against `libqb\` (design D2): the counts and every helper of
  `gl_helper_code.h`; and the test that every routine has a call-site program (delta spec
  `testing/compiler-tests`). Verify: both pass, and each fails naming the routine when a row or a program is taken
  out (tried once each, put back).

## 3. The compiler

- [x] 3.1 `sema` (design D3): `SUB _GL` accepted in `reserved_proc`, `RuntimePart::Gl` (the checker's `has_gl()`); the
  rejected forms of `v59_a`/`x…` as the old compiler's errors; "Cannot call SUB _GL directly"; the OpenGL names
  looked up in `gl_table` only with the SUB, each routine an external `Proc` of `LibraryKind::OpenGl` made when
  first named; the conversions of its arguments. Front-end tests: `typed` (a FUNCTION's result type, an argument's
  conversion, a constant's type), `ir`, `check-fail` (each rejected form). Verify: `cargo test`; each scenario of
  `language/opengl` that tier 1 can see has its test.
- [x] 3.2 `codegen-cpp` and `driver` (design D4): `SUB__GL`, the calls through `library.rs` with no prototype and
  no header, `DEP_GL=y` first in `make_flags` (its unit test beside the others). Verify: `cargo test`; the
  `cpp` front-end test of a `SUB _GL` program; `opengl/test` builds; the call-site check of `--glob "gl_*"` finds
  every program equal.
- [x] 3.3 The examples of "not supported yet": `crates\driver\tests\cli.rs` `unknown_statement` and any front-end
  test or snapshot that used `SUB _GL` take a form left marked (`FOR` over an element of a member array, as
  `MEMBER_FOR`); `SOMEDAY.md`'s row on statements with a leading underscore loses the OpenGL statements. Verify:
  `cargo test`; `grep` finds no "not supported yet" example naming `SUB _GL`.

## 4. Run checks

- [x] 4.1 The `v59` programs of design D6 run with the new compiler and pass (`--suite verification --glob
  "v59_*"`); those that pass join `tests\verification\pass.list`, the rest go to `known.list` with a reason.
  `opengl/test` joins `tests\upstream\pass.list`. Verify: the runs pass; the tier 1 tests of the lists pass.
- [x] 4.2 `crates\base\src\old_wording.tsv` regenerated (`tools\diagnostics\old_wording.py`), `--check` passes, so
  that each rejected form's note is the old compiler's message. Verify: `--check` passes.

## 5. Lists and closing

- [x] 5.1 `SOMEDAY.md`'s "QB64pe behaviours to review": a row for each behaviour kept QB64pe's way that `v59`
  shows (for example the OpenGL names vanishing without `SUB _GL`, an `END` in `SUB _GL` as error 271). Verify:
  each row names its `v59` program.
- [x] 5.2 The closing runs: `python tools\close\close.py --plan`, each command run by its own, then
  `tools\close\close.py --check` passes. Then the counts of the test READMEs by `tools\legacy_tests\counts.py`, and
  `STATUS.md`'s "Next" names what is left of M4 (the 9 marked programs and step 6, the coverage review). Then
  `python tools\testtime\test_time.py`. Verify: `--check` passes; the upstream demand report marks nothing.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
