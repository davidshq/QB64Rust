# 08 — What a strict QuickBASIC 4.5 mode would need

**Status:** not planned. QB64pe offers no such mode (no option for it exists in `source\`), and the rewrite targets
observed QB64pe behaviour (`study\07-expert-panel.md` R2). This document records the differences so the option stays
open.

**Sources:** the study documents `01`–`03`. Items marked *verify* come from reading code or from memory of QB4.5 and
have not been run on either system.

## How a mode would fit the new design

A per-program switch (command-line flag or metacommand) that changes:
- **IR lowering**: arithmetic with overflow checks, different literal typing, trappable errors.
- **Runtime calls**: checked variants of a few runtime functions.
- **Front-end checks**: reject QB64 extensions and accept QB4.5-only syntax.

The architecture recommended by the panel (typed IR with explicit conversions and explicit error points) is what makes
this cheap: most items below are lowering rules, not parser changes.

## Numeric differences

| QB4.5 behaviour | QB64pe today | Change needed | Ref |
|---|---|---|---|
| Integer arithmetic overflow raises error 6 | No check; ≤32-bit operands computed in C `int`, wraps (signed overflow is undefined behaviour in C++) | Checked add/sub/mul per integer width | `02` §1.4 |
| Storing an out-of-range value raises error 6 (`x% = 70000`) | Rounds, then truncates silently | Range check on every narrowing store | `02` §1.5 |
| Integer division or MOD by zero: trappable error 11 | Fatal: `qb_safe_idiv`/`qb_safe_mod` raise critical error 11; not trappable (`09`) | Check divisor before dividing; make error 11 trappable | `03` §2.10 |
| Float division by zero: error 11 (*verify*) | IEEE infinity | Check divisor | `03` §2.10 |
| SINGLE expression results stay SINGLE (e.g. `1 / 3` is SINGLE) (*verify*) | `int / int` typed `_FLOAT` (long double); SINGLE literals emitted as C doubles | QB4.5 typing rules in lowering; emit `float` constants | `02` §1.4 |
| `^` result type follows operands | Always computed via `long double` pow | Typed power | `02` §1.4 |
| `_INTEGER64`, `_FLOAT`, `_UNSIGNED`, `_BIT`, `_BYTE`, `_OFFSET` do not exist | Supported | Reject in strict mode | `01` §7 |

## Missing QB4.5 features

| Feature | QB64pe today | Ref |
|---|---|---|
| `DEF FN` functions | Not implemented (`Def` registered as a stub) | `01` §8.2 |
| `ON COM`, `ON PEN`, `ON PLAY`, `ON UEVENT` | Reserved words only | `01` §8.2, `03` §2.11 |
| `TRON` / `TROFF` | "Command not implemented" | `02` §5.2 |
| `FRE`, `SETMEM`, `IOCTL`, `IOCTL$`, `FILEATTR` | "Command not implemented" | `02` §5.2 |
| `PEN`, `KYBD:`, `CONS:` devices; real `LPTn:` device files | Not found in the runtime | `03` §6.2 |
| `COMn:` serial ports | Windows only | `03` §6.2 |
| `CALL ABSOLUTE` with arbitrary machine code | Tiny x86 interpreter; only the classic mouse stub works | `03` §2.12 |
| `CALL INTERRUPT` beyond INT 33h | Only INT 33h mouse | `03` §2.12 |
| `OUT`/`INP` beyond VGA DAC, `&H3DA`, `&H60` | Ignored | `03` §2.12 |
| `CHAIN` on non-Windows | Windows only | `03` §6.3 |
| `ON n GOTO/GOSUB` range check (error 5 for n > 255) | Only negative values checked | `02` §9.2 |

## Language-level differences

| Item | Note |
|---|---|
| Names that are QB64 keywords | Most QB64 keywords start with `_`, which QB4.5 identifiers can't anyway. Check the few un-prefixed additions against QB4.5 programs (*verify*) |
| Single-underscore names | QB64pe forbids user names starting with one `_`; QB4.5 allowed none either, so no change |
| Error message texts and numbers | QB4.5 texts would be needed for a faithful mode |
| Fixed-length strings initialised to NUL bytes | QB64pe; QB4.5 behaviour to *verify* |
| `CONST` expressions | QB64pe's evaluator differs (right-associative `^`, integer-forcing `ABS`, no range check) | 
| Memory model: `FRE`, `VARPTR`/`VARSEG` values, `DEF SEG` defaults | QB64pe emulates DGROUP at segment `&H50`; values differ from a real QB4.5 session |
| Timing: `SOUND` / `PLAY` durations, `TIMER` resolution | QB64pe quantises `TIMER` to 1/18.2 s like QB4.5; audio timing via miniaudio |

## Effort estimate (rough)

- Numeric checks and literal typing: moderate; lowering rules plus a differential test suite against real QB4.5
  (DOSBox) output.
- `DEF FN`, `ON PLAY`, `TRON`, `FRE`: small to moderate each.
- Hardware and DOS-device emulation (`CALL ABSOLUTE`, `INTERRUPT`, `OUT`, `COM`, `LPT`, `PEN`): large and open-ended;
  a strict mode would probably still exclude them.
- An oracle is needed: QuickBASIC 4.5 under DOSBox, with a harness to capture program output.
