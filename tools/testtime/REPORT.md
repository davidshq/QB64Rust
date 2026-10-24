# Time spent testing

Made by `python tools\testtime\test_time.py` from the transcripts of the Claude Code sessions; do not edit.
What the columns mean, and what the numbers cannot say: `README.md` beside this file.

407 sessions from 2026-10-02 to 2026-10-24; 7775 runs on record.

## In all

| | Time | Share of active time |
|---|---|---|
| Active time (Claude working, the user not waited for) | 145 h 15 min | |
| Inside commands of any kind | 64 h 26 min | 44% |
| **Waiting for test runs** | 27 h 50 min | 19% |
| Waiting for measurements with the old compiler | 4 h 20 min | 3% |
| Waiting for builds | 4 h 50 min | 3% |
| Everything else (reading, thinking, writing code and documents, other commands) | 108 h 13 min | 75% |
| Test runs and measurements in the background (not waited for) | 24 h 45 min | |

## By day

| Day | Sessions | Active | Tests | Measurements | Builds | Tests, share | Test runs | In the background |
|---|---|---|---|---|---|---|---|---|
| 2026-10-02 | 7 | 4 h 30 min | 1 min 22 s | 1 min 58 s | 1 min 59 s | 1% | 6 | 44 min 13 s |
| 2026-10-03 | 14 | 5 h 19 min | 36 min 27 s | 16 min 24 s | 4 min 57 s | 11% | 153 | 1 h 39 min |
| 2026-10-04 | 14 | 6 h 31 min | 1 h 33 min | 10 min 40 s | 26 min 43 s | 24% | 297 | 5 min 57 s |
| 2026-10-05 | 13 | 3 h 39 min | 33 min 23 s | 2 min 21 s | 9 min 10 s | 15% | 129 | 0 s |
| 2026-10-06 | 7 | 2 h 12 min | 35 min 09 s | 1 min 42 s | 3 min 34 s | 26% | 114 | 0 s |
| 2026-10-07 | 26 | 9 h 40 min | 1 h 01 min | 26 min 50 s | 14 min 09 s | 11% | 415 | 1 h 24 min |
| 2026-10-08 | 13 | 3 h 56 min | 29 min 47 s | 15 min 14 s | 7 min 03 s | 13% | 231 | 20 min 48 s |
| 2026-10-09 | 24 | 5 h 51 min | 56 min 22 s | 1 min 58 s | 7 min 57 s | 16% | 290 | 58 min 32 s |
| 2026-10-10 | 19 | 7 h 30 min | 40 min 16 s | 1 h 06 min | 3 min 53 s | 9% | 279 | 1 h 07 min |
| 2026-10-11 | 16 | 7 h 55 min | 1 h 55 min | 18 min 36 s | 19 min 51 s | 24% | 347 | 31 min 36 s |
| 2026-10-12 | 25 | 8 h 27 min | 1 h 41 min | 15 min 16 s | 8 min 46 s | 20% | 300 | 34 min 11 s |
| 2026-10-13 | 17 | 5 h 09 min | 1 h 11 min | 11 min 58 s | 11 min 22 s | 23% | 327 | 9 min 39 s |
| 2026-10-14 | 39 | 10 h 59 min | 2 h 17 min | 15 min 48 s | 23 min 38 s | 21% | 379 | 48 min 47 s |
| 2026-10-15 | 22 | 5 h 48 min | 1 h 02 min | 3 min 56 s | 8 min 42 s | 18% | 210 | 11 min 07 s |
| 2026-10-16 | 28 | 8 h 06 min | 2 h 03 min | 7 min 46 s | 5 min 23 s | 25% | 292 | 2 h 30 min |
| 2026-10-17 | 30 | 11 h 13 min | 1 h 14 min | 9 min 48 s | 15 min 28 s | 11% | 348 | 3 h 05 min |
| 2026-10-18 | 8 | 2 h 23 min | 39 min 06 s | 52 s | 7 min 02 s | 27% | 121 | 35 min 46 s |
| 2026-10-19 | 20 | 7 h 03 min | 1 h 40 min | 7 min 24 s | 13 min 39 s | 24% | 373 | 2 h 27 min |
| 2026-10-20 | 15 | 3 h 50 min | 1 h 10 min | 40 s | 8 min 59 s | 31% | 208 | 55 min 01 s |
| 2026-10-21 | 20 | 9 h 24 min | 2 h 35 min | 8 min 12 s | 23 min 36 s | 27% | 437 | 3 h 07 min |
| 2026-10-22 | 14 | 6 h 41 min | 1 h 45 min | 12 min 08 s | 22 min 27 s | 26% | 327 | 1 h 09 min |
| 2026-10-23 | 12 | 5 h 56 min | 1 h 19 min | 3 min 01 s | 31 min 31 s | 22% | 279 | 39 min 16 s |
| 2026-10-24 | 4 | 3 h 00 min | 46 min 47 s | 1 min 42 s | 10 min 26 s | 26% | 100 | 1 h 37 min |

A day's runs are those started on it; a session that runs past midnight has its active time on the day it
started, so a share above 100% can show on the day after.

## By kind of run

