$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): constant expressions as bounds, and a bound beyond 32 bits.
DIM a(2 * 3 + 1) AS LONG
DIM b(-(2 ^ 2) TO 10 \ 3) AS LONG
PRINT LBOUND(a); UBOUND(a); LBOUND(b); UBOUND(b)
SYSTEM
