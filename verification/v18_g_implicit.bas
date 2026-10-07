$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): an array used without DIM.
ON ERROR GOTO h
z(3) = 4
PRINT "z(3):"; z(3); LBOUND(z); UBOUND(z)
z(11) = 1
PRINT "after z(11) = 1"
SYSTEM

h:
PRINT "handler"; ERR
RESUME NEXT