| Bucket | Kind | Runs | Total | Median | Longest | Passed | Failed | Result not known |
|---|---|---|---|---|---|---|---|---|
| test | slice list | 221 | 9 h 08 min | 1 min 49 s | 11 min 24 s | 141 | 23 | 57 |
| test | tier 1, part (cargo test -p) | 1124 | 6 h 05 min | 6 s | 2 h 05 min | 708 | 225 | 191 |
| test | tier 1 (cargo test) | 801 | 5 h 27 min | 16 s | 3 min 19 s | 162 | 234 | 405 |
| test | call-site check | 241 | 4 h 01 min | 12 s | 16 min 20 s | 147 | 64 | 30 |
| test | upstream pass list | 123 | 3 h 49 min | 1 min 24 s | 11 min 17 s | 90 | 3 | 30 |
| test | repo check | 745 | 2 h 12 min | 5 s | 3 min 45 s | 649 | 59 | 37 |
| test | verification pass list | 37 | 1 h 55 min | 2 min 45 s | 7 min 45 s | 30 | 5 | 2 |
| test | full corpus | 40 | 1 h 49 min | 1 min 18 s | 22 min 19 s | 0 | 25 | 15 |
| test | fmt | 630 | 1 h 39 min | 5 s | 2 min 30 s | 1 | 66 | 563 |
| test | clippy | 524 | 1 h 24 min | 6 s | 1 min 55 s | 102 | 89 | 333 |
| test | full verification suite | 69 | 1 h 13 min | 10 s | 30 min 14 s | 24 | 8 | 37 |
| test | full upstream suite | 37 | 1 h 12 min | 45 s | 22 min 26 s | 0 | 25 | 12 |
| test | extension tests (npm) | 164 | 1 h 10 min | 12 s | 8 min 52 s | 106 | 8 | 50 |
| test | differential pass list | 57 | 1 h 06 min | 54 s | 4 min 08 s | 30 | 3 | 24 |
| test | wording check | 74 | 59 min 50 s | 43 s | 2 min 36 s | 60 | 3 | 11 |
| test | corpus, selected programs | 259 | 54 min 34 s | 7 s | 2 min 02 s | 190 | 35 | 34 |
| test | openspec validate | 393 | 31 min 02 s | 2 s | 58 s | 242 | 111 | 40 |
| test | verification, selected programs | 88 | 29 min 58 s | 8 s | 8 min 01 s | 43 | 17 | 28 |
| test | coverage check | 49 | 25 min 08 s | 32 s | 1 min 04 s | 29 | 15 | 5 |
| test | upstream, selected programs | 84 | 21 min 33 s | 10 s | 1 min 13 s | 53 | 23 | 8 |
| test | tests of the Python tools | 153 | 19 min 58 s | 3 s | 1 min 00 s | 116 | 23 | 14 |
| test | pass list checked from a full run | 23 | 6 min 01 s | 3 s | 2 min 03 s | 8 | 13 | 2 |
| test | call-site self-test | 25 | 5 min 18 s | 4 s | 1 min 57 s | 24 | 0 | 1 |
| test | other suite | 1 | 20 s | 20 s | 20 s | 1 | 0 | 0 |
| measure | verification programs (old compiler) | 292 | 3 h 03 min | 8 s | 24 min 51 s | | | |
| measure | recording expected results (old compiler) | 220 | 1 h 50 min | 7 s | 21 min 41 s | | | |
| measure | full corpus (old compiler) | 14 | 1 h 48 min | 6 min 14 s | 15 min 56 s | | | |
| measure | old compiler run directly | 230 | 1 h 20 min | 3 s | 10 min 03 s | | | |
| measure | other suite (old compiler) | 4 | 37 min 14 s | 8 s | 36 min 57 s | | | |
| measure | corpus, selected programs (old compiler) | 64 | 35 min 15 s | 15 s | 6 min 24 s | | | |
| measure | slice list (old compiler) | 6 | 21 min 36 s | 3 min 00 s | 6 min 51 s | | | |
| measure | upstream pass list (old compiler) | 4 | 14 min 43 s | 2 min 38 s | 6 min 51 s | | | |
| measure | full upstream suite (old compiler) | 4 | 9 min 33 s | 3 min 00 s | 3 min 31 s | | | |
| measure | upstream, selected programs (old compiler) | 6 | 22 s | 4 s | 6 s | | | |
| build | cargo build | 936 | 5 h 09 min | 11 s | 10 min 02 s | 270 | 157 | 509 |
| build | extension build (npm) | 33 | 6 min 42 s | 10 s | 37 s | 0 | 0 | 33 |

## Runs repeated with nothing edited

A run of a statement that the same session had run before, word for word, with no file edited in between
(`README.md`, "Repeats"). By the result of the run before it: after a pass the repeat could tell nothing new,
unless something else had changed.

| Bucket | Kind | Repeats | Time | After a pass | Time | After a failure | After a result not known |
|---|---|---|---|---|---|---|---|
| test | slice list | 13 | 18 min 07 s | 9 | 13 min 08 s | 4 | 0 |
| test | tier 1, part (cargo test -p) | 84 | 11 min 32 s | 36 | 7 min 25 s | 36 | 12 |
| test | tier 1 (cargo test) | 133 | 50 min 20 s | 18 | 6 min 48 s | 51 | 64 |
| test | call-site check | 17 | 11 min 36 s | 4 | 5 min 52 s | 11 | 2 |
| test | upstream pass list | 7 | 8 min 24 s | 7 | 8 min 24 s | 0 | 0 |
| test | repo check | 37 | 7 min 20 s | 13 | 1 min 12 s | 20 | 4 |
| test | verification pass list | 5 | 13 min 00 s | 3 | 7 min 23 s | 2 | 0 |
| test | fmt | 38 | 6 min 20 s | 0 | 0 s | 17 | 21 |
| test | clippy | 47 | 6 min 00 s | 2 | 39 s | 20 | 25 |
| test | extension tests (npm) | 11 | 2 min 49 s | 4 | 33 s | 0 | 7 |
| test | differential pass list | 1 | 11 s | 1 | 11 s | 0 | 0 |
| test | wording check | 6 | 4 min 26 s | 5 | 3 min 51 s | 0 | 1 |
| test | corpus, selected programs | 4 | 1 min 14 s | 1 | 36 s | 2 | 1 |
| test | openspec validate | 22 | 1 min 25 s | 2 | 9 s | 3 | 17 |
| test | verification, selected programs | 2 | 7 s | 2 | 7 s | 0 | 0 |
| test | coverage check | 3 | 1 min 54 s | 3 | 1 min 54 s | 0 | 0 |
| test | upstream, selected programs | 4 | 1 min 25 s | 0 | 0 s | 2 | 2 |
| test | tests of the Python tools | 5 | 16 s | 2 | 10 s | 3 | 0 |
| test | call-site self-test | 1 | 2 s | 1 | 2 s | 0 | 0 |
| build | cargo build | 100 | 25 min 29 s | 16 | 6 min 39 s | 54 | 30 |
| build | extension build (npm) | 2 | 5 s | 0 | 0 s | 0 | 2 |
| | **All** | 542 | 2 h 52 min | 129 | 1 h 05 min | 225 | 188 |

