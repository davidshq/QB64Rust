## MODIFIED Requirements

### Requirement: The copy of libqb equals the reference commit
This repo SHALL hold a copy of QB64pe's `Makefile`, `internal\c` (without the toolchain), `licenses` and
`COPYING.txt`, each file byte for byte as at the commit its `SOURCE.md` names, which SHALL be the reference
clone's commit, but for the copy's own edits. Each edit SHALL be a file of `tools\libqb\edits` that names its
target file and the exact lines it replaces, and `SOURCE.md` SHALL list the edits. A tool SHALL write the copy from
the clone and its edits and SHALL say, with `--check`, which files differ from the commit's with the edits applied,
are missing or are extra; an edit whose lines do not occur exactly once in its target SHALL stop the tool.

#### Scenario: A file edited by hand
- **WHEN** a file of the copy is edited and the check is run
- **THEN** it exits with status 1 and names the file

#### Scenario: The copy as written
- **WHEN** the copy has just been written by the tool and the check is run
- **THEN** it exits with status 0

#### Scenario: An edit of the copy
- **WHEN** an edit file names a file of the copy and lines it holds once, and the tool writes the copy
- **THEN** that file holds the edit's new lines, `SOURCE.md` names the edit, and the check exits with status 0

#### Scenario: An edit that does not fit
- **WHEN** an edit's old lines do not occur in its target, or occur twice
- **THEN** the tool stops with status 1 and writes nothing
