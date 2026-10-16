$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (21_len_of_element)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
PRINT LEN(x.v(0))
SYSTEM
