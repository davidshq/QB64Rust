$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (57_copy_index_outside)
TYPE t
    n AS LONG
    v(1) _DYNAMIC AS LONG
END TYPE
REDIM a(2) AS t
DIM p AS t
ON ERROR GOTO h
REDIM p.v(4 TO 5) AS LONG: p.v(5) = 55: p.n = 6
a(9) = p
PRINT a(0).n; LBOUND(a(0).v); UBOUND(a(0).v)
p = a(9)
PRINT p.n
SYSTEM
h:
PRINT "[error"; ERR; "]";
RESUME NEXT
SYSTEM
