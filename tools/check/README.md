# check: the one entry point for testing

Spec `testing/check-workflow`; the OpenSpec change `testing-reset` (its `design.md` has the measurements). Run it
through `just` (the `justfile` at the top of the repository) or as `python tools/check/check.py`:

| Command | Runs |
|---|---|
| `just quick` | the quick layer: `fmt`, `build` (release), `clippy`, `clippy-linux` (the same lints for the Linux target, checked from Windows with no linking), `nextest` (the workspace's Rust tests), `lists` (`inputs.rs`: the lists and recordings), `doc`, `tools` (the Python tools' tests, `tool_tests.py`, all but `tools\check`'s), `tools-check` (`tools\check`'s own tests, about 80 s, run when `tools\check` or a folder its tests read changes), `libqb-copy` (the copy against its edits), `reference` (the language reference's pages against their sources, `tools\reference\README.md`), `v37-libs` (the libraries of the `v37` verification programs, `verification\v37_lib\build_libs.py`, git-ignored; `verification` needs it, so a fresh worktree builds them), `slice` (the slice list); and the upstream and verification lists narrowed to the programs a change reaches, when it reaches only programs |
| `just full` | every step: the quick layer and the upstream and verification lists, the call-site check, the CLI tests that build (`--ignored`), qbasic, addprefix, the format runs, the differential list, the language reference's examples, the slice without folding, the coverage check, the Linux family in WSL, the sandbox family last |
| `just merge` | the merge into `main`, in a worktree of the merge commit: every step again without the pass cache (but what passed in an earlier merge with the same fingerprint, below), but the slice, upstream and verification lists and the Linux slice, which its whole suites stand for; and its own steps (`steps.py`, layer `merge`, from the old `close.py`'s merge set): the known lists and the old wording table regenerated first (`update-lists`, `wording`, a plan of their own, so that the rest is planned on the files they wrote), then the wording check, the three whole suites with `--coverage` (`target\coverage\merge\*.txt`, the records narrowing reads), the pass lists held to their results (`--check-list`), the built-ins' list (`builtins`), the two demand reports, the count blocks (`counts.py --strict`; these three and the `--check-list` runs run after a whole suite with a failure too, `after_failure`, since they read its results, not its verdict) and the Linux whole suites, after a `linux-sync --checksum` (the exact copy; a step's `forced` form, used by `--force` too) at the tree's first merge, and the usual sync from git's list once an earlier merge's sync passed (`check-rejudge` design D6). Files it rewrites are for the user to stage |
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

**`just full --uncommitted`** runs `full` in spite of uncommitted changes under `crates\`, `libqb\` and `tests\`,
where it would refuse, so that a change can be tested whole before it is committed (user, 2026-11-03). The report
says so in one line. Its results stand by the files' contents, as always, so nobody may edit those folders in that
tree while it runs: a step could then pass on files other than those its record names. `merge` refuses the switch,
since a merge's results are to describe the merge commit.

**After a fix to a failed `just merge`**, check out the fix's commit in the worktree and run `just merge` again: a
step that passed in an earlier merge with the same fingerprint stands (only a merge's pass does: a pass of `full`
or of one step may have used the pass cache), so only what the fix reaches runs. `just merge --force` runs every
step (OpenSpec change `check-rejudge`, design D3; user, 2026-11-02).

**A re-judge instead of a run.** A whole suite (`corpus-whole`, `upstream-whole`, `verification-whole` and their
`linux-` steps) whose inputs changed since its last run, passed or failed, only in the lists that mark failures
known (`steps.py`'s `JUDGE_LISTS`: `known_failures.txt`, `tests\*\known.list`, `tests\*\known.linux.list`) is
planned as "re-judge: only the known lists changed": the runner's `--rejudge` applies today's lists to the stored
results in under a second, with no program built or run, and the steps that read the results run after it. Each
record keeps a `run_fingerprint`, the fingerprint without those lists, to tell. A failure with nothing changed runs
again, as before; `--force` runs.

## Where a step runs

A step that builds and runs programs can run on the host or in the suites VM, the Hyper-V VM `qb64rust-suites`
(`tools\winvm\README.md`, "The suites VM"; OpenSpec change `vm-suites`). These steps are marked `vm` in
`steps.py`: `slice`, `upstream`, `verification`, `callsite`, `cli`, `qbasic`, `addprefix`, `format`,
`format-corpus`, `differential`, `reference-examples`, `nofold`, `coverage`, `community`, and the merge's `corpus-whole`, `upstream-whole` and
`verification-whole`. Every other step runs on the host: `fmt`, `build`, `clippy`, `clippy-linux`, `nextest`, `lists`, `doc`,
`tools`, `tools-check`, the Linux steps, the merge's own readers.

| Run | Where the `vm` steps go |
|---|---|
| `just quick`, `just check <step>`, a program named alone | the host |
| `just full`, `just merge` | the VM. When it cannot be used (no Hyper-V, no VM, the VM running, a CI run, Linux), the host, and the report says why in one line |
| any of them with `--where host` | the host |
| any of them with `--where vm` | the VM. When it cannot be used, those steps fail with the reason, and the host's steps run |

```
just full --where host
just check slice --where vm
```

A program named alone (`just check v01_numeric`) always runs on the host; name its step to run it in the VM.

**What a run in the VM does.** The host builds `target\release\qb64rust.exe` as usual. Before the first VM step,
`tools\winvm\suites.ps1 -Start` restores the VM's checkpoint, attaches its cache disk at the parent folder of the
tree, and copies in the tree (the files git tracks or would track) and, when it changed, the QB64pe clone without
`.git`. The tree's copy is then at the same path in the VM as on the host. Each VM step runs through
`tools\winvm\step.ps1`, from a job file `check.py` writes (`target\check\vm\<step>.json`): it copies the release
build in, checks that the VM's `qb64rust` builds with the toolchain installed there, not the clone's, and runs the
step's command in the test account's session. Its log shows the guest's output as it comes, about every 10 s.
After the last VM step, and at the end of the run whatever happened, `suites.ps1 -Stop` turns the VM off. One
restore per run, however many VM steps.

- The runner keeps no build cache in the VM (`QB64RUST_NO_BUILD_CACHE`). Its pass caches are kept, on the cache
  disk, apart from the host's.
- Network programs run only in the VM. Their `.external` sidecar starts with `network:`. On a Windows host the
  runner skips them (SKIP), because a program that opens a port makes Windows Defender Firewall ask the user. A
  VM step sets `QB64RUST_ALLOW_NETWORK=1`, which runs them. So `just quick` and `--where host` do not test them.
- Clipboard programs run only in the VM too. Their sidecar starts with `clipboard:`, and a VM step sets
  `QB64RUST_ALLOW_CLIPBOARD=1`. On the host they would overwrite the user's clipboard (`DECISIONS.md` 2026-11-07).
- `community` checks the front end over the community repositories (`tests\community\README.md`). Their pinned
  clones are in `..\community`, fetched on the host with `tools\community\community.py fetch`. A run with the step
  also copies them to the VM (`suites.ps1 -Community`), again only when `heads.txt` changed. It fails when a
  program on `tests\community\pass.list` fails, and it is a large run (`CLAUDE.md` working rule 6), so it goes to
  the VM.
- `cli` builds the CLI tests' binary on the host (`cargo test --no-run`), and the binary runs in the VM with
  `--ignored`, in `crates\driver`.
- `verification` and `verification-whole` build the `v37` libraries in the VM first, since git ignores them.
- **What comes back to the host**: each step's `outputs` in `steps.py`, its results file (`results-partial.json` or
  `results-<suite>.json`), and the files of the programs that did not pass in it, but no executable; for the
  whole suites, their coverage record `target\coverage\merge\<name>.txt` too. The steps after them, such as
  `corpus-list`, `builtins` and `counts`, read those on the host. A re-judge runs on the host.

**The place is part of a result.** A step's record holds `"where"`, and its fingerprint holds the VM when it ran
there, with the files of `tools\winvm\` that take part in a step: `suites.ps1`, `step.ps1` and `outputs.py`. So a pass on the host never stands for a run in the VM, and the other way
round. `just status` says "in the VM" for such a pass.

**The toolchain on the host.** When a step on the host builds programs and `qb64rust toolchain path` names the QB64pe
clone's toolchain, `qb64rust`'s fallback, the report has one line saying so, with `qb64rust toolchain install`.
The run goes on: the two are the same release.

## What stands

Each step records its result in `target\check\<step>.json` with a fingerprint: SHA-256 over its command and over
the path and bytes of every file its inputs match (`steps.py`: globs over the repository, `!` taking back), the
inputs outside the repository (`toolchain`: the C++ toolchain's executables; `clone`: the reference clone's commit
and uncommitted changes; `community`: the commits of the community clones, from their `heads.txt`) and the fingerprints of the steps it needs. A passing result **stands** while its
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

The steps that run a pass list (slice, upstream, verification, differential, reference-examples, nofold) pass `--pass-list`: a pass
list holds programs that pass (D-052), so a known failure on one fails the run.

Inside a step the runner's pass cache (`target\pass-cache-2`) serves what passed with the same build inputs
(design D1; `tools\legacy_tests\README.md`, `--pass-cache`), and starts the programs that failed last time first,
then new ones, then the longest (`target\check\times\<suite>.json`). A program with an `.external` sidecar (the
desktop, sound, the clock, the network) is never served.

## The cost tests

`test_check.py` plans in a fake repository and holds what standard changes plan (spec `testing/check-workflow`,
"The cost of a check is held by tests"): documents alone plan nothing; new verification programs, their
recordings and list rows plan `lists` and those programs; a change under one crate plans `fmt`, `build`, `clippy`,
`clippy-linux`, `nextest`, `lists`, `doc` and `slice`, and one of its tests alone (other than `inputs.rs`) the same but `build`, `lists` and `slice`; a libqb edit naming its built-ins plans `nextest`, `libqb-copy` and the
programs that hold them, a pass that `full` runs again; a change to a shared verification support file plans the
whole list; a change to a tool plans `tools`, and a change under `tools\check` plans `tools-check` (with `tools` too for `tool_tests.py`, and for a folder both read). A step named alone has the fingerprint it has in its layer. A glob
widened so that one of them plans more fails.

    python -m unittest discover -s tools/check -p "test_*.py"

## Tools to install

`cargo install just --locked` and `cargo install cargo-nextest --locked`. Without nextest the `nextest` step runs
`cargo test` and its report says so, with the install line. `bacon` is optional (`crates\README.md`).
