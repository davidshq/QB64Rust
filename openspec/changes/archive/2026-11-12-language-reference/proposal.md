# Proposal

## Why

QB64Rust has no help text. A hover shows a built-in's signature and the word "Built-in"
(`crates\lsp\src\hover.rs`), and `study\47` names the missing help as a gap for a new user. The QB64pe wiki
cannot fill it: it states no license, so no wiki text may be shipped (`DECISIONS.md` 2026-10-02), and it describes
QB64pe, not this compiler's errors and divergences.

`study\49` plans a reference written from this repository's own sources: the merged specs, the recorded
programs, the built-in table and the two divergence registers. It is to be written by Opus and Sonnet sessions,
about 35 of them. The user cannot read 500 pages to check them, so the checking must be mechanical. The user
accepted the study's plan and the defaults of its five questions (2026-11-11, "do it").

## What Changes

- **A tool, `tools\reference\reference.py`**, with `inventory` (every word that needs a page), `evidence` (what the
  repository knows about one word), `skeleton`, `render` (the generated blocks, the index and the appendix of
  constants), `--check` and `overlap` (a local comparison with the wiki cache). Its hand-kept part is
  `tools\reference\words.toml`.
- **Pages in `docs\reference\entries\`**, one per word, in a fixed layout. Every rule names its source in a
  `src:` comment that `--check` resolves: a spec requirement, a recorded program, a divergence row, or code read.
- **Examples as recorded programs** in `tests\reference\examples\`, run by the corpus suite of the test runner
  with `--corpus-root tests\reference`, held to `tests\reference\pass.list`.
- **The availability of each page computed**, by compiling the call-site programs of its words, its examples and
  its probe lines with `qb64rust -z`, into `tests\reference\availability.json`.
- **Two steps of `check`**: `reference` (quick layer: `--check` with the compiler) and `reference-examples` (full
  layer, may run in the suites VM).
- **Six pilot pages**: `LEFT$`, `LINE`, `SELECT CASE`, `$INCLUDE`, `_MEMCOPY` and `FRE`.
- **The batches of `study\49` §7 as tasks** of this change, one session each.

Out of scope, each a later change or a `SOMEDAY.md` row:
- Showing the reference in the hover or by F1, and shipping it in the downloads (`SOMEDAY.md`, group `f1-help`).
- A site built from the pages.
- A page per OpenGL routine: the OpenGL batch writes one chapter and one generated table.
