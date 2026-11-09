# testing/community-programs Specification

## Purpose
Checks that programs people wrote and published for QB64pe pass QB64Rust's front end, from clones of their
repositories pinned by commit, without copying any of their files into this repository.

## Requirements

### Requirement: A manifest of pinned repositories
The repository SHALL hold a manifest naming each community repository: a short name, its URL, the full commit it
is pinned to, its license as GitHub reports it, the URL and commit of each of its submodules, and the compiler
options its programs need (an include root). The manifest SHALL hold no local path.

#### Scenario: A repository with a submodule
- **WHEN** DRAW is in the manifest
- **THEN** its entry names DRAW's commit, and the URL and commit of its submodule `includes/QB64_GJ_LIB`

#### Scenario: A repository its programs need as the compiler root
- **WHEN** the QB64-PE Libraries Pack is in the manifest
- **THEN** its entry says that its programs are checked with the repository's own folder as `--include-root`

### Requirement: Clones outside the repository
A fetch command SHALL clone each repository of the manifest into a folder beside the tree, outside the repository,
at its pinned commit and with its submodules at their pinned commits. It SHALL be the only part that uses the
network. No file of a clone SHALL be copied into the repository.

#### Scenario: A fetch after the manifest changed
- **WHEN** an entry's commit is changed in the manifest and the fetch runs
- **THEN** that clone is at the new commit, and the other clones are not fetched again

#### Scenario: A program with no license
- **WHEN** the QB64-PE Museum, which has no license, is checked
- **THEN** its programs are read from its clone, and the repository holds only their names on the pass list

### Requirement: A fetch that trusts nothing in the repositories
The fetch SHALL treat every repository as untrusted. It SHALL clone a submodule only from the URL in the
manifest, never from the repository's `.gitmodules`, and SHALL never use `--recurse-submodules`. Every git command
it runs SHALL have symlinks turned off, `file` remotes refused, object checks on, no hooks, no LFS downloads and no
prompts. It SHALL refuse to run with a git older than 2.45.1.

#### Scenario: A submodule URL changed in the repository
- **WHEN** a repository's `.gitmodules` at the new pin names a different URL for a submodule than the manifest does
- **THEN** the fetch clones the submodule from the manifest's URL

#### Scenario: A repository with a symlink
- **WHEN** a clone's pinned commit holds a symlink
- **THEN** the fetch checks it out as a plain file holding the link's target, not as a link

#### Scenario: An old git
- **WHEN** the git on the path is older than 2.45.1
- **THEN** the fetch fails before any clone, naming the version it needs

### Requirement: An audit before a pin is added or moved
An audit command SHALL list, for each clone at its pin: its files that are not text (executables, libraries,
scripts, archives, installers), its symlinks and submodules, its files over 10 MB, its path names that Windows
treats specially or that hold `..`, and its `.vscode` and `.devcontainer` folders. For a moved pin it SHALL list
only what changed since the old pin. It SHALL change nothing and fail nothing.

#### Scenario: A pin moved
- **WHEN** DRAW's pin is moved and a `.dll` was added between the old and new commit
- **THEN** the audit of DRAW lists that `.dll`, and nothing that was already there at the old pin

### Requirement: The front-end check
The check SHALL run `qb64rust -z` on every `.bas` file of every clone, with that repository's options, and SHALL
record for each program whether it passed (no error) and its first error. It SHALL not use the network, build
C++ or run a program. The first error SHALL be cut to 200 characters, with any path outside the clone replaced by
`<path>`, and the check SHALL print no source line of a program. A program whose check takes more than 120 s SHALL
be stopped and recorded as failing.

#### Scenario: An include outside the clone
- **WHEN** a program's `$INCLUDE` names a file outside its clone and that file has an error
- **THEN** the recorded first error shows `<path>` in place of that file's path

#### Scenario: A program that hangs the front end
- **WHEN** the check of one program has run for 120 s
- **THEN** it is stopped and recorded as "timed out", and the other programs are checked

#### Scenario: A program that includes a missing file
- **WHEN** a clone's `.bas` file is a module that includes a file only its main program has
- **THEN** the check records it as failing with its first error, and the run fails only if the program is on the
  pass list

#### Scenario: A clone at the wrong commit
- **WHEN** a clone is missing or not at its pinned commit
- **THEN** the check fails before any program, naming the repository and the fetch command

### Requirement: A pass list that only grows
A pass list in the repository SHALL name the programs that must pass, each by its repository's short name and its
path in the clone. A run SHALL fail when a listed program fails or is missing. A program that passes and is not
listed SHALL be reported, and SHALL be added only by an explicit update of the list.

#### Scenario: A program that passes for the first time
- **WHEN** a fix makes `draw/DRAW.BAS` pass and the list does not name it
- **THEN** the run passes and reports it as new, and an update of the list adds it

#### Scenario: A regression
- **WHEN** a listed program fails after a change to the compiler
- **THEN** the run fails, naming the program and its first error

### Requirement: The community step of check
`check` SHALL have a step that runs the front-end check on the pass list, in the full layer, in the suites VM by
default. Its result SHALL rest on the release build, the tool, the manifest, the pass list and the clones' commits.

#### Scenario: Nothing changed
- **WHEN** the step passed and neither the compiler, the tool, the manifest, the pass list nor a clone's commit has
  changed
- **THEN** the step stands and is not run again
