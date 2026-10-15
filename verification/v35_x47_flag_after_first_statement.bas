$CONSOLE:ONLY
PRINT 0
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (47_flag_after_first_statement)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
PRINT UBOUND(x.v)
SYSTEM