## The 20 longest runs

| Start | Kind | Time | Background |
|---|---|---|---|
| 2026-10-16 17:47:28 | tier 1, part (cargo test -p) | 2 h 05 min | yes |
| 2026-10-02 15:24:43 | other suite (old compiler) | 36 min 57 s | yes |
| 2026-10-21 15:19:53 | full verification suite | 30 min 14 s | yes |
| 2026-10-10 11:37:46 | verification programs (old compiler) | 24 min 51 s | yes |
| 2026-10-21 15:19:51 | full upstream suite | 22 min 26 s | yes |
| 2026-10-21 15:19:50 | full corpus | 22 min 19 s | yes |
| 2026-10-03 16:05:50 | recording expected results (old compiler) | 21 min 41 s | yes |
| 2026-10-03 16:28:51 | recording expected results (old compiler) | 21 min 01 s | yes |
| 2026-10-21 15:45:53 | call-site check | 16 min 20 s | yes |
| 2026-10-19 17:47:31 | full verification suite | 16 min 07 s | yes |
| 2026-10-03 17:08:21 | full corpus (old compiler) | 15 min 56 s | yes |
| 2026-10-03 17:25:48 | full corpus (old compiler) | 15 min 22 s | yes |
| 2026-10-03 16:50:01 | full corpus (old compiler) | 14 min 38 s | yes |
| 2026-10-17 18:55:39 | full corpus | 14 min 36 s | yes |
| 2026-10-21 15:48:20 | verification programs (old compiler) | 14 min 05 s | yes |
| 2026-10-10 09:58:39 | full corpus (old compiler) | 11 min 59 s | yes |
| 2026-10-10 11:18:53 | full corpus (old compiler) | 11 min 33 s | yes |
| 2026-10-19 16:30:00 | slice list | 11 min 24 s | yes |
| 2026-10-19 16:30:02 | upstream pass list | 11 min 17 s | yes |
| 2026-10-17 16:36:03 | slice list | 10 min 22 s | yes |

## By change

Each moment of a session belongs to the OpenSpec change named last before it (`README.md`, "Changes"):
a guess from the workflow's own commands and the files read and edited, not a record kept by hand. *Elapsed*
is calendar time from the first moment to the archive (or the last moment seen), nights and other work
included; *Active* is Claude working on it (the rule of "Active time"); an *idle stretch* is a gap of more
than 5 minutes inside a session (the user away or reading). *Closing stretch*: calendar time from the last
edit of product code to the archive. *Archived, not seen*: the change is in the archive, but no transcript
shows it archived (by `openspec archive` or by moving its folder); it ends where it was last seen.

