# Time spent testing

Made by `python tools\testtime\test_time.py` from the transcripts of the Claude Code sessions; do not edit.
What the columns mean, and what the numbers cannot say: `README.md` beside this file.

188 sessions from 2026-10-02 to 2026-10-13; 3554 runs on record.

## In all

| | Time | Share of active time |
|---|---|---|
| Active time (Claude working, the user not waited for) | 68 h 22 min | |
| Inside commands of any kind | 29 h 43 min | 43% |
| **Waiting for test runs** | 11 h 22 min | 17% |
| Waiting for measurements with the old compiler | 3 h 02 min | 4% |
| Waiting for builds | 2 h 01 min | 3% |
| Everything else (reading, thinking, writing code and documents, other commands) | 51 h 55 min | 76% |
| Test runs and measurements in the background (not waited for) | 7 h 35 min | |

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
| 2026-10-13 | 10 | 2 h 45 min | 44 min 36 s | 5 min 09 s | 6 min 18 s | 27% | 191 | 8 min 17 s |

A day's runs are those started on it; a session that runs past midnight has its active time on the day it
started, so a share above 100% can show on the day after.

## By kind of run

| Bucket | Kind | Runs | Total | Median | Longest | Passed | Failed | Result not known |
|---|---|---|---|---|---|---|---|---|
| test | slice list | 85 | 2 h 50 min | 1 min 49 s | 6 min 40 s | 60 | 2 | 23 |
| test | tier 1 (cargo test) | 497 | 2 h 11 min | 11 s | 2 min 29 s | 132 | 144 | 221 |
| test | full corpus | 30 | 1 h 22 min | 2 min 07 s | 7 min 33 s | 6 | 11 | 13 |
| test | upstream pass list | 48 | 1 h 14 min | 1 min 22 s | 6 min 40 s | 32 | 1 | 15 |
| test | tier 1, part (cargo test -p) | 504 | 1 h 11 min | 5 s | 10 min 04 s | 302 | 129 | 73 |
| test | repo check | 362 | 52 min 52 s | 4 s | 3 min 45 s | 311 | 30 | 21 |
| test | fmt | 305 | 48 min 14 s | 5 s | 2 min 30 s | 0 | 34 | 271 |
| test | extension tests (npm) | 110 | 45 min 04 s | 14 s | 8 min 52 s | 68 | 8 | 34 |
| test | clippy | 281 | 37 min 59 s | 5 s | 1 min 23 s | 67 | 43 | 171 |
| test | differential pass list | 28 | 36 min 17 s | 47 s | 4 min 36 s | 9 | 1 | 18 |
| test | call-site check | 64 | 26 min 36 s | 15 s | 2 min 13 s | 45 | 9 | 10 |
| test | corpus, selected programs | 114 | 24 min 08 s | 7 s | 1 min 53 s | 93 | 12 | 9 |
| test | openspec validate | 181 | 14 min 47 s | 3 s | 33 s | 124 | 41 | 16 |
| test | full upstream suite | 14 | 12 min 29 s | 43 s | 2 min 33 s | 0 | 6 | 8 |
| test | tests of the Python tools | 40 | 8 min 14 s | 5 s | 1 min 00 s | 29 | 9 | 2 |
| test | call-site self-test | 19 | 5 min 46 s | 7 s | 1 min 58 s | 18 | 0 | 1 |
| test | upstream, selected programs | 33 | 5 min 45 s | 5 s | 33 s | 18 | 12 | 3 |
| test | other suite | 1 | 20 s | 20 s | 20 s | 1 | 0 | 0 |
| measure | full corpus (old compiler) | 15 | 1 h 48 min | 5 min 00 s | 15 min 56 s | | | |
| measure | recording expected results (old compiler) | 108 | 1 h 29 min | 7 s | 21 min 41 s | | | |
| measure | verification programs (old compiler) | 106 | 1 h 07 min | 10 s | 24 min 51 s | | | |
| measure | old compiler run directly | 152 | 1 h 04 min | 3 s | 10 min 03 s | | | |
| measure | other suite (old compiler) | 4 | 37 min 14 s | 8 s | 36 min 57 s | | | |
| measure | corpus, selected programs (old compiler) | 41 | 32 min 14 s | 19 s | 6 min 26 s | | | |
| measure | slice list (old compiler) | 9 | 23 min 29 s | 2 min 38 s | 6 min 51 s | | | |
| measure | upstream pass list (old compiler) | 7 | 16 min 36 s | 2 min 36 s | 6 min 51 s | | | |
| measure | full upstream suite (old compiler) | 5 | 9 min 45 s | 2 min 38 s | 3 min 31 s | | | |
| measure | differential pass list (old compiler) | 4 | 2 min 04 s | 31 s | 52 s | | | |
| measure | upstream, selected programs (old compiler) | 5 | 18 s | 4 s | 6 s | | | |
| build | cargo build | 364 | 2 h 16 min | 9 s | 10 min 02 s | 118 | 65 | 181 |
| build | extension build (npm) | 18 | 5 min 20 s | 20 s | 41 s | 0 | 0 | 18 |

## Runs repeated with nothing edited

A run of a statement that the same session had run before, word for word, with no file edited in between
(`README.md`, "Repeats"). By the result of the run before it: after a pass the repeat could tell nothing new,
unless something else had changed.

| Bucket | Kind | Repeats | Time | After a pass | Time | After a failure | After a result not known |
|---|---|---|---|---|---|---|---|
| test | slice list | 4 | 9 min 43 s | 4 | 9 min 43 s | 0 | 0 |
| test | tier 1 (cargo test) | 106 | 28 min 54 s | 18 | 6 min 48 s | 41 | 47 |
| test | full corpus | 7 | 12 min 40 s | 4 | 9 min 34 s | 2 | 1 |
| test | tier 1, part (cargo test -p) | 53 | 3 min 26 s | 21 | 1 min 55 s | 23 | 9 |
| test | repo check | 24 | 5 min 41 s | 6 | 32 s | 14 | 4 |
| test | fmt | 15 | 3 min 12 s | 0 | 0 s | 11 | 4 |
| test | extension tests (npm) | 12 | 4 min 29 s | 5 | 2 min 13 s | 0 | 7 |
| test | clippy | 28 | 2 min 45 s | 1 | 3 s | 11 | 16 |
| test | call-site check | 4 | 43 s | 1 | 20 s | 3 | 0 |
| test | corpus, selected programs | 13 | 3 min 20 s | 11 | 2 min 29 s | 1 | 1 |
| test | openspec validate | 8 | 23 s | 2 | 9 s | 1 | 5 |
| test | tests of the Python tools | 3 | 7 s | 1 | 2 s | 2 | 0 |
| test | call-site self-test | 1 | 2 s | 1 | 2 s | 0 | 0 |
| test | upstream, selected programs | 3 | 1 min 03 s | 0 | 0 s | 2 | 1 |
| build | cargo build | 47 | 9 min 28 s | 6 | 2 min 19 s | 27 | 14 |
| build | extension build (npm) | 2 | 5 s | 0 | 0 s | 0 | 2 |
| | **All** | 330 | 1 h 26 min | 81 | 36 min 09 s | 138 | 111 |

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
