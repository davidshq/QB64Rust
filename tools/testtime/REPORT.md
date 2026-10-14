# Time spent testing

Made by `python tools\testtime\test_time.py` from the transcripts of the Claude Code sessions; do not edit.
What the columns mean, and what the numbers cannot say: `README.md` beside this file.

198 sessions from 2026-10-02 to 2026-10-14; 3789 runs on record.

## In all

| | Time | Share of active time |
|---|---|---|
| Active time (Claude working, the user not waited for) | 72 h 07 min | |
| Inside commands of any kind | 31 h 39 min | 44% |
| **Waiting for test runs** | 12 h 10 min | 17% |
| Waiting for measurements with the old compiler | 3 h 11 min | 4% |
| Waiting for builds | 2 h 07 min | 3% |
| Everything else (reading, thinking, writing code and documents, other commands) | 54 h 38 min | 76% |
| Test runs and measurements in the background (not waited for) | 7 h 37 min | |

## By day

| Day | Sessions | Active | Tests | Measurements | Builds | Tests, share | Test runs | In the background |
|---|---|---|---|---|---|---|---|---|
| 2026-10-02 | 7 | 4 h 30 min | 1 min 22 s | 1 min 26 s | 1 min 59 s | 1% | 6 | 44 min 13 s |
| 2026-10-03 | 14 | 5 h 19 min | 36 min 59 s | 16 min 25 s | 5 min 58 s | 12% | 153 | 1 h 39 min |
| 2026-10-04 | 14 | 6 h 31 min | 1 h 38 min | 9 min 42 s | 25 min 18 s | 25% | 297 | 5 min 57 s |
| 2026-10-05 | 13 | 3 h 39 min | 34 min 42 s | 13 s | 9 min 27 s | 16% | 129 | 0 s |
| 2026-10-06 | 7 | 2 h 12 min | 35 min 56 s | 1 min 45 s | 3 min 54 s | 27% | 114 | 0 s |
| 2026-10-07 | 26 | 9 h 40 min | 1 h 05 min | 27 min 09 s | 15 min 23 s | 11% | 415 | 1 h 24 min |
| 2026-10-08 | 13 | 3 h 56 min | 32 min 16 s | 15 min 17 s | 7 min 19 s | 14% | 231 | 20 min 48 s |
| 2026-10-09 | 24 | 5 h 51 min | 1 h 01 min | 5 min 03 s | 8 min 40 s | 18% | 278 | 58 min 32 s |
| 2026-10-10 | 19 | 7 h 30 min | 45 min 52 s | 1 h 05 min | 5 min 28 s | 10% | 275 | 1 h 07 min |
| 2026-10-11 | 16 | 7 h 55 min | 1 h 59 min | 19 min 07 s | 21 min 40 s | 25% | 341 | 31 min 37 s |
| 2026-10-12 | 25 | 8 h 27 min | 1 h 45 min | 15 min 34 s | 9 min 45 s | 21% | 286 | 34 min 11 s |
| 2026-10-13 | 17 | 5 h 09 min | 1 h 17 min | 12 min 16 s | 11 min 47 s | 25% | 327 | 9 min 39 s |
| 2026-10-14 | 3 | 1 h 21 min | 15 min 16 s | 1 min 30 s | 30 s | 19% | 36 | 0 s |

A day's runs are those started on it; a session that runs past midnight has its active time on the day it
started, so a share above 100% can show on the day after.

## By kind of run

