$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): float indexes: which element does each one name?
DIM x(10) AS LONG
FOR i = 0 TO 10: x(i) = i * 10: NEXT
PRINT "x(1.5):"; x(1.5)
PRINT "x(2.5):"; x(2.5)
PRINT "x(0.5):"; x(0.5)
PRINT "x(1.4999):"; x(1.4999)
PRINT "x(3.5000001):"; x(3.5000001)
PRINT "x(-0.4):"; x(-0.4)
s! = 4.5: PRINT "x(s! = 4.5):"; x(s!)
d# = 5.5: PRINT "x(d# = 5.5):"; x(d#)
f## = 6.5: PRINT "x(f## = 6.5):"; x(f##)
x(7.5) = 1
PRINT "after x(7.5) = 1:"; x(7); x(8)
ON ERROR GOTO h
PRINT "x(-0.5):"; x(-0.5)
PRINT "x(-0.6):"; x(-0.6)
PRINT "x(10.5):"; x(10.5)
PRINT "x(10.4):"; x(10.4)
SYSTEM

h:
PRINT "handler"; ERR
RESUME NEXT
