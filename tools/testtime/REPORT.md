# Time spent testing

Made by `python tools\testtime\test_time.py` from the transcripts of the Claude Code sessions; do not edit.
What the columns mean, and what the numbers cannot say: `README.md` beside this file.

290 sessions from 2026-10-02 to 2026-10-17; 5148 runs on record.

## In all

| | Time | Share of active time |
|---|---|---|
| Active time (Claude working, the user not waited for) | 99 h 02 min | |
| Inside commands of any kind | 42 h 41 min | 43% |
| **Waiting for test runs** | 18 h 10 min | 18% |
| Waiting for measurements with the old compiler | 3 h 44 min | 4% |
| Waiting for builds | 2 h 57 min | 3% |
| Everything else (reading, thinking, writing code and documents, other commands) | 74 h 09 min | 75% |
| Test runs and measurements in the background (not waited for) | 11 h 43 min | |

## By day

| Day | Sessions | Active | Tests | Measurements | Builds | Tests, share | Test runs | In the background |
|---|---|---|---|---|---|---|---|---|
| 2026-10-02 | 7 | 4 h 30 min | 1 min 22 s | 1 min 59 s | 1 min 59 s | 1% | 6 | 44 min 13 s |
| 2026-10-03 | 14 | 5 h 19 min | 36 min 59 s | 16 min 25 s | 5 min 58 s | 12% | 153 | 1 h 39 min |
| 2026-10-04 | 14 | 6 h 31 min | 1 h 38 min | 10 min 42 s | 25 min 18 s | 25% | 297 | 5 min 57 s |
| 2026-10-05 | 13 | 3 h 39 min | 34 min 42 s | 2 min 24 s | 9 min 27 s | 16% | 129 | 0 s |
| 2026-10-06 | 7 | 2 h 12 min | 35 min 56 s | 1 min 45 s | 3 min 54 s | 27% | 114 | 0 s |
| 2026-10-07 | 26 | 9 h 40 min | 1 h 05 min | 27 min 09 s | 15 min 23 s | 11% | 415 | 1 h 24 min |
| 2026-10-08 | 13 | 3 h 56 min | 32 min 16 s | 15 min 17 s | 7 min 19 s | 14% | 231 | 20 min 48 s |
| 2026-10-09 | 24 | 5 h 51 min | 1 h 04 min | 2 min 02 s | 8 min 40 s | 18% | 290 | 58 min 32 s |
| 2026-10-10 | 19 | 7 h 30 min | 44 min 49 s | 1 h 07 min | 5 min 01 s | 10% | 279 | 1 h 07 min |
| 2026-10-11 | 16 | 7 h 55 min | 2 h 00 min | 19 min 07 s | 21 min 33 s | 25% | 347 | 31 min 36 s |
| 2026-10-12 | 25 | 8 h 27 min | 1 h 45 min | 15 min 43 s | 9 min 45 s | 21% | 300 | 34 min 11 s |
| 2026-10-13 | 17 | 5 h 09 min | 1 h 17 min | 12 min 24 s | 11 min 47 s | 25% | 327 | 9 min 39 s |
| 2026-10-14 | 39 | 10 h 59 min | 2 h 22 min | 16 min 13 s | 25 min 17 s | 22% | 379 | 48 min 47 s |
| 2026-10-15 | 22 | 5 h 48 min | 1 h 03 min | 4 min 07 s | 9 min 39 s | 18% | 210 | 11 min 07 s |
| 2026-10-16 | 28 | 8 h 06 min | 2 h 16 min | 7 min 58 s | 6 min 55 s | 28% | 287 | 2 h 30 min |
| 2026-10-17 | 6 | 3 h 22 min | 31 min 34 s | 4 min 06 s | 9 min 51 s | 16% | 130 | 36 min 28 s |

A day's runs are those started on it; a session that runs past midnight has its active time on the day it
started, so a share above 100% can show on the day after.

## By kind of run

