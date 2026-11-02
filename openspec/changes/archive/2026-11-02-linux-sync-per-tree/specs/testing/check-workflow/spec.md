# Spec Delta

## ADDED Requirements

### Requirement: One Linux copy per source tree
The Linux steps SHALL run in a WSL copy that follows one Windows source tree, named from that tree's folder, so
that the checkout and a worktree each have their own. The copy SHALL be brought up to date from the files git
reports changed since the last sync, without a scan of the tree, and from every file compared by content when the
run is forced.

#### Scenario: A worktree's run
- **WHEN** `check full` runs in the worktree `QB64Rust-full`
- **THEN** its Linux steps sync to and run in `~/qb64rust-full`, and the checkout's `~/qb64rust` is untouched

#### Scenario: Nothing changed
- **WHEN** the tree's copy was synced from it and no file changed
- **THEN** the sync gives rsync only the files `git status` reports (the tree's uncommitted edits), scans no
  other file, and finishes in seconds

#### Scenario: An edit undone
- **WHEN** a file was edited at the last sync and the edit is undone before this one
- **THEN** the file is copied again, as it was at the last sync's status list

## MODIFIED Requirements

### Requirement: The merge into main
`check merge` SHALL run, without any result standing and without the pass cache, every step of `check full` but
the pass lists that its whole suites stand for, and the steps of the merge: the known lists and the old wording
table regenerated before the build, the wording held to its recordings, the corpus, compile and verification
suites run whole with their coverage records written, each pass list held to its whole suite's results, the
built-ins' list held to the records, the demand reports, the count blocks (failing when one is left as it is) and
the three suites whole on Linux, in a copy brought up to date by content. Like `check full`, it SHALL refuse a
checkout with uncommitted changes under `crates/`, `libqb/` or `tests/`.

#### Scenario: The coverage records are of the merge
- **WHEN** `check merge` passes
- **THEN** `target/coverage/merge` holds the records of the three whole suites of that commit, which `check
  quick`'s libqb narrowing reads

#### Scenario: The exact copy
- **WHEN** `check merge` or `check --force` runs the Linux sync
- **THEN** every file of the copy is compared by content, whatever its size and time
