# MKSMBF$

`MKSMBF$` and `MKDMBF$` give the bytes of a number in Microsoft Binary Format: 4 bytes for a single and 8 for a
double. Old BASIC programs wrote their files in this format. `CVSMBF` and `CVDMBF` read it back.

## Syntax

<!-- begin generated: syntax -->
```text
MKDMBF$(value#)
MKSMBF$(value!)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- The number. Any numeric type is taken.

## Result

A STRING of 4 bytes (`MKSMBF$`) or 8 bytes (`MKDMBF$`).

## Behavior

The string holds the number in Microsoft Binary Format, as libqb writes it, not in the IEEE format of `MKS$` and
`MKD$`: `MKSMBF$(1.5)` is the bytes 0, 0, 64 and 129.
<!-- src: language/builtin-functions "Microsoft Binary Format conversions"; verification\v58_f10_mbf -->

`CVSMBF(MKSMBF$(x))` and `CVDMBF(MKDMBF$(x))` give the number back.
<!-- src: language/builtin-functions "Microsoft Binary Format conversions"; tests\reference\examples\mksmbf-str_1 -->

## Errors

- A string as the argument is a compile error.
  <!-- src: verification\v58_x08_mksmbf_string -->
- `MKSMBF` without its `$` is a compile error.
  <!-- src: verification\v58_x09_mksmbf_no_suffix -->
- `MKSMBF$` in a `CONST` is a compile error.
  <!-- src: verification\v58_x109_mksmbf_const -->

## Availability

<!-- begin generated: availability -->
Supported. All 5 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:mksmbf-str_1 -->
```qb64
s$ = MKSMBF$(1.5)
PRINT LEN(s$); CVSMBF(s$)
d$ = MKDMBF$(-2.25#)
PRINT LEN(d$); CVDMBF(d$)
END
```

Output:

```text
 4  1.5 
 8 -2.25 

Press any key to continue
```
<!-- end generated -->

## See also

`CVSMBF`, `MKS$`, `MKD$`.