| Bucket | Kind | Runs | Total | Median | Longest | Passed | Failed | Result not known |
|---|---|---|---|---|---|---|---|---|
| test | slice list | 134 | 5 h 37 min | 2 min 04 s | 8 min 12 s | 87 | 4 | 43 |
| test | tier 1, part (cargo test -p) | 716 | 4 h 24 min | 5 s | 2 h 05 min | 451 | 165 | 100 |
| test | tier 1 (cargo test) | 625 | 3 h 05 min | 13 s | 2 min 29 s | 148 | 183 | 294 |
| test | upstream pass list | 72 | 1 h 56 min | 1 min 38 s | 6 min 40 s | 47 | 2 | 23 |
| test | repo check | 546 | 1 h 27 min | 5 s | 3 min 45 s | 481 | 41 | 24 |
| test | differential pass list | 51 | 1 h 07 min | 54 s | 6 min 15 s | 26 | 2 | 23 |
| test | full corpus | 33 | 1 h 07 min | 1 min 14 s | 6 min 54 s | 0 | 21 | 12 |
| test | fmt | 463 | 1 h 06 min | 5 s | 2 min 30 s | 0 | 55 | 408 |
| test | call-site check | 117 | 1 h 03 min | 14 s | 8 min 34 s | 75 | 28 | 14 |
| test | clippy | 389 | 52 min 32 s | 6 s | 1 min 23 s | 81 | 61 | 247 |
| test | extension tests (npm) | 110 | 45 min 04 s | 14 s | 8 min 52 s | 68 | 8 | 34 |
| test | corpus, selected programs | 181 | 40 min 18 s | 7 s | 2 min 02 s | 146 | 19 | 16 |
| test | full upstream suite | 33 | 36 min 36 s | 42 s | 5 min 49 s | 0 | 22 | 11 |
| test | openspec validate | 247 | 21 min 44 s | 3 s | 33 s | 164 | 62 | 21 |
| test | upstream, selected programs | 65 | 14 min 48 s | 10 s | 56 s | 42 | 17 | 6 |
| test | tests of the Python tools | 78 | 13 min 34 s | 4 s | 1 min 01 s | 61 | 14 | 3 |
| test | call-site self-test | 21 | 5 min 29 s | 7 s | 1 min 58 s | 20 | 0 | 1 |
| test | other suite | 1 | 20 s | 20 s | 20 s | 1 | 0 | 0 |
| test | pass list checked from a full run | 12 | 17 s | 1 s | 3 s | 3 | 9 | 0 |
| measure | full corpus (old compiler) | 14 | 1 h 48 min | 6 min 14 s | 15 min 56 s | | | |
| measure | verification programs (old compiler) | 196 | 1 h 46 min | 9 s | 24 min 51 s | | | |
| measure | recording expected results (old compiler) | 164 | 1 h 40 min | 7 s | 21 min 41 s | | | |
| measure | old compiler run directly | 211 | 1 h 13 min | 3 s | 10 min 03 s | | | |
| measure | other suite (old compiler) | 4 | 37 min 14 s | 8 s | 36 min 57 s | | | |
| measure | corpus, selected programs (old compiler) | 61 | 35 min 33 s | 15 s | 6 min 26 s | | | |
| measure | slice list (old compiler) | 6 | 21 min 36 s | 3 min 00 s | 6 min 51 s | | | |
| measure | upstream pass list (old compiler) | 4 | 14 min 43 s | 2 min 38 s | 6 min 51 s | | | |
| measure | full upstream suite (old compiler) | 4 | 9 min 33 s | 3 min 00 s | 3 min 31 s | | | |
| measure | upstream, selected programs (old compiler) | 5 | 18 s | 4 s | 6 s | | | |
| build | cargo build | 567 | 3 h 18 min | 10 s | 10 min 02 s | 163 | 108 | 296 |
| build | extension build (npm) | 18 | 5 min 20 s | 20 s | 41 s | 0 | 0 | 18 |

## Runs repeated with nothing edited

A run of a statement that the same session had run before, word for word, with no file edited in between
(`README.md`, "Repeats"). By the result of the run before it: after a pass the repeat could tell nothing new,
unless something else had changed.

