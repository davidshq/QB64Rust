# Time spent testing

Made by `python tools\testtime\test_time.py` from the transcripts of the Claude Code sessions; do not edit.
What the columns mean, and what the numbers cannot say: `README.md` beside this file.

275 sessions from 2026-10-02 to 2026-10-16; 4860 runs on record.

## In all

| | Time | Share of active time |
|---|---|---|
| Active time (Claude working, the user not waited for) | 93 h 34 min | |
| Inside commands of any kind | 40 h 32 min | 43% |
| **Waiting for test runs** | 16 h 56 min | 18% |
| Waiting for measurements with the old compiler | 3 h 38 min | 4% |
| Waiting for builds | 2 h 46 min | 3% |
| Everything else (reading, thinking, writing code and documents, other commands) | 70 h 13 min | 75% |
| Test runs and measurements in the background (not waited for) | 11 h 02 min | |

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
| 2026-10-16 | 19 | 6 h 00 min | 1 h 33 min | 5 min 38 s | 5 min 02 s | 26% | 191 | 2 h 25 min |

A day's runs are those started on it; a session that runs past midnight has its active time on the day it
started, so a share above 100% can show on the day after.

## By kind of run

| Bucket | Kind | Runs | Total | Median | Longest | Passed | Failed | Result not known |
|---|---|---|---|---|---|---|---|---|
| test | slice list | 130 | 5 h 24 min | 1 min 58 s | 8 min 12 s | 86 | 3 | 41 |
| test | tier 1, part (cargo test -p) | 674 | 4 h 14 min | 5 s | 2 h 05 min | 416 | 161 | 97 |
| test | tier 1 (cargo test) | 612 | 2 h 58 min | 13 s | 2 min 29 s | 148 | 176 | 288 |
| test | upstream pass list | 72 | 1 h 56 min | 1 min 38 s | 6 min 40 s | 47 | 2 | 23 |
| test | repo check | 512 | 1 h 18 min | 5 s | 3 min 45 s | 450 | 39 | 23 |
| test | fmt | 437 | 1 h 03 min | 5 s | 2 min 30 s | 0 | 51 | 386 |
| test | differential pass list | 48 | 59 min 16 s | 52 s | 4 min 36 s | 23 | 2 | 23 |
| test | clippy | 371 | 50 min 05 s | 6 s | 1 min 23 s | 78 | 59 | 234 |
| test | full corpus | 29 | 49 min 26 s | 1 min 08 s | 6 min 20 s | 0 | 18 | 11 |
| test | call-site check | 110 | 48 min 44 s | 14 s | 2 min 42 s | 70 | 26 | 14 |
| test | extension tests (npm) | 110 | 45 min 04 s | 14 s | 8 min 52 s | 68 | 8 | 34 |
| test | corpus, selected programs | 165 | 36 min 48 s | 7 s | 2 min 02 s | 136 | 15 | 14 |
| test | full upstream suite | 29 | 21 min 30 s | 38 s | 2 min 33 s | 0 | 18 | 11 |
| test | openspec validate | 231 | 19 min 55 s | 3 s | 33 s | 152 | 59 | 20 |
| test | upstream, selected programs | 55 | 12 min 49 s | 10 s | 56 s | 35 | 16 | 4 |
| test | tests of the Python tools | 61 | 12 min 18 s | 5 s | 1 min 01 s | 49 | 10 | 2 |
| test | call-site self-test | 21 | 5 min 29 s | 7 s | 1 min 58 s | 20 | 0 | 1 |
| test | other suite | 1 | 20 s | 20 s | 20 s | 1 | 0 | 0 |
| measure | full corpus (old compiler) | 14 | 1 h 48 min | 6 min 14 s | 15 min 56 s | | | |
| measure | verification programs (old compiler) | 185 | 1 h 38 min | 9 s | 24 min 51 s | | | |
| measure | recording expected results (old compiler) | 154 | 1 h 37 min | 7 s | 21 min 41 s | | | |
| measure | old compiler run directly | 207 | 1 h 13 min | 3 s | 10 min 03 s | | | |
| measure | other suite (old compiler) | 4 | 37 min 14 s | 8 s | 36 min 57 s | | | |
| measure | corpus, selected programs (old compiler) | 60 | 35 min 26 s | 15 s | 6 min 26 s | | | |
| measure | slice list (old compiler) | 6 | 21 min 36 s | 3 min 00 s | 6 min 51 s | | | |
| measure | upstream pass list (old compiler) | 4 | 14 min 43 s | 2 min 38 s | 6 min 51 s | | | |
| measure | full upstream suite (old compiler) | 4 | 9 min 33 s | 3 min 00 s | 3 min 31 s | | | |
| measure | upstream, selected programs (old compiler) | 5 | 18 s | 4 s | 6 s | | | |
| build | cargo build | 531 | 3 h 03 min | 10 s | 10 min 02 s | 156 | 101 | 274 |
| build | extension build (npm) | 18 | 5 min 20 s | 20 s | 41 s | 0 | 0 | 18 |

## Runs repeated with nothing edited

A run of a statement that the same session had run before, word for word, with no file edited in between
(`README.md`, "Repeats"). By the result of the run before it: after a pass the repeat could tell nothing new,
unless something else had changed.

| Bucket | Kind | Repeats | Time | After a pass | Time | After a failure | After a result not known |
|---|---|---|---|---|---|---|---|
| test | slice list | 2 | 3 min 01 s | 2 | 3 min 01 s | 0 | 0 |
| test | tier 1, part (cargo test -p) | 51 | 3 min 06 s | 22 | 1 min 50 s | 24 | 5 |
| test | tier 1 (cargo test) | 110 | 31 min 42 s | 18 | 6 min 50 s | 41 | 51 |
| test | upstream pass list | 1 | 11 s | 1 | 11 s | 0 | 0 |
| test | repo check | 31 | 6 min 18 s | 11 | 58 s | 16 | 4 |
| test | fmt | 20 | 3 min 46 s | 0 | 0 s | 13 | 7 |
| test | differential pass list | 1 | 11 s | 1 | 11 s | 0 | 0 |
| test | clippy | 32 | 3 min 10 s | 1 | 3 s | 14 | 17 |
| test | call-site check | 5 | 42 s | 1 | 10 s | 4 | 0 |
| test | extension tests (npm) | 10 | 2 min 38 s | 3 | 22 s | 0 | 7 |
| test | corpus, selected programs | 3 | 1 min 08 s | 1 | 36 s | 1 | 1 |
| test | openspec validate | 10 | 34 s | 2 | 9 s | 3 | 5 |
| test | upstream, selected programs | 4 | 1 min 25 s | 0 | 0 s | 2 | 2 |
| test | tests of the Python tools | 3 | 7 s | 1 | 2 s | 2 | 0 |
| test | call-site self-test | 1 | 2 s | 1 | 2 s | 0 | 0 |
| build | cargo build | 60 | 11 min 40 s | 7 | 2 min 22 s | 32 | 21 |
| build | extension build (npm) | 2 | 5 s | 0 | 0 s | 0 | 2 |
| | **All** | 346 | 1 h 09 min | 72 | 16 min 48 s | 152 | 122 |

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
| 2026-10-14 19:19:01 | slice list | 8 min 12 s |  |
| 2026-10-07 11:26:51 | old compiler run directly | 8 min 05 s |  |
