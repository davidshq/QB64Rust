# Time spent testing

How much of the work on this project goes into running tests, measured from what was run and not from memory
(user, 2026-10-13: "track each test you run and how long it takes so we can get an idea over time of how much time
we are spending testing vs coding").

```
python tools\testtime\test_time.py
```

| File | Content |
|---|---|
| `test_time.py` | Reads the transcripts of the Claude Code sessions and writes the three files below |
| `runs.csv` | One row per run: `session` (the first 8 characters of its id), `start` (local time), `bucket`, `kind`, `seconds`, `background` (1: not waited for), `result` (`pass`, `fail`, or empty: not known), `repeat_after` (empty, or the result of the run this one repeats: `pass`, `fail`, `unknown`) |
| `sessions.csv` | One row per session: `start`, `end`, `active_seconds`, `command_seconds` |
| `REPORT.md` | The summary: in all, by day, by kind of run with results, runs repeated with nothing edited, the 20 longest runs. Made from the two CSV files; not edited by hand |
| `test_record.py` | Tests of the script's rules (`python -m unittest discover -s tools/testtime -p "test_*.py"`): a change to what counts as a run, or to what is read as its result, comes with a change there |

## Where the numbers come from

Claude Code keeps a transcript of every session on the machine it ran on, outside this repo: one file per session,
a line per message, each with its time. A command Claude runs is a line when it starts and a line when its result
comes back; the difference is how long Claude waited for it. So every test run is on record by having been run:
nothing is added to the tests, and nothing has to be remembered while working. The history goes back to the
first session (2026-10-02) for the same reason.

The script finds the transcripts of sessions started in this repo and in the folder above it (which shares its
`CLAUDE.md`); `--transcripts <folder>` names another place. It writes no command text, path or prompt: only the
kind of each run and its times.

**Claude Code deletes old transcripts.** A session whose transcript is gone keeps its rows in the CSV files: the
script replaces the rows of the sessions it can read and leaves the others. So the files must be committed, and
the script run before the transcripts of a period are gone: **at the close of each change** (`CLAUDE.md` working
rule 9).

## Kinds

A command counts by what it starts (`classify` in the script; `--check` classifies commands given on standard
input and writes nothing):

| Bucket | Kinds |
|---|---|
| `test` | Checks of this project's own work: tier 1 (`cargo test`, whole or a part), `clippy`, `fmt`, the corpus, upstream and differential runs with the new compiler (slice list, pass lists, selected programs, full suites), the call-site check and its self-test, the tests of the Python tools, the extension's tests, the repo check, `openspec validate` |
| `measure` | Finding out what QB64pe does: the verification programs, recording expected results, any suite run with the old compiler, the old compiler run directly |
| `build` | `cargo build`, the extension's build. A `cargo test` builds too; that time is in the test |

A command line is read statement by statement (split at `;`, `&&`, `||` and line ends), so two runs of the
runner on one line are two rows, each of its own kind. Not a run: a tool asked for its options (`--help`), a
script's name given to another program (`git diff -- tools\callsite\callsite.py`), `cargo` in quotes (`grep 'cargo
test'`, a commit message), and the text of a heredoc. An option is read up to the first pipe only: `| grep -h`
does not make a run a call for help. A shell variable set by a statement of its own (`R="python
tools/legacy_tests/run_legacy_tests.py …"`) stands for its value in the statements after it: `$R --suite corpus
--list …` is a run of its kind, and the assignment is none (before 2026-10-14 the assignment was counted as a
full corpus run and the runs through `$R` were missed: 23 commands of 9 sessions). A value with quotes or a `;`
inside it is not read as one: such an assignment is still counted as the run it names, and the runs through it
are missed. A corpus run with a `--corpus-root` of its own (a few programs made for a
trial) is "corpus, selected programs".

A command that starts several runs (`cargo fmt; cargo clippy; cargo test` on one line) is one row per run. The
transcript has one time for the whole line; it is split between the runs in proportion to what each kind takes
when it is run alone in the same session (the median of those runs), and evenly where the session has no such
run for one of them. Only the session's own runs are used, so the rows of a session that is over stay as they
are when the record is made again.

## Results

`result` is what the transcript shows of a test run or a build; a measurement has none.

- **By its output**, where the run says it: the runner's summary line (`"FAIL": 0`), `test result: ok` or
  `FAILED` of `cargo test`, the counts of the call-site check and of the repo check, `is valid` of `openspec
  validate`, `OK` or `FAILED (` of the Python tests, `passing` and `failing` of the extension's tests, a compile
  error, a clippy warning, `Diff in` of `cargo fmt --check`.
- **By the exit code**, only when the command is that one statement with no pipe after it: with a pipe
  (`… | tail -3`) or another statement behind it, the exit code is not the run's.
- **Empty (not known)** otherwise: the output was cut down to lines that say nothing either way (`cargo test |
  grep FAILED` shows nothing after a pass, and nothing after a compile error), went to a file, or the run was in
  the background (its output is not in the transcript; only its exit code is, and that counts as above).

Runs of one line share one output: a `FAIL` line of one of two corpus runs marks both. A compile error beside
no line of a test marks `cargo test` as failed only when it is the one run of its line; beside a clippy or
build run the error may be that run's, and the tests' result stays empty.

## Repeats

`repeat_after` is set when the same session had run the same statement before, word for word (a pipe and what
follows it left out), and no file was edited between the two by the Edit or Write tool. Its value is the result
of the earlier run. A repeat after a pass had nothing new to check, as far as the transcript shows; a repeat
after a failure is a second look at the same failure (often with another filter on the output).

An environment variable set by a statement of its own before the run (`$env:QB64RUST_NO_FOLD='1'`, `export X=1`)
is part of the statement until it is removed, so the slice list without folding is no repeat of the one with it.

What it does not see: a file changed by a shell command or by the user, a `git` operation, a build cache that
was filled or cleared. So the table "Runs repeated with nothing
edited" is an upper bound of the runs that could have been left out, and it does not count a rerun of the same
tests by a different command line.

## Reading the report

- **Active time** is the time Claude worked: the time inside commands, plus every gap between two lines of a
  transcript that is no longer than 5 minutes (reading, thinking, writing). A longer gap is the user being away
  and counts as nothing. It is machine time of the sessions, not the user's time.
- **Waiting for test runs** is the time of the test runs Claude waited for. That, as a share of the active time,
  is the number the question asks for. Everything that is not a test, a measurement or a build is in "everything
  else": reading code, thinking, writing code and documents, and every other command. The transcripts do not say
  which of that is "coding".
- **In the background** are runs started without waiting for them. Their time is not part of "waiting": Claude
  went on working meanwhile. It is machine time, and it is in the totals of "By kind of run".
- **By day**: a run is on the day it started; a session's active time is on the day the session started.

## What the numbers cannot say

- **A test run started through a script file is not seen.** Only the command line is read: `python
  tools\legacy_tests\run_legacy_tests.py …` is a run, a scratch script that calls it is not. So start test runs by
  their own command, or the record misses them (the closing runs of `m3-get-put-bload` on 2026-10-13 went through
  such a script and are missing: about 15 minutes in the background).
- **Waiting for the user to allow a command is inside its time.** The transcript has no time for the moment a
  command really started. One whose result came later than its time limit after it was asked for (ten
  minutes, or the limit the command asked for when that is longer) is left out
  altogether (it waited for a permission); a shorter wait cannot be told from running. A command the user
  refused is no run.
- **A command that reaches its time limit is not stopped: it goes on in the background** (the limit is two
  minutes unless the command asks for more; five and ten were asked for). Such a run is two rows: the time waited
  for, and the rest in the background, which carries the result.
- **A background run counts until its end was reported** to the session; one whose session ended first is left
  out.
- **Which change a run belonged to** is not recorded.
- Sessions of subagents are part of their parent's transcript and counted with it.
