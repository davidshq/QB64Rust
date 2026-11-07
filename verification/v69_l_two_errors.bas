$CONSOLE:ONLY
' Verification (OpenSpec change eval-order-left-to-right, task 2.1): two parts of one expression that raise
' different errors. The first in the order of evaluation is the one the handler reports. Two index checks give
' error 9 in either order.
DIM a(5) AS LONG, b(5) AS LONG, t(5) AS STRING, e AS STRING, d AS DOUBLE, i AS LONG, j AS LONG
ON ERROR GOTO h
PRINT "ASC(e) + a(9):";: PRINT ASC(e) + a(9)
PRINT "a(9) + ASC(e):";: PRINT a(9) + ASC(e)
d = 40000
PRINT "CINT(d) + ASC(e):";: PRINT CINT(d) + ASC(e)
PRINT "ASC(e) - CINT(d):";: PRINT ASC(e) - CINT(d)
PRINT "CHR$(-1) + t(9):";: PRINT CHR$(-1) + t(9)
PRINT "t(9) + CHR$(-1):";: PRINT t(9) + CHR$(-1)
PRINT "SQR(-1) + a(9):";: PRINT SQR(-1) + a(9)
i = 7: j = 8
PRINT "a(i) + b(j):";: PRINT a(i) + b(j)
PRINT "done"
SYSTEM

h:
PRINT " handler"; ERR
RESUME NEXT
