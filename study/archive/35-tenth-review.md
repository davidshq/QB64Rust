# 35. Tenth review: architect's review during M3 (2026-10-18)

Asked by the user: as the software architect working with the panel, review what has been done and recommend.
Are there major changes to make, ways the project is going astray, ways to increase velocity? "It feels like
this is taking a long time." Big bugs and errors only, no nits.

Evidence read: `STATUS.md`, `ROADMAP.md`, `DECISIONS.md`, `SOMEDAY.md`, `study\00` (§1, §2, §12, §13, the
document map), `study\archive\30` (the ninth review), `study\archive\31` (the velocity plan), `tools\testtime\REPORT.md` and
`changes.csv`, the open change `m3-constant-evaluation` (`design.md`, `tasks.md`), `crates\driver\src\build.rs`,
`vscode\README.md` and `vscode\src`, the workspace manifests, `git log`. Run: `cargo test --workspace
--no-fail-fast` to a log, 364 passed, 0 failed.

## 1. Verdict

Nothing is broken and nothing is architecturally astray. Tier 1 is green, every closing run in `STATUS.md` is
green, and the ninth review two days ago found no bug by reading either. The problem is not correctness or
design. The last week has bought diminishing returns at a constant cost per change, and nobody can yet use the
new compiler for anything.

## 2. Is it actually slow?

| Measure | Value |
|---|---|
| Calendar time | 16 days (2026-10-02 to 2026-10-18) |
| Active Claude time | 107 h |
| Rust written | about 52,000 lines, 9 crates |
| Typical M3 change | 1 to 3 h active, including its closing runs |
| Time inside no change (reviews, process, documents) | 25 h, about 23% |
| Waiting for test runs | 18% |

A compiler that plays a real game screen for screen identically to the old one in 16 days is fast by any
standard. The feeling of slowness has three real causes:

- **The long tail.** Early M3 changes freed 20 to 60 upstream programs each. The last six freed 2 to 9 each, and
  the demand report's first lines are now 2 programs apiece. Same ceremony, smaller prize.
- **Meta-work.** Ten reviews and panels in 16 days (`study\archive\15`, `20` to `34`), five of them on test cadence
  alone. Each costs an hour of Claude's time and reading time for the user.
- **The user's time.** Every change ends with "kept behaviours" to rule on. `SOMEDAY.md`'s "QB64pe behaviours to
  review" holds 110 rows and grows with each change.

## 3. Recommendations, in order

1. **Declare M3 done at the next merge and switch to making the compiler usable.** M3's exit criterion (emit the
   fragments, link with libqb, pass the expected-output, differential and screen tests) is met. The remaining
   tail (`_DEVICES`, `$CHECKING:OFF`, `SUB _GL`, 43 upstream programs) belongs in M4, which is parity anyway.
   The next three changes should be:
   - the plain copy of QB64pe's `internal\c` into this repo (queued since 2026-10-02, still waiting);
   - the extension's diagnostics on save switched from `qb64pe -z` to the new compiler (columns and several
     errors per run were a day-one decision and still reach no user);
   - a `.vsix`.

   That is the first moment anyone would prefer this over QB64pe. Velocity that nobody can feel is not velocity.
   *Kind:* scope, the user's decision.

2. **Run the verification families against the new compiler as a suite.** There are 2,842 recorded programs
   across 45 families, each measured to design a change, and the new compiler is checked against them only
   through hand-written slice programs and front-end tests. The runner already records them; a compare mode is a
   short job in Claude's time and turns the largest body of evidence into regression tests. It also removes the
   per-change task of making slice programs by hand for forms a family already covers. This is the largest
   correctness gap found. *Kind:* risk to correctness; a cost that stays (machine time of the runs).

3. **Cut the per-change ceremony to the merge cadence.** Rulings on kept behaviours, reviews, and edits of
   `ROADMAP.md` and `STATUS.md` happen once per merge into `main` (every two or three changes), not per change. A
   kept behaviour is recorded in `SOMEDAY.md` and is otherwise silent until the user asks. Stop appending to
   `study\00` §5 (about 3,000 lines that no session reads whole); the family's recorded output is the fact.
   *Kind:* the user's time, Claude's time per change.

4. **Stop bit-fidelity work where QB64pe's own behaviour depends on the machine.** `m3-constant-evaluation` built a
   software x87 float, a big-float certification layer and a table of `mpmath` values (about 2,400 lines) to match
   `CONST` values whose old results depend on the libraries `qb64pe.exe` was built with. The user's ruling D-026
   (correctly rounded, one register row) was the right call. Make it the standing rule, so the next such case
   costs a paragraph and not a module. *Kind:* a cost that stays (code to maintain), scope. BUT DON'T GET RID OF EXISTING FIDELITY WE BUILT.

5. **Demote M4's exit criterion.** "The compiler built from `qb64pe.bas` passes the suite" (`study\archive\20`) means
   debugging a 30,000-line program nobody will ship, since self-hosting and the text-mode IDE are both out of
   scope (`DECISIONS.md` 2026-10-02). Keep `qb64pe.bas` compiling with no diagnostic as the test; running the
   result against the suite becomes optional. *Kind:* scope, the user's decision. WE WANT TO KEEP THIS QB64PE.BAS AS A LATER SOMEDAY TASK.

6. **Leave alone:** the crate split, the IR, the C++ emitter as a transcription of the old compiler's, the three
   test tiers, the measurement discipline, and the checker's 72 fields until M4 starts (`study\archive\30` §6, §11).

## 4. What needs the user

Items 1, 4 and 5 are scope decisions for the user. Items 2 and 3 can start on the user's word. None of them
changes the code's direction; they change where the hours go.

**Answered** (`DECISIONS.md` 2026-10-18): item 1 taken (user: "Do step 1"); items 2 and 3 approved; items 4 and 5
taken with the user's notes above (the fidelity built stays; running the compiler made from `qb64pe.bas` is a
someday task). Built: item 3 (`CLAUDE.md` working rules 11, 12, `ROADMAP.md` M4, `SOMEDAY.md`); item 2 waits
behind the three changes of `ROADMAP.md` "After M3".
