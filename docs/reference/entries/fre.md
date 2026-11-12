# FRE

`FRE` is not available. QB64pe lists it as a built-in function but rejects every use of it with "Command not
implemented", and QB64Rust does the same.

## Syntax

```text
FRE(number)
FRE(string$)
```

The forms a QuickBASIC program uses. Neither compiles.

## Behavior

Every call of `FRE` is a compile error, whatever its argument: `FRE(0)`, `FRE(-1)`, `FRE(-2)` and `FRE("")`. The
error is given also where the line can never run.
<!-- src: language/builtin-functions "The old compiler's stubs"; verification\v57_s09_fre_minus2; verification\v57_s10_fre_string; tests\corpus\runtime_comparison\218_fre -->

## Errors

- A call of `FRE`: "`FRE` is not implemented in QB64pe", with QB64pe's "Command not implemented" as a note.
  <!-- src: language/builtin-functions "The old compiler's stubs"; verification\v57_s09_fre_minus2 -->
- `CALL FRE(0)`: "`CALL FRE`: no SUB of this name", as `CALL` of any function is.
  <!-- src: language/builtin-functions "The old compiler's stubs"; verification\v57_s19_fre_call -->

## Differences

A program ported from QuickBASIC 4.5 or BASIC PDS 7.1 that calls `FRE` must be changed. The porting guide
suggests commenting the call out ([PORTING-PDS.md](../../PORTING-PDS.md), section 2, "Replace what PDS has and
QB64 does not").

## Availability

<!-- begin generated: availability -->
Rejected, as in QB64pe. The compiler gives:

- `FRE` is not implemented in QB64pe
<!-- end generated -->

<!-- probe: PRINT FRE(-1) -->
<!-- probe: x = FRE("") -->

## See also

`SETMEM`, `_MEMNEW`.