| Change | State | Sessions | Started | Ended | Elapsed | Active | Waiting: tests | measurements | builds | In the background | Repeats | Longest wait | Idle stretches | Longest idle | Closing stretch |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| m1-vscode-extension-v0 | archived | 6 | 2026-10-02 22:56 | 2026-10-03 14:20 | 15 h 23 min | 2 h 24 min | 21 min 21 s | 4 s | 3 min 25 s | 11 min 03 s | 3 | 2 min 53 s | 3 (13 h 08 min) | 9 h 00 min | 29 min 21 s |
| m2-golden-corpus | archived | 3 | 2026-10-03 15:21 | 2026-10-03 17:51 | 2 h 29 min | 1 h 20 min | 30 s | 12 min 41 s | 0 s | 1 h 28 min | 6 | 6 min 24 s | 7 (1 h 04 min) | 14 min 05 s |  |
| m2-workspace-and-slice | archived, not seen | 3 | 2026-10-03 18:08 | 2026-10-04 10:24 (last seen) | 16 h 15 min | 1 h 10 min | 8 min 25 s | 7 min 53 s | 2 min 07 s | 0 s | 12 | 5 min 01 s | 7 (14 h 48 min) | 13 h 23 min |  |
| m2-procedures-and-errors | archived | 8 | 2026-10-04 10:54 | 2026-10-04 18:42 | 7 h 48 min | 2 h 52 min | 27 min 55 s | 4 min 37 s | 7 min 10 s | 0 s | 12 | 4 min 58 s | 4 (2 h 59 min) | 2 h 14 min | 2 min 55 s |
| m2-upstream-tests | archived, not seen | 3 | 2026-10-04 18:23 | 2026-10-04 20:32 (last seen) | 2 h 09 min | 1 h 23 min | 20 min 28 s | 6 s | 13 min 14 s | 1 min 04 s | 7 | 10 min 02 s | 1 (9 min 29 s) | 9 min 29 s |  |
| m2-parser-breadth | archived | 17 | 2026-10-04 20:49 | 2026-10-07 20:37 | 71 h 48 min | 5 h 20 min | 1 h 12 min | 3 min 10 s | 14 min 00 s | 55 min 02 s | 25 | 2 min 34 s | 5 (52 min 15 s) | 14 min 16 s | 9 min 04 s |
| m2-control-flow-slice | archived, not seen | 19 | 2026-10-05 18:34 | 2026-10-07 14:42 (last seen) | 44 h 08 min | 5 h 50 min | 53 min 08 s | 25 min 06 s | 9 min 52 s | 24 min 36 s | 30 | 10 min 03 s | 8 (21 h 40 min) | 18 h 16 min |  |
| m2-arrays-and-types | archived | 6 | 2026-10-07 14:56 | 2026-10-07 20:37 | 5 h 41 min | 1 h 56 min | 24 min 58 s | 43 s | 3 min 41 s | 9 min 48 s | 20 | 2 min 22 s | 1 (1 h 46 min) | 1 h 46 min | 1 h 48 min |
| m2-core-builtins | archived | 2 | 2026-10-07 20:52 | 2026-10-08 11:24 | 14 h 31 min | 1 h 50 min | 19 min 32 s | 3 min 40 s | 5 min 18 s | 6 min 20 s | 5 | 2 min 33 s | 2 (12 h 37 min) | 12 h 25 min | 33 min 07 s |
| m2-language-server | archived | 3 | 2026-10-08 12:18 | 2026-10-08 14:10 | 1 h 52 min | 48 min 40 s | 5 min 24 s | 0 s | 2 min 53 s | 55 s | 10 | 56 s | 2 (49 min 02 s) | 32 min 05 s | 37 min 33 s |
| m2-numeric-types | archived, not seen | 19 | 2026-10-08 14:33 | 2026-10-09 23:06 (last seen) | 32 h 33 min | 5 h 49 min | 55 min 06 s | 9 min 16 s | 8 min 09 s | 1 h 08 min | 28 | 6 min 32 s | 6 (21 h 05 min) | 17 h 22 min |  |
| m2-builtin-statements | archived | 9 | 2026-10-09 23:39 | 2026-10-10 16:18 | 16 h 38 min | 2 h 57 min | 28 min 25 s | 13 min 15 s | 3 min 39 s | 18 min 28 s | 21 | 2 min 11 s | 3 (8 h 13 min) | 5 h 25 min | 3 min 01 s |
| m2-demand-report-and-runner-tests | archived | 1 | 2026-10-10 16:40 | 2026-10-10 16:44 | 4 min 04 s | 4 min 04 s | 2 s | 0 s | 0 s | 0 s | 0 | 2 s | 0 (0 s) | 0 s | 37 s |
| m2-screen-state-check | archived | 1 | 2026-10-10 19:32 | 2026-10-10 20:15 | 43 min 04 s | 43 min 04 s | 3 min 57 s | 5 min 18 s | 0 s | 9 min 08 s | 1 | 1 min 50 s | 0 (0 s) | 0 s | 13 min 57 s |
| m2-defxxx-common-arrays | archived | 20 | 2026-10-10 20:41 | 2026-10-12 10:30 | 37 h 49 min | 8 h 03 min | 2 h 09 min | 10 min 02 s | 20 min 12 s | 31 min 21 s | 33 | 3 min 48 s | 8 (14 h 29 min) | 6 h 06 min | 49 min 18 s |
| m2-console-screenhide | archived, not seen | 11 | 2026-10-12 10:59 | 2026-10-12 19:57 (last seen) | 8 h 57 min | 3 h 24 min | 58 min 48 s | 4 min 31 s | 5 min 30 s | 7 min 53 s | 23 | 3 min 50 s | 6 (2 h 41 min) | 1 h 55 min |  |
| m3-window-output | archived | 3 | 2026-10-12 20:09 | 2026-10-12 22:06 | 1 h 56 min | 1 h 52 min | 17 min 55 s | 6 min 20 s | 1 min 22 s | 16 min 35 s | 6 | 2 min 29 s | 0 (0 s) | 0 s | 9 min 47 s |
| m3-get-put-bload | archived | 4 | 2026-10-12 22:37 | 2026-10-13 11:26 | 12 h 48 min | 1 h 27 min | 11 min 42 s | 2 min 34 s | 3 min 14 s | 2 min 11 s | 18 | 1 min 07 s | 2 (11 h 02 min) | 10 h 57 min | 25 min 20 s |
| m3-sound | archived | 6 | 2026-10-13 11:43 | 2026-10-13 14:50 | 3 h 07 min | 1 h 45 min | 26 min 12 s | 2 min 05 s | 2 min 24 s | 0 s | 15 | 4 min 30 s | 1 (5 min 01 s) | 5 min 01 s | 19 min 05 s |
| m3-console-files | archived | 5 | 2026-10-13 15:19 | 2026-10-14 08:45 | 17 h 25 min | 1 h 50 min | 28 min 07 s | 2 min 53 s | 5 min 29 s | 0 s | 9 | 2 min 25 s | 0 (0 s) | 0 s | 15 h 38 min |
| m3-scripted-game | archived | 4 | 2026-10-14 09:02 | 2026-10-14 10:53 | 1 h 51 min | 1 h 15 min | 14 min 49 s | 1 min 27 s | 30 s | 0 s | 5 | 5 min 54 s | 0 (0 s) | 0 s |  |
| m3-images-fonts-mouse | archived | 13 | 2026-10-14 10:58 | 2026-10-14 14:37 | 3 h 38 min | 2 h 34 min | 38 min 22 s | 5 min 31 s | 7 min 19 s | 0 s | 2 | 5 min 57 s | 1 (23 min 08 s) | 23 min 08 s | 37 min 45 s |
| m3-mem | archived | 4 | 2026-10-14 14:42 | 2026-10-14 17:42 | 3 h 00 min | 2 h 32 min | 22 min 02 s | 2 min 14 s | 8 min 26 s | 16 min 32 s | 9 | 2 min 09 s | 0 (0 s) | 0 s | 11 min 18 s |
| m3-asserts-debug-zlib | archived, not seen | 4 | 2026-10-14 17:50 | 2026-10-14 20:03 (last seen) | 2 h 13 min | 1 h 38 min | 36 min 56 s | 1 min 01 s | 3 min 45 s | 6 min 50 s | 5 | 8 min 12 s | 2 (28 min 30 s) | 21 min 24 s |  |
| m3-redim-retain | archived | 6 | 2026-10-14 20:33 | 2026-10-14 22:03 | 1 h 30 min | 1 h 14 min | 15 min 14 s | 40 s | 2 min 17 s | 25 min 03 s | 1 | 2 min 46 s | 3 (21 min 39 s) | 10 min 08 s | 24 min 09 s |
| m3-typefields | archived | 11 | 2026-10-14 22:20 | 2026-10-15 13:57 | 15 h 36 min | 2 h 55 min | 37 min 53 s | 1 min 47 s | 5 min 01 s | 9 min 51 s | 10 | 5 min 59 s | 4 (2 h 10 min) | 1 h 12 min | 4 min 27 s |
| m3-whole-array-assign | archived | 4 | 2026-10-15 14:14 | 2026-10-15 15:25 | 1 h 10 min | 1 h 02 min | 18 min 40 s | 42 s | 1 min 04 s | 0 s | 1 | 4 min 53 s | 0 (0 s) | 0 s | 5 min 44 s |
| m3-declare-library | archived | 18 | 2026-10-15 15:56 | 2026-10-16 14:33 | 22 h 37 min | 3 h 05 min | 39 min 00 s | 58 s | 3 min 04 s | 52 s | 6 | 10 min 01 s | 7 (16 h 53 min) | 15 h 23 min | 30 min 05 s |
| m3-measured-copies | archived, not seen | 1 | 2026-10-16 14:47 | 2026-10-16 16:10 (last seen) | 1 h 22 min | 52 min 42 s | 24 min 22 s | 6 s | 1 min 10 s | 0 s | 5 | 6 min 01 s | 0 (0 s) | 0 s |  |
| m3-divergence-trace | archived, not seen | 1 | 2026-10-16 15:38 | 2026-10-16 15:58 (last seen) | 20 min 42 s | 20 min 42 s | 8 min 01 s | 0 s | 38 s | 0 s | 0 | 2 min 38 s | 0 (0 s) | 0 s |  |
| m3-old-wording-notes | archived, not seen | 1 | 2026-10-16 15:58 | 2026-10-16 16:08 (last seen) | 9 min 28 s | 9 min 28 s | 3 min 21 s | 2 s | 0 s | 0 s | 0 | 49 s | 0 (0 s) | 0 s |  |
| m3-sound-handles | archived | 5 | 2026-10-16 16:34 | 2026-10-16 22:44 | 6 h 10 min | 1 h 45 min | 18 min 18 s | 4 min 00 s | 1 min 27 s | 2 h 24 min | 2 | 10 min 01 s | 1 (2 h 05 min) | 2 h 05 min | 21 min 49 s |
| m3-versioninfo | archived | 5 | 2026-10-16 22:57 | 2026-10-17 00:05 | 1 h 07 min | 55 min 21 s | 23 min 02 s | 5 s | 1 min 10 s | 4 min 28 s | 1 | 5 min 14 s | 0 (0 s) | 0 s | 13 min 49 s |
| m3-arraycopy | archived | 4 | 2026-10-17 00:14 | 2026-10-17 09:05 | 8 h 50 min | 1 h 40 min | 20 min 46 s | 2 min 20 s | 4 min 10 s | 18 min 29 s | 3 | 2 min 38 s | 2 (11 min 32 s) | 5 min 47 s | 6 min 19 s |
| m3-events | archived | 5 | 2026-10-17 09:17 | 2026-10-17 11:34 | 2 h 16 min | 1 h 36 min | 13 min 16 s | 1 min 29 s | 3 min 42 s | 16 min 48 s | 4 | 2 min 19 s | 1 (6 min 03 s) | 6 min 03 s | 37 min 29 s |
| test-parallel-runs | archived | 4 | 2026-10-17 11:06 | 2026-10-17 11:34 | 27 min 53 s | 15 min 45 s | 2 min 54 s | 0 s | 0 s | 3 min 30 s | 0 | 2 min 12 s | 1 (6 min 55 s) | 6 min 55 s |  |
| m3-errorline-chr-const | archived, not seen | 4 | 2026-10-17 12:40 | 2026-10-17 13:41 (last seen) | 1 h 01 min | 53 min 15 s | 4 min 46 s | 0 s | 0 s | 12 min 30 s | 0 | 32 s | 0 (0 s) | 0 s |  |
| m3-network-embed | archived, not seen | 6 | 2026-10-17 13:55 | 2026-10-17 19:04 (last seen) | 5 h 08 min | 1 h 49 min | 17 min 07 s | 2 min 05 s | 5 min 13 s | 41 min 39 s | 5 | 1 min 00 s | 2 (28 min 42 s) | 20 min 47 s |  |
| test-while-building | archived, not seen | 4 | 2026-10-17 17:22 | 2026-10-17 19:01 (last seen) | 1 h 38 min | 23 min 13 s | 6 min 50 s | 0 s | 26 s | 24 min 27 s | 1 | 2 min 36 s | 0 (0 s) | 0 s |  |
| m3-constant-evaluation | archived | 5 | 2026-10-17 19:57 | 2026-10-18 17:33 | 21 h 35 min | 2 h 19 min | 34 min 40 s | 2 min 28 s | 7 min 22 s | 41 min 44 s | 10 | 2 min 03 s | 10 (18 h 20 min) | 10 h 00 min | 1 h 19 min |
| libqb-copy | archived | 3 | 2026-10-18 18:08 | 2026-10-19 01:20 | 7 h 11 min | 1 h 23 min | 25 min 22 s | 11 s | 2 min 14 s | 29 min 39 s | 17 | 10 min 04 s | 5 (5 h 41 min) | 4 h 04 min | 11 min 49 s |
| extension-diagnostics | archived | 3 | 2026-10-19 01:29 | 2026-10-19 09:09 | 7 h 40 min | 46 min 54 s | 9 min 37 s | 0 s | 51 s | 5 min 11 s | 0 | 1 min 36 s | 1 (10 min 03 s) | 10 min 03 s | 3 min 11 s |
| extension-vsix | archived | 3 | 2026-10-19 09:19 | 2026-10-19 15:51 | 6 h 31 min | 1 h 13 min | 14 min 44 s | 0 s | 3 min 13 s | 19 min 06 s | 3 | 1 min 03 s | 4 (4 h 58 min) | 3 h 46 min | 16 min 56 s |
| libqb-object-flags | archived, not seen | 2 | 2026-10-19 16:01 | 2026-10-19 16:56 (last seen) | 54 min 36 s | 58 min 22 s | 17 min 27 s | 15 s | 1 min 21 s | 39 min 21 s | 3 | 1 min 17 s | 1 (9 min 33 s) | 9 min 33 s |  |
| const-rulings | archived, not seen | 2 | 2026-10-19 16:19 | 2026-10-19 16:56 (last seen) | 36 min 14 s | 1 min 47 s | 0 s | 0 s | 0 s | 0 s | 0 | 0 s | 0 (0 s) | 0 s |  |
| verification-compare | archived, not seen | 5 | 2026-10-19 17:31 | 2026-10-20 12:22 (last seen) | 18 h 50 min | 1 h 37 min | 25 min 45 s | 8 s | 5 min 27 s | 43 min 16 s | 11 | 2 min 38 s | 3 (3 h 31 min) | 1 h 44 min |  |
| cws-whole-game | archived, not seen | 3 | 2026-10-19 22:38 | 2026-10-20 12:06 (last seen) | 13 h 28 min | 1 h 44 min | 22 min 38 s | 6 min 50 s | 1 min 37 s | 11 min 14 s | 4 | 2 min 28 s | 2 (32 min 22 s) | 17 min 00 s |  |
| cpp-failure-errors | archived | 2 | 2026-10-20 12:36 | 2026-10-20 14:10 | 1 h 33 min | 37 min 36 s | 15 min 04 s | 0 s | 1 min 18 s | 12 min 27 s | 13 | 2 min 38 s | 0 (0 s) | 0 s | 3 min 24 s |
| m4-checker-groups | archived | 5 | 2026-10-20 14:17 | 2026-10-20 14:44 | 27 min 30 s | 20 min 56 s | 4 min 58 s | 0 s | 1 min 07 s | 11 min 02 s | 0 | 1 min 22 s | 0 (0 s) | 0 s | 15 min 57 s |
| m4-devices-continue-paths | archived | 4 | 2026-10-20 14:53 | 2026-10-20 16:36 | 1 h 43 min | 1 h 35 min | 35 min 46 s | 40 s | 5 min 27 s | 20 min 16 s | 15 | 2 min 34 s | 0 (0 s) | 0 s | 13 min 46 s |
| m4-names-arrays | archived, not seen | 4 | 2026-10-20 17:04 | 2026-10-21 09:37 (last seen) | 16 h 33 min | 1 h 29 min | 35 min 42 s | 2 min 09 s | 3 min 16 s | 5 min 00 s | 2 | 8 min 01 s | 1 (6 min 56 s) | 6 min 56 s |  |
| m4-builtins-param-types | archived | 2 | 2026-10-21 09:42 | 2026-10-21 11:09 | 1 h 26 min | 44 min 01 s | 15 min 55 s | 2 min 20 s | 1 min 49 s | 7 min 48 s | 5 | 2 min 13 s | 0 (0 s) | 0 s | 1 h 02 min |
| m4-handlers-logging-checking | archived, not seen | 7 | 2026-10-21 11:17 | 2026-10-21 16:11 (last seen) | 4 h 53 min | 2 h 00 min | 29 min 41 s | 2 min 28 s | 3 min 36 s | 1 h 58 min | 12 | 2 min 03 s | 6 (1 h 07 min) | 26 min 07 s |  |
| dry-cleanup | archived | 3 | 2026-10-21 11:24 | 2026-10-21 12:43 | 1 h 19 min | 46 min 49 s | 19 min 14 s | 0 s | 3 min 33 s | 17 min 28 s | 0 | 2 min 27 s | 3 (23 min 53 s) | 11 min 44 s | 47 min 47 s |
| m4-upstream-rejections | archived | 4 | 2026-10-21 15:27 | 2026-10-21 21:08 | 5 h 41 min | 1 h 47 min | 24 min 25 s | 3 s | 5 min 33 s | 26 min 22 s | 5 | 2 min 43 s | 4 (2 h 01 min) | 1 h 34 min | 19 min 16 s |
| m4-qb64pe-sources | archived | 3 | 2026-10-21 21:26 | 2026-10-22 10:47 | 13 h 21 min | 1 h 46 min | 36 min 46 s | 1 min 14 s | 5 min 53 s | 19 min 15 s | 7 | 2 min 14 s | 2 (36 min 30 s) | 31 min 18 s | 5 min 46 s |
| m4-qb64pe-builtins | archived | 4 | 2026-10-22 11:51 | 2026-10-22 13:57 | 2 h 06 min | 1 h 18 min | 28 min 28 s | 2 min 13 s | 3 min 33 s | 20 min 29 s | 16 | 2 min 00 s | 0 (0 s) | 0 s | 48 min 31 s |
| m4-line-numbers-whole-arrays | archived | 4 | 2026-10-22 14:04 | 2026-10-22 16:26 | 2 h 21 min | 1 h 34 min | 36 min 40 s | 2 min 51 s | 5 min 25 s | 30 min 46 s | 17 | 3 min 07 s | 2 (23 min 11 s) | 14 min 49 s | 47 min 49 s |
| m4-names-implicit-arrays | archived, not seen | 4 | 2026-10-22 16:35 | 2026-10-23 11:28 (last seen) | 18 h 52 min | 2 h 24 min | 33 min 12 s | 57 s | 20 min 07 s | 6 min 23 s | 16 | 2 min 26 s | 2 (15 h 59 min) | 15 h 36 min |  |
| m4-memory-records | archived, not seen | 3 | 2026-10-23 11:47 | 2026-10-23 15:27 (last seen) | 3 h 39 min | 1 h 53 min | 28 min 47 s | 1 min 16 s | 8 min 47 s | 12 min 13 s | 6 | 2 min 28 s | 0 (0 s) | 0 s |  |
| m4-statements-metacommands | archived, not seen | 4 | 2026-10-23 15:56 | 2026-10-24 10:23 (last seen) | 18 h 27 min | 2 h 58 min | 51 min 15 s | 1 min 34 s | 14 min 13 s | 33 min 19 s | 18 | 5 min 00 s | 4 (11 h 07 min) | 4 h 42 min |  |
| m4-builtins-remaining | open | 2 | 2026-10-24 11:01 | 2026-10-24 14:44 (last seen) | 3 h 43 min | 2 h 41 min | 29 min 17 s | 1 min 42 s | 9 min 30 s | 1 h 26 min | 4 | 1 min 56 s | 2 (59 min 20 s) | 49 min 48 s |  |
| (no change named) |  | 407 | 2026-10-02 14:19 | 2026-10-24 11:15 |  | 31 h 12 min | 1 h 54 min | 1 h 32 min | 13 min 59 s | 3 h 11 min | 57 | 10 min 01 s | 54 (20 h 48 min) | 6 h 14 min |  |

