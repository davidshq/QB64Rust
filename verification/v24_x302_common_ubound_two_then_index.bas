$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): COMMON c(), then UBOUND(c, 2), then c(1) = 5.
ON ERROR GOTO h
COMMON c()
PRINT UBOUND(c, 2);
c(1) = 5
PRINT c(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
