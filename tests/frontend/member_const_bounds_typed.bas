' TEST: typed
$CONSOLE:ONLY
' A static member array's bounds that name a CONST above the block, a constant of the auto-included file or a
' function of the constant evaluator are read in pass 2 at the block and lay the type out (m4-memory-records
' design D7; verification\v24_x196, v51_x09-x11, x14): LEN of the type, of a type that holds it and of a whole
' member array, and UBOUND of each member array, are those of the bounds read
CONST top = 3
TYPE t1
    v(top) AS LONG
    w(top * 2 - 1) AS LONG
    p(_PI) AS INTEGER
    a(1 TO ABS(top - 5)) AS _BYTE
END TYPE
TYPE t2
    id AS INTEGER
    inner AS t1
END TYPE
DIM x AS t1, y AS t2
PRINT UBOUND(x.v); UBOUND(x.w); UBOUND(x.p); UBOUND(x.a); LEN(x); LEN(y); LEN(y.inner.w())
REDIM x.w(0 TO 5)
SYSTEM
