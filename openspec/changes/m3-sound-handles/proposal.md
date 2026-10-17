# Proposal

## Why

The next step of M3 is the user's choice (`STATUS.md`): **`_SNDOPEN` and the `_SND*` family** (user, 2026-10-16).
They are the first line of the upstream demand report: `_SNDOPEN` holds back 6 programs, `_MIDISOUNDBANK` 4, and
`_SNDCLOSE`, `_SNDNEW`, `_SNDCOPY`, `_SNDLEN` and `_SNDLOOP` the rest. Eight upstream programs name one of them, and
for seven of them nothing else is held back (`qb64rust --report unsupported` on each, 2026-10-16):

| Upstream program | Names held back | After this change |
|---|---|---|
| `audio_handle_test/handle_test` | `_SNDOPEN`, `_SNDCOPY` | can pass |
| `audio_mem_test/newsound_test` | `_SNDNEW`, `_SNDCLOSE` (`_MEMSOUND` compiles already) | can pass |
| `audio_mem_test/sndopen_mem_test` | `_SNDOPEN` (from memory), `_SNDLEN` | can pass |
| `audio_test/test` | `_SNDOPEN`, `_SNDLOOP`, `_SNDCLOSE` | can pass |
| `console_only/midi` | `_MIDISOUNDBANK`, `_SNDOPEN` | can pass (its `.mid` and `.sf2` lie in the clone) |
| `midi/midisoundbank_file_test` | the same | can pass |
| `midi/midisoundbank_memory_test` | the same | can pass |
| `midi/midisoundbank_embed_test` | the same and `$EMBED` | stays held back by `$EMBED` |

Read and tried before this proposal (the old compiler's C++ with `qb64pe -z`, two programs built and run twice,
outside the repository; the clone's `git status --porcelain` empty before and after; the facts go into
`study\00` §5 with the measurement programs in task 1.1):

- **Every form is a plain call** of its row and template, as `SOUND` and `PLAY` were
  (`func__sndopen(__STRING_W,qbs_new_txt_len("memory",6),0|1)`, `func__sndnew( 800 , 1 , 16 , 8000 ,0|1|2|4)`,
  `sub__sndbal(*__LONG_H, 1 ,NULL,NULL,NULL,1)`, `sub__sndplayfile(…,NULL, 0 ,2)`, `func__sndrawlen(*__LONG_R,0|1)`).
  The old compiler checks two of them itself (`qb64pe.bas` 11809–11847): `_WAVE`'s array must be a `_BYTE`
  array and `_SNDRAWBATCH`'s a `SINGLE` array. `$MIDISOUNDFONT` is an error ("a deprecated keyword, use
  `_MIDISOUNDBANK` instead", `qb64pe.bas` 2092).
- **All of them need the audio part** (`DEPENDENCY_MINIAUDIO`, 27 names beside `SOUND` and `PLAY`), which
  `m3-sound` brought: the same make flag, the same `audio.a`, MIDI and its default sound bank included.
- **No handle function raises a run-time error.** A handle that is not open makes a statement do nothing and a
  function give 0 (`IsHandleValid` in `audio.cpp`); a file that cannot be opened gives handle 0.
- **What a program can observe is exact and repeatable, with nothing heard.** A WAV built in a string and opened
  with `_SNDOPEN(w$, "memory")` gave its handle, `_SNDLEN` (0.5), `_SNDGETPOS` after `_SNDSETPOS` (0.25),
  `_SNDCOPY`, and `_SNDPLAYING` and `_SNDPAUSED` after `_SNDPLAY`, `_SNDPAUSE`, `_SNDSTOP` and `_SNDLOOP` at volume 0,
  equal in two runs. A MIDI file built in a string opens with the default sound bank and has an exact length (1 s).
  `_SNDNEW` with `_MEMSOUND` gives the block's size, type and element size. So the tests need no binary file.
  Two values depend on the machine or on time: `_SNDRATE` (the device's rate) and `_SNDRAWLEN` (the queue drains
  while the program runs).
- **Two behaviours look questionable:** a sound just opened, or just stopped, has `_SNDPAUSED` -1; `_SNDLEN` of a
  `_SNDNEW` sound is a SINGLE widened to DOUBLE (`.1000000014901161` for 800 frames at 8000 Hz).