| Bucket | Kind | Repeats | Time | After a pass | Time | After a failure | After a result not known |
|---|---|---|---|---|---|---|---|
| test | slice list | 2 | 3 min 01 s | 2 | 3 min 01 s | 0 | 0 |
| test | tier 1, part (cargo test -p) | 53 | 3 min 27 s | 22 | 1 min 50 s | 25 | 6 |
| test | tier 1 (cargo test) | 110 | 31 min 42 s | 18 | 6 min 50 s | 41 | 51 |
| test | upstream pass list | 1 | 11 s | 1 | 11 s | 0 | 0 |
| test | repo check | 32 | 6 min 23 s | 12 | 1 min 04 s | 16 | 4 |
| test | differential pass list | 1 | 11 s | 1 | 11 s | 0 | 0 |
| test | fmt | 21 | 3 min 49 s | 0 | 0 s | 13 | 8 |
| test | call-site check | 6 | 46 s | 1 | 10 s | 5 | 0 |
| test | clippy | 33 | 3 min 13 s | 1 | 3 s | 14 | 18 |
| test | extension tests (npm) | 10 | 2 min 38 s | 3 | 22 s | 0 | 7 |
| test | corpus, selected programs | 3 | 1 min 08 s | 1 | 36 s | 1 | 1 |
| test | openspec validate | 11 | 37 s | 2 | 9 s | 3 | 6 |
| test | upstream, selected programs | 4 | 1 min 25 s | 0 | 0 s | 2 | 2 |
| test | tests of the Python tools | 3 | 7 s | 1 | 2 s | 2 | 0 |
| test | call-site self-test | 1 | 2 s | 1 | 2 s | 0 | 0 |
| build | cargo build | 61 | 12 min 46 s | 8 | 3 min 28 s | 32 | 21 |
| build | extension build (npm) | 2 | 5 s | 0 | 0 s | 0 | 2 |
| | **All** | 354 | 1 h 11 min | 74 | 17 min 59 s | 154 | 126 |

## The 20 longest runs

