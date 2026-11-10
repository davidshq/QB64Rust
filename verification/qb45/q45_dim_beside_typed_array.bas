' QB 4.5 (beside SOMEDAY's row on DIM c(3) without AS beside DIM c(2) AS DOUBLE, v48_x16): does the second DIM
' make another array, size the first, or stop the build? QB64pe makes a second array, reachable only as c!. The
' REDIM of a dynamic one is the second case. The compile messages show which lines are errors.
ON ERROR GOTO h
OPEN "OUT.TXT" FOR OUTPUT AS #1
DIM c(2) AS DOUBLE
DIM c(3)
c(1) = 1# / 3
PRINT #1, "1. DIM c(2) AS DOUBLE, then DIM c(3):"; c(1); UBOUND(c)
REDIM d(2) AS DOUBLE
REDIM d(3)
d(1) = 1# / 3
PRINT #1, "2. REDIM d(2) AS DOUBLE, then REDIM d(3):"; d(1); UBOUND(d)
CLOSE #1
SYSTEM
h:
PRINT #1, " error"; ERR;
RESUME NEXT