## By kind of work

What each tool call did (`README.md`, "Kinds of work"): counts of actions and the bytes edits wrote. These
are not shares of time: the transcript does not say how long a piece of writing took.

| Kind of work | Actions | Bytes written | Share of bytes written |
|---|---|---|---|
| product code | 4424 | 3599542 | 27% |
| tests and fixtures | 2890 | 2280815 | 17% |
| measurement preparation | 657 | 616291 | 5% |
| documents and planning | 4295 | 5043196 | 38% |
| generated files | 7 | 977 | 0% |
| scratch files | 821 | 1644768 | 12% |
| reading | 5089 | 0 |  |
| test runs | 4163 | 0 |  |
| measurement runs | 688 | 0 |  |
| builds | 684 | 0 |  |
| other commands | 14933 | 0 |  |
| other | 253 | 8922 | 0% |

Actions per change:

| Change | product code | tests and fixtures | measurement preparation | documents and planning | generated files | scratch files | reading | test runs | measurement runs | builds | other commands | other |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| const-rulings | 0 | 0 | 0 | 9 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 |
| cpp-failure-errors | 23 | 3 | 0 | 16 | 0 | 1 | 7 | 40 | 0 | 5 | 59 | 1 |
| cws-whole-game | 12 | 66 | 3 | 21 | 0 | 28 | 94 | 71 | 6 | 3 | 230 | 1 |
| dry-cleanup | 25 | 35 | 0 | 29 | 0 | 6 | 44 | 42 | 0 | 8 | 69 | 0 |
| extension-diagnostics | 57 | 11 | 0 | 16 | 0 | 3 | 31 | 36 | 0 | 4 | 88 | 1 |
| extension-vsix | 135 | 10 | 0 | 32 | 0 | 1 | 33 | 45 | 0 | 5 | 120 | 5 |
| libqb-copy | 31 | 19 | 0 | 45 | 0 | 1 | 37 | 42 | 1 | 6 | 101 | 7 |
| libqb-object-flags | 36 | 42 | 0 | 24 | 0 | 0 | 9 | 52 | 1 | 2 | 134 | 1 |
| m1-vscode-extension-v0 | 85 | 2 | 2 | 82 | 0 | 17 | 95 | 44 | 2 | 1 | 170 | 2 |
| m2-arrays-and-types | 92 | 33 | 1 | 56 | 0 | 20 | 48 | 67 | 10 | 25 | 188 | 2 |
| m2-builtin-statements | 135 | 100 | 39 | 126 | 0 | 30 | 225 | 106 | 41 | 15 | 207 | 3 |
| m2-console-screenhide | 87 | 71 | 31 | 178 | 0 | 15 | 144 | 94 | 35 | 13 | 247 | 3 |
| m2-control-flow-slice | 185 | 102 | 30 | 149 | 0 | 13 | 197 | 149 | 14 | 27 | 444 | 5 |
| m2-core-builtins | 77 | 44 | 11 | 70 | 0 | 6 | 84 | 53 | 11 | 0 | 106 | 5 |
| m2-defxxx-common-arrays | 407 | 219 | 62 | 277 | 0 | 48 | 352 | 188 | 47 | 46 | 528 | 2 |
| m2-demand-report-and-runner-tests | 2 | 2 | 0 | 24 | 0 | 0 | 0 | 2 | 0 | 0 | 6 | 0 |
| m2-golden-corpus | 0 | 22 | 3 | 42 | 0 | 0 | 21 | 13 | 22 | 0 | 103 | 1 |
| m2-language-server | 80 | 18 | 0 | 35 | 0 | 1 | 29 | 38 | 0 | 6 | 79 | 0 |
| m2-numeric-types | 309 | 121 | 48 | 233 | 0 | 29 | 241 | 176 | 33 | 31 | 620 | 9 |
| m2-parser-breadth | 384 | 125 | 9 | 150 | 0 | 10 | 255 | 164 | 21 | 33 | 392 | 3 |
| m2-procedures-and-errors | 148 | 57 | 18 | 91 | 0 | 9 | 91 | 72 | 10 | 11 | 207 | 7 |
| m2-screen-state-check | 11 | 29 | 2 | 49 | 0 | 4 | 18 | 11 | 4 | 0 | 38 | 2 |
| m2-upstream-tests | 37 | 38 | 0 | 15 | 0 | 6 | 21 | 50 | 2 | 4 | 137 | 4 |
| m2-workspace-and-slice | 31 | 10 | 0 | 42 | 0 | 5 | 12 | 38 | 15 | 8 | 126 | 2 |
| m3-arraycopy | 38 | 20 | 11 | 64 | 0 | 8 | 43 | 59 | 9 | 12 | 208 | 1 |
| m3-asserts-debug-zlib | 48 | 46 | 0 | 44 | 0 | 12 | 19 | 52 | 4 | 9 | 228 | 3 |
| m3-console-files | 141 | 52 | 5 | 59 | 0 | 7 | 65 | 81 | 20 | 9 | 166 | 0 |
| m3-constant-evaluation | 62 | 28 | 3 | 43 | 0 | 16 | 45 | 87 | 10 | 11 | 218 | 3 |
| m3-declare-library | 155 | 56 | 7 | 148 | 0 | 15 | 108 | 70 | 5 | 18 | 360 | 1 |
| m3-divergence-trace | 7 | 16 | 0 | 7 | 0 | 1 | 3 | 6 | 0 | 0 | 19 | 0 |
| m3-errorline-chr-const | 3 | 44 | 1 | 42 | 0 | 3 | 15 | 30 | 0 | 0 | 75 | 5 |
| m3-events | 37 | 30 | 10 | 37 | 0 | 8 | 10 | 51 | 12 | 8 | 215 | 1 |
| m3-get-put-bload | 16 | 27 | 3 | 40 | 3 | 17 | 106 | 39 | 8 | 18 | 87 | 1 |
| m3-images-fonts-mouse | 53 | 58 | 5 | 74 | 0 | 25 | 96 | 79 | 18 | 12 | 342 | 3 |
| m3-measured-copies | 33 | 23 | 1 | 13 | 0 | 10 | 20 | 28 | 0 | 3 | 78 | 0 |
| m3-mem | 130 | 24 | 4 | 57 | 0 | 31 | 77 | 67 | 15 | 29 | 299 | 2 |
| m3-network-embed | 70 | 60 | 8 | 56 | 0 | 0 | 39 | 71 | 11 | 12 | 301 | 2 |
| m3-old-wording-notes | 5 | 6 | 0 | 15 | 0 | 1 | 7 | 4 | 1 | 0 | 12 | 0 |
| m3-redim-retain | 31 | 15 | 2 | 42 | 0 | 8 | 27 | 39 | 4 | 6 | 148 | 1 |
| m3-scripted-game | 0 | 37 | 0 | 55 | 0 | 16 | 47 | 30 | 7 | 1 | 106 | 3 |
| m3-sound | 32 | 34 | 9 | 100 | 0 | 8 | 33 | 63 | 7 | 10 | 141 | 2 |
| m3-sound-handles | 15 | 79 | 8 | 64 | 0 | 5 | 33 | 38 | 17 | 4 | 193 | 0 |
| m3-typefields | 116 | 47 | 10 | 77 | 0 | 19 | 94 | 86 | 17 | 24 | 400 | 6 |
| m3-versioninfo | 31 | 8 | 1 | 29 | 0 | 1 | 24 | 42 | 1 | 2 | 105 | 3 |
| m3-whole-array-assign | 31 | 16 | 3 | 56 | 0 | 6 | 38 | 35 | 9 | 5 | 147 | 1 |
| m3-window-output | 40 | 77 | 1 | 91 | 0 | 23 | 118 | 38 | 9 | 5 | 71 | 2 |
| m4-builtins-param-types | 19 | 1 | 1 | 23 | 0 | 5 | 14 | 39 | 6 | 4 | 97 | 1 |
| m4-builtins-remaining | 58 | 52 | 8 | 66 | 1 | 26 | 19 | 69 | 10 | 10 | 235 | 1 |
| m4-checker-groups | 37 | 0 | 0 | 14 | 0 | 2 | 5 | 16 | 0 | 5 | 45 | 0 |
| m4-devices-continue-paths | 36 | 46 | 2 | 51 | 1 | 17 | 33 | 86 | 5 | 10 | 213 | 4 |
| m4-handlers-logging-checking | 40 | 39 | 17 | 44 | 0 | 10 | 26 | 80 | 8 | 17 | 299 | 2 |
| m4-line-numbers-whole-arrays | 15 | 18 | 14 | 51 | 0 | 12 | 35 | 96 | 8 | 11 | 192 | 1 |
| m4-memory-records | 43 | 11 | 16 | 66 | 0 | 24 | 23 | 74 | 8 | 19 | 298 | 1 |
| m4-names-arrays | 50 | 28 | 14 | 67 | 0 | 6 | 96 | 52 | 6 | 14 | 123 | 1 |
| m4-names-implicit-arrays | 117 | 27 | 14 | 43 | 1 | 24 | 40 | 89 | 11 | 35 | 357 | 1 |
| m4-qb64pe-builtins | 42 | 28 | 11 | 36 | 0 | 6 | 10 | 86 | 7 | 6 | 193 | 1 |
| m4-qb64pe-sources | 45 | 28 | 17 | 56 | 0 | 12 | 69 | 98 | 10 | 7 | 200 | 1 |
| m4-statements-metacommands | 109 | 23 | 4 | 99 | 1 | 14 | 59 | 113 | 9 | 34 | 357 | 1 |
| m4-upstream-rejections | 28 | 10 | 1 | 41 | 0 | 6 | 15 | 56 | 4 | 10 | 197 | 0 |
| test-parallel-runs | 0 | 10 | 0 | 15 | 0 | 0 | 14 | 6 | 0 | 0 | 20 | 1 |
| test-while-building | 7 | 10 | 0 | 14 | 0 | 1 | 9 | 16 | 0 | 1 | 29 | 0 |
| verification-compare | 9 | 59 | 4 | 20 | 0 | 3 | 38 | 68 | 2 | 8 | 175 | 6 |
| (no change named) | 286 | 518 | 183 | 565 | 0 | 160 | 1429 | 431 | 135 | 42 | 3584 | 125 |