| Start | Kind | Time | Background |
|---|---|---|---|
| 2026-10-16 17:47:28 | tier 1, part (cargo test -p) | 2 h 05 min | yes |
| 2026-10-02 15:24:43 | other suite (old compiler) | 36 min 57 s | yes |
| 2026-10-10 11:37:46 | verification programs (old compiler) | 24 min 51 s | yes |
| 2026-10-03 16:05:50 | recording expected results (old compiler) | 21 min 41 s | yes |
| 2026-10-03 16:28:51 | recording expected results (old compiler) | 21 min 01 s | yes |
| 2026-10-03 17:08:21 | full corpus (old compiler) | 15 min 56 s | yes |
| 2026-10-03 17:25:48 | full corpus (old compiler) | 15 min 22 s | yes |
| 2026-10-03 16:50:01 | full corpus (old compiler) | 14 min 38 s | yes |
| 2026-10-10 09:58:39 | full corpus (old compiler) | 11 min 59 s | yes |
| 2026-10-10 11:18:53 | full corpus (old compiler) | 11 min 33 s | yes |
| 2026-10-06 15:15:27 | tier 1, part (cargo test -p) | 10 min 04 s |  |
| 2026-10-07 11:35:14 | old compiler run directly | 10 min 03 s |  |
| 2026-10-16 12:15:48 | tier 1, part (cargo test -p) | 10 min 02 s |  |
| 2026-10-04 19:51:29 | cargo build | 10 min 02 s |  |
| 2026-10-16 17:37:27 | tier 1, part (cargo test -p) | 10 min 01 s |  |
| 2026-10-10 11:48:42 | full corpus (old compiler) | 10 min 01 s |  |
| 2026-10-11 14:23:28 | old compiler run directly | 10 min 01 s |  |
| 2026-10-03 14:08:10 | extension tests (npm) | 8 min 52 s | yes |
| 2026-10-16 23:31:25 | call-site check | 8 min 34 s |  |
| 2026-10-14 19:19:01 | slice list | 8 min 12 s |  |

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
| m1-vscode-extension-v0 | archived | 6 | 2026-10-02 22:56 | 2026-10-03 14:20 | 15 h 23 min | 2 h 24 min | 21 min 21 s | 4 s | 4 min 19 s | 11 min 03 s | 3 | 2 min 59 s | 3 (13 h 08 min) | 9 h 00 min | 29 min 21 s |
| m2-golden-corpus | archived | 3 | 2026-10-03 15:21 | 2026-10-03 17:51 | 2 h 29 min | 1 h 20 min | 41 s | 12 min 42 s | 0 s | 1 h 28 min | 6 | 6 min 26 s | 7 (1 h 04 min) | 14 min 05 s |  |
| m2-workspace-and-slice | archived, not seen | 3 | 2026-10-03 18:08 | 2026-10-04 10:24 (last seen) | 16 h 15 min | 1 h 10 min | 8 min 31 s | 7 min 53 s | 2 min 40 s | 0 s | 12 | 5 min 01 s | 7 (14 h 48 min) | 13 h 23 min |  |
| m2-procedures-and-errors | archived | 8 | 2026-10-04 10:54 | 2026-10-04 18:42 | 7 h 48 min | 2 h 52 min | 31 min 52 s | 4 min 40 s | 5 min 19 s | 0 s | 12 | 4 min 58 s | 4 (2 h 59 min) | 2 h 14 min | 2 min 55 s |
| m2-upstream-tests | archived, not seen | 3 | 2026-10-04 18:23 | 2026-10-04 20:32 (last seen) | 2 h 09 min | 1 h 23 min | 21 min 25 s | 6 s | 13 min 14 s | 1 min 04 s | 7 | 10 min 02 s | 1 (9 min 29 s) | 9 min 29 s |  |
| m2-parser-breadth | archived | 17 | 2026-10-04 20:49 | 2026-10-07 20:37 | 71 h 48 min | 5 h 20 min | 1 h 15 min | 3 min 30 s | 14 min 55 s | 55 min 02 s | 25 | 2 min 34 s | 5 (52 min 15 s) | 14 min 16 s | 9 min 04 s |
| m2-control-flow-slice | archived, not seen | 19 | 2026-10-05 18:34 | 2026-10-07 14:42 (last seen) | 44 h 08 min | 5 h 50 min | 54 min 57 s | 25 min 08 s | 10 min 20 s | 24 min 36 s | 30 | 10 min 04 s | 8 (21 h 40 min) | 18 h 16 min |  |
| m2-arrays-and-types | archived | 6 | 2026-10-07 14:56 | 2026-10-07 20:37 | 5 h 41 min | 1 h 56 min | 25 min 33 s | 43 s | 4 min 08 s | 9 min 48 s | 20 | 2 min 22 s | 1 (1 h 46 min) | 1 h 46 min | 1 h 48 min |
| m2-core-builtins | archived | 2 | 2026-10-07 20:52 | 2026-10-08 11:24 | 14 h 31 min | 1 h 50 min | 20 min 33 s | 3 min 40 s | 5 min 18 s | 6 min 20 s | 5 | 2 min 33 s | 2 (12 h 37 min) | 12 h 25 min | 33 min 07 s |
| m2-language-server | archived | 3 | 2026-10-08 12:18 | 2026-10-08 14:10 | 1 h 52 min | 48 min 40 s | 5 min 51 s | 0 s | 3 min 07 s | 55 s | 10 | 56 s | 2 (49 min 02 s) | 32 min 05 s | 37 min 33 s |
| m2-numeric-types | archived, not seen | 19 | 2026-10-08 14:33 | 2026-10-09 23:06 (last seen) | 32 h 33 min | 5 h 49 min | 1 h 04 min | 9 min 18 s | 8 min 53 s | 1 h 08 min | 28 | 6 min 32 s | 6 (21 h 05 min) | 17 h 22 min |  |
| m2-builtin-statements | archived | 9 | 2026-10-09 23:39 | 2026-10-10 16:18 | 16 h 38 min | 2 h 57 min | 31 min 00 s | 13 min 28 s | 4 min 47 s | 18 min 28 s | 21 | 2 min 01 s | 3 (8 h 13 min) | 5 h 25 min | 3 min 01 s |
| m2-demand-report-and-runner-tests | archived | 1 | 2026-10-10 16:40 | 2026-10-10 16:44 | 4 min 04 s | 4 min 04 s | 11 s | 0 s | 0 s | 0 s | 0 | 6 s | 0 (0 s) | 0 s | 37 s |
| m2-screen-state-check | archived | 1 | 2026-10-10 19:32 | 2026-10-10 20:15 | 43 min 04 s | 43 min 04 s | 4 min 37 s | 5 min 23 s | 0 s | 9 min 08 s | 1 | 1 min 53 s | 0 (0 s) | 0 s | 13 min 57 s |
| m2-defxxx-common-arrays | archived | 20 | 2026-10-10 20:41 | 2026-10-12 10:30 | 37 h 49 min | 8 h 03 min | 2 h 14 min | 10 min 33 s | 21 min 54 s | 31 min 21 s | 33 | 3 min 48 s | 8 (14 h 29 min) | 6 h 06 min | 49 min 18 s |
| m2-console-screenhide | archived, not seen | 11 | 2026-10-12 10:59 | 2026-10-12 19:57 (last seen) | 8 h 57 min | 3 h 24 min | 1 h 00 min | 4 min 52 s | 6 min 07 s | 7 min 53 s | 23 | 3 min 51 s | 6 (2 h 41 min) | 1 h 55 min |  |
| m3-window-output | archived | 3 | 2026-10-12 20:09 | 2026-10-12 22:06 | 1 h 56 min | 1 h 52 min | 18 min 44 s | 6 min 21 s | 1 min 41 s | 16 min 35 s | 6 | 2 min 29 s | 0 (0 s) | 0 s | 9 min 47 s |
| m3-get-put-bload | archived | 4 | 2026-10-12 22:37 | 2026-10-13 11:26 | 12 h 48 min | 1 h 27 min | 12 min 04 s | 2 min 38 s | 3 min 33 s | 2 min 11 s | 18 | 1 min 07 s | 2 (11 h 02 min) | 10 h 57 min | 25 min 20 s |
| m3-sound | archived | 6 | 2026-10-13 11:43 | 2026-10-13 14:50 | 3 h 07 min | 1 h 45 min | 27 min 13 s | 2 min 06 s | 2 min 30 s | 0 s | 15 | 4 min 30 s | 1 (5 min 01 s) | 5 min 01 s | 19 min 05 s |
| m3-console-files | archived | 5 | 2026-10-13 15:19 | 2026-10-14 08:45 | 17 h 25 min | 1 h 50 min | 31 min 31 s | 3 min 14 s | 5 min 29 s | 0 s | 9 | 2 min 25 s | 0 (0 s) | 0 s | 15 h 38 min |
| m3-scripted-game | archived | 4 | 2026-10-14 09:02 | 2026-10-14 10:53 | 1 h 51 min | 1 h 15 min | 15 min 04 s | 1 min 30 s | 30 s | 0 s | 5 | 5 min 54 s | 0 (0 s) | 0 s |  |
| m3-images-fonts-mouse | archived | 13 | 2026-10-14 10:58 | 2026-10-14 14:37 | 3 h 38 min | 2 h 34 min | 40 min 17 s | 5 min 37 s | 7 min 23 s | 0 s | 2 | 5 min 57 s | 1 (23 min 08 s) | 23 min 08 s | 37 min 45 s |
| m3-mem | archived | 4 | 2026-10-14 14:42 | 2026-10-14 17:42 | 3 h 00 min | 2 h 32 min | 22 min 24 s | 2 min 17 s | 9 min 28 s | 16 min 32 s | 9 | 2 min 09 s | 0 (0 s) | 0 s | 11 min 18 s |
| m3-asserts-debug-zlib | archived, not seen | 4 | 2026-10-14 17:50 | 2026-10-14 20:03 (last seen) | 2 h 13 min | 1 h 38 min | 37 min 17 s | 1 min 12 s | 4 min 13 s | 6 min 50 s | 5 | 8 min 12 s | 2 (28 min 30 s) | 21 min 24 s |  |
| m3-redim-retain | archived | 6 | 2026-10-14 20:33 | 2026-10-14 22:03 | 1 h 30 min | 1 h 14 min | 16 min 58 s | 42 s | 2 min 24 s | 25 min 03 s | 1 | 2 min 46 s | 3 (21 min 39 s) | 10 min 08 s | 24 min 09 s |
| m3-typefields | archived | 11 | 2026-10-14 22:20 | 2026-10-15 13:57 | 15 h 36 min | 2 h 55 min | 38 min 39 s | 1 min 54 s | 5 min 24 s | 9 min 51 s | 10 | 5 min 59 s | 4 (2 h 10 min) | 1 h 12 min | 4 min 27 s |
| m3-whole-array-assign | archived | 4 | 2026-10-15 14:14 | 2026-10-15 15:25 | 1 h 10 min | 1 h 02 min | 18 min 50 s | 44 s | 1 min 14 s | 0 s | 1 | 4 min 53 s | 0 (0 s) | 0 s | 5 min 44 s |
| m3-declare-library | archived | 18 | 2026-10-15 15:56 | 2026-10-16 14:33 | 22 h 37 min | 3 h 05 min | 40 min 13 s | 1 min 02 s | 3 min 31 s | 52 s | 6 | 10 min 02 s | 7 (16 h 53 min) | 15 h 23 min | 30 min 05 s |
| m3-measured-copies | archived, not seen | 1 | 2026-10-16 14:47 | 2026-10-16 16:10 (last seen) | 1 h 22 min | 52 min 42 s | 24 min 37 s | 6 s | 1 min 13 s | 0 s | 5 | 6 min 01 s | 0 (0 s) | 0 s |  |
| m3-divergence-trace | archived, not seen | 1 | 2026-10-16 15:38 | 2026-10-16 15:58 (last seen) | 20 min 42 s | 20 min 42 s | 7 min 27 s | 0 s | 1 min 21 s | 0 s | 0 | 2 min 38 s | 0 (0 s) | 0 s |  |
| m3-old-wording-notes | archived, not seen | 1 | 2026-10-16 15:58 | 2026-10-16 16:08 (last seen) | 9 min 28 s | 9 min 28 s | 3 min 26 s | 2 s | 0 s | 0 s | 0 | 1 min 40 s | 0 (0 s) | 0 s |  |
| m3-sound-handles | archived | 5 | 2026-10-16 16:34 | 2026-10-16 22:44 | 6 h 10 min | 1 h 45 min | 18 min 49 s | 4 min 05 s | 1 min 29 s | 2 h 24 min | 2 | 10 min 01 s | 1 (2 h 05 min) | 2 h 05 min | 21 min 49 s |
| m3-versioninfo | archived | 5 | 2026-10-16 22:57 | 2026-10-17 00:05 | 1 h 07 min | 55 min 21 s | 34 min 08 s | 5 s | 1 min 49 s | 4 min 28 s | 1 | 8 min 34 s | 0 (0 s) | 0 s | 13 min 49 s |
| m3-arraycopy | archived | 4 | 2026-10-17 00:14 | 2026-10-17 09:05 | 8 h 50 min | 1 h 40 min | 19 min 10 s | 2 min 26 s | 5 min 59 s | 18 min 29 s | 3 | 2 min 38 s | 2 (11 min 32 s) | 5 min 47 s | 6 min 19 s |
| m3-events | open | 1 | 2026-10-17 09:17 | 2026-10-17 10:44 (last seen) | 1 h 26 min | 1 h 20 min | 12 min 08 s | 1 min 37 s | 3 min 52 s | 16 min 07 s | 4 | 3 min 18 s | 1 (6 min 03 s) | 6 min 03 s |  |
| (no change named) |  | 290 | 2026-10-02 14:19 | 2026-10-17 09:17 |  | 22 h 42 min | 1 h 30 min | 1 h 24 min | 9 min 45 s | 1 h 49 min | 55 | 10 min 01 s | 39 (17 h 31 min) | 6 h 14 min |  |

