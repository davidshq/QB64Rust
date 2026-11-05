# 42. Recording variable values: how other languages and tools do it

Written 2026-11-04, for the OpenSpec change `value-trace` (`qb64rust --trace-values`). It answers one question:
do other languages offer a summary of the values each variable takes over a whole run?

**Source of these facts.** They come from Claude's general knowledge, not from a search or from running the
tools. Check them before relying on a detail, such as a version number or which languages a tool supports.

## Summary

Few compilers offer this as a built-in option. The idea is well known, though, and most languages can get it
from a separate tool. The tools fall into four groups:

1. Tools that record the values a variable takes. Daikon is the nearest match.
2. Debuggers that record a whole run and answer questions afterwards (time-travel debuggers).
3. Tracing libraries and editor tools, which mostly write logs, not summaries.
4. Static analysis, which works out the values a variable *could* hold without running the program.

A whole-run summary of every variable, from one compile option, would be unusual for any language, and new for
BASIC as far as we know.

## 1. Recording the values a variable takes

**Daikon** (Michael Ernst's research group; Java, C and C++, C#, Perl, and others):
- It instruments a program and records variable values at the entry and exit of each procedure.
- It then reports patterns that held in every run, such as "`x` is one of {1, 2, 3}", "`i` < `n`" or
  "`p` is never null".
- It is the closest tool to `--trace-values`. Our record holds the same kind of raw data, without the step that
  finds patterns. That step could be added to `tools\trace\value_trace.py` later.

**Value profiling in compilers** (GCC's `-fprofile-values`, LLVM's profile-guided optimization):
- The compiler records the most common values at chosen points, such as the size passed to `memcpy` or the
  target of an indirect call.
- The data feeds the optimizer. A user does not see it as a report.
- Research on value profiling goes back to the late 1990s (Calder, Feller and Eustace, 1997).

## 2. Record the run, ask questions later

**Time-travel or omniscient debuggers** record a whole run, so that any variable can be inspected at any moment
afterwards:
- rr and Pernosco for C, C++ and Rust on Linux; Pernosco can show every change of a variable over time;
- WinDbg's Time Travel Debugging on Windows;
- UndoDB (Undo);
- Replay.io for JavaScript in the browser;
- Chronon for Java (no longer sold).

They are more powerful than a summary: they keep the order of values and every element of every array. They are
also heavy. A recording is large, most work on one platform only, and they answer questions one variable at a
time instead of giving a summary of all of them.

## 3. Tracing libraries and editor tools

- **Python** has hooks in its interpreter: `sys.settrace`, and `sys.monitoring` since Python 3.12. Libraries
  built on them, such as PySnooper, snoop and birdseye, log each change of a variable in a chosen function.
- **JavaScript** editor tools such as Quokka and Wallaby show values beside the code as it runs.
- **Debugger logpoints** print a message at a chosen line without stopping. Examples are VS Code's logpoints,
  gdb's `dprintf` and gdb's tracepoints. They log what is asked for at the lines chosen, not every store.

These give logs over time, not a summary per variable. They need the user to pick functions or lines first.

## 4. Static analysis: the values a variable could hold

This answers a different question: not "which values occurred in these runs", but "which values are possible in
any run". These tools use abstract interpretation and are mostly used for safety-critical C code, such as
aviation and car software:
- **Frama-C**, with its Eva plug-in ("evolved value analysis"), gives a range or set of values for every variable
  at every point of the program.
- **Polyspace** (MathWorks) shows a variable's possible range when the pointer rests on it.
- **Astrée** proves the absence of runtime errors, such as overflow or division by zero, from the same ranges.

Their answers are safe but can be loose: a range may include values that never happen. For QB64Rust this would be
a separate change, built on the IR. `value-trace` leaves it out of scope (its proposal, "Out of scope").

## 5. BASIC's own history

- **QuickBASIC 4.5 and QBasic** had `TRON` and `TROFF`, which trace execution line by line. QuickBASIC 4.5's
  debugger also had Watch, Instant Watch and Watchpoints.
- **QB64pe's IDE** has a watch list for a program built with `$DEBUG`.

All of these show values live, while the user watches. We know of no BASIC with a summary of values over a whole
run.

## 6. What this means for `value-trace`

- The design follows well-known practice: the instrumentation of coverage tools and of value profiling, and the
  kind of data Daikon records. The one addition is the list of writes that cannot be tied to a variable, such as
  `POKE`, so that the report says where it is incomplete.
- Two later extensions have a clear model elsewhere:
  - patterns over the recorded values, as Daikon finds them;
  - static ranges, as Frama-C's Eva gives them.
  Neither is part of `value-trace`.
