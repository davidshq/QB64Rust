# Tasks

Standing rules: `CLAUDE.md`. This change touches no compiler crate: the closing runs are the extension's and tier
1. No program's executable changes, so the slice, upstream and call-site runs are skipped by the pass cache, and
`close.py --check` names them as of the earlier build.

## 1. Paperwork first

- [x] 1.1 `DECISIONS.md` row: the `.vsix` is a `win32-x64` package carrying `qb64rust.exe` and the copy of libqb;
  Build and Run go through `qb64rust` under `qb64rust.buildWith: auto`; releases are GitHub release assets made
  from a tag `vscode-v<version>` that the user pushes (user, 2026-10-19). Checked by reading it.

## 2. Staging and packaging (design D1, D7)

- [x] 2.1 `vscode\scripts\stage_bundle.py`: `bundle\bin\qb64rust.exe` (`--binary`, default
  `target\release\qb64rust.exe`), `bundle\libqb\` from `git ls-files libqb`, `vscode\LICENSE` (where `vsce`
  looks; the repository's root `LICENSE`, MIT, added by this change on the user's word), and
  `bundle\THIRD-PARTY-NOTICES.txt` from `cargo tree` and `cargo metadata` (normal dependencies of
  `qb64rust-driver` for the Windows target, each package's licence files; a package without one takes
  `vscode\scripts\licenses\<name>.txt`, else fails, naming it). `vscode\bundle\` and `vscode\LICENSE` are ignored by
  git. Tests in `vscode\scripts\test_stage_bundle.py` (a temporary repo-shaped folder: the tracked-only
  copy leaves a stray `.o` out, a missing binary fails, a crate without a licence file fails). Checked by
  `python -m unittest` on the file, and by a real staging run whose notice names `astro-float-num`.
- [x] 2.2 `package.json`: version `0.2.0`; `package` stages and runs `vsce package --target win32-x64` without
  `--skip-license`; `.vscodeignore` keeps `bundle\**`. `vscode\CHANGELOG.md` with the 0.2.0 section (bundled
  compiler, `buildWith`, the include root change). Checked by `npm run package` writing
  `qb64rust-win32-x64-0.2.0.vsix`.
- [x] 2.3 `vscode\scripts\check_vsix.py` with its test (`test_check_vsix.py`, a small zip made in the test): the
  required files of spec `editor/packaging` present, no `.ts`, `.o` or `c_compiler`. Checked against the
  `.vsix` of 2.2, whose size goes into `vscode\DEVELOPMENT.md` "Package" along with the new commands.

## 3. Finding the binary and preparing libqb (design D2, D3)

- [x] 3.1 `findCompiler` takes `bundled` (after the setting, before `PATH`, `source: "bundle"`).
  `languageServer.ts` passes the bundle path, and the language status shows `(bundled)`. Unit tests in
  `discovery.test.ts`: the spec's "Nothing set", "Setting wins over the bundled binary", "Development install
  without a bundled binary". Checked by `npm run test:unit`.
- [x] 3.2 `compiler\libqbStore.ts`: `prepareLibqb` (copy to a temporary sibling, rename into place, reuse a
  complete copy) and `removeOtherVersions`, called from `extension.ts` at activation. The lock path for
  `QB64RUST_OBJECTS_LOCK`. Pure parts unit-tested on a temporary folder (`libqbStore.test.ts`): a complete copy is
  reused, a partial one (no completion marker) is redone, and other versions are removed while the current one
  stays. Checked by `npm run test:unit`.

## 4. Building with qb64rust (design D4, D5, D6)

- [x] 4.1 `compiler\parseQb64rustOutput.ts` (errors, warnings and notes with columns, relative file names
  resolved against the source's folder, the "all marked" flag, the summary and folder lines skipped). Unit tests
  on recorded output of the real binary: a type error, an error in an include, a marked error with its note, a
  warning with `-w`. `CompilerMessage.column`. Checked by `npm run test:unit`.
- [x] 4.2 `compiler\qb64rust.ts` (`buildArgs`, `build`), `config.ts` (`buildWith`, `effectiveIncludeRoot`), the
  `buildWith` setting in `package.json`, the `qb64rust-build` collection in `diagnostics.ts`, `Builder.build`
  choosing the compiler, with the "Build with QB64pe" button, `statusBar.ts` naming the building compiler, and
  `languageServer.ts` giving the effective include root (restart on `compilerPath`). Unit test of `buildArgs` and
  `effectiveIncludeRoot`. Checked by `npm run lint` and `npm run test:unit`.
- [x] 4.3 `tasks.ts`: async `provideTasks`/`resolveTask`. The build task uses the builder's compiler and command
  line, with libqb prepared and the lock in its environment.
- [x] 4.4 Integration tests (with `QB64RUST_TEST_QB64PE` and `QB64RUST_TEST_QB64RUST`), in a new
  `buildQb64rust.test.ts` (the suite sets `buildWith: auto`): the spec's "Built by qb64rust" (`hello.bas` builds and
  runs, `Builder.lastCommand` is `qb64rust`'s, the libqb copy with its objects is under the test's global storage
  and the bundle holds none), "Failed build with qb64rust" (error in an included file at its column, source
  `qb64rust`, cleared on edit), "Only unsupported forms" (the button builds with QB64pe; an unmarked error offers
  none), "qb64pe chosen", "qb64rust chosen but missing", and "Build task with qb64rust".
  `languageServer.test.ts` gets "Include from QB64pe's folder". The test workspace sets `buildWith: qb64pe`, so the
  existing build and task tests keep testing QB64pe. Found on the way: a `compilerPath` change now restarts the
  server, and two restarts close together left two clients running on VS Code 1.100 (everything published twice);
  `LanguageServer.restart` now stops a client whose start a later restart overtook. Checked by `npm test`'s parts
  with both variables set: unit 72, grammar, integration 54 of 54 on stable and on 1.100, nothing pending.
- [x] 4.5 `vscode\README.md`: requirements (QB64pe 4.7.0 for the toolchain, the bundled compiler, the first build
  of libqb), the settings table (`buildWith`, `path` with the bundle, `includeRoot`'s new default), install from
  the release's `.vsix`. Checked by reading it against the specs.

## 5. Release workflow (design D8)

- [x] 5.1 `.github\workflows\release-vsix.yml` (tag and version check first, release build, `npm ci`, unit tests,
  staging with `--binary`, `check_vsix.py`, `gh release create` with the `CHANGELOG.md` section as notes,
  `contents: write`). `vscode-extension.yml` packages the platform `.vsix` and runs `check_vsix.py`. The tag
  check's logic is in `check_vsix.py --tag <tag>` so its test covers the spec's "Mismatched tag". Checked by the
  unit tests and by `tools\repo_check` on the new files. The workflow itself first runs when the user pushes the
  tag.

## 6. Close

- [x] 6.1 The closing runs: `tools\close\close.py --check` passes. The new Python tests are run beside the other
  tools' tests.
- [x] 6.2 `python tools\testtime\test_time.py`. `STATUS.md` "Next" (the three "After M3" changes built; the tag
  `vscode-v0.2.0` waits on the user's push after the merge).

## Workflow follow-up

- The user pushes `vscode-v0.2.0` after the merge into `main`; the release workflow attaches the `.vsix`.
