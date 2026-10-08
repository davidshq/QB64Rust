$CONSOLE:ONLY
' Verification (m2-core-builtins review): LEN of a whole TYPE element whose index holds another LEN.
TYPE t
    a AS LONG
    b AS DOUBLE
END TYPE
DIM v(3) AS t, w AS t, i AS INTEGER
PRINT "LEN nested:"; LEN(v(LEN(w))); LEN(v(LEN(i)).b); LEN(v(LEN(i)))
SYSTEM
