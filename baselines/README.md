# Baselines

Results of the legacy test suites run against a known compiler. Every later run (old compiler on
another machine, or the new compiler) is compared with these.

## `qb64pe-16f629784e-win64.json` — old compiler, 2026-10-02

| Item | Value |
|---|---|
| Compiler | QB64pe `main` at `16f629784e` (version 4.7.0-GLFW), built locally with `setup_win.cmd` |
| C++ toolchain | llvm-mingw 20260922 (clang 23.1.2), UCRT, x86_64 |
| Machine | Windows 11 Pro 10.0.26200, 64-bit |
| Runner | `tools\legacy_tests\run_legacy_tests.py --suite all` |
| Duration | about 37 minutes (2,211 s), tests run one at a time |

| Suite | Kind | Pass | Fail |
|---|---|---|---|
| compile_tests | expected output | 330 | 1 |
| compile_tests | expected compile error | 56 | 0 |
| compile_tests | compile only | 17 | 0 |
| qbasic_testcases | compile only | 143 | 0 |
| **Total** | | **546** | **1** |

### Known failure

| Test | Cause | Treatment |
|---|---|---|
| `http/read_example` | Downloads `https://www.example.com` and compares it with a saved copy. The site has changed its page (577 bytes now, 713 expected). Not a compiler or runtime problem. | Environment-dependent: exclude from pass/fail comparisons, or replace with a local HTTP server |

### Setup problems hit while building the old compiler (not fixed in the read-only clone)

1. `setup_mingw.cmd` downloads the **latest** llvm-mingw with no version pin or checksum. Here that was 20260922.
2. Its `move` of the extracted toolchain failed with `Access is denied` (probably a file still held by an
   on-access scanner). The script did not notice, deleted the remainder, and the build later failed at link time
   with `unable to find library -lpthread`, `-lopengl32`, etc. The `x86_64-w64-mingw32` folder was missing.
   Fix used: extract the same zip elsewhere and copy it into `internal\c\c_compiler` with `robocopy /E /R:10`.
3. Calling `setup_win.cmd` non-interactively waits 60 seconds at a `choice` prompt for 64/32-bit. Calling
   `setup_mingw.cmd 64` first avoids it.
4. The clone's `.output` files have LF line endings while programs on Windows write CRLF. The runner treats CRLF
   as LF; the bash runner relies on git converting expected files on checkout.

### What this baseline does not cover

Graphics output beyond 12 image tests, audio, interactive input, the IDE, and the 143 qbasic programs' run-time
behaviour (they are only compiled). See `study\07-expert-panel.md` Session 5 for the planned additional layers.
