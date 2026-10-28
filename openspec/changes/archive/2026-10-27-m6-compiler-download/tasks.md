# Tasks

Standing rules: CLAUDE.md. What this change does differently: one zip is built, unpacked under `target\` and used
from outside the repository by hand (task 4.3). No real release is made (the user pushes the next tag). No
recording with `qb64pe.exe`.

## 1. Records

- [x] 1.1 `DECISIONS.md`: a row for the user's choice of 2026-10-26 (the compiler download goes on the
  `vscode-v<version>` release beside the `.vsix`, from the same build; the workspace version equals
  `package.json`'s, checked at the tag). It names `openspec\changes\m6-compiler-download\design.md`. Verify: the
  row is there.

## 2. The driver

- [x] 2.1 `build.rs`: `find_libqb_in(option, env, exe_dir, repo)` and `find_libqb` over it (design D3: the
  option, the variable, `<exe dir>\libqb`, the repository's; the "not found" message names the places).
  `refresh_objects` maps a failed write of `objects.stamp` to the message of D3. Unit tests of D10 (the order, a
  folder without `qbx.cpp`, none found, `objects.stamp` made a folder). Update the "Locating the reference runtime"
  CLI test whose message changes, if any. Verify: `cargo test -p qb64rust-driver build` passes, and tier 1 passes.
- [x] 2.2 `main.rs`: `--version`, `--help`, `-h` as the only argument (design D4); `USAGE` gains
  `qb64rust --version | --help`. CLI tests in `crates\driver\tests\cli.rs` for the three scenarios of spec
  `compiler/cli` "Version and help", the version from `env!("CARGO_PKG_VERSION")`. Verify: those tests pass.
- [x] 2.3 `Cargo.toml`: workspace version `0.2.0`; `cargo build` updates `Cargo.lock`. Verify: `qb64rust
  --version` prints `qb64rust 0.2.0`, and tier 1 passes (including the language server's tests, which see the new
  `serverInfo` version).

## 3. Packaging scripts

- [x] 3.1 `tools\licenses\check.py`: layouts and `--zip` (design D7); `--vsix` unchanged. Tests in
  `tools\licenses\test_check.py`: a zip with the notices under `<top>/` passes; one with a notice missing or
  changed fails, naming it. Verify: `python -m unittest discover -s tools/licenses -p "test_*.py"` passes, and
  `python tools/licenses/check.py` still passes on the repository.
- [x] 3.2 `vscode\scripts\check_vsix.py` `check_tag`: also compares the workspace version of `Cargo.toml`
  (design D5); `--notes-out` appends the paragraph of D8. Cases in `test_check_vsix.py` (versions equal; Cargo
  differs, both numbers named; the notes end with the paragraph). Verify: `python -m unittest discover -s
  vscode/scripts -p "test_*.py"` passes.
- [x] 3.3 `MANUAL.md` at the repository's root (design D9), the sections of spec `compiler/download` "The manual",
  with no full local path. Verify: read against `USAGE` and spec `compiler/toolchain` (sizes, folder, variables).
- [x] 3.4 `vscode\scripts\package_zip.py` (`--out`, `--check`; design D6) and `test_package_zip.py` (design D10:
  the layout from a fake bundle, the required files, a missing notice, a forbidden `.o`, a top folder that does
  not match the name, and every option `MANUAL.md` names in code spans found in `USAGE` read from `main.rs`).
  Verify: the Python tests of `vscode\scripts` pass.

## 4. The release

- [x] 4.1 `.github\workflows\release-vsix.yml` (design D8): the zip made and checked after the `.vsix` check,
  both files in one `gh release create`, the title `QB64Rust $tag`; the header comment names the two version files
  to raise for a release and the zip. Verify: the workflow file parses (`python -c` over `yaml.safe_load` in a
  script under `target\`, or `actionlint` if present), and each command it runs has been run by hand in 4.3.
- [x] 4.2 `vscode\CHANGELOG.md` under 0.3.0: the compiler download, `--version` and `--help`. Verify: the entry
  names the zip's file name as `package_zip.py` writes it.
- [x] 4.3 By hand, once (design D10): `npm run package` in `vscode`, `python scripts/package_zip.py --out
  ..\target\m6-zip` and `--check` on the result; unpack it into `target\m6-zip\unpacked`; in a working directory
  outside the repository (a folder under the system temp folder), with `QB64RUST_LIBQB_ROOT` and
  `QB64RUST_QB64PE_ROOT` empty and `QB64RUST_TOOLCHAIN_ROOT` the clone's `internal\c\c_compiler`, run the unpacked
  `qb64rust.exe --version`, then `-x hello.bas -o hello.exe`, then `hello.exe`. Record in this task, with
  repo-relative paths only: the zip's size and file count, the version line, that `hello.exe` printed its line,
  and that `objects.stamp` and the objects are in the unpacked `libqb`. Also run it with
  `QB64RUST_TOOLCHAIN_ROOT` empty and `QB64RUST_TOOLCHAIN_HOME` an empty folder, which gives the "No toolchain found"
  message. Verify: recorded here.

  **Done (2026-10-27):** release build of `qb64rust`; `npm run package` packed `qb64rust-win32-x64-0.2.0.vsix`
  (1522 files; `check_vsix.py --tag vscode-v0.2.0` ok, licences ok). `package_zip.py --out target\m6-zip` wrote
  `qb64rust-win-x64-0.2.0.zip`, 8,555,230 bytes, 1237 files; `--check` ok, licences ok. Its `qb64rust.exe` has
  the same hash as the bundle's, so as the `.vsix`'s. Unpacked to `target\m6-zip\unpacked\qb64rust-0.2.0`, run
  from a folder under the system temp folder with the variables as above: `--version` printed `qb64rust 0.2.0`
  (exit 0); `-x hello.bas -o hello.exe` built in 21 s (exit 0), libqb's objects included; `hello.exe` printed
  `hello from the download` (it then waited at libqb's "press any key" prompt, as any console program without
  `SYSTEM` does, and was stopped). The unpacked `libqb` then held `objects.stamp` and 75 `.o`/`.a` files; the
  folder is 36 MB with them (the manual's disk figure corrected to about 500 MB with the toolchain); the
  repository's `libqb\objects.stamp` was not touched. With `QB64RUST_TOOLCHAIN_ROOT` empty and
  `QB64RUST_TOOLCHAIN_HOME` an empty folder: exit 1, "no C++ toolchain found: install it with `qb64rust toolchain
  install` (… a 190 MB download, about 460 MB on disk), or give a toolchain folder with --toolchain-root <dir> or
  the environment variable QB64RUST_TOOLCHAIN_ROOT". The workflow's new step and the release step's `gh` call were
  not run (no release is made); their two `package_zip.py` commands are the ones run here. The `.vsix` of the
  0.2.0 release built before this change is kept as `target\m6-zip\before-qb64rust-win32-x64-0.2.0.vsix`.

## 5. Lists and closing

- [x] 5.1 Specs: `openspec validate m6-compiler-download --strict` passes. `LAYOUT.md` has rows for `MANUAL.md`
  and `vscode\scripts\package_zip.py`. `crates\README.md`'s build instructions say where a release binary finds
  libqb, in one line. Verify: all three.
- [x] 5.2 The closing runs: `python tools\close\close.py --plan`, each command run by its own, then `--check`
  passes. Then the counts by `tools\legacy_tests\counts.py`, `STATUS.md`'s "Next" and "Waiting on the user"
  item 1 (the compiler as its own download is built), and `python tools\testtime\test_time.py`. Verify: `--check`
  passes.

## 6. After review

- [x] 6.1 The copy's own objects lock (review finding 1, the pragmatic engineer's recommendation; design D3
  "Added at review"): `lock_objects` takes `<libqb>\objects.lock` when `QB64RUST_OBJECTS_LOCK` is not set, and goes
  on without a lock when that file cannot be opened; `.gitignore` gains `/libqb/objects.lock`; unit test
  `the_copys_own_objects_lock`. Rerun for what it can affect: tier 1 (645 pass), fmt, clippy, the CLI tests that
  build (80 pass; "a build changes nothing git sees" among them), and a fresh zip (`--check` ok) unpacked under
  `target\m6-zip\unpacked` with two first builds started at once without the variable: both built (exit 0), both
  programs ran, `objects.lock` and `objects.stamp` in the unpacked `libqb`. Not rerun: the suites, the call-site
  check, the coverage check and the format runs. The runner and the CLI tests set `QB64RUST_OBJECTS_LOCK`, so they
  never take the new branch, and nothing a program compiles to changed. The finding of the divergences file's case
  (`QB64pe-` in the index, `QB64PE-` in the references) is the user's; findings 3 and 4 are left as they are.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- The first real release with the zip is the user's next `vscode-v*` tag. Before tagging, raise `package.json` and
  `Cargo.toml` together.
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
