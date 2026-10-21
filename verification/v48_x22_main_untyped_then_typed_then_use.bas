$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): DIM c(2), then DIM c(3) AS DOUBLE, then DIM c!(1) is rejected or not; the plain c below.
ON ERROR GOTO h
DIM c(2)
c(1) = 0.5
DIM c(3) AS DOUBLE
c(1) = 1 / 3
PRINT c(1); c!(1); UBOUND(c); UBOUND(c!)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