## By kind of work

What each tool call did (`README.md`, "Kinds of work"): counts of actions and the bytes edits wrote. These
are not shares of time: the transcript does not say how long a piece of writing took.

| Kind of work | Actions | Bytes written | Share of bytes written |
|---|---|---|---|
| product code | 3240 | 2798539 | 29% |
| tests and fixtures | 2106 | 1696908 | 17% |
| measurement preparation | 499 | 490802 | 5% |
| documents and planning | 3118 | 3665001 | 38% |
| generated files | 3 | 293 | 0% |
| scratch files | 546 | 1078061 | 11% |
| reading | 3987 | 0 |  |
| test runs | 2496 | 0 |  |
| measurement runs | 549 | 0 |  |
| builds | 426 | 0 |  |
| other commands | 9162 | 0 |  |
| other | 156 | 5660 | 0% |

Actions per change:

| Change | product code | tests and fixtures | measurement preparation | documents and planning | generated files | scratch files | reading | test runs | measurement runs | builds | other commands | other |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
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
| m3-arraycopy | 38 | 20 | 11 | 64 | 0 | 8 | 43 | 56 | 9 | 14 | 209 | 1 |
| m3-asserts-debug-zlib | 48 | 46 | 0 | 44 | 0 | 12 | 19 | 52 | 4 | 9 | 228 | 3 |
| m3-console-files | 141 | 52 | 5 | 59 | 0 | 7 | 65 | 81 | 20 | 9 | 166 | 0 |
| m3-declare-library | 155 | 56 | 7 | 148 | 0 | 15 | 108 | 70 | 5 | 18 | 360 | 1 |
| m3-divergence-trace | 7 | 16 | 0 | 7 | 0 | 1 | 3 | 5 | 0 | 1 | 19 | 0 |
| m3-events | 31 | 10 | 10 | 33 | 0 | 3 | 5 | 40 | 11 | 8 | 163 | 1 |
| m3-get-put-bload | 16 | 27 | 3 | 40 | 3 | 17 | 106 | 39 | 8 | 18 | 87 | 1 |
| m3-images-fonts-mouse | 53 | 58 | 5 | 74 | 0 | 25 | 96 | 79 | 18 | 12 | 342 | 3 |
| m3-measured-copies | 33 | 23 | 1 | 13 | 0 | 10 | 20 | 28 | 0 | 3 | 78 | 0 |
| m3-mem | 130 | 24 | 4 | 57 | 0 | 31 | 77 | 67 | 15 | 29 | 299 | 2 |
| m3-old-wording-notes | 5 | 6 | 0 | 15 | 0 | 1 | 7 | 4 | 1 | 0 | 12 | 0 |
| m3-redim-retain | 31 | 15 | 2 | 42 | 0 | 8 | 27 | 39 | 4 | 6 | 148 | 1 |
| m3-scripted-game | 0 | 37 | 0 | 55 | 0 | 16 | 47 | 30 | 7 | 1 | 106 | 3 |
| m3-sound | 32 | 34 | 9 | 100 | 0 | 8 | 33 | 63 | 7 | 10 | 141 | 2 |
| m3-sound-handles | 15 | 79 | 8 | 64 | 0 | 5 | 33 | 38 | 17 | 4 | 193 | 0 |
| m3-typefields | 116 | 47 | 10 | 77 | 0 | 19 | 94 | 86 | 17 | 24 | 400 | 6 |
| m3-versioninfo | 31 | 8 | 1 | 29 | 0 | 1 | 24 | 41 | 1 | 3 | 105 | 3 |
| m3-whole-array-assign | 31 | 16 | 3 | 56 | 0 | 6 | 38 | 35 | 9 | 5 | 147 | 1 |
| m3-window-output | 40 | 77 | 1 | 91 | 0 | 23 | 118 | 38 | 9 | 5 | 71 | 2 |
| (no change named) | 217 | 462 | 163 | 431 | 0 | 117 | 1191 | 340 | 120 | 27 | 2290 | 76 |
