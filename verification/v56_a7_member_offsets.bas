$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): VARPTR and VARSEG of members reached through member arrays,
' nested types and array elements, with an index outside its bounds.
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
DIM r AS t, ro AS o, ar(1 TO 4) AS t, ao(2) AS o
PRINT "r"; VARPTR(r); VARPTR(r.m(0)); VARPTR(r.m(2)); VARPTR(r.v); VARSEG(r.m(2))
PRINT "ro"; VARPTR(ro); VARPTR(ro.n.a); VARPTR(ro.n.m(1)); VARPTR(ro.k(1).v); VARPTR(ro.k(0).m(3))
PRINT "ar"; VARPTR(ar(1)); VARPTR(ar(2).m(1)); VARPTR(ar(4).v); VARSEG(ar(4).v) - VARSEG(ar(1))
PRINT "ao"; VARPTR(ao(1).k(1).m(2)); VARPTR(ao(2).n.v)
ON ERROR GOTO h
PRINT "out"; VARPTR(r.m(4))
PRINT "out"; VARPTR(ar(5).v)
PRINT "out"; VARPTR(ro.k(2).a)
r.m(2) = 513: ar(2).m(1) = 258
DEF SEG = VARSEG(r): PRINT PEEK(VARPTR(r.m(2))); PEEK(VARPTR(r.m(2)) + 1)
DEF SEG = VARSEG(ar(1)): PRINT PEEK(VARPTR(ar(2).m(1))); PEEK(VARPTR(ar(2).m(1)) + 1)
DEF SEG
SYSTEM
h:
PRINT "error"; ERR
RESUME NEXT
