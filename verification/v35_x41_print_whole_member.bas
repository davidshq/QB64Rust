$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (41_print_whole_member)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
PRINT x.v
SYSTEM
