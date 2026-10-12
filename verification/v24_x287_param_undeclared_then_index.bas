$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): an array first named as an argument, then used with two indexes in the main module.
ON ERROR GOTO h
show nx()
nx(1, 2) = 5
PRINT nx(1, 2)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
SUB show (a())
    PRINT UBOUND(a);
END SUB
