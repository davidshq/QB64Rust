$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): under OPTION BASE 1, DIM a(0 + 0) below ON ERROR GOTO: is the
' error at program start trapped.
ON ERROR GOTO h
PRINT "first line"
OPTION BASE 1
DIM a(0 + 0)
PRINT "after DIM:"; LBOUND(a); UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
