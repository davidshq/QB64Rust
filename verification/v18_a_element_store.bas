$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): the store rule of a numeric array element. Under RESUME NEXT, is a
' store into an element made when its index raised, when its value raised, when both raised? Which ERR is
' reported, and what does a read with a bad index give?
DIM x(10) AS LONG
ON ERROR GOTO h
x(9) = 5: x(9) = ASC("")
PRINT "x(9) = ASC(empty):"; x(9)
x(0) = 77: x(10) = 88
x(11) = 5
PRINT "after x(11) = 5:"; x(0); x(10)
x(-1) = 6
PRINT "after x(-1) = 6:"; x(0); x(10)
x(11) = ASC("")
PRINT "after x(11) = ASC(empty):"; x(0); x(10)
x(1) = 3: x(1) = x(11)
PRINT "x(1) = x(11):"; x(1)
y = 5: y = x(11)
PRINT "y = x(11):"; y
y = 5: y = x(-1)
PRINT "y = x(-1):"; y
x(2) = 4: x(2) = x(11) + 1
PRINT "x(2) = x(11) + 1:"; x(2)
x(3) = 4: x(x(11) + 3) = 9
PRINT "x(x(11) + 3) = 9:"; x(0); x(3)
PRINT "print"; x(11); "rest"
PRINT "after print"
z& = 1
x(z& + 10) = 5
PRINT "x(z& + 10) = 5:"; x(0); x(10)
SYSTEM

h:
PRINT "handler"; ERR
RESUME NEXT
