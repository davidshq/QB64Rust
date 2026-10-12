$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): below $DYNAMIC, an array never declared passed to an array parameter.
ON ERROR GOTO h
DIM i AS LONG
'$DYNAMIC
FOR i = 1 TO 2
    fill nx()
    PRINT UBOUND(nx)
NEXT
REDIM nx(5)
PRINT UBOUND(nx)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
SUB fill (a())
    PRINT UBOUND(a);
    a(1) = 5
END SUB
