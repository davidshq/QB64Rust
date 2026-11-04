# Spec Delta

## ADDED Requirements

### Requirement: A program judged by its diverged output
When `tests\verification\diverged\<name>.out.txt` exists, the compare SHALL judge the program by that file in
place of QB64pe's recording, on every platform, by the same rules. When `diverged\<name>.log.txt` exists, it
SHALL stand in for the recorded log. A recording with `qb64pe` SHALL neither read nor write these files.

#### Scenario: Our own output passes
- **WHEN** `v17_g_precedence` has a sidecar `diverged\v17_g_precedence.out.txt` that gives `5 IMP 3 IMP 0: 4`
  where QB64pe's recording gives 7, and the build prints what the sidecar holds
- **THEN** the program passes

#### Scenario: A regression beside the divergence
- **WHEN** the build prints the divergent line as the sidecar holds it, but another line differs from the sidecar
- **THEN** the program fails at the stage "result", naming the sidecar and the first line that differs

#### Scenario: The log
- **WHEN** a program has `diverged\<name>.log.txt` and its run logs what that file holds, not what QB64pe logged
- **THEN** the program passes

### Requirement: A diverged rejection stands on a pinned line
A `diverged\<name>.out.txt` that holds `(not compiled)` SHALL pass only when the compile fails as the rule for
`(not compiled)` asks and the earliest unmarked error of the program's own file is on the line its row of
`lines.list` gives. Without such a row the program SHALL fail at the stage "error line".

#### Scenario: Rejected on the pinned line
- **WHEN** `v17_e_const_fdiv0` has a sidecar `(not compiled)` and a row of `lines.list` giving line 3, and the
  compile fails with its earliest unmarked error on line 3
- **THEN** the program passes

#### Scenario: No row
- **WHEN** a program's sidecar holds `(not compiled)` and `lines.list` has no row for it
- **THEN** the program fails at the stage "error line", saying that a diverged rejection needs a row

### Requirement: The diverged sidecars are checked
Tier 1 SHALL fail when a file of `tests\verification\diverged\` is not `<name>.out.txt` or `<name>.log.txt` of a
program of the suite, when that program is not on `pass.list`, when a `.log.txt` has no `.out.txt` beside it,
or when the program has no `divergences\<name>.divergences` naming at least one row of `QB64PE-DIVERGENCES.md`.

#### Scenario: No divergences file
- **WHEN** `diverged\v04_accidental.out.txt` exists and `divergences\v04_accidental.divergences` does not
- **THEN** the tier-1 check of the lists fails and names the program

#### Scenario: Still on known.list
- **WHEN** a program has a `diverged\` sidecar and its row is on `known.list`
- **THEN** the tier-1 check of the lists fails and names the program

## MODIFIED Requirements

### Requirement: Pass list and known differences
`tests\verification\pass.list` SHALL name programs that the new compiler passes, and comparing it SHALL pass
every one. A program of `pass.list` with a `diverged\` sidecar passes by that sidecar. `tests\verification\known.list`
SHALL name every other program of `verification\`, each with a reason:
a row of `QB64PE-DIVERGENCES.md`, "not supported yet", "varies", "not recorded" (no `<name>.out.txt`), "recorded
elsewhere" (recorded with another `--verification-root`), or "bug: <what>". No program SHALL be in both lists or
in neither. A program of `known.list` that passes in a full compare SHALL be reported so that it can move to
`pass.list`, unless its reason is "varies".

#### Scenario: Every program is accounted for
- **WHEN** the tier-1 check of the lists runs
- **THEN** each `.bas` of `verification\` is named exactly once across `pass.list` and `known.list`, and each
  name in either list is a program of `verification\`

#### Scenario: Pass list run
- **WHEN** `run_legacy_tests.py --suite verification --qb64 target/release/qb64rust.exe --list
  tests/verification/pass.list` is run
- **THEN** every listed program passes

#### Scenario: A diverged program on the pass list
- **WHEN** a program of `pass.list` has a `diverged\` sidecar and the pass list is run
- **THEN** the program is judged by the sidecar, not by QB64pe's recording
