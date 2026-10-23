$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): VARPTR of members of a member of a TYPE variable.
TYPE t
    a AS LONG
    m(3) AS INTEGER
    v AS _BYTE
END TYPE
TYPE o
    z AS INTEGER
    n AS t
    k(1) AS t
END TYPE
DIM pad AS INTEGER, r AS t, ro AS o, ar(1 TO 4) AS t, ao(2) AS o
pad = 1
PRINT VARPTR(pad); VARPTR(ro.n.a); VARPTR(ro.n.v); VARPTR(ro.n.m(1)); VARPTR(ro.k(1).v); VARPTR(ro.k(0).m(3))
SYSTEM
