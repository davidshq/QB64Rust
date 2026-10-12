# 26. Fifth review: code and plan after the arrays-and-`TYPE` slice (2026-10-07)

Asked by the user: a panel review of the codebase and the plans; no nits, no churn. The panel (`CLAUDE.md` rule 6,
role-played inline): a pragmatic engineer, a QB64 engineer, a compiler/languages engineer, a Rust engineer, a test
engineer, and a **QB64 programmer** (the eventual user; added because two findings are about what reaches users
first).

Evidence: `STATUS.md`, `ROADMAP.md`, `DECISIONS.md`, `study\24`, `study\25`, `crates\README.md`, the IR, emitter and
`sema` sources, the two open changes' `tasks.md`, and the working tree as it stood (the `m2-parser-breadth` group 7
work, `pp.rs` among it, not yet committed). Measured for this review with the debug build of the working tree:

- every upstream and corpus program not on a pass list, not deferred and not an `.err` program, run through
  `--dump typed`, its diagnostics grouped by kind (227 upstream, 136 corpus);
- every such program with no diagnostic at all, run end to end with `run_legacy_tests.py` (5 upstream, 5 corpus).

## 1. Verdict

Direction unchanged; the architecture holds. The IR decision (`study\25`, design D10) worked out as intended,
places went in without strain, and the ratchet lists keep doing their job. The panel found **one wrong-code bug,
the gap in the tests that let it through, one plan premise that is factually stale, and one framing that keeps
37 upstream tests in M3 for no reason**. Each is below with what to do; none needs rework of existing code.

## 2. Wrong code in the working tree: `$IF` and the metacommand flags

`tests\upstream\compile_tests\precomp-flags\consoleonly` compiles with no diagnostic and prints `$CONSOLE
inactive.` where the old compiler prints `$CONSOLE active (with ONLY option).`. The old compiler defines six names
for `$IF` from settings and metacommands (`qb64pe.bas` 1714–1722 at the start, 1980–2006 as `$ASSERTS`,
`$ASSERTS:CONSOLE`, `$CONSOLE`, `$CONSOLE:ONLY` and `$DEBUG` are met): `_EXPLICIT_`, `_EXPLICITARRAY_`,
`_ASSERTS_`, `_CONSOLE_`, `_DEBUG_`, `_SOCKETS_`. The new preprocessor (`syntax\src\pp.rs`) knows only the
predefined names (`WIN`, `VERSION`, …), so these read as undefined, `$IF _CONSOLE_` is false, and the wrong branch
is compiled.

**Do (before the group 7 work is committed):** a `$IF`/`$ELSEIF` condition naming one of the six is "not supported
yet". Model them later, measured first (when they are set relative to the metacommand's line, and what `OPTION
_EXPLICIT` does to `_EXPLICIT_` given that it is program-wide).

## 3. The gap that let it through: clean programs that never run

Tier 2 runs only `slice.list` and `pass.list`; a full corpus run happens at the end of a change. A change that
makes a program compile cleanly does not run it until someone adds it to a list, so "never wrong code" is checked
over the listed programs only. The bug above sits in exactly that gap. The same sweep found four upstream programs
that compile cleanly and **pass** but are not on `pass.list` (`auto_include\no-debug`,
`declare_library_external\test`, `declare_library_external\test_stripped`, `parens()\parens()`): upstream is
28 of 279, not 24.

**Do (test engineer, accepted by all):** tier 1 (`inputs.rs`) computes the set of corpus and upstream programs
that compile without a diagnostic and requires it to be contained in `slice.list` ∪ `pass.list` ∪ a fourth
shrink-only list of programs known not to pass for a recorded reason (today the five corpus `KFAIL` programs:
`PRINT` with a comma and the `RESUME 0` loop). A newly clean program then fails tier 1 until it is run in tier 2
and either added to a pass list or explained. Small, and it makes the invariant cover every program the compiler
accepts.

## 4. A stale premise: member arrays are not behind `$UNSTABLE` any more

`SOMEDAY.md` defers "arrays as `TYPE` members" as "still behind `$UNSTABLE`", and the 2026-10-02 decision defers
them with the rest of the complex array work. In the pinned clone that is no longer true: only the `_STATIC` and
`_DYNAMIC` markers need `$UNSTABLE:TYPEFIELDS` (`qb64pe.bas` 15699, commit `2b7a61850b` "Require
Unstable:TypeFields for type field markers"). A plain `TYPE t: AS LONG a(12, 15): END TYPE` is ordinary QB64pe 4.7
code. `deferred.list` was built from the `$UNSTABLE:TYPEFIELDS` pattern, so it misses them: **55 of the 227
remaining upstream programs report "arrays as `TYPE` members"**, the most common blocker of the yardstick.

The panel splits on what to do; the user decides:

- **(a) Un-defer plain member arrays** (QB64 engineer, compiler engineer): they are standard language now, and
  `Place` already has member-of-element; a member array is the converse (element of member). Keep `_STATIC`,
  `_DYNAMIC` and `$UNSTABLE:TYPEFIELDS` in `SOMEDAY.md`. Schedule them with "the rest of arrays and `TYPE`"
  (step 9). Recommended.
- **(b) Keep them deferred** and move these programs to `deferred.list`, so "x of 279" becomes "x of about 224"
  and stays honest (pragmatic engineer, if the old compiler's member-array layer proves as tangled as `study\02`
  says).

Either way the current denominator overstates what the plan intends to reach.

## 5. `$CONSOLE` tests are not M3 work

`STATUS.md` queues "programs without `$CONSOLE:ONLY`" for M3, with a screen-state oracle. All 37 upstream programs
that use plain `$CONSOLE` also do `_DEST _CONSOLE` (most with `$SCREENHIDE`): their oracle is stdout, like every
other upstream test, and they need no screen-state dump. `$CONSOLE` (34 programs) and `_DEST` (36) are among the
four most common upstream blockers.

**Do:** treat `$CONSOLE` + `_DEST _CONSOLE` (+ `$SCREENHIDE`) as M2 work, measured first; keep the screen-state
oracle for programs that draw. Check once that the GitHub Windows runner can start the hidden window (the local
baseline passes these with `qb64pe.exe`).

## 6. Order of work: what the data says comes next

Blockers of the remaining programs, by kind (debug build of the working tree):

| Yardstick | Programs blocked by exactly one kind | Most common kinds overall |
|---|---|---|
| Corpus (136 left) | 56; of them about 40 by one plain built-in (`LEN`, `VAL`, `STR$`, `MID$`, `ASC`, `SQR`, `STRING$`, `ABS`, `INT`, …) or `SELECT CASE` / `ON … GOTO` | file I/O (`KILL` 23, `OPEN`/`CLOSE` 22, `PRINT #` 16), `LEN` 13, `DATA`/`READ` 10 |
| Upstream (227 left) | 30; `VAL` 5, comment `$INCLUDE` 4, `_TOSTR$` 4, `$CONSOLE` 3 | member arrays 55, `REDIM` 50, `_DEST` 36, `$CONSOLE` 34, implicit arrays 30, `$INCLUDE` 27, no `$CONSOLE:ONLY` 20 |

