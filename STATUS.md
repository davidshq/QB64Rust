# Status and next steps

Updated 2026-10-09 (session 36).

**This file holds the current state only**: where we are, what is open, what comes next, the numbers of the last
full runs, known bugs. It holds no history. When a step is done, replace its line; do not add a paragraph about it.
Where everything else lives:

| Kind of information | Home |
|---|---|
| What a change did, task by task, and what was found on the way | the change's `tasks.md` and `design.md` under `openspec\changes\` (archived ones in `archive\`), and `git log` |
| A measured fact about the old compiler | `study\00` §5 (one home per fact) |
| A decision | `DECISIONS.md`; a decided difference from QB64pe: `DIVERGENCES.md` |
| What the compiler supports, how to build and test it | `crates\README.md`; per test set: `tests\*\README.md` |
| Milestones, done and not done | `ROADMAP.md` |
| History up to 2026-10-09 (what this file held before) | `study\progress-record.md` (closed) |

## Where we are

M0 (baseline) and M1 (VS Code extension on the old compiler) are complete. **M2 (front end) is in progress**: steps
1 to 8 of the order of work are done, the last being OpenSpec change `m2-numeric-types` (archived 2026-10-09).
The compiler parses every program the old compiler accepts with no false error, and compiles scalars, procedures,
error handling, every operator, `CONST`, control flow, `SELECT CASE`, static arrays and `TYPE` of the main module,
43 built-in functions, all 17 numeric types and fixed-length strings (`crates\README.md`). There is a thin language
server (`qb64rust lsp`). No OpenSpec change is open.

## Open items

- **The GitHub CI run after the next push has not been seen.** It settles task 10.2 of `m2-numeric-types` and task
  5.1 of `m2-language-server` (both changes were archived with that task open), and it is the first run of the
  changed `tier2` job (QB64pe's sources at the reference clone's commit, `DECISIONS.md` 2026-10-09). Work is checked
  locally; the user does not treat CI as a concern for now.

Every question of the seventh review is answered (`study\28` §9, `DECISIONS.md` 2026-10-09): the local reference
clone (`16f629784e`) is the one reference for the old compiler; Civil War Strategy is in `tests\programs` as the
real-program graphics target; the formatter is out of M2's exit criterion; the call-site check is tried inside
step 9; the screen-state check comes straight after step 9 (step 9a below).

## Next

The order was set by the fifth and sixth reviews (`study\26` §6, `study\27` §3).

9. **Next: built-in statements and functions by demand** (`study\27` §3): a built-in statement operation in the IR,
   then sequential file I/O, `DATA`/`READ`/`RESTORE`, `SWAP`, `RANDOMIZE`/`RND`/`TIMER`, console `INPUT`/`LINE
   INPUT`, `SHELL`/`COMMAND$`/`ENVIRON$`, each measured first; the remaining plain functions as the corpus, upstream
   or a user names them. To be proposed as an OpenSpec change. **Includes the trial of the call-site check**
   (`DECISIONS.md` 2026-10-09, `study\28` §3.2): for the first file I/O statements, compare the libqb call emitted
   with the old compiler's (`qb64pe -z`); keep it as the standard way a plain built-in comes in if the normalised
   call is stable, else drop it.
9a. **The screen-state check, with the libqb copy** (`DECISIONS.md` 2026-10-09, `study\28` §3.1): copy
    `..\QB64pe\internal\c` into this repo at `16f629784e`; add the exit hook that writes the screen state to a file
    when a test asks; teach the runner to record it with the old compiler and compare it for the new one. Its own
    OpenSpec change, design first: what is written per screen mode, how a program ends by itself, scripted keys,
    a fixed seed. First recordings need only the old compiler: small drawing programs cut from Civil War Strategy
    (`tests\programs\README.md`, stage 2), then upstream's programs without `$CONSOLE:ONLY` and `qbasic_testcases`.
10. `$CONSOLE` with `_DEST _CONSOLE` and `$SCREENHIDE`, measured first (`study\26` §5); check once that the GitHub
    Windows runner can start the hidden window.
11. `DEFxxx`, then the rest of arrays and `TYPE`: `REDIM`, dynamic arrays, `OPTION BASE`, plain member arrays
    (`study\26` §4; their `_STATIC`/`_DYNAMIC` markers stay in `SOMEDAY.md`). `DEFINT`, `COMMON` and arrays in
    procedures are also the first things Civil War Strategy needs (`tests\programs\README.md`).

## Numbers at the last full runs

2026-10-09, session 35, release build against the reference clone (upstream and `programs/` updated in session 36):

| Yardstick | Result |
|---|---|
| `tests\corpus\slice.list` (tier 2) | 198 of 198 |
| Full corpus | 214 pass, none wrong at run time; 74 rejected with "not supported yet" (`tests\corpus\README.md`) |
| Upstream | **47 of 279** (`tests\upstream\pass.list`); none wrong. It was reported as 45 while CI linked the 4.7.0 release's libqb: two programs need the later `VAL` of the reference clone |
| Real programs (`tests\programs`) | Civil War Strategy: the front end reads it with no real error; every diagnostic is "not supported yet" |
| Differential | 59 of 59 |
| Shrink-only lists | 0 false errors, 0 parse gaps, 64 only-marked rejections (`tests\upstream\README.md`) |
| Tier 1 (`cargo test`) | about 10 s |
| Tier 2 as CI runs it, locally | about 12 min |

How to read them: the corpus is the signal for step 9 (its rejected programs wait on built-in statements and
functions, and on step 11). Upstream will stay near 47 until steps 10 and 11: its blockers are `_DEST` and the rest
of arrays (`study\27` §4).

## Known bugs

None open.

## Queued for later milestones

| When | Item |
|---|---|
| M3 | Plain copy of `..\QB64pe\internal\c` at the reference clone's commit, `16f629784e` (`DECISIONS.md` 2026-10-02 and 2026-10-09) |
| M3 | Civil War Strategy builds and its screens match (`tests\programs\README.md`, stages 2 and 3) |
| Any time | A formatter that matches `-y` (out of M2's exit criterion, `DECISIONS.md` 2026-10-09) |
| M3 | Programs that draw (without `$CONSOLE:ONLY` and not writing to `_DEST _CONSOLE`), with an oracle such as a screen-state dump at exit compared between old and new compiler (`study\22` §3.2, `study\28` §3) |
| M4 | `qb64pe.bas` reached through its own include files, smallest first (`study\22` §4.2) |
| Help/hover work | Ask the QB64pe maintainers about the wiki licence before shipping any wiki text |
