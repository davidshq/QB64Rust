$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (35_redim_unmarked_no_to)
TYPE t
    v(1) AS LONG
END TYPE
DIM x AS t
x.v(1) = 1
REDIM x.v(1)
PRINT UBOUND(x.v); x.v(1)
SYSTEM
