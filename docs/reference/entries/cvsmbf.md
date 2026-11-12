# CVSMBF

`CVSMBF` and `CVDMBF` read a number from bytes in Microsoft Binary Format, 4 bytes for a single and 8 for a double:
what `MKSMBF$` and `MKDMBF$` wrote, or an old BASIC program's file.

## Syntax

<!-- begin generated: syntax -->
```text
CVDMBF(stringData$)
CVSMBF(stringData$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A string of at least 4 bytes (`CVSMBF`) or 8 bytes (`CVDMBF`).

## Result

A SINGLE (`CVSMBF`) or a DOUBLE (`CVDMBF`).

## Behavior

The first 4 or 8 bytes are read as a number in Microsoft Binary Format, as libqb reads it: `CVSMBF(MKSMBF$(3.75))` is
3.75. Bytes past those are ignored.
<!-- src: language/builtin-functions "Microsoft Binary Format conversions"; verification\v58_f10_mbf; tests\reference\examples\cvsmbf_1 -->

## Errors

- A string too short, as `CVSMBF("ab")`, raises run-time error 5, and the result is 0.
  <!-- src: language/builtin-functions "Microsoft Binary Format conversions"; verification\v58_f10_mbf -->
- A number as the argument is a compile error: "`CVSMBF` needs a string".
  <!-- src: verification\v58_x10_cvsmbf_number -->
- `CVSMBF` with a type suffix is a compile error.
  <!-- src: verification\v58_x11_cvsmbf_suffix -->

## Availability

<!-- begin generated: availability -->
Supported. All 4 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:cvsmbf_1 -->
```qb64
PRINT CVSMBF(MKSMBF$(3.75))
PRINT CVDMBF(MKDMBF$(1.25#))
END
```

Output:

```text
 3.75 
 1.25 

Press any key to continue
```
<!-- end generated -->

## See also

`MKSMBF$`, `CVS`, `CVD`.
