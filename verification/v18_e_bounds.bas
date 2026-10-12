$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): bounds forms, LBOUND and UBOUND (forms, errors, type).
CONST lo = -3, hi = 4
DIM b(-2 TO 3, 5) AS LONG
DIM z(0) AS LONG
DIM w(lo TO hi) AS DOUBLE
DIM one(7 TO 7) AS INTEGER
b(-2, 0) = 1: b(3, 5) = 2
PRINT "b(-2, 0); b(3, 5); b(0, 0):"; b(-2, 0); b(3, 5); b(0, 0)
PRINT "LBOUND(b); UBOUND(b):"; LBOUND(b); UBOUND(b)
PRINT "LBOUND(b, 2); UBOUND(b, 2):"; LBOUND(b, 2); UBOUND(b, 2)
PRINT "LBOUND(b, 1); UBOUND(b, 1):"; LBOUND(b, 1); UBOUND(b, 1)
PRINT "z:"; LBOUND(z); UBOUND(z)
PRINT "w:"; LBOUND(w); UBOUND(w)
PRINT "one:"; LBOUND(one); UBOUND(one); one(7)
PRINT "UBOUND(b) / 7:"; UBOUND(b) / 7
PRINT "UBOUND(b) * 1000000000:"; UBOUND(b) * 1000000000
PRINT "LBOUND(b, 1.5); LBOUND(b, 2.5):"; LBOUND(b, 1.5); LBOUND(b, 2.5)
d = 2: PRINT "UBOUND(b, d):"; UBOUND(b, d)
ON ERROR GOTO h
PRINT "LBOUND(b, 3):"; LBOUND(b, 3)
PRINT "LBOUND(b, 0):"; LBOUND(b, 0)
PRINT "UBOUND(b, -1):"; UBOUND(b, -1)
PRINT "UBOUND(z, 2):"; UBOUND(z, 2)
SYSTEM

h:
PRINT "handler"; ERR
RESUME NEXT