| Bucket | Kind | Runs | Total | Median | Longest | Passed | Failed | Result not known |
|---|---|---|---|---|---|---|---|---|
| test | slice list | 88 | 3 h 00 min | 1 min 49 s | 6 min 40 s | 63 | 2 | 23 |
| test | tier 1 (cargo test) | 513 | 2 h 18 min | 11 s | 2 min 29 s | 138 | 147 | 228 |
| test | full corpus | 31 | 1 h 23 min | 1 min 54 s | 7 min 33 s | 6 | 12 | 13 |
| test | tier 1, part (cargo test -p) | 541 | 1 h 17 min | 5 s | 10 min 04 s | 324 | 143 | 74 |
| test | upstream pass list | 49 | 1 h 15 min | 1 min 21 s | 6 min 40 s | 33 | 1 | 15 |
| test | repo check | 392 | 57 min 24 s | 4 s | 3 min 45 s | 336 | 34 | 22 |
| test | fmt | 324 | 51 min 44 s | 5 s | 2 min 30 s | 0 | 35 | 289 |
| test | extension tests (npm) | 110 | 45 min 04 s | 14 s | 8 min 52 s | 68 | 8 | 34 |
| test | clippy | 296 | 40 min 58 s | 6 s | 1 min 23 s | 70 | 46 | 180 |
| test | differential pass list | 29 | 36 min 28 s | 46 s | 4 min 36 s | 10 | 1 | 18 |
| test | call-site check | 79 | 33 min 10 s | 16 s | 2 min 13 s | 54 | 14 | 11 |
| test | corpus, selected programs | 127 | 27 min 19 s | 7 s | 1 min 53 s | 103 | 14 | 10 |
| test | openspec validate | 193 | 15 min 44 s | 3 s | 33 s | 132 | 44 | 17 |
| test | full upstream suite | 15 | 12 min 52 s | 42 s | 2 min 33 s | 0 | 7 | 8 |
| test | tests of the Python tools | 43 | 8 min 35 s | 5 s | 1 min 00 s | 32 | 9 | 2 |
| test | upstream, selected programs | 36 | 6 min 11 s | 6 s | 33 s | 21 | 12 | 3 |
| test | call-site self-test | 21 | 5 min 56 s | 7 s | 1 min 58 s | 20 | 0 | 1 |
| test | other suite | 1 | 20 s | 20 s | 20 s | 1 | 0 | 0 |
| measure | full corpus (old compiler) | 15 | 1 h 48 min | 5 min 00 s | 15 min 56 s | | | |
| measure | recording expected results (old compiler) | 120 | 1 h 30 min | 7 s | 21 min 41 s | | | |
| measure | verification programs (old compiler) | 122 | 1 h 14 min | 10 s | 24 min 51 s | | | |
| measure | old compiler run directly | 165 | 1 h 05 min | 3 s | 10 min 03 s | | | |
| measure | other suite (old compiler) | 4 | 37 min 14 s | 8 s | 36 min 57 s | | | |
| measure | corpus, selected programs (old compiler) | 41 | 32 min 14 s | 19 s | 6 min 26 s | | | |
| measure | slice list (old compiler) | 9 | 23 min 29 s | 2 min 38 s | 6 min 51 s | | | |
| measure | upstream pass list (old compiler) | 7 | 16 min 36 s | 2 min 36 s | 6 min 51 s | | | |
| measure | full upstream suite (old compiler) | 5 | 9 min 45 s | 2 min 38 s | 3 min 31 s | | | |
| measure | differential pass list (old compiler) | 4 | 2 min 04 s | 31 s | 52 s | | | |
| measure | upstream, selected programs (old compiler) | 5 | 18 s | 4 s | 6 s | | | |
| build | cargo build | 386 | 2 h 22 min | 8 s | 10 min 02 s | 121 | 71 | 194 |
| build | extension build (npm) | 18 | 5 min 20 s | 20 s | 41 s | 0 | 0 | 18 |

## Runs repeated with nothing edited

A run of a statement that the same session had run before, word for word, with no file edited in between
(`README.md`, "Repeats"). By the result of the run before it: after a pass the repeat could tell nothing new,
unless something else had changed.

| Bucket | Kind | Repeats | Time | After a pass | Time | After a failure | After a result not known |
|---|---|---|---|---|---|---|---|
| test | slice list | 4 | 9 min 43 s | 4 | 9 min 43 s | 0 | 0 |
| test | tier 1 (cargo test) | 108 | 29 min 31 s | 20 | 7 min 26 s | 41 | 47 |
| test | full corpus | 7 | 12 min 40 s | 4 | 9 min 34 s | 2 | 1 |
| test | tier 1, part (cargo test -p) | 58 | 3 min 44 s | 25 | 2 min 10 s | 24 | 9 |
| test | repo check | 27 | 5 min 53 s | 8 | 40 s | 15 | 4 |
| test | fmt | 16 | 3 min 33 s | 0 | 0 s | 12 | 4 |
| test | extension tests (npm) | 12 | 4 min 29 s | 5 | 2 min 13 s | 0 | 7 |
| test | clippy | 29 | 3 min 05 s | 1 | 3 s | 11 | 17 |
| test | call-site check | 4 | 43 s | 1 | 20 s | 3 | 0 |
| test | corpus, selected programs | 13 | 3 min 20 s | 11 | 2 min 29 s | 1 | 1 |
| test | openspec validate | 8 | 23 s | 2 | 9 s | 1 | 5 |
| test | tests of the Python tools | 3 | 7 s | 1 | 2 s | 2 | 0 |
| test | upstream, selected programs | 3 | 1 min 03 s | 0 | 0 s | 2 | 1 |
| test | call-site self-test | 1 | 2 s | 1 | 2 s | 0 | 0 |
| build | cargo build | 47 | 9 min 28 s | 6 | 2 min 19 s | 27 | 14 |
| build | extension build (npm) | 2 | 5 s | 0 | 0 s | 0 | 2 |
| | **All** | 342 | 1 h 27 min | 89 | 37 min 08 s | 141 | 112 |

## The 20 longest runs

| Start | Kind | Time | Background |
|---|---|---|---|
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
| 2026-10-04 19:51:29 | cargo build | 10 min 02 s |  |
| 2026-10-10 11:48:42 | full corpus (old compiler) | 10 min 01 s |  |
| 2026-10-11 14:23:28 | old compiler run directly | 10 min 01 s |  |
| 2026-10-03 14:08:10 | extension tests (npm) | 8 min 52 s | yes |
| 2026-10-07 11:26:51 | old compiler run directly | 8 min 05 s |  |
| 2026-10-11 21:56:14 | full corpus | 7 min 33 s |  |
| 2026-10-10 11:02:08 | full corpus (old compiler) | 7 min 29 s |  |
| 2026-10-12 15:41:09 | full corpus | 7 min 25 s |  |
| 2026-10-11 17:28:48 | full corpus | 7 min 11 s |  |
