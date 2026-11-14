# Fuzzing the front end

Coverage-guided fuzzing of the lexer, the parser, the checker, the code generator and the language server's calls,
with `cargo-fuzz` and libFuzzer (OpenSpec change `fuzz-front-end`; spec `testing/fuzzing`). A run is started by
hand. It is not a step of `just quick`, `just full` or `just merge`.

## The targets

The fuzz crate is `fuzz\`. It is outside the Cargo workspace, with its own `Cargo.lock` and a nightly Rust pinned
in `fuzz\rust-toolchain.toml`. Each target calls `qb64rust_driver::probe`, which the mutation test and the
regression test also call:

| Target | What runs | Fails on |
|---|---|---|
| `parse` | the lexer and the parser | a panic, or a tree that does not print back the input's exact bytes |
| `compile` | the lexer, the parser and the checker; for an accepted program of at most 32 KB, lowering and C++ generation (a debug build with coverage counters and value tracing) | a panic, or a broken round trip |
| `server` | the language server's check, format and index calls, on a document that is not a file | a panic |

`$INCLUDE` finds no file, and the program is read as compiled on Windows, on either host.

Every target also fails on an input that takes more than 10 seconds or more than 2 GB. Inputs are at most 16 KB.

## Running

```
just fuzz parse 10              # one target for 10 minutes, 2 jobs
just fuzz all 20 --jobs 4       # the three targets one after another, 20 minutes each
python tools/fuzz/fuzz.py --clean compile    # shrink the kept corpus of a target (cargo fuzz cmin)
```

On Windows, `fuzz.py` syncs this tree's Linux copy (`tools\linux\sync.sh`) and runs itself again there, in WSL.
It then:

1. writes the dictionary `fuzz\target\keywords.dict` from the compiler's tables (the ignored test
   `fuzz_regressions::write_fuzz_dictionary` of the driver);
2. builds the targets with `cargo fuzz build --sanitizer none`;
3. copies the seeds into `fuzz\target\seeds\`: the corpus programs of at most 16 KB and the regression inputs;
4. runs `cargo fuzz run` for the minutes given, with the stack the compiler gives itself (64 MB). More than one
   job runs in libFuzzer's fork mode, which stops at the first failure;
5. on a failure, runs `cargo fuzz tmin` on the input, for 5 minutes at most;
6. prints one line per target, for example `fuzz parse: 10.1 min, 190554 inputs, no failure`.

The exit status is 0 when nothing failed, 1 when a target found a failing input, and 2 when the run could not be
made. A run ends at the first failure, `all` too. The log of each run is `fuzz\target\<target>.log` in the Linux
copy (`~/qb64rust` for this checkout).

Without a sanitizer a target runs about five times as fast as with AddressSanitizer (design D4).
AddressSanitizer finds memory errors of `unsafe` code, which the workspace forbids. `--sanitizer address`
still gives that build.

The corpus each target grows is kept in `fuzz\corpus\<target>\` of the Linux copy, so a second run starts where
the first stopped. Like `fuzz\target\` and `fuzz\artifacts\`, it is git-ignored.

The `Fuzz` workflow (`.github\workflows\fuzz.yml`) runs the same script on GitHub's Linux runner, for long runs.
Start it by hand under Actions, with a target and a number of minutes. It keeps the corpus in the Actions cache and
uploads the failing inputs when a run fails.

## Where findings go

A failing input is written to `fuzz\artifacts\<target>\` and copied back to the same path in the Windows tree.
The summary line names the smallest form of it. Then:

- **A panic or a broken round trip:** fix it in its crate. Copy the input to `tests\fuzz\regressions\` in the same
  commit, named as its `README.md` says. `cargo test` runs every file there through all three targets.
- **A stack overflow on deep nesting:** a design question (a nesting limit with a diagnostic). Put it to the user.
- **A slow input (more than 10 seconds):** fix it if the cause is plainly quadratic code. Otherwise add a row to
  `SOMEDAY.md`, and keep the file out of the regressions folder until it is fixed.

## Installing in WSL

Once, in the WSL distribution `Ubuntu` that the Linux steps use (`tools\linux\README.md`):

```
rustup toolchain install nightly-2026-11-13 --profile minimal    # the channel of fuzz/rust-toolchain.toml
cargo install cargo-fuzz --locked
```

`fuzz.py` says so when `cargo-fuzz` is missing. When the channel in `fuzz\rust-toolchain.toml` changes, install
the new one the same way.

## Tests

```
python -m unittest discover -s tools/fuzz -p "test_*.py"
```

They check the commands built for each target, the summary read from libFuzzer's output, the seeds and the
failing inputs found. `check`'s `tools` step runs them.
