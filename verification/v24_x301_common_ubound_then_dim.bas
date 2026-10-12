$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): COMMON c(), then UBOUND(c), then DIM c(2, 5), then c(1, 2).
ON ERROR GOTO h
COMMON c()
PRINT UBOUND(c);
DIM c(2, 5)
c(1, 2) = 8
PRINT UBOUND(c, 2); c(1, 2)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
