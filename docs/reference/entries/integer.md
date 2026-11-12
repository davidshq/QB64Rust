# INTEGER

`INTEGER` is the type of a 16-bit whole number, from -32768 to 32767. Its suffix is `%`.

## Syntax

```text
DIM name AS INTEGER
DIM name AS _UNSIGNED INTEGER
name%
name~%
```

## Behavior

An INTEGER takes 2 bytes. `_UNSIGNED INTEGER`, suffix `~%`, holds 0 to 65535.
<!-- src: language/numeric-semantics "Numeric types"; verification\v24_a_define; verification\v21_a_decls -->

A floating-point value stored into an INTEGER is first narrowed to SINGLE, then rounded half to even: 2.5 is
stored as 2 and 3.5 as 4.
<!-- src: language/numeric-semantics "Storing into an integer variable"; verification\v01_numeric -->

A value outside the range is cut to its low 16 bits, with no error: 70000 is stored as 4464.
<!-- src: language/numeric-semantics "Storing into an integer variable"; tests\reference\examples\integer_1 -->

Arithmetic on INTEGERs is done in 32 bits, so a result does not wrap until it is stored: with `i% = 32767`,
`i% + 1` is 32768.
<!-- src: language/numeric-semantics "Integer arithmetic width"; tests\reference\examples\integer_1 -->

A whole-number literal without a suffix is an INTEGER when it fits: `-3` is one, `40000` is a LONG.
<!-- src: language/numeric-semantics "Integer literal typing" -->

## Differences

From QuickBASIC 4.5: it raises error 6, "Overflow", when INTEGER arithmetic leaves the INTEGER range inside an
expression. QB64Rust computes in 32 bits, as QB64pe does (`DIVERGENCES-QB45.md` Q-004).
<!-- src: Q-004 -->

From QB64pe: built with `-O2`, its 32-bit arithmetic need not wrap where it leaves the LONG range. QB64Rust always
wraps, with or without `-O2` (`DIVERGENCES-QB64PE.md` D-001).
<!-- src: D-001 -->

## Availability

<!-- begin generated: availability -->
Supported. All 13 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:integer_1 -->
```qb64
DIM score AS INTEGER
score = 32767
PRINT "score ="; score; "takes"; LEN(score); "bytes"
score = 2.5: PRINT "2.5 stored:"; score
score = 3.5: PRINT "3.5 stored:"; score
n% = 70000: PRINT "70000 stored:"; n%
score = 32767: PRINT "score + 1 in an expression:"; score + 1
END
```

Output:

```text
score = 32767 takes 2 bytes
2.5 stored: 2 
3.5 stored: 4 
70000 stored: 4464 
score + 1 in an expression: 32768 

Press any key to continue
```
<!-- end generated -->

## See also

`LONG`, `_BYTE`, `_UNSIGNED`, `CINT`, `DEFINT`.
