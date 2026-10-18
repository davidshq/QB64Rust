# Archived studies

Closed documents, kept for the record. Read one only when a live document points to it for a fact or a reason
the task in hand needs, or when the user asks (`CLAUDE.md` working rule 10). Nothing here is edited, so paths
inside these documents may refer to their old locations (`study\11`, `study\20`, `verification\qb64fresh\`). A
study keeps its number when it is archived.

## Parts of active studies

Moved 2026-10-18. Each file has the same name as the active study it came from and keeps its headings; a section
not found in the active file is here.

| File | What was moved | Why |
|---|---|---|
| `00-synthesis.md` | §1 (the decisions table of 2026-10-02), §2's milestone table, §10 (the testing position as of 2026-10-03 to 2026-10-10), the closed row of §13's table | Their current homes are `DECISIONS.md`, `ROADMAP.md`, `crates\README.md`, the `tests\*\README.md` files and `CLAUDE.md` working rule 9; their figures had gone out of date |
| `04-ide-debugger.md` | Parts A–D and Part E §1–§4: the text-mode IDE's internals (the compiler side of the IDE, `ide2`, text buffer, dialog toolkit, every dialog and menu, help, configuration, export, converters) | The IDE is not ported (`DECISIONS.md` 2026-10-02); the debugger (Part F) and the assessment (Part G) stay active for M5 |

## Reviews of other repositories

Nothing in the active plan depends on them. Their conclusions are summarised in `study\00-synthesis.md` §11; the
decisions they led to are in `DECISIONS.md`.

| File | Subject | Conclusion |
|---|---|---|
| `11-existing-vscode-extension.md` | The user's `vscode-qb64fresh` and the community `qb64pe-vscode` | Reference only, no code copied (the import recommendation was overruled) |
| `12-qb64fresh-review.md` | QB64Fresh, the user's earlier Rust rewrite | No code; take its 261 test programs and process lessons |
| `13-reference-docs.md` | Documentation sources (wiki, manuals, QB64Fresh specs) | Wiki fetched by `tools\wiki\fetch_wiki.py`; manuals stay local |
| `14-docs-new-2-branch.md` | The `docs-new-2` branch on the user's QB64pe fork | A map, never a spec |
| `qb64fresh-scripts\` | Measurement scripts and raw results behind `12` | Locations come from environment variables; see each script's header |

## Reviews and plans whose outcomes are decided and built

Archived 2026-10-18. What each led to is in `DECISIONS.md` (the rows that name it), `CLAUDE.md` or `ROADMAP.md`.

| File | Content (`study\00`'s document map until 2026-10-18) | Where its outcome lives |
|---|---|---|
| `08-qb45-strict-mode.md` | What a strict QuickBASIC 4.5 mode would need (not planned) | Not planned (`DECISIONS.md` 2026-10-02); the differences are in `DIVERGENCES-QB45.md` |
| `15-pre-coding-review.md` | Pre-coding review: bytes not strings, early vertical slice, diagnostics policy, minimal M1 backend, libqb copy | `DECISIONS.md`, `clippy.toml` |
| `19-test-cadence.md` | Test cadence: four tiers, what a full run costs, how the new compiler's runs are kept fast | `CLAUDE.md` working rule 9; the tiers in `crates\README.md` |
| `20-review-and-order.md` | Review of code and plan after the first slice (2026-10-04); panel outcome; the order of work (replaced by `22` §5) | `ROADMAP.md`; its order replaced by `22`, `23`, `24`, then `STATUS.md` |
| `21-rust-review.md` | Rust review of the workspace setup: lints adopted and not, CI, the repo check, how to apply them | `Cargo.toml`, `clippy.toml`, `rust.yml` |
| `22-review-tests-and-order.md` | Second review (2026-10-04): test inputs from QB64pe and QB64Fresh and what is taken, the upstream yardstick (x of 279); its order of work is replaced by `23` §4 | `tests\upstream\`, `ROADMAP.md` (M4) |
| `23-review-and-order.md` | Third review (2026-10-05): path check of the codebase, the panel's outcome; its order of work is replaced by `24` §4 | `DECISIONS.md` |
| `24-review-arrays-order.md` | Fourth review (2026-10-07): code against plan after the control-flow slice's groups 1–5, why arrays and `TYPE` move before the type table and built-ins, `STATUS.md` as entry point; the current order of work | `DECISIONS.md` |
| `25-ir-review.md` | IR review (2026-10-07): what the lowering does and the emitter still does, keep or merge, the place question for the arrays-and-`TYPE` slice with the old compiler's store rules to measure | `DECISIONS.md` |
| `26-fifth-review.md` | Fifth review (2026-10-07): a wrong-code bug in `$IF`, clean programs that tier 2 never runs, member arrays no longer behind `$UNSTABLE`, `$CONSOLE` tests as M2 work, blockers by kind and the new order (all accepted 2026-10-07; plain member arrays un-deferred) | `DECISIONS.md`, `tests\known_clean_not_passing.list` |
| `27-sixth-review.md` | Sixth review (2026-10-08): architecture holds, no bug found; the corpus is blocked by built-in statements, file I/O and `DATA` that no step names (step 9 to become "by demand"), upstream by types and arrays; "`Ty` as a type table" dropped; the language server's design points (one binary, parser only, the encoding boundary, includes from open documents, two diagnostic sources); all accepted 2026-10-08 | `DECISIONS.md` |
| `28-seventh-review.md` | Seventh review (2026-10-09): not going astray; the expected outputs come from the reference clone while CI links the 4.7.0 release's libqb (16 commits apart); graphics are on the critical path to M4 (built-ins used by `qb64pe.bas`, the corpus and upstream, counted), a screen-state oracle and a call-site check proposed; Civil War Strategy assessed as the real-program graphics target; `STATUS.md` cut to the current state; the formatter off M2's exit criterion; the user's answers and what was done are its §9 (the local clone is the reference, the game copied, the formatter out of M2); the screen-state check and the call-site check wait for a yes | `DECISIONS.md`, `tests\programs\` |
| `29-eighth-review.md` | Eighth review (2026-10-10, before step 9a): nothing to change in the code, no bug found; the screen-state hook needs neither the libqb copy nor a libqb change (one extra C++ file linked through `-f:ExtraLinkerFlags`/`CXXLIBS_EXTRA`, `atexit`, programs ending with `SYSTEM`, keys by `PostMessage`), so the copy can stay at M3; keep 9a small, then step 11 before 10 (the game's needs, measured); a demand report over the input sets; tests for the runner; the user's answers and what was done are its §8 (all accepted; the demand report's first numbers; the CI recommendation withdrawn: the reference commit's compiler differs from the release's in typed `VAL`) | `tools\screen\`, `DECISIONS.md` |
| `30-ninth-review.md` | Ninth review (2026-10-16, during `m3-declare-library`): the compiler is in better shape than its documents; no bug found, every local check green; `STATUS.md` and `CLAUDE.md` have become history files (403 lines; an 11.5 KB table cell), decisions Claude made alone are scattered and unconfirmed (a "Waiting on the user" list proposed), `main` is eleven changes behind and tier 2 has never run on GitHub as its own job; the checker's 72 fields and the emitter's transcription of the old C++ noted for M4 and M6; what not to change; an order of actions | `DECISIONS.md` 2026-10-16, `STATUS.md` |
| `31-development-velocity-plan.md` | Development velocity plan (2026-10-16): four changes, each marked with its result as it is built: CLI tests that never wait for a person or for ever, one home per kind of document, closing runs that run each program and setting once, and a record of where the working time goes | `openspec\config.yaml`, `tools\testtime\`, `tools\legacy_tests\` |
| `32-test-cadence-panel.md` | Test cadence and parallel runs, panel notes (2026-10-17): what each closing run is and costs; why runs of the new compiler cannot overlap (one lock for the clone, though they share only libqb's objects) and a readers-writer lock to fix it; a lighter close with the full set at each merge; the three questions answered (adopted, built as `test-parallel-runs`) | `CLAUDE.md` working rule 9 |
| `33-close-tool-panel.md` | The close tool, panel notes (2026-10-17): why `tools\close\close.py` exists (a close worked from memory missed steps); the panel's six points (a program on no list, tier 1 as CI runs it, a run before a rebuild, log times, a reason printed once, the plan is Bash), all done; `--merge` left for a next step | `tools\close\README.md` |
| `34-developing-while-tests-run.md` | Developing while tests run (2026-10-17): what professional teams do (immutable artifacts, content-addressed caches, impact analysis, tiers, background runners); what stops work here: runs read the compiler the next build replaces, and the build cache keys on the clone's `qbx.o`, which every `qb64pe.exe` compile rewrites, so a recording throws the cache away; four recommendations, the first three built as `test-while-building` | `CLAUDE.md` working rule 9, `tools\legacy_tests\README.md` |

## Closed records

| File | Content (`study\00`'s document map until 2026-10-18) |
|---|---|
| `progress-record.md` | Closed record of what `STATUS.md` held until 2026-10-09: the per-change history of M0 to M2 step 8, unchanged |
| `support-record.md` | Closed record of the catalogue "What the compiler supports so far" that `crates\README.md` held until 2026-10-16 (to `m3-sound-handles`), unchanged: the code each feature lives in |
