# 37. M6, runtime modernisation: what it is for, and the twelfth review (2026-10-26)

Asked by the user after M5 was declared done and branch `mb5` (M5b) was ready to merge: explain M6 and why it is
needed, then draft a study of it. The review that comes with a merge into `main` (`CLAUDE.md` working rule 11) is
§7, as `study\36` held the eleventh.

The panel is role-played inline (working rule 6): the pragmatic engineer (PRAG), the QB64 engineer (QB64), the
compiler/languages engineer (COMP), the Rust engineer (RUST), the test engineer (TEST), and two extra roles because
M6 is about the runtime and who uses it: the runtime engineer (RT: libqb's threads, strings and error path) and a
user advocate (USER: someone who writes BASIC programs and installs the extension, since M6 is justified, if at
all, by what reaches such a person). Effort is counted in Claude's time (working rule 8).

Evidence read: `ROADMAP.md` (M6), `DECISIONS.md`, `study\00` §2 and §8, `study\03` §1.5, §2.1, §2.10, §9,
`study\07` sessions 7 and 8 (R6, R7), `study\archive\30` §7, the IR's statement of its error rules
(`crates\ir\src\lib.rs`), the emitter's event and error dispatch (`codegen-cpp\src\events.rs`, `decl.rs`),
`tools\libqb\edits\`, `SOMEDAY.md` ("QB64pe behaviours to review"), `vscode\README.md` (what a user installs).
Run (§3): two probe programs built by the new compiler; counts over `libqb\`.

## 1. Verdict in one paragraph

M6 is one line in the roadmap: "the ABI owned by the new compiler; error model, string heap and threading
redesigned behind golden tests", started as a second backend beside `codegen-cpp`. It was written on 2026-10-02
from a reading of libqb's source (`study\03` §9.2) before anything was measured, as the place to put the
runtime's design debts. Measured now, **none of the three debts shows as a failure a user meets**: a program that
traps a million errors runs to its end (§3.2), the string heap's moves are invisible from BASIC, and the flaky
results the test suites see come from the machine (sound devices, the clipboard, console hand-over), not from
libqb's flags. Meanwhile **the IR fixes the observable error behaviour** as QB64pe's pending-error rules, measured
program by program, so a new error mechanism would have to reproduce them exactly and could change nothing a
program sees. A second backend and a new runtime would double what every change must keep working, for a gain
inside the runtime only. The panel recommends **not** starting M6 as written. It recommends replacing it with
two smaller things that do reach users (§5): repairing libqb's bodies in place through the edits mechanism that
already exists (`tools\libqb\edits`, ABI unchanged, so every proof stands), and removing the last need for an
installed QB64pe (its toolchain and `lldb-dap`). The rewrite stays in `SOMEDAY.md` with the conditions that would
justify it (§5.4).

## 2. What M6 says, and where it came from

- `study\07` session 7 (RT, 2026-10-02): leave the vendored libraries and the bit-exact units alone; fix early the
  `#define int32` macros and the `static` locals; **redesign later, only when the new compiler owns the ABI**: the
  error model, the moving string heap, the thread synchronisation built on non-atomic flags.
- `study\07` R6: the first backend targets libqb's ABI; "change the ABI only once the old compiler no longer
  produces code for it". R7: keep a C++ backend while `DECLARE LIBRARY` with C/C++ headers and `SUB _GL` are
  supported.
- `study\archive\30` §7 (2026-10-16): `codegen-cpp` is pinned to libqb's ABI by the call-site check, so M6 must
  start as a second backend crate; it is never modernised in place.

So M6 has a mechanism (a second backend) and a list of debts, but no "done when" and no user-facing goal. Every
other milestone has one.

## 3. What exists today (measured 2026-10-26)

### 3.1 The runtime and the ABI

- libqb: `libqb.cpp` 27,119 lines, `qbx.cpp` 1,608, the `libqb\` modules 25,077, besides the vendored libraries.
  892 indented `static` declarations in `libqb.cpp` (the `study\03` count still holds); one `std::atomic`.
- The ABI is pinned by the call-site check: 2,649 programs, one per built-in form and argument type, each
  emitting the same libqb call as `qbx.cpp` from `qb64pe -z` (the full set of this merge).
- The copy of libqb is not frozen: `tools\libqb\copy_libqb.py` applies listed edits, each with its reason and a
  `DIVERGENCES.md` row. One exists (`d031-coth.edit`, `_COTH`). **An edit of a function's body leaves the ABI and
  the call-site check untouched.**

### 3.2 The error model

What the earlier studies said: `error()` returns and every function checks `is_error_pending()` (111 places in
`libqb.cpp`); a trapped error calls `QBMAIN` again "and never unwinds", so "programs that trap many errors" grow
the native stack without bound (`study\03` §9.2 item 1).

Measured: a program that raises and traps error 5 a million times in the main module, `RESUME NEXT` each time,
prints `done 1000000` and ends normally; so does one that raises it a million times inside a SUB with the handler
in the main module. `RESUME` returns out of the re-entered `QBMAIN`. The unbounded growth needs a handler that
never resumes and is re-entered, which is an error of its own in QuickBASIC terms. Not a failure a user meets.

What the IR states (`crates\ir\src\lib.rs`, the module comment): errors are **pending**, the first of a
statement is the one serviced, a store with a placeholder value still happens, `PRINT` stops at a raising item,
`READ` does not, a raising `IF` condition runs the body, a raising `FOR` header runs the body with the variable
unassigned, and so on, each measured (`verification\v17`, `v18`, `v22`, `v24`, `v35`). These are the
observable rules of QB64pe's error model, and the suites hold the new compiler to them. A runtime with real
unwinding would need to reproduce every one of them on top of it. The redesign could change only the inside.

### 3.3 The string heap

The heap moves every string's bytes on a compaction (`study\03` §2.1). Seen from BASIC: nothing, but the address
of a variable-length string's bytes (`SADD`), which in QuickBASIC too was good only until strings moved (from
memory of QuickBASIC's help, not checked here). The hazard is inside libqb: a C pointer held across an allocation. No failure of the
suites has been traced to it.

### 3.4 Threads

The flags without atomics (`study\03` §1.5) rely on x86's memory order. The unstable results the suites record
are, by their own investigations, the machine's: `PLAY(0)` read while the queue drains (40 or 41, the old
compiler's build too), `_SNDLEN` at the device's sample rate, the clipboard watcher holding the clipboard, a
`SHELL` waiting on Windows Terminal's hand-over, and today the OpenGL viewport set by the desktop (`STATUS.md`,
"Still to watch" and "Known bugs"). On ARM the flags would be a real risk; the project builds for `win32-x64`
only.

### 3.5 What a user needs installed

The extension ships `qb64rust` and the copy of libqb, but **a QB64pe installation is still required**: its C++
toolchain builds every program and its `lldb-dap` runs under the debugger (`vscode\README.md`, "Install";
`DECISIONS.md` 2026-10-19). This is the one dependency on QB64pe that a user sees.

### 3.6 What users would see changed

`SOMEDAY.md`'s "QB64pe behaviours to review" holds about 165 rows. Many of them are libqb behaviours kept QB64pe's
way: an `INPUT` that never asks again, handles that raise nothing, `_MEMEXISTS` after `ERASE`, `_MEMFREE` of a
zero-sized block ending the program. Each can be fixed with an edit of a body, a divergence row and its test,
without a second backend.

## 4. The options

| | What | Claude's time | Cost that stays | Risk |
|---|---|---|---|---|
| A | M6 as written: a second backend and a runtime that owns its ABI (Rust or C++), the three debts redesigned | very large: tens of changes, a runtime of about 50,000 lines in part rewritten, every program family run through both | two backends and two runtimes kept equal by every later change; tier 2 run twice | high: the bit-exact units, timing, graphics; parity is measured again from zero for the new backend |
| B | Repair libqb in place: edits of bodies through `tools\libqb\edits`, each with its row and test; the `static` locals and flags where a failure is shown, the questionable behaviours the user rules on | small per item; a change per group of rows | the edits are kept as the clone moves (`copy_libqb.py --check` says when one no longer applies) | low: the ABI and the call-site check do not move |
| C | No more QB64pe install: a toolchain the project fetches (pinned, checksummed: `study\07` R13) and its own `lldb-dap`, with the libqb objects built on first use as now | moderate: download, verification, the extension's first-run flow, CI | a toolchain version to keep pinned | medium: a toolchain change can change float results (`study\00` §8, `printf`), held by the full suites |
| D | Nothing: M5b is the last milestone; the project is maintained against QB64pe's releases | none | none | none |

Other platforms (Linux, macOS) are not on the list: libqb already supports them, so they need the driver and
the extension, not a new runtime. They are a separate decision (`DECISIONS.md` 2026-10-03: "macOS/Linux later").

## 5. The panel

**USER:** What I notice as a user: I still have to install QB64pe to use this, and a handful of things
misbehave the QB64pe way (the `SOMEDAY.md` list). I would never notice a new string heap.

**RT:** The three debts are real in the source. They are not shown in a program. I wrote the 2026-10-02 list from
reading. The million-error probe answers the worst item. The `static` locals matter where `SUB _GL` and event
handlers re-enter libqb, and that is reached today only under the display lock, which serialises it.

**QB64:** The behaviours are the product. Every one the suites measure is in the IR now, so a new runtime buys
freedom we have already decided not to use (`DECISIONS.md` 2026-10-08: a deterministic behaviour that changes
results is kept). Where we do want a change, an edit of the body is the honest way: one row, one test.

**COMP:** R6 said "change the ABI once the old compiler no longer produces code for it". That day has not come,
and the call-site check is the strongest proof we have. A second backend gives it up for those programs. I
wanted the ABI redesign in 2026-10; the measurements say it would be done for its own sake.

**RUST:** A Rust runtime is attractive (memory safety, one language), but libqb's useful half is C++ libraries
(audio, images, fonts, GLFW) and `SUB _GL` and `DECLARE LIBRARY` keep C++ in every build (R7). We would end up
with a Rust layer over the same C++, in two languages instead of one.

**TEST:** Option A doubles tier 2's machine time for every change from then on, and the parity measured over
three weeks would be measured again. B keeps the suites as they are; each edit comes with its program. C is held
by the full set at a merge: the suites already run the toolchain on every program.

**PRAG:** Effort is not the argument (rule 8): A is large even in Claude's time, but the reason against it is the
cost that stays (two backends to keep equal, twice the machine time) for no change a user can see. B is the
same work in small pieces, each showing its benefit. C removes the one dependency users meet. Do C first, then B
by the user's rulings on the `SOMEDAY.md` rows.

### 5.1 Recommendation

1. **Rewrite M6 in `ROADMAP.md`** as "the runtime without QB64pe": (a) a toolchain and `lldb-dap` the project
   provides, so the extension needs no QB64pe install (option C); (b) libqb repaired in place through
   `tools\libqb\edits`, by the user's rulings on "QB64pe behaviours to review" and wherever a failure is shown
   (option B). Done when: the extension builds, runs and debugs a program on a machine with no QB64pe; and the
   rows the user ruled to change are edits with their tests.
2. **Move the second backend and the three redesigns to `SOMEDAY.md`**, with the conditions that would bring
   them back: a platform where the flags fail (ARM), a failure traced to the string heap or the error re-entry,
   or a decision to diverge from QB64pe's observable error rules.
3. **Keep `study\archive\30` §7's rule** (no modernising of `codegen-cpp` in place): it still holds for any ABI
   change.

### 5.2 The first change, if the user takes (a)

`m6-toolchain`: the driver finds a toolchain in this order: `qb64rust.toolchain` (setting), one the extension
fetched into its storage, QB64pe's. The fetch takes a pinned llvm-mingw release by URL and SHA-256 (the version
QB64pe 4.7.0 ships, so float results stay the same; checked by the full set), unpacks it, and builds libqb's
objects at first use as now. `qb64rust dap` takes `lldb-dap` from the same toolchain. Tests: the CLI tests with a
toolchain folder named by the setting; the extension's tests of the first-run flow with the download faked.
What a user downloads grows by the toolchain (hundreds of MB) once, on first build, not in the `.vsix`.

### 5.3 What option B looks like

A change takes a group of rows the user has ruled on (for example, the handle functions that raise nothing), and
for each: the edit in `tools\libqb\edits\`, a `DIVERGENCES.md` row (and `UPSTREAM.md` when it is QB64pe's bug), a
slice or verification program showing the new behaviour, and the old one recorded beside it. Nothing waits on
the user except the rulings themselves, which are the list working rule 11 keeps silent until asked.

### 5.4 What would bring A back

Any of: a target where the flags' reliance on x86 memory order fails (ARM Windows or macOS on Apple silicon);
a reproduced failure from the string heap or the error re-entry; a decision to give BASIC a different error model
(structured handling, `TRY`); or QB64pe changing its ABI in a way the copy cannot follow.

## 6. Risks of the recommendation

- The toolchain is large and its download can fail; the extension must fall back to a QB64pe install and say why.
- A newer toolchain could change float printing: the pin stays at QB64pe's version until a full set says otherwise.
- Edits of libqb accumulate. `copy_libqb.py --check` shows when one no longer applies after the clone moves; each
  is small and has its own reason.

## 7. The twelfth review: M5 and M5b

What was built since the eleventh review: M5's three changes (debug build and symbol file, the DAP proxy over
`lldb-dap`, the extension's launch) and M5b's three (the formatter, its layout settings and the format oracle,
the language-server features). The panel reads them as on course; the points below need a change.

1. **A test that assumes a thread has started** (TEST). `stale_index_job_skipped` failed under the load of the
   full set: the worker had not started the first index job when the text changed, so that job was skipped too.
   Fixed in this merge (it waits for the job to start). The language-server tests that use a slow index
   (`edit_while_waiting`, `cancel_while_waiting`) were read: they hold under either order. Rule for later tests of
   the server: a test that needs a job running waits for it to start, never for a time.
2. **A machine-dependent recording** (TEST, QB64). `verification\v59_g_readback` recorded the OpenGL viewport,
   which the desktop sets; the old compiler's own build reads another size today. A `.normalize` sidecar masks it
   now. The family's other programs that print a viewport read back one they set themselves, and passed.
3. **The debugger and the formatter still depend on QB64pe's install** (USER): `lldb-dap` from its toolchain. This
   is §5.1 (a).
4. **`STATUS.md` had grown again** (PRAG): ten blocks of numbers from earlier merges. Trimmed in this merge to the
   last two, as its own rule says. The table of the M3 steps stays (a catalogue, not history); it could move to
   `ROADMAP.md` at the next edit there.

Nothing else is astray: the suites passed on the merged tree with one machine failure (`STATUS.md`).

## 8. What the user decides

1. **M6 rewritten as "the runtime without QB64pe"** (§5.1 items 1 and 2), or M6 as written (option A), or nothing
   more (option D)?
2. If rewritten: **the toolchain first** (§5.2), or the libqb repairs first?
3. For the repairs: when to go through `SOMEDAY.md`'s "QB64pe behaviours to review" (about 165 rows), all at once
   or by area as each change comes up?
