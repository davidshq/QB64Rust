# Design

## Context

See `proposal.md` for why. What shapes the approach:

- **The suites VM has no network** (spec `testing/suites-vm`, "The suites VM has no network"). `suites.ps1`
  already carries the reference clone in: it archives `..\QB64pe` without `.git` with `tar`, copies the archive
  to the cache disk and unpacks it, when a stamp of the clone's commit and changes differs (`tools\winvm\suites.ps1`
  lines 206–233). The tree's copy holds no git-ignored files.
- **A step's result rests on its inputs** (`tools\check\steps.py`, `Step.inputs` and `Step.external`). Inputs
  outside the repo have a fingerprint of their own in `tools\check\check.py`, `external()`: the toolchain by its
  files, the reference clone by its commit and changes.
- **The checker already scans folders.** `qb64rust --report unsupported` runs the front end over every `.bas` file
  below a folder and counts the "not supported yet" messages (`crates\driver\src\report.rs`). It does not say
  which programs passed, and it skips the C++ writer. `qb64rust -z` checks one program and writes its C++.
- **What the survey measured** (`study\44` §1, §2.1): in the Museum 459 of 520 `.bas` files check clean; most of
  the rest are modules that only their main program can include. A program needs its include files, `.ico` files
  for `$EXEICON`, `.ui` files of InForm, and the Libraries Pack's `.ini` descriptors, so a clone must be whole,
  not sparse.

## Goals / Non-Goals

**Goals:**
- One command fetches every clone; one step checks every program; the repo holds names and commits only.
- A regression in a listed program fails the step, with the program and its first error in the log.
- The same tool runs on the host for one repository, so a fix can be tried on DRAW without a VM run.

**Non-Goals:**
- Telling main programs from modules. The pass list decides what must pass; a module that fails is reported once
  and then ignored.
- A demand report (`--report unsupported`) of the set. It can be run by hand on a clone; the step does not need it.

## Decisions

### D1. The manifest is TOML: `tests\community\repos.toml`

One table per repository:

```toml
[[repo]]
name = "draw"                      # the prefix of its programs on the pass list
url = "https://github.com/grymmjack/DRAW"
commit = "05770d7…"                # full SHA-1
license = "MIT"                    # as GitHub reports it; NOASSERTION and none stay as they are
submodules = { "includes/QB64_GJ_LIB" = { url = "https://github.com/grymmjack/QB64_GJ_LIB", commit = "2651577…" } }
include_root = ""                  # a clone's name: that clone's folder is `--include-root`
```

`include_root = "librariespack"` on the Libraries Pack itself makes its folder the compiler root of its own
examples; on `mplayer64` it could name a folder holding InForm-PE and Toolbox64, if its includes need that
(checked in task 1.1).

Python 3.11's `tomllib` reads it with no dependency. *Alternatives:* a tab-separated list like the other
`*.list` files (no room for submodules without a second file); JSON (no comments).

The first manifest holds the repositories of `study\44` §1.1, §1.2 (the Museum, the Libraries Pack, `Resources`
without its media, `qb40/all`), §1.3, §1.4 and §1.5 (about 80 repositories). `qb40/abc-archive` is left out: its programs are packed and need
extracting, a step of its own.

### D2. The clones are beside the tree: `..\community\<name>`

