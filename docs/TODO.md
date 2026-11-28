# To do

How an item is built and removed:
- A larger item becomes an OpenSpec change, named in `STATUS.md` under "Next". Its row is removed when the change
  is archived.
- A small item needs no OpenSpec change (`CLAUDE.md` working rule 11 says what "small" means and what a direct
  change still records). It is built directly, and its row is removed when it is built.


| # | Item | Why it is here | Source |
|---|---|---|---|
| 1 | The formatter writes `WINDOW SCREEN (0, 0)-(639.5, 479.5)` as `WINDOW SCREEN(0, 0)-(639.5, 479.5)`, as if `SCREEN` were the `SCREEN()` function. Keep the space after the keyword `SCREEN` of `WINDOW` (and of `VIEW SCREEN`, to check). | Found formatting Civil War Strategy's `ui\screen.bm` | 2026-11-28 |
