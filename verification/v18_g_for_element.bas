$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): an array element as FOR variable.
DIM x(3) AS LONG
FOR x(1) = 1 TO 3
    PRINT x(1);
NEXT
PRINT
SYSTEM
