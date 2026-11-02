# check: the one entry point for testing

Spec `testing/check-workflow`; the OpenSpec change `testing-reset` (its `design.md` has the measurements). Run it
through `just` (the `justfile` at the top of the repository) or as `python tools/check/check.py`:

| Command | Runs |
|---|---|
| `just quick` | the quick layer: `fmt`, `build` (release), `clippy`, `nextest` (the workspace's Rust tests), `lists` (`inputs.rs`: the lists and recordings), `doc`, `tools` (the Python tools' tests, `tool_tests.py`), `libqb-copy` (the copy against its edits), `v37-libs` (the libraries of the `v37` verification programs, `verification\v37_lib\build_libs.py`, git-ignored; `verification` needs it, so a fresh worktree builds them), `slice` (the slice list); and the upstream and verification lists narrowed to the programs a change reaches, when it reaches only programs |
| `just full` | every step: the quick layer and the upstream and verification lists, the call-site check, the CLI tests that build (`--ignored`), qbasic, addprefix, the format runs, the differential list, the slice without folding, the coverage check, the Linux family in WSL, the sandbox family last |
| `just merge` | the merge into `main`, in a worktree of the merge commit: every step again without the pass cache, but the slice, upstream and verification lists and the Linux slice, which its whole suites stand for; and its own steps (`steps.py`, layer `merge`, from the old `close.py`'s merge set): the known lists and the old wording table regenerated first (`update-lists`, `wording`, a plan of their own, so that the rest is planned on the files they wrote), then the wording check, the three whole suites with `--coverage` (`target\coverage\merge\*.txt`, the records narrowing reads), the pass lists held to their results (`--check-list`), the built-ins' list (`builtins`), the two demand reports, the count blocks (`counts.py --strict`; these three and the `--check-list` runs run after a whole suite with a failure too, `after_failure`, since they read its results, not its verdict) and the Linux whole suites, after a `linux-sync --checksum` (the exact copy; a step's `forced` form, used by `--force` too). Files it rewrites are for the user to stage |
| `just check <target>` | a step by name, after the steps it needs unless they stand (`build` before a list); a crate folder (`crates/sema`: that crate's tests); a suite folder or list (`verification`, `tests/corpus/slice.list`); one program by name (`v66_a01_varptr_dynamic_owner`, a corpus or upstream program's name), after the steps its suite's step needs (`build`; for a verification program `v37-libs` too) unless they stand |
| `just status` | each step: standing, to run, or failed, with its last run and log |
| `just history [days]` | per day (the last 14 with runs by default): the runs, the steps run and standing, the failures and the wall time; then the steps that took the most time. Every `check` run, one with nothing to run too, adds a line to `target\check\history.jsonl` |
| `just force [quick\|full]` | the layer again, what stands too, without the pass cache |

A step given as a string (the WSL steps, the merge's shell lines) runs in Git's bash, found from `git
--exec-path`: on Windows `System32\bash.exe`, WSL's, comes first otherwise. `--plan` prints what would run;
`--wait` stays in the foreground. A plan longer than a minute (by the steps' last
times) goes to the background, its report in `target\check\run.log` and each step's output in
`target\check\logs\<step>.log`. Nothing in the project refuses a command or holds a session because a run has not
happened.

**A run in a worktree.** `full` and `merge` run in a worktree of a commit, made beside the checkout
(`git worktree add ../qb64rust-full HEAD`), where `..\QB64pe` and `..\..\qb64contain` are the same folders
(`CLAUDE.md`, "Where things live"); elsewhere `check` refuses. For the next commit, `git checkout <commit>` in that
worktree: what stands is not run again, where a new worktree builds from cold and runs every step. Its first plan
hashes every file (some minutes); later plans read only what changed. The Linux steps run in the tree's own WSL
copy (`~/qb64rust` for the checkout, `~/qb64rust-full` for the worktree; `tools\linux\copy.sh`), so each copy
follows one tree and `linux-sync` copies only the files git reports changed since the last sync, not every file
by content, except under `--force`. Claude runs it as `just full --wait` in the
background, the report kept in `target\scratch\<date>\full.log`.

## What stands

Each step records its result in `target\check\<step>.json` with a fingerprint: SHA-256 over its command and over
the path and bytes of every file its inputs match (`steps.py`: globs over the repository, `!` taking back), the
inputs outside the repository (`toolchain`: the C++ toolchain's executables; `clone`: the reference clone's commit
and uncommitted changes) and the fingerprints of the steps it needs. A passing result **stands** while its
fingerprint is unchanged, whatever the commit, branch or working tree: committing changes nothing. A failure never
stands. `--force` runs everything again. The hashes are remembered by size and time in `target\check\hashes.json`,
so a plan reads only the files changed since the last; a plan of the quick layer takes about 2 s.

`steps.py` is the specification of what each result rests on: a file a step reads and its globs miss makes a stale
pass stand. `test_check.py` holds every glob to some file and each runner suite's step to every file its
`--list-only` run names (`tools\legacy_tests\README.md`).

## Narrowing

When the inputs a step's last pass had and the present ones differ only in one suite's programs (`verification\
<name>.*`, the sidecars of `tests\verification\`, `tests\corpus\<group>\<name>.*` and `.files\`, an upstream
category's folder), in the rows added to its list or changed in its `known.list`, the step runs only those programs.
A file directly under `verification\` beside no `.bas` of its name (`v16_inc_a.bi`, `cp437_all_bytes.bin`) is a
support item of several programs, so a change to it is not narrowed.
An edit of libqb whose edit files all name their built-ins (`builtins:` in `tools\libqb\edits\*.edit`) narrows the
program steps to the programs whose coverage records hold them (`target\coverage\merge\*.txt`, written by the
merge's full suites with `--coverage`); without those records the steps run whole. That narrowing is a guess (the
records are the last merge's, without the programs added since): its pass is marked `"narrowed": "coverage"`,
stands for `quick` and never for `full`, and the mark stays through later narrowed runs until the step runs whole.

The steps that run a pass list (slice, upstream, verification, differential, nofold) pass `--pass-list`: a pass
list holds programs that pass (D-052), so a known failure on one fails the run.

Inside a step the runner's pass cache (`target\pass-cache-2`) serves what passed with the same build inputs
(design D1; `tools\legacy_tests\README.md`, `--pass-cache`), and starts the programs that failed last time first,
then new ones, then the longest (`target\check\times\<suite>.json`). A program with an `.external` sidecar (the
desktop, sound, the clock, the network) is never served.

## The cost tests

`test_check.py` plans in a fake repository and holds what standard changes plan (spec `testing/check-workflow`,
"The cost of a check is held by tests"): documents alone plan nothing; new verification programs, their
recordings and list rows plan `lists` and those programs; a change under one crate plans `fmt`, `build`, `clippy`,
`nextest`, `lists`, `doc` and `slice`, and one of its tests alone (other than `inputs.rs`) the same but `build`, `lists` and `slice`; a libqb edit naming its built-ins plans `nextest`, `libqb-copy` and the
programs that hold them, a pass that `full` runs again; a change to a shared verification support file plans the
whole list; a change to a tool plans `tools`. A step named alone has the fingerprint it has in its layer. A glob
widened so that one of them plans more fails.

    python -m unittest discover -s tools/check -p "test_*.py"

## Tools to install

`cargo install just --locked` and `cargo install cargo-nextest --locked`. Without nextest the `nextest` step runs
`cargo test` and its report says so, with the install line. `bacon` is optional (`crates\README.md`).
