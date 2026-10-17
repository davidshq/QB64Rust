# Tasks

Standing rules: CLAUDE.md. Differently here: the measurements of group 1 were taken before the proposal; the
resource files are recorded by `verification\v40_rc.py`, not by the runner.

## 1. Measurements and records

- [x] 1.1 Record what was measured (`verification\v40_*`, 47 programs; `verification\v40_rc\`; the scripts
  `v40_rc.py` and `v40_inc\show_version.ps1`): the facts of the proposal into `study\00` §5 (a `v40` paragraph),
  the family into `verification\README.md`, a row in `DECISIONS.md` for the step and the user's answer on the
  manifest (2026-10-16), the sentence on an executable that cannot start in `DIVERGENCES.md`'s paragraph on
  rejections that are not divergences. Verify: `python tools\repo_check\check_repo.py --untracked` is clean (no
  local path in `v40_rc\`).

## 2. Front end

- [x] 2.1 `sema`: `VersionInfo` and `Settings::version_info` (`lib.rs`), `version_info_meta` in `check\mod.rs`
  (design D1, D3), the errors and the warning. Verify: front-end tests `versioninfo_errors.bas` (`check-fail`, each
  form of `v40_x*` and D3's) and `versioninfo_ok.bas` (`check-ok`) pass; the `-w` warning of `v40_d` in a CLI test.
- [x] 2.2 `ir`: the setting carried and dumped (design D2). Verify: an `ir` front-end test shows it.

## 3. Build

- [x] 3.1 `driver`: `version_files`, `write_resources`, the make flag and the cache key (design D2, D4). Verify: the
  tier-1 driver test over `verification\v40_rc\` (design D5) passes for every recorded program that builds; the
  cache-key unit test; `cargo test`, fmt and clippy green.
- [x] 3.2 The slice program `s130_versioninfo` (design D5), recorded with `qb64pe.exe` (`--record`) and run with this
  compiler; upstream `versioninfo`, `versioninfo2`, `versioninfo3` run and added to `tests\upstream\pass.list`.
  Verify: each passes (`--glob`); `winresource/test` still held back by its `$EXEICON` name only.

## 4. Close

- [x] 4.1 The closing runs (working rule 9), the shrink-only lists, the demand reports and
  `tools\diagnostics\old_wording.py` regenerated, the counts by `tools\legacy_tests\counts.py`. Verify: every run
  passes or fails only by known failures.
- [x] 4.2 Documents: `STATUS.md`, `ROADMAP.md`, `SOMEDAY.md` (the `$VERSIONINFO` row goes; the questionable
  behaviours: a trailing comment kept in a value, `$VERSIONINFOX` accepted, numeric forms the old compiler passes to
  a resource compiler that refuses them). Verify: `openspec validate m3-versioninfo --strict` passes.
- [x] 4.3 `python tools\testtime\test_time.py`. Verify: its five files are named among those to stage.

## Workflow follow-up

- Archive the change when the user has looked at it, then `m3-arraycopy` (measured first, `verification\v41_*`).
