$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): COMMON c(), then c(1) = 5, then DIM c(2, 5).
ON ERROR GOTO h
COMMON c()
c(1) = 5
DIM c(2, 5)
PRINT UBOUND(c, 2)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
