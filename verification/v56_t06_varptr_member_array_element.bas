$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): VARPTR of an element of a member array of a TYPE variable.
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
PRINT VARPTR(pad); VARPTR(r.m(0)); VARPTR(r.m(2))
ON ERROR GOTO h
PRINT VARPTR(r.m(4))
SYSTEM
h:
PRINT "error"; ERR
RESUME NEXT
SYSTEM
