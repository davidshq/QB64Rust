$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): REDIM c(2) AS DOUBLE then REDIM c(3) in the main module: which array the plain c names.
ON ERROR GOTO h
REDIM c(2) AS DOUBLE
c(1) = 1 / 3
REDIM c(3)
c(1) = c(1) + 1
PRINT c(1); LEN(c(1)); UBOUND(c); UBOUND(c!); UBOUND(c#)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
