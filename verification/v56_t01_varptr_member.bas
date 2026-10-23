$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): VARPTR and VARSEG of a member of a TYPE variable, and its bytes through PEEK.
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
PRINT VARPTR(pad); VARPTR(r.a); VARPTR(r.v); VARSEG(r.v)
r.v = 7: DEF SEG = VARSEG(r.v): PRINT PEEK(VARPTR(r.v)): DEF SEG
SYSTEM
