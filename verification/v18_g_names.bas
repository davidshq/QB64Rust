$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): an array and a scalar of the same name; arrays with suffixes.
DIM a(3)
a = 5
a(1) = 2
PRINT "a; a(1):"; a; a(1)
DIM b%(3)
b%(1) = 7
PRINT "b%(1):"; b%(1)
DIM c(3) AS LONG
c(1) = 8
c&(2) = 9
PRINT "c(1); c&(2); c(2):"; c(1); c&(2); c(2)
DIM d$(2)
d$(1) = "dee"
PRINT "d$(1): "; d$(1)
DIM e(2) AS STRING
e$(1) = "ee"
PRINT "e(1): "; e(1)
PRINT "a(1) / 3:"; a(1) / 3
PRINT "c(1) / 3:"; c(1) / 3
SYSTEM
