$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (28_swap_whole_values)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t, y AS t
REDIM x.v(3 TO 4) AS LONG
x.v(4) = 4
SWAP x, y
PRINT LBOUND(x.v); UBOUND(x.v); LBOUND(y.v); UBOUND(y.v); y.v(4)
SYSTEM
