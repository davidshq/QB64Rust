$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): below $DYNAMIC, the first statement that uses an array without
' DIM, run three times.
ON ERROR GOTO h
DIM i AS LONG
'$DYNAMIC
FOR i = 1 TO 3
    im(3) = im(3) + 1
NEXT
PRINT im(3); UBOUND(im)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
