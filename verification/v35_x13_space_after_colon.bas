$CONSOLE:ONLY
$UNSTABLE: TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (13_space_after_colon)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
PRINT UBOUND(x.v)
SYSTEM
