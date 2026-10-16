$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (16_empty_parens)
TYPE t
    v() _DYNAMIC AS LONG
END TYPE
DIM x AS t
REDIM x.v(0 TO 2) AS LONG
PRINT UBOUND(x.v)
SYSTEM
