$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (42_input_into_element)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
INPUT x.v(1)
PRINT x.v(1)
SYSTEM
