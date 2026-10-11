$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a dynamic array that is not allocated (after ERASE): LBOUND,
' UBOUND with and without a dimension, a read, a store, and the value each gives after its error.
ON ERROR GOTO h
DIM v AS LONG
REDIM a(2 TO 3) AS LONG
a(2) = 7
ERASE a
PRINT "LBOUND(a):";
v = 99: v = LBOUND(a): PRINT v
PRINT "UBOUND(a):";
v = 99: v = UBOUND(a): PRINT v
PRINT "UBOUND(a, 1):";
v = 99: v = UBOUND(a, 1): PRINT v
PRINT "UBOUND(a, 2):";
v = 99: v = UBOUND(a, 2): PRINT v
PRINT "read a(2):";
v = 99: v = a(2): PRINT v
PRINT "read a(0):";
v = 99: v = a(0): PRINT v
PRINT "store a(2) = 5:";
a(2) = 5
PRINT
PRINT "store a(0) = 5:";
a(0) = 5
PRINT
REDIM a(2 TO 3) AS LONG
PRINT "after REDIM again: a(2), LBOUND, UBOUND"; a(2); LBOUND(a); UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