## What Changes

- **Rows for the whole audio family** in the form the old compiler accepts each: the functions `_SNDOPEN`,
  `_SNDCOPY`, `_SNDLEN`, `_SNDGETPOS`, `_SNDPLAYING`, `_SNDPAUSED`, `_SNDRATE`, `_SNDOPENRAW`, `_SNDRAWLEN`,
  `_SNDNEW`; the statements `_SNDCLOSE`, `_SNDPLAY`, `_SNDPLAYCOPY`, `_SNDPLAYFILE`, `_SNDPAUSE`, `_SNDSTOP`,
  `_SNDLOOP`, `_SNDVOL`, `_SNDBAL`, `_SNDSETPOS`, `_SNDLIMIT`, `_SNDRAW`, `_SNDRAWBATCH`, `_SNDRAWDONE`,
  `_MIDISOUNDBANK` and `_WAVE`. The forms the old compiler rejects are errors that are not "not supported yet",
  the array of `_WAVE` and of `_SNDRAWBATCH` included. `_WAVE` is not a `_SND` name; it comes in because it is the
  last built-in of the audio part and needs nothing the others do not (design D1).
- **`_MEMSOUND` is run by a test** and leaves the list of built-ins no test runs: `_SNDNEW` now gives it a handle.
- **`$MIDISOUNDFONT`** is the old compiler's error, no longer "not supported yet".
- **Tests of sound handles that make no noise**: console slice programs that build their WAV and MIDI data in
  strings (and a WAV file written by the program for `_SNDOPEN` from a file and `_SNDPLAYFILE`), play only at
  volume 0 or samples of 0, and print what is exact; `_SNDRATE` and `_SNDRAWLEN` only against one-sided limits. A
  call-site program per accepted form; front-end tests; the shrink-only lists regenerated; the upstream programs
  above run and added to `tests\upstream\pass.list`.

## Capabilities

### New Capabilities

None. The sound handles are requirements of `language/sound`.

### Modified Capabilities

- `language/sound`: the handle functions and statements, sounds from a file, from memory and new, raw sound,
  MIDI sound banks, `_WAVE`, `$MIDISOUNDFONT`; what each gives a program, with no run-time error for a handle
  that is not open.
- `language/builtin-statements`: the statements join the list of supported statements.
- `language/builtin-functions`: the functions join the list of supported functions.
- `language/mem-blocks`: `_MEMSOUND` is run by a test: the block of a `_SNDNEW` sound.
- `testing/compiler-tests`: `_MEMSOUND` leaves the built-ins no test runs.
- `compiler/pipeline`: a program that names any built-in of the audio part, not only `SOUND` or `PLAY`, is built
  with the audio make flag.

## Impact

- Changed: `crates\sema` (`builtins.rs`: the rows, `NEVER_RUN` without `_MEMSOUND`; `check\builtins.rs`: the
  element type of the arrays of `_WAVE` and `_SNDRAWBATCH`; `check\mod.rs`: `$MIDISOUNDFONT`), the shrink-only
  lists, `tests\upstream\pass.list`; new `tests\callsite\snd*.bas`, `tests\frontend\sound_handles_*.bas`,
  `tests\corpus\slice\s127_*` and onwards, `verification\v39_*`. The IR, the emitter and the driver only where a
  form does not come out as the old compiler's (none expected: `RuntimePart::Audio` and its flag exist).
- No change to libqb or the reference clone: the audio part is built already.
- Not in this change: `$EMBED` and `_EMBEDDED$` (so `midisoundbank_embed_test` stays held back); checking what is
  heard; the behaviour on a machine without a sound device (every function gives 0 there; see the design's
  risks); the other first lines of the demand report (`_ARRAYCOPY`, `$VERSIONINFO`, the event forms).
- `STATUS.md`, `ROADMAP.md`, `crates\README.md`, `tests\corpus\README.md`, `tests\upstream\README.md`, `study\00`
  §5, `SOMEDAY.md` (the row of the other sound built-ins goes; the questionable behaviours above), and
  `CLAUDE.md`'s layout table where a count or a family is named, updated when done.
