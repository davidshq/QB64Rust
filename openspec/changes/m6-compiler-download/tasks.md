# Tasks

Standing rules: CLAUDE.md. What this change does differently: one zip is built, unpacked under `target\` and used
from outside the repository by hand (task 4.3). No real release is made (the user pushes the next tag). No
recording with `qb64pe.exe`.

## 1. Records

- [ ] 1.1 `DECISIONS.md`: a row for the user's choice of 2026-10-26 (the compiler download goes on the
  `vscode-v<version>` release beside the `.vsix`, from the same build; the workspace version equals
  `package.json`'s, checked at the tag). It names `openspec\changes\m6-compiler-download\design.md`. Verify: the
  row is there.

## 2. The driver

- [ ] 2.1 `build.rs`: `find_libqb_in(option, env, exe_dir, repo)` and `find_libqb` over it (design D3: the
  option, the variable, `<exe dir>\libqb`, the repository's; the "not found" message names the places).
  `refresh_objects` maps a failed write of `objects.stamp` to the message of D3. Unit tests of D10 (the order, a
  folder without `qbx.cpp`, none found, `objects.stamp` made a folder). Update the "Locating the reference runtime"
  CLI test whose message changes, if any. Verify: `cargo test -p qb64rust-driver build` passes, and tier 1 passes.
- [ ] 2.2 `main.rs`: `--version`, `--help`, `-h` as the only argument (design D4); `USAGE` gains
  `qb64rust --version | --help`. CLI tests in `crates\driver\tests\cli.rs` for the three scenarios of spec
  `compiler/cli` "Version and help", the version from `env!("CARGO_PKG_VERSION")`. Verify: those tests pass.
- [ ] 2.3 `Cargo.toml`: workspace version `0.2.0`; `cargo build` updates `Cargo.lock`. Verify: `qb64rust
  --version` prints `qb64rust 0.2.0`, and tier 1 passes (including the language server's tests, which see the new
  `serverInfo` version).

## 3. Packaging scripts

- [ ] 3.1 `tools\licenses\check.py`: layouts and `--zip` (design D7); `--vsix` unchanged. Tests in
  `tools\licenses\test_check.py`: a zip with the notices under `<top>/` passes; one with a notice missing or
  changed fails, naming it. Verify: `python -m unittest discover -s tools/licenses -p "test_*.py"` passes, and
  `python tools/licenses/check.py` still passes on the repository.
- [ ] 3.2 `vscode\scripts\check_vsix.py` `check_tag`: also compares the workspace version of `Cargo.toml`
  (design D5); `--notes-out` appends the paragraph of D8. Cases in `test_check_vsix.py` (versions equal; Cargo
  differs, both numbers named; the notes end with the paragraph). Verify: `python -m unittest discover -s
  vscode/scripts -p "test_*.py"` passes.
- [ ] 3.3 `MANUAL.md` at the repository's root (design D9), the sections of spec `compiler/download` "The manual",
  with no full local path. Verify: read against `USAGE` and spec `compiler/toolchain` (sizes, folder, variables).
- [ ] 3.4 `vscode\scripts\package_zip.py` (`--out`, `--check`; design D6) and `test_package_zip.py` (design D10:
  the layout from a fake bundle, the required files, a missing notice, a forbidden `.o`, a top folder that does
  not match the name, and every option `MANUAL.md` names in code spans found in `USAGE` read from `main.rs`).
  Verify: the Python tests of `vscode\scripts` pass.

## 4. The release

- [ ] 4.1 `.github\workflows\release-vsix.yml` (design D8): the zip made and checked after the `.vsix` check,
  both files in one `gh release create`, the title `QB64Rust $tag`; the header comment names the two version files
  to raise for a release and the zip. Verify: the workflow file parses (`python -c` over `yaml.safe_load` in a
  script under `target\`, or `actionlint` if present), and each command it runs has been run by hand in 4.3.
- [ ] 4.2 `vscode\CHANGELOG.md` under 0.3.0: the compiler download, `--version` and `--help`. Verify: the entry
  names the zip's file name as `package_zip.py` writes it.
- [ ] 4.3 By hand, once (design D10): `npm run package` in `vscode`, `python scripts/package_zip.py --out
  ..\target\m6-zip` and `--check` on the result; unpack it into `target\m6-zip\unpacked`; in a working directory
  outside the repository (a folder under the system temp folder), with `QB64RUST_LIBQB_ROOT` and
  `QB64RUST_QB64PE_ROOT` empty and `QB64RUST_TOOLCHAIN_ROOT` the clone's `internal\c\c_compiler`, run the unpacked
  `qb64rust.exe --version`, then `-x hello.bas -o hello.exe`, then `hello.exe`. Record in this task, with
  repo-relative paths only: the zip's size and file count, the version line, that `hello.exe` printed its line,
  and that `objects.stamp` and the objects are in the unpacked `libqb`. Also run it with
  `QB64RUST_TOOLCHAIN_ROOT` empty and `QB64RUST_TOOLCHAIN_HOME` an empty folder, which gives the "No toolchain found"
  message. Verify: recorded here.

## 5. Lists and closing

- [ ] 5.1 Specs: `openspec validate m6-compiler-download --strict` passes. `LAYOUT.md` has rows for `MANUAL.md`
  and `vscode\scripts\package_zip.py`. `crates\README.md`'s build instructions say where a release binary finds
  libqb, in one line. Verify: all three.
- [ ] 5.2 The closing runs: `python tools\close\close.py --plan`, each command run by its own, then `--check`
  passes. Then the counts by `tools\legacy_tests\counts.py`, `STATUS.md`'s "Next" and "Waiting on the user"
  item 1 (the compiler as its own download is built), and `python tools\testtime\test_time.py`. Verify: `--check`
  passes.

## Workflow follow-up

- Archive the change once the user has reviewed it (`/opsx:archive`).
- The first real release with the zip is the user's next `vscode-v*` tag. Before tagging, raise `package.json` and
  `Cargo.toml` together.
- Tell the user which files to stage and suggest a commit message (nothing is staged by Claude).
