$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): a use c(1) above DIM c(3), beside DIM c(2) AS LONG in the main module.
ON ERROR GOTO h
DIM c(2) AS LONG
c(1) = 5
PRINT c(1);
DIM c(3)
c!(1) = 0.5
PRINT c(1); c!(1)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
