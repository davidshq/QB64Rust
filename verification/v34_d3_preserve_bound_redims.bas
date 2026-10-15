$CONSOLE:ONLY
' Verification (m3-redim-retain, task 4.1): REDIM _PRESERVE with a FUNCTION in a bound that gives the same array
' new bounds while the statement runs, as v34_d does with _RETAIN.
ON ERROR GOTO h
REDIM SHARED z(1 TO 3) AS LONG
z(2) = 2: z(3) = 3
REDIM _PRESERVE z(2 TO shrinkz) AS LONG
PRINT LBOUND(z); UBOUND(z); z(2); z(3)
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

FUNCTION shrinkz
    REDIM z(1 TO 2) AS LONG
    z(2) = 20
    shrinkz = 3
END FUNCTION
