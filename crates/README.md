# The QB64Rust compiler crates

The new compiler, `qb64rust`. One crate per pipeline stage, so the layering is enforced by the dependency graph
(design of the OpenSpec change `m2-workspace-and-slice`, D1). Dependencies point only downward.

| Crate | Package | What it holds |
|---|---|---|
| `base` | `qb64rust-base` | `FileId`, byte `Span`, `SourceMap` with the line index (CR LF, LF, lone CR), `Diagnostic`, the 100-error cap |
| `syntax` | `qb64rust-syntax` | Byte lexer, lossless tree (green nodes + cursor; printing it gives back the file byte for byte), parser with one module per statement family |
| `builtins` | `qb64rust-builtins` | The built-in table, generated at build time from `tools\builtins\builtins.json` |
| `sema` | `qb64rust-sema` | Variables (a name plus a type), literal typing, computation types, explicit conversions, integer constant folding; the typed tree |
| `ir` | `qb64rust-ir` | The ABI-neutral IR (no libqb names, no C types) and its lowering from the typed tree |
| `codegen-cpp` | `qb64rust-codegen-cpp` | IR to the fragments `qbx.cpp` includes (`global.txt`, `main0.txt`, ...) |
| `driver` | `qb64rust-driver` | The `qb64rust` binary: command line, pipeline, build through the reference clone's `Makefile` |

```
driver -> codegen-cpp -> ir -> sema -> syntax -> base
                     \      \--> builtins <--/
```

Source is bytes throughout (`study\15` §1): files are never converted to `str`; positions are byte offsets;
columns in diagnostics are byte columns.

## Lints

`cargo clippy --workspace --all-targets -- -D warnings` must be clean (CI runs it). Beyond the defaults
(`study\21`):

- `sema`, `ir`, `codegen-cpp`: no `_ =>` arm on an enum (`wildcard_enum_match_arm`). List the variants; where a
  default is right, say why in `#[expect(clippy::wildcard_enum_match_arm, reason = "...")]`. The lint sees only
  `match`: write a type predicate or a per-type choice as a `match`, not as `==` or `if … else`.
- `clippy.toml` disallows converting bytes to `str` (`from_utf8`, `from_utf8_lossy`) and reading text
  (`read_to_string`, `read_line`, `BufRead::lines`). Show source bytes with `show_bytes`.
- Casts that can lose or reinterpret bits are linted. Lengths, offsets and ids use `base::to_u32` (source files are
  at most `MAX_SOURCE_LEN`); a deliberate wrap keeps its `as` with `#[expect]` and the reason.
- Release builds keep overflow checks: arithmetic BASIC defines as wrapping is written `wrapping_*`.

## Building and running

```
cargo build --release
target\release\qb64rust.exe -x prog.bas -o prog.exe        # compile (needs ..\QB64pe, see below)
target\release\qb64rust.exe -z prog.bas                    # write the C++ fragments only, print their folder
target\release\qb64rust.exe --dump typed prog.bas          # tokens | tree | typed | ir | cpp
```

Building an executable uses the QB64pe reference clone for `qbx.cpp`, libqb and the toolchain: `--qb64pe-root
<dir>`, else the environment variable `QB64RUST_QB64PE_ROOT`, else `..\QB64pe` next to this repository. The
fragments, the copy of `qbx.cpp`, `qbx.o` and the `.sym` file go into `<exe>.qb64rust\` next to the executable
(deleted after a successful build unless `--keep-build`); libqb objects are built into the clone's git-ignored
folders if missing. No tracked file of the clone changes.

What the compiler supports so far (the first slice): `$CONSOLE:ONLY`, comments, `:`, `DIM` of scalars (`INTEGER`,
`LONG`, `_INTEGER64`, `SINGLE`, `DOUBLE`, `_FLOAT`, `STRING`), `[LET] v = e`, implicit variables with suffixes,
`PRINT` with `;`, `,` and the auto-semicolon, `END`, numeric and string literals, unary `-`, `+ - * /`,
parentheses, string `+`, and `INSTR`. Anything else gets a "not supported yet" error, never wrong code.

## Tests

| Tier (`study\19`) | Command | What |
|---|---|---|
| 1 | `cargo test` | Unit tests; lexer and parser snapshots; `tests\frontend\` by mode line; every corpus program through the front end (no panic, exact round trip); the programs of `tests\corpus\slice.list` without diagnostics; the command line |
| 1, by hand | `cargo test -p qb64rust-driver --test cli -- --ignored` | The command-line scenarios that build an executable |
| 2 | `python tools\legacy_tests\run_legacy_tests.py --suite corpus --qb64 target\release\qb64rust.exe --list tests\corpus\slice.list` | The listed corpus programs end to end against the output recorded from `qb64pe.exe` |

`tests\frontend\*.bas` start with a mode line, `' TEST: <mode>`:

| Mode | Checks |
|---|---|
| `parse-ok` | no syntax errors; the tree prints back to the file |
| `check-ok` | no errors after `sema` |
| `check-fail` | at least one error; the diagnostics are a snapshot |
| `typed`, `ir`, `cpp` | no errors; the typed tree, the IR or the C++ fragments are a snapshot |

The mode line is a comment, so `qb64pe.exe` ignores it.

### Snapshots

Snapshot tests use [`insta`](https://insta.rs). The `.snap` files are committed next to the tests
(`crates\syntax\tests\snapshots\`, `crates\driver\tests\snapshots\`, the latter named `<file>.<mode>.snap`). A
changed output fails `cargo test` and writes a `.snap.new` file. Review with `cargo insta review` (install with
`cargo install cargo-insta`) or read the `.snap.new` file and rename it over the `.snap` file to accept it. To
write all new snapshots in one run: `INSTA_FORCE_PASS=1 INSTA_UPDATE=new cargo test`, then review every
`.snap.new`.

`QB64RUST_NO_FOLD=1` turns integer constant folding off; it is for checking that folding changes no result (run
tier 2 with it set), not for normal use.
