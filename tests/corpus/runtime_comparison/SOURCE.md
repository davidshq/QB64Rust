# Source of these programs

The 261 `.bas` files here are byte-for-byte copies of `tests/runtime_comparison/*.bas` from QB64Fresh (the user's
earlier Rust rewrite, MIT licence), at commit `85e6df8` (2026-02-03), the last commit that changed that folder.
Copied 2026-10-03 under `CLAUDE.md` rule 5: every program was read in full. The numbers run from 01 to 264 with
gaps (45, 55, 249 do not exist).

Not copied: QB64Fresh's `README.md`, `run_comparison.sh`, `diff_results.sh`, `DIFFERENCES.md` and its recorded
results. They were measured on Linux against QB64Fresh and were never checked against `qb64pe.exe`. The expected
results here (`.output`, `.err`) were recorded fresh with the old compiler (`tests\corpus\README.md`).

Some programs are based on QB64pe's own tests (QB64pe is MIT-licensed, `licenses/license_qb64.txt` in QB64pe): 240
(`compile_tests/console_only/data.bas`), 241, 242, 243, 246 and 247 (subsets of QB64pe `compile_tests`), and 250
(inspired by `qbasic_testcases/misc/rot13.bas`).

The programs' comments were written for comparing QB64Fresh with QB64pe ("if supported", "may differ"). They
are kept unchanged; the comments are not part of what is tested.

Sidecars added here (not from QB64Fresh): `<name>.norun` (compile only, never run) and `<name>.normalize`
(output rules); see `tests\corpus\README.md`.
