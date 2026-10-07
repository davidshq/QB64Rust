$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): several dimensions: each index checked against its own bounds.
DIM m(2, 3) AS LONG
FOR i = 0 TO 2: FOR j = 0 TO 3: m(i, j) = i * 10 + j: NEXT: NEXT
PRINT m(0, 0); m(1, 2); m(2, 3)
ON ERROR GOTO h
PRINT "m(3, 0):"; m(3, 0)
PRINT "m(0, 4):"; m(0, 4)
PRINT "m(1, 4) (flat index in range):"; m(1, 4)
DIM c(1, 1, 1) AS INTEGER
c(1, 0, 1) = 5
PRINT "c(1, 0, 1):"; c(1, 0, 1); c(0, 0, 0)
SYSTEM

h:
PRINT "handler"; ERR
RESUME NEXT