Upstream programs are feature tests with many blockers each (75 have six or more kinds), so "x of 279" will move
in steps, not steadily; the corpus is the better short-term signal. The arrays slice moved upstream by 1, not
because the `study\24` reasoning was wrong (it was about the model, and the model now holds) but because the
upstream programs need `REDIM`, member arrays and dynamic arrays as well.

**Recommended (pragmatic engineer and QB64 programmer; compiler engineer agrees):** after `m2-parser-breadth`
(in flight; its `$INCLUDE` is worth 27 upstream programs), do **a core built-ins tranche with `SELECT CASE` and
`ON … GOTO`** before step 7. The plain string and math built-ins need no unsigned type and no type table; the
value model they target is stable since D10. Then the thin language server (its outline, folding and go to
definition need only the parser, so it gives users something whatever `sema` supports), then step 7 with the
differential tester, which the remaining ~200 built-ins need most. `$CONSOLE`/`_DEST` (§5) and `REDIM` with member
arrays (§4) follow. Nothing already decided is undone; steps 7 and 8 swap their first part.

## 7. One document point: `ROADMAP.md` is already behind

`ROADMAP.md` (new, untracked) says "Now: the IR review" with 127 corpus and 23 upstream passes, and lists the
arrays slice as to do; `STATUS.md` has the arrays slice closing at 144 and 24. Progress numbers now live in
`STATUS.md`, `ROADMAP.md`, `crates\README.md`, `tests\upstream\README.md` and `study\00`. **Do:** `ROADMAP.md`
keeps milestones and done/not done only, no numbers and no "Now" marker, and points to `STATUS.md` for both.

## 8. Looked at and left alone

- **Rust:** nothing at the level asked. The lints of `study\21` are respected; `codegen-cpp\src\lib.rs` (1,288
  lines) should be split by concern when the built-ins land, not before.
- **Built-ins are hand-checked one by one in `sema`** (`chr`, `instr` in `check\expr.rs`), while the emitter's
  `call` is already generic over the table. Fine for two; step 8 must make the checking side table-driven too,
  with the old compiler's argument conversion measured once per decoded slot type.
- **"Keep" choices accumulate** (eleven implemented as "keep", awaiting step 7). Cheap to change while few; deciding
  them in one sitting at step 7 as planned is fine, but not later than the built-ins, which will add more.
- **Measure-first cost.** High (`verification\v19_proc_names` alone is 1,948 programs) and worth it: §2 is what
  happens where it was skipped.
- **Document volume** against code: `study\24` §3 already addressed the entry point; nothing more.

## 9. Decisions for the user

**Decided 2026-10-07** (`DECISIONS.md`): 1 was already done (the six names are "not supported yet"); 2 yes,
implemented (`tests\known_clean_not_passing.list`; of the four upstream programs, three were already on `pass.list`
and `auto_include\no-debug` is no longer clean, its `$IF _DEBUG_` now marked); 3 option (a); 4, 5 and 6 yes.

1. §2: mark the six `$IF` flag names "not supported yet" before committing group 7. (Bug fix; no real choice.)
2. §3: the tier-1 "clean implies listed" check and the fourth list; add the four upstream programs to `pass.list`.
3. §4: (a) un-defer plain member arrays, or (b) move the 55 programs to `deferred.list`.
4. §5: `$CONSOLE` + `_DEST _CONSOLE` as M2 work.
5. §6: core built-ins with `SELECT CASE` and `ON … GOTO` right after parser breadth, ahead of the language server
   and step 7.
6. §7: `ROADMAP.md` without numbers.
