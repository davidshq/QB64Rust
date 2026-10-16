$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, task 6.2): does the old compiler accept _OFFSET of a variable of a type with a
' _DYNAMIC member (the spec "Dynamic member arrays" names it a compile error; not measured before)?
TYPE t
    a AS INTEGER
    v(1) _DYNAMIC AS LONG
END TYPE
DIM x AS t
PRINT _OFFSET(x) <> 0
PRINT _OFFSET(x.a) = _OFFSET(x)
SYSTEM
