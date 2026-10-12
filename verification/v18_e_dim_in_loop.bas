$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a DIM line of a static array run twice (in a loop), and run after use.
FOR i = 1 TO 2
    DIM a(5) AS LONG
    a(1) = a(1) + 1
    PRINT "pass"; i; a(1)
NEXT
SYSTEM
