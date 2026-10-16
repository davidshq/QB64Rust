$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (19_redim_wrong_as)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
REDIM x.v(0 TO 5) AS INTEGER
PRINT UBOUND(x.v)
SYSTEM
