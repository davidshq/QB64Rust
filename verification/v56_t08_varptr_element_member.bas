$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): VARPTR and VARSEG of members of elements of an array of a TYPE, also through member arrays and an index outside its bounds.
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
PRINT VARPTR(pad); VARPTR(ar(1)); VARPTR(ar(2).m(1)); VARPTR(ar(4).v); VARSEG(ar(4).v) - VARSEG(ar(1))
PRINT VARPTR(ao(1).k(1).m(2)); VARPTR(ao(2).n.v)
ar(2).m(1) = 258
DEF SEG = VARSEG(ar(1)): PRINT PEEK(VARPTR(ar(2).m(1))); PEEK(VARPTR(ar(2).m(1)) + 1): DEF SEG
ON ERROR GOTO h
PRINT VARPTR(ar(5).v)
PRINT VARPTR(ar(1).m(4))
SYSTEM
h:
PRINT "error"; ERR
RESUME NEXT
SYSTEM
