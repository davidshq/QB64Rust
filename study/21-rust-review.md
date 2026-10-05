# 21 — Rust review of the workspace setup: lints, checks and CI

Written 2026-10-04 (session 10). A review of the compiler workspace's tooling (lints, profiles, toolchain, CI,
project-specific checks) from a Rust engineer's point of view, asked for by the user. The user accepted the
recommendations in §3 ("do as suggested"); the order of work there was followed and is done except where §4 says
otherwise.

Read for the review: `Cargo.toml`, the crate manifests, `rust-toolchain.toml`, `rustfmt.toml`, `crates\README.md`,
`.github\workflows\`, and the code that the stricter lints below pointed at.

## 1. Verdict

The setup was sound: formatted, clippy-clean at the default level, `unsafe_code = "forbid"`, lints set once in
`[workspace.lints]`, toolchain pinned (1.88), `Cargo.lock` committed, 35 crates in the lock file. Nothing needed
replacing. The changes make the code stricter where a compiler goes wrong silently, before step 4 of "Next" (the
full numeric type set) multiplies the places that match on types.

## 2. Measured

Clippy with a dozen extra lints over the whole workspace (before the changes):

| Lint | Library code | Notes |
|---|---|---|
| `must_use_candidate` (pedantic) | 282 | Noise at this size; the main reason not to switch on `pedantic` as a whole |
| `as_conversions` | 149 | Mostly harmless widening; not adopted |
| `indexing_slicing` | 112 | Normal in a parser over bytes; not adopted |
| `unwrap_used` | 47 (+34 in tests) | Mostly `writeln!` into a `String`; not adopted |
| `cast_possible_truncation`, `cast_possible_wrap` | 48 | Adopted (§3, item 5) |
| `wildcard_enum_match_arm` | 28 | Adopted for `sema`, `ir`, `codegen-cpp` (item 3) |
| `disallowed_methods` (as configured in item 4) | 4 in `sema`, 6 elsewhere | Adopted |

Two findings in the code itself:

- **Constant folding differed between debug and release builds.** It computed `a + b`, `a - b`, `a * b` on `i128`
  with plain operators: an overflow panics in a debug build (tests) and wraps silently in a release build (the
  shipped compiler). Unreachable while operands are at most `_INTEGER64`, but two `_UNSIGNED _INTEGER64` operands
  overflow `i128`.
- **`_ =>` arms on `Ty`.** `wrap()` in the folder ended with `_ => v as i64`; the C++ emitter allocated and cleared
  variables with `t =>` / `_ =>` for every non-string type. A new type (`_BYTE`, the unsigned types) would have
  been accepted by all of them without a warning.

The repo check (item 9) found one breach of rule 2 already committed: `verification\m1-extension.md` quoted a hover
text containing a full local path (fixed; the old text remains in git history).

## 3. Recommendations and what was done

| # | Recommendation | Done |
|---|---|---|
| 1 | Rust CI on Windows: `cargo fmt --check`, clippy with `-D warnings`, `cargo test`, all `--locked` | `.github\workflows\rust.yml` (tier 1 of `study\19`; needs no QB64pe). `core.autocrlf` off before checkout so tests see the committed bytes |
| 2 | Overflow checks in release; explicit wrapping in the folder | `[profile.release] overflow-checks = true`; the folder uses `wrapping_add/sub/mul/neg` on `i64` (wrapping in 64 bits, then to the target type, keeps the same low bits). New case in `tests\frontend\integer_width.bas` |
| 3 | No `_ =>` on the semantic enums | `#![warn(clippy::wildcard_enum_match_arm)]` at the roots of `sema`, `ir`, `codegen-cpp`. Matches on `Ty`, `ExprKind`, `Storage` list their variants. By-reference arguments list every expression kind, so array elements (by reference in QB64) must be decided there. A match on token kinds (hundreds of variants) keeps `_` with `#[expect(..., reason)]` |
| 4 | Enforce "source is bytes" (`study\15` §1) | `clippy.toml`: `disallowed-methods` for `fs::read_to_string`, `str::from_utf8`, `String::from_utf8`, `String::from_utf8_lossy`. Names and words in `sema` go through `show_bytes`; text that is not BASIC source (JSON, toolchain output, test lists) keeps its conversion with `#[expect]` and a reason |
| 5 | Cast lints | `cast_possible_truncation`, `cast_possible_wrap`, `cast_sign_loss` for the workspace. `base::MAX_SOURCE_LEN` (offsets are `u32`) and `base::to_u32` for every length, offset and id bounded by it; the driver rejects a larger file before any stage runs. Deliberate wraps keep `as` with `#[expect]` |
| 6 | Panic hook in the driver ("internal compiler error", distinct exit code) | Done 2026-10-04 (`m2-parser-breadth` task 1.2): message with location and input file, executable removed, exit code 3. The language server will need `catch_unwind` per request |
| 7 | `cargo-deny` for licences and advisories | Later: when a dependency beyond `insta` and `serde_json` is added. Fits rule 5 (no GPL code by way of a crate) |
| 8 | `rust-version` in the manifest | `rust-version = "1.88"`, as `rust-toolchain.toml` |
| 9 | Project rules as a CI check | `tools\repo_check\check_repo.py`, run by `.github\workflows\repo-check.yml` on every push: full local paths, temp folders and network shares (rule 2), and control bytes other than tab, LF and the CR of CR LF (rule 7). Recorded output is exempt from the byte check |

Checked after the changes: clippy clean with `-D warnings`; `cargo test` 72 pass, 1 ignored, also in a fresh
clone; tier 2 (the 31 programs of `tests\corpus\slice.list`) passes with the release build, with folding on and
off.

## 4. Not recommended, and later

- **`clippy::pedantic` as a whole:** about 280 of its findings are `must_use_candidate`. Pick single lints instead.
- **`unwrap_used`, `indexing_slicing`, `as_conversions` everywhere:** noise in a byte parser; `unwrap` in tests is
  fine.
- **dylint or other nightly-only custom lints:** `clippy.toml` plus `#[expect]` covers what is needed.
- **Miri:** nothing to find with `unsafe` forbidden.
- **Later** (`SOMEDAY.md`): fuzzing the lexer and parser ("no panic, exact round trip" is an ideal fuzz
  property), a policy for bumping the pinned toolchain (1.88 is about 15 months old), `cargo-deny` (item 7).

## 5. How to apply

- A match on `Ty`, an operator or an expression kind lists its variants. When a variant really should take a
  default, say why in `#[expect(clippy::wildcard_enum_match_arm, reason = "...")]`.
- The lint sees only `match`. A default written as `t == Ty::Str`, `if … else`, `matches!` or `.max()` on the
  derived order escapes it, so a type predicate or a per-type choice is written as a `match` too
  (`Ty::is_numeric`, `promote`, codegen's `is_qbs`; found in the review of these changes). Promotion by `.max()`
  over the declaration order stays until step 4's type table gives each type an explicit rank.
- Bytes from a source file stay `&[u8]`/`Vec<u8>`; shown in a diagnostic through `show_bytes`.
- A length, offset or id goes through `to_u32`; an `as` that can lose bits gets `#[expect]` with the semantic
  reason (usually D-001/D-002).
- Arithmetic that BASIC defines as wrapping is written `wrapping_*`; any other overflow panics, in release too.
