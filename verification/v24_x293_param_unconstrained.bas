$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): a parameter that nothing gives a number of dimensions, passed an array never declared.
ON ERROR GOTO h
show nx()
PRINT UBOUND(nx)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
SUB show (a())
    PRINT LBOUND(a); UBOUND(a);
END SUB
