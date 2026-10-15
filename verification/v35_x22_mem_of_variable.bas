$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (22_mem_of_variable)
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
DIM m AS _MEM
m = _MEM(x)
PRINT m.SIZE
SYSTEM