Next to `..\QB64pe\`, so the VM layout gains one folder at the same place (`<parent>\community`), and nothing
under `target\` (which `tools\clean` prunes) holds them. `community.py` takes `--root` for another place;
`QB64RUST_COMMUNITY` overrides it too. *Alternatives:* `<qb64contain>\community`, beside QB 4.5 (one more folder
level for the VM to mirror, and `qb64contain` is not in the VM's layout); `target\community` (pruned, and not in
the tree's copy).

### D3. Whole clones at the commit; submodules by their pinned commit

`fetch` runs `git clone --filter=blob:none` and `git checkout <commit>` for a new clone, `git fetch` and checkout
for a moved one, and `git -C <sub> checkout <commit>` for each submodule (cloned from the URL that the manifest
gives, never from the repository's `.gitmodules`, D8). Blobs are fetched as the checkout needs them, so the Museum costs its checkout (about 335 MB), not
its history. A clone whose `HEAD` is the pinned commit is left alone. *Alternative:* sparse clones of `.bas`
files only, as the survey used; they missed `.ico`, `.ui`, `.ini` and other included files and gave false
failures (`study\44` §2.1).

### D4. The check runs `qb64rust -z` per program, eight at a time

`community.py check` lists every `.bas` file of every clone (case-insensitive, sorted), runs `qb64rust -z <file>
-o <scratch>\<n>.exe` with the repository's `--include-root` when D1 says so, eight processes at a time, and keeps
the first `error:` line of each. It writes `target\community\results.json` (program, passed, first error, seconds)
and prints one line per listed failure, per missing program and per new pass. The C++ goes to a scratch folder
removed at the end. *Alternatives:* `--report unsupported` (no per-program verdict, no C++ writer); a new
`qb64rust` option to check many files at once (faster, but a second code path that the per-program `-z` of a user
does not take).

### D5. The pass list: `tests\community\pass.list`, `<name>/<path in clone>` per line

Sorted, with `/` as separator. A run fails when a listed program fails or its file is gone. `--update-list` adds
every program that passed and is not listed, and never removes a line: a program dropped from a repository is
taken off by hand with the commit that moves the pin. *Alternative:* a list of known failures, as
`tests\verification\known.list` is. Here most failures are modules that will never pass alone, so listing the
passes is shorter and says what matters.

### D6. The step `community`: full layer, VM by default, its own external input

In `tools\check\steps.py`: `Step("community", FULL, ("python", "tools/community/community.py", "check"),
inputs=BUILT + ("tests/community/**", "tools/community/**"), needs=("build",), external=("community",),
vm=True, outputs=("target/community/results.json",))`. `check.py`'s `external("community")` is the manifest's
commits joined with each clone's `HEAD`, so a clone fetched to another commit makes the step run again. It is not
in the quick layer: about 3,000 `.bas` files (the Museum 520, `QB64-samples` 1,104, `Resources` 576, the rest about 800) at
roughly 0.1 to 2 s each is ten minutes or more. `check community --where
host` runs it on the host, which is allowed: it opens no window, starts no `SHELL` and uses no network, but it is
more than 20 programs, so Claude asks the user first (`CLAUDE.md` working rule 6).

### D7. `suites.ps1` copies `..\community` as it copies the clone

The same `tar` route, with its own stamp: the list of `<name> <HEAD>` of the host's clones. The copy is made only
when a run includes the step `community`, since it is several hundred MB. *Alternative:* a VHD of the clones,
attached as a second disk (no copy time, but a disk to make and keep in step with the manifest).

### D8. Fetching untrusted repositories: the manifest's URLs and hardened git settings

The repositories are untrusted code. This change never runs them: `-z` stops before `make`
(`crates\driver\src\compile.rs`, the `cpp_only` return). So the fetch is the part that must be safe.

- **Submodule URLs come from the manifest.** A repository's `.gitmodules` is not read. A new pin cannot point a
  submodule at a new place without a change to `repos.toml` that the user sees.
- **Every git command of `fetch` runs with** `-c core.symlinks=false -c protocol.file.allow=never -c
  transfer.fsckObjects=true -c core.hooksPath=<empty folder>`, and with `GIT_LFS_SKIP_SMUDGE=1` and
  `GIT_TERMINAL_PROMPT=0` in its environment. It never uses `--recurse-submodules`. A clone brings no hooks or
  config from its remote. These settings close the paths that are left: symlinks and submodules on a
  case-insensitive filesystem (CVE-2024-32002), local-file remotes, malformed objects and LFS downloads.
- **`fetch` refuses a git older than 2.45.1**, the first release with the fix for CVE-2024-32002. The host had
  2.46.0 on 2026-11-09.
- **Only the commit in the manifest is checked out.** An author's later push reaches the clones only when a pin
  is moved, and moving a pin goes through D9.

*Alternative:* fetching in the suites VM. It has no network by design (spec `testing/suites-vm`).

### D9. An audit before a pin is added or moved

`community.py audit [--only <name>]` reads each clone at its pin and lists:
- the files that are not text: executables, libraries, archives and installers (`.exe`, `.dll`, `.com`, `.bat`,
  `.cmd`, `.ps1`, `.vbs`, `.msi`, `.zip`, `.7z` and others);
- symlinks, and submodules with their URLs;
- files over 10 MB;
- path names that Windows treats specially (`CON`, `NUL`, `AUX`, `COM1`, …) or that hold `..`;
- `.vscode\` and `.devcontainer\` folders, which can run commands when a folder is opened as trusted in VS Code;
- for a moved pin, only what changed since the old pin.

The user reads the report before the pin is committed. A repository whose purpose is risky is left out of the
manifest, not made safe. For example, study 44 left out `hsehestedt/WIM-Tools`, which edits Windows images as
an administrator. The audit has no verdict of its own and fails nothing. It is a list for a person to read.

The clones are never opened as a trusted workspace in VS Code. `tests\community\README.md` says so.

### D10. What the check reads and what it writes

- **The output carries names and first errors only.** The check writes each program's name, whether it passed,
  and its first `error:` line, cut to 200 characters with any path outside the clone replaced by `<path>`.
  Source lines from a program are never printed. *Why:* error text can quote any file an `$INCLUDE` or
  `$EXEICON` names, including the user's own files (`CLAUDE.md` rule 2). It is also text that Claude reads, and
  comments in 80 repositories are a channel for prompt injection, so it stays short.
- **`nm` runs only on the toolchain's own libraries.** `-z` runs the toolchain's `nm` on the library that a
  `DECLARE LIBRARY` names (`crates\driver\src\library.rs`, `Nm::symbols`). When that library is a binary inside a
  clone, `nm` would parse an untrusted file. The step runs in the VM by default, and the host runs it for one
  named repository at a time (D6). So no other guard is added. The audit (D9) lists the libraries a repository
  holds.
- **A time limit per program.** The check stops a `qb64rust -z` after 120 s and records it as failing with
  "timed out". A file that hangs or floods the front end then costs one slot, not the run.

## Risks / Trade-offs

- [A community program is built or run] → Not in this change. The later change that builds and runs programs
  does so in the suites VM only, from its checkpoint, never on the host. It reviews each program that uses
  `SHELL`, the network, `KILL` or `_OPENHOST`/`_OPENCLIENT` before it runs (`CLAUDE.md` working rule 6).
- [A clone holds malware: an executable or a script] → Nothing here runs it. Defender scans the checkout on the
  host. The audit (D9) lists such files so that no person or tool runs them by mistake.
- [Text in a clone tries to instruct Claude] → Claude treats a clone's contents and the check's output as data.
  The output is kept to names and first errors (D10).

- [A repository rewrites its history and the pinned commit disappears] → `fetch` fails for that repository and
  names it; the pin is moved to a commit that exists, with the pass list adjusted in the same commit.
- [A program's check depends on the folder it is in, or on the date] → the check runs each program from its own
  folder as `qb64pe` does. A program that varies is taken off the list with a comment.
- [A large program hits the check's time limit] → each program has 120 s (D10). The step's VM limit is set from
  the first full run, with room (D6).
- [A fix in the compiler that makes many new programs pass leaves them unlisted] → each run prints the count of
  new passes, and the closing task of a change runs `--update-list`.
- [Disk] → about 1 GB beside the tree and the same on the VM's cache disk.

## Migration Plan

None: a new step. To remove it, delete the step and the folder `..\community`.

## Open Questions

- The step's time limit in the VM, set after the first full run (D6).
