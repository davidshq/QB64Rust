# Community programs

Programs that people wrote and published for QB64pe, checked with QB64Rust's front end. The set is 66 GitHub
repositories chosen in `study\44` (the QB64-PE Museum, the Libraries Pack, DRAW, InForm-PE, the Lance Haffner
games, HamCalc64 and others). The spec is `openspec\specs\testing\community-programs`.

**No file of any of these repositories is kept here.** This folder holds only names and commits:

| File | What it holds |
|---|---|
| `repos.toml` | The manifest: for each repository its name, URL, pinned commit, license, submodules (URL and commit) and `include_root` |
| `pass.list` | The programs that must pass, one `<name>/<path in the clone>` per line, sorted, `/` as separator |
| `README.md` | This file |

## Where the clones live

Beside the tree, outside this repository: `..\community\<name>`, next to `..\QB64pe\`. `QB64RUST_COMMUNITY` or
`--root` names another place. Nothing under `target\` holds them, because `tools\clean` prunes it. They take about
1 GB; the Museum alone is 335 MB.

## Commands

```
python tools\community\community.py fetch [--only <name>]
python tools\community\community.py audit [--only <name>] [--since <commit>]
python tools\community\community.py check [--only <name>] [--update-list] [--jobs N]
```

- **`fetch`** is the only command that uses the network. It clones a repository at its pinned commit, moves a
  clone whose pin changed, and leaves alone a clone that is already at its pin. A submodule is cloned from the
  URL in `repos.toml` at its pinned commit, never from the repository's `.gitmodules`.
- **`audit`** lists what a person should read before a pin is added or moved: executables, scripts, archives,
  symlinks, submodules, files over 10 MB, path names Windows treats specially, and `.vscode` and `.devcontainer`
  folders. With `--since <commit>` it lists only what changed since that commit. It fails nothing.
- **`check`** runs `qb64rust -z` on every `.bas` file of the clones (the front end and the C++ writer; no C++
  build, nothing run). It writes `target\community\results.json` and prints each listed program that failed or is
  missing, and each program that passed and is not listed. `--update-list` adds the new passes to `pass.list`.

The step is `check community`. It runs in the suites VM by default (`check community --where vm`), since it is far
more than 20 programs (`CLAUDE.md` working rule 6). The VM has no network, so `suites.ps1` copies the clones from
the host to the VM's cache disk.

## The pass list

A run fails when a program on `pass.list` fails or its file is gone. A program that passes and is not listed is
reported and does not fail the run; `--update-list` adds it. The list never loses a line by itself: a program that
a repository dropped is taken off by hand, in the commit that moves the pin. Most failures are modules that only
their main program can include, so the list names the passes, not the failures.

## The clones are untrusted

The repositories are other people's code, and some have no license. Because of that:

- **Nothing in a clone is run.** `-z` stops before the C++ is built.
- **Never open `..\community` or a clone as a trusted workspace in VS Code.** A `.vscode` or `.devcontainer`
  folder in a clone can run commands when a folder is trusted.
- **Do not copy a file of a clone into this repository.** The Lance Haffner games are AGPL-3.0, and the Museum and
  many others have no license.
- **A pin is added or moved only after its audit has been read.** `community.py audit` produces the report. A
  repository whose purpose is risky is left out of the manifest.
- The check's output holds program names and first errors only, cut to 200 characters, with paths outside the
  clone shown as `<path>`. No source line is printed.

## Counts

The first full run in the suites VM, 2026-11-09, release build of branch `after-pr26`: 826 s with 8 jobs. A second
run, which held the new pass list, took 850 s and found 0 regressions and 0 missing. Two programs timed out at
120 s (`qb64-pe-museum/mnrvovrfc/Demos/forceq-graph.bas` and a Libraries Pack animation demo); the slowest program
that passed took 63 s. 27 files with non-ASCII names (DRAW's `PLANS` documents, 5 in `Resources`; no source file)
are not copied to the VM, because Windows `tar.exe` crashes on them.

Most failures are modules that only their main program includes, or programs that QB64pe rejects too. The failures
that look like compiler gaps are rows of `SOMEDAY.md` with the source `community`. The survey of `study\44` counted
458 passes in the Museum; this run has 468 of 519. DRAW has 104 of 200 programs (the survey checked 72 of them).

| Repository | Programs | Pass |
|---|---|---|
| `lhg_baseball` | 47 | 10 |
| `lhg_football` | 54 | 2 |
| `lhg_probb` | 28 | 4 |
| `lhg_collegebb` | 38 | 3 |
| `draw` | 200 | 104 |
| `chessbrainqb64` | 1 | 1 |
| `hamcalc64` | 467 | 467 |
| `inform-pe` | 47 | 45 |
| `xref` | 13 | 0 |
| `quickpakscientific` | 204 | 102 |
| `advgen` | 6 | 0 |
| `final-quest-2` | 6 | 2 |
| `qb_wild` | 2 | 1 |
| `questforlubok` | 1 | 1 |
| `ktank` | 6 | 6 |
| `qb64-pe-museum` | 519 | 468 |
| `qb64-pe-librariespack` | 79 | 68 |
| `resources` | 484 | 364 |
| `all` | 79 | 57 |
| `gx` | 24 | 23 |
| `qb64-interpreter` | 11 | 10 |
| `qb64-projects` | 67 | 67 |
| `srt-stats-monitor` | 3 | 1 |
| `nginx-stats-monitor` | 1 | 0 |
| `vga-font-editor` | 11 | 8 |
| `qb64-mod-player` | 1 | 1 |
| `p5js.bas` | 54 | 52 |
| `qb64_opengl_demos` | 37 | 35 |
| `ini-manager` | 4 | 4 |
| `tui` | 1 | 1 |
| `qb-wm` | 1 | 0 |
| `gui-menu-bar` | 3 | 2 |
| `qb64-platformer-test` | 2 | 1 |
| `alien-alley` | 1 | 1 |
| `qbasic` | 5 | 5 |
| `raster-master` | 41 | 22 |
| `raylib-64` | 18 | 16 |
| `toolbox64` | 9 | 5 |
| `qb64guitools` | 14 | 1 |
| `qb64-samples` | 828 | 632 |
| `spaceshooter2k` | 10 | 6 |
| `pendulum` | 2 | 2 |
| `hunter-revenge` | 1 | 0 |
| `vedit` | 1 | 0 |
| `3d-grapher` | 1 | 0 |
| `akframework` | 7 | 0 |
| `qb64wrappers` | 4 | 2 |
| `particle-life` | 18 | 11 |
| `qb64` | 3 | 1 |
| `mszp` | 4 | 1 |
| `qb64-opengl-engine` | 3 | 0 |
| `opengl-obj-loader` | 3 | 0 |
| `3d-library-for-qb64` | 1 | 0 |
| `qb64-3d-renderer` | 2 | 2 |
| `basic-raytracing` | 1 | 1 |
| `schemat` | 1 | 1 |
| `tcpip-experiments` | 6 | 4 |
| `qb64-sqlite` | 3 | 1 |
| `mplayer64` | 77 | 73 |
| `instantarchive` | 1 | 1 |
| `localizer` | 1 | 1 |
| `chess` | 1 | 0 |
| `qb64library` | 23 | 2 |
| `software-3d-perspective-correct-textured-triangles` | 24 | 24 |
| `fzxngn` | 35 | 34 |
| `qbx` | 327 | 259 |
| **all 66** | **3977** | **3018** |
