' TEST: cpp
$CONSOLE:ONLY
' The emitted declarations of the new numeric types (m2-numeric-types task 4.1): the C type of each, a `_BIT * n`
' in its 32- or 64-bit storage, the old compiler's variable names (`qb64pe.bas` 25797-25819), and a FUNCTION of a
' new type with parameters of new types, never called.
DIM a AS _BYTE, b AS _UNSIGNED _BYTE, c AS _UNSIGNED INTEGER, d AS _UNSIGNED LONG
DIM e AS _UNSIGNED _INTEGER64, f AS _OFFSET, g AS _UNSIGNED _OFFSET
DIM h AS _BIT, j AS _BIT * 7, k AS _UNSIGNED _BIT * 40
PRINT LEN(b); LEN(g)
SYSTEM

FUNCTION never~%% (p1 AS _UNSIGNED LONG, p2%&)
END FUNCTION
