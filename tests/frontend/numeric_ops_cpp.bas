' TEST: cpp
$CONSOLE:ONLY
' The C++ of stores and operations per family of the new types (m2-numeric-types design D5, task 5.2): a float into a
' target of 16 bits or fewer is rounded from SINGLE (`qbr_float_to_long`), into a wider one or a `_BIT` by `qbr`; a
' `_BIT` store is masked (unsigned) or sign-extended (signed); unsigned 64-bit literals end in `ull`; an `_OFFSET`
' `*` with a float is rounded by `qbr`; `PRINT` casts to the believed type (a `_BIT` to `int64`). Same output as
' qb64pe.exe (the `_BIT * 40` is declared first: in the old compiler it overwrites the `_BIT` declared just before it,
' DIVERGENCES.md D-009).
DIM w AS _UNSIGNED _BIT * 40, ub AS _UNSIGNED _BYTE, ui AS _UNSIGNED INTEGER, uq AS _UNSIGNED _INTEGER64, o AS _OFFSET
DIM u7 AS _UNSIGNED _BIT * 7, b3 AS _BIT * 3
ub = 2.5000001#: ui = -1.5: uq = 1.8D+19
u7 = 300: b3 = 5: w = -1
o = 3: o = o * 1.5
PRINT ub; ui; uq; u7; b3; w; o
PRINT uq + 1; ub * ub; w + 18446744073709551615~&&
SYSTEM
