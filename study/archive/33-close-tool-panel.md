# 33. The close tool: panel notes (2026-10-17)

Written after `tools\close\close.py` was built at the close of `m3-errorline-chr-const` (user, choosing option A:
a checker, not a runner; `DECISIONS.md` 2026-10-17), at the user's request: "expert panel on this". The panel is
role-played inline (`CLAUDE.md` working rule 6) and counts effort by working rule 8. Its added role: a
**developer-experience engineer**, because the tool exists to make Claude follow a process, which is a question of
how a workflow tool is used.

## 1. Why the tool exists

At the close of `m3-errorline-chr-const` the closing set of working rule 9 was worked from memory. The change's
`verification` family was not checked as the rule says (a comparison by a script of Claude's own stood for it, not
said), and `cargo test` stopped at the first failing test binary (a line-ending failure from a `.gitattributes`
edit), so about 230 tests never ran, among them a new unit test that failed. The tool prints the closing commands
in order (`--plan`) and judges their logs (`--check`); each run is still started by its own command, so
`tools\testtime` sees it.

## 2. The panel

**Verdict:** keep it; the checker design is right (a runner script would hide the runs from the record). It caught
a stale tier-1 log on its first use.

- **Pragmatic engineer.** The rule's hook ("follow `--plan`, end with `--check`"; task 4.1 names the check) is
  enough. `--known` on the command line each time is fine for a temporary failure; fix the failure instead.
- **QB64 engineer.** A new corpus program on no list is reached by no closing run. Flag it.
- **Compiler/languages engineer.** The run without folding is asked for by a fixed list of files (constant
  evaluation, literals, operators); a numeric conversion elsewhere still needs `--nofold` by judgment, since the
  `fold` switch is read in five files that change in almost every change. Accepted as it is.
- **Rust engineer.** Tier 1 should run as CI does: `--workspace --locked`. File times as the staleness measure err
  the safe way after a fresh checkout.
- **Test engineer.** Add a test of a run after an edit and before a rebuild. Show each log's time, so an old log
  that passes is seen to be old. The merge into `main` has a full set of its own, run less often and so easier to
  get wrong: a `--merge` plan with the same check.
- **Developer-experience engineer.** A reason given with `--not-repeated` was printed on every line it covered;
  print it once. The plan is Bash; say so, since the primary shell here is PowerShell.

## 3. What was done (user, 2026-10-17: "Do")

1. `UNLISTED`: a new or changed `.bas` under `tests\corpus\` on no list and with no `.err` or `.norun`.
2. Tier 1 as `cargo test --workspace --locked --no-fail-fast`.
3. The test of a run after an edit and before a rebuild. Writing it found a case the check missed: a build log
   newer than the edit while the executable is not (a build that stopped, another profile). The build step is now
   stale when the executable is older than a changed input.
4. Each `ok` line shows when its log was written.
5. A `--not-repeated` reason is printed once, numbered, with the steps it covers.
6. The plan's first line says it is Bash.

Also done: the `verification\v40_rc` files the `.gitattributes` edit made CRLF were checked out again from the index
(they had LF in the working tree; nothing staged or unstaged), and `resource_files_equal_the_old_compilers` passes.

Left for a next small step: `--plan --merge` (the full set at a merge into `main`, rule 9).
