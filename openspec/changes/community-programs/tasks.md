# Tasks

Standing rules: CLAUDE.md. This change fetches repositories over the network on the host (task 2.3); nothing it
checks is copied into the repo, and nothing in a clone is run (design D8–D10).

## 1. The manifest

- [x] 1.1 Write `tests\community\repos.toml` (design D1) with the repositories of `study\44` §1.1–§1.5, each
      pinned to the full commit of its default branch on the day, its license as the GitHub API reports it, its
      submodules' URLs and commits (DRAW's `includes/QB64_GJ_LIB`, design D8), and `include_root` for the Libraries Pack (its own name) and for MPlayer64 if its includes need InForm-PE and
      Toolbox64.
      Leave out `qb40/abc-archive` (packed programs) and `gmunaro/openstrut-v1` (provenance, `study\44` §1.1).
- [x] 1.2 Write `tests\community\README.md`: what the set is, where the clones live (design D2), the fetch and
      check commands, the pass list's rule (design D5), and that no file of a clone is kept here. Say that the
      clones are untrusted: they are never opened as a trusted workspace in VS Code, nothing in them is run, and
      a pin is added or moved only after its audit has been read (design D8, D9).

## 2. The tool

- [x] 2.1 Write `tools\community\community.py` with `fetch [--only <name>]`, `check [--only <name>]
      [--update-list] [--jobs N]` and `--root` (design D2–D5). Unit tests in `tools\community\test_community.py`
      with a fake manifest, a local bare repository as the remote, and a fake `qb64rust` script: a fetch to a
      moved pin, a clone at the wrong commit, a listed failure, a missing program, a new pass, `--update-list`
      that only adds. Also the safety of design D8 and D10: a submodule cloned from the manifest's URL when
      `.gitmodules` names another; a symlink checked out as a plain file; a hook in the fake remote that does not
      run; a git older than 2.45.1 refused; an error line cut to 200 characters with an outside path as
      `<path>`; a fake `qb64rust` that hangs, stopped at the time limit (shortened in the test). Verify with
      `python -m unittest discover -s tools\community`.
- [x] 2.2 Add `community.py audit [--only <name>] [--since <commit>]` (design D9), with unit tests: a fake clone
      holding an `.exe`, a symlink, a file over the size limit, a `CON` path and a `.vscode` folder lists each
      once; with `--since`, only what changed is listed.
- [x] 2.3 Run `community.py fetch` on the host. Verify that each clone's `HEAD` is its pin and that the folder
      `..\community` is outside the repo (`git status` shows nothing new). Run `community.py audit` and give the
      user its report before the manifest is handed over to stage. Drop any repository the user rules out.
- [ ] 2.4 (Skipped: the user chose the VM only, 2026-11-09; the survey's 68 is compared in 3.3.) Run `community.py check --only draw` on the host (72 programs: more than 20, so ask the user first,
      `CLAUDE.md` working rule 6; otherwise go straight to task 3.2). Compare with `study\44` §1: the count of
      passes should be at or above the survey's 68.

## 3. The step and the VM

- [x] 3.1 Add `external("community")` to `tools\check\check.py` and the step `community` to `tools\check\steps.py`
      (design D6). Extend `tools\check\test_check.py` for the new external input and step. Verify with the tool
      tests (`just check tools-check`).
- [x] 3.2 Copy `..\community` to the cache disk in `tools\winvm\suites.ps1` with its own stamp, only for a run with
      the step (design D7). Verify with `check community --where vm`: the copy is made once, a second run reuses
      it, and the step's log and `target\community\results.json` are on the host.
- [x] 3.3 Run the step in the VM on the whole set, then `community.py check --update-list` to write the first
      `tests\community\pass.list` from that run's results. Set the step's VM time limit from its time (design,
      Open Questions). Record the counts per repository in `tests\community\README.md`.

## 4. Records

- [x] 4.1 Edit the merged specs in place: `testing/community-programs` (new) and `testing/suites-vm`, "What
      persists between runs". Add `docs\LAYOUT.md` rows for `tests\community\` and `tools\community\`, and the
      step to `tools\check\README.md`'s table.
- [x] 4.2 Write the programs that fail and look like compiler gaps (not modules alone) as rows of `SOMEDAY.md`,
      one per construct, with the program that shows it. Update `study\44` §6 recommendation 3 as done.
- [x] 4.3 Update `STATUS.md`'s "Next".

## Workflow follow-up

- Archive the change after the user reviews it (`/opsx:archive`).
