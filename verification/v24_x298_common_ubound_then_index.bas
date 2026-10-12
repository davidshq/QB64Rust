$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): COMMON c(), then UBOUND(c), then c(1, 2).
ON ERROR GOTO h
COMMON c()
PRINT UBOUND(c);
c(1, 2) = 5
PRINT c(1, 2)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
