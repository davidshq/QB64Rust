# Fuzz regression inputs

Each file here is an input that a fuzz run once failed on (OpenSpec change `fuzz-front-end`, design D6; spec
`testing/fuzzing`). The test `fuzz_regressions` of the `driver` crate runs every file through the properties of
all three fuzz targets (`parse`, `compile`, `server`), so `cargo test` fails again if the bug comes back. It
runs on Windows and Linux and needs no fuzzer.

**A file** holds the failing input's bytes unchanged, as `cargo fuzz tmin` left it. It may hold control
characters, NUL bytes or invalid UTF-8; `.gitattributes` keeps git from changing its line ends.

**Its name** is `<target>-<what it broke>.bas`: the target that found it, then a few words in lower case with
hyphens. For example `parse-unclosed-paren-in-data.bas`.

**To add one:**

1. Copy the minimized input that `just fuzz` printed (under `fuzz\artifacts\<target>\`) here under its new name.
2. Run `cargo test -p qb64rust-driver --test it fuzz_regressions` and see it fail.
3. Fix the bug in its crate, in the same commit as the file, and see the test pass.
4. Read the file before staging it: it must hold nothing private (`CLAUDE.md` rule 2), which
   `tools\repo_check\check_repo.py` also checks.

An input that failed by taking more than 10 seconds is added only once it is fixed. The test only requires that
it returns; check the fix with `cargo fuzz run <target> <file>` in WSL and say so in the commit message.
