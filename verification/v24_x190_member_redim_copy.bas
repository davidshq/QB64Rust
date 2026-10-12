$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): after REDIM of an unmarked member array with other bounds: a
' copy q = p, a second variable, LEN.
ON ERROR GOTO h
TYPE t1
    v(0 TO 3) AS LONG
    tail AS LONG
END TYPE
DIM p AS t1, q AS t1, r AS t1
p.tail = 8: p.v(3) = 4
PRINT "REDIM p.v(0 TO 9):";
REDIM p.v(0 TO 9)
p.v(9) = 99
PRINT UBOUND(p.v); UBOUND(q.v); LEN(p); LEN(q)
PRINT "q = p:";
q = p
PRINT UBOUND(q.v);
PRINT q.v(9); q.tail; UBOUND(r.v)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
