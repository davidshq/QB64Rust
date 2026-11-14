# CVI

`CVI`, `CVL`, `CVS` and `CVD` read a number from the bytes of a string: 2 bytes as an INTEGER, 4 as a LONG, 4 as a
SINGLE and 8 as a DOUBLE. They read back what `MKI$`, `MKL$`, `MKS$` and `MKD$` wrote.

## Syntax

<!-- begin generated: syntax -->
```text
CVD(stringData$)
CVI(stringData$)
CVL(stringData$)
CVS(stringData$)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- A string of at least as many bytes as the type has.

## Result

An INTEGER, a LONG, a SINGLE or a DOUBLE.

## Behavior

The first 2, 4 or 8 bytes are read as the number, the lowest byte first: `CVI("AB")` is 16961, which is
65 + 66 * 256. Bytes past those are ignored.
<!-- src: verification\v32_c_mk_cv; tests\reference\examples\cvi_1 -->

A fixed-length string may be read: with `f = MKL$(-7)` for a `STRING * 4`, `CVL(f)` is -7.
<!-- src: verification\v32_c_mk_cv -->

## Errors

- A string shorter than the type raises run-time error 5: `CVI("")`, `CVI("A")`, `CVL("AB")`, `CVD("ABC")`.
  <!-- src: verification\v32_c_mk_cv -->
- A number as the argument is a compile error: "`CVI` needs a string".
  <!-- src: language/builtin-functions "Supported built-in functions"; verification\v32_x104_cvi_long -->

## Availability

<!-- begin generated: availability -->
Supported. All 9 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:cvi_1 -->
```qb64
s$ = MKI$(-7)
PRINT LEN(s$); CVI(s$)
PRINT CVL(MKL$(100000)); CVS(MKS$(0.5)); CVD(MKD$(1 / 3))
PRINT CVI("AB")
END
```

Output:

```text
 2 -7 
 100000  .5  .3333333333333333 
 16961 

Press any key to continue
```
<!-- end generated -->

## See also

[`MKI$`](mki-str.md), [`_CV`](_cv.md), [`CVSMBF`](cvsmbf.md), [`GET`](get.md).
